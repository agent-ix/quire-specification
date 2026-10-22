// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-322 "Record conformance" and event equality (TC-281 `EV-` and `RCF-`
//! vectors): the published catalog's operand positions and constraints are
//! read from `quire-verification-contracts`, and the decisions the catalog
//! does not carry (record conformance, the event equality exception, call
//! argument typing and slot typing) are the prose rules of FR-322 and FR-440
//! applied over a small selected document.

use ix_trace_rs::trace;
use quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1;
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// A model declaration of the selected document.
struct Declared {
    form: &'static str,
    supertypes: &'static [&'static str],
    /// Own fields: (name, type).
    fields: &'static [(&'static str, &'static str)],
}

fn document() -> BTreeMap<&'static str, Declared> {
    let record = |supertypes, fields| Declared {
        form: "record_value_type",
        supertypes,
        fields,
    };
    let event = |supertypes, fields| Declared {
        form: "event_type",
        supertypes,
        fields,
    };
    BTreeMap::from([
        ("Money", record(&[], &[("amount_minor", "Integer")])),
        (
            "TaxedMoney",
            record(&["Money"], &[("tax_minor", "Integer"), ("note", "Text")]),
        ),
        ("GiftTaxedMoney", record(&["TaxedMoney"], &[])),
        ("Other", record(&[], &[("amount_minor", "Integer")])),
        ("OrderPlaced", event(&[], &[("amount_minor", "Integer")])),
        ("ExpressOrderPlaced", event(&["OrderPlaced"], &[])),
        (
            "Order",
            Declared {
                form: "object_type",
                supertypes: &[],
                fields: &[],
            },
        ),
    ])
}

/// A type node of a lowered package.
#[derive(Clone, PartialEq, Debug)]
enum Ty {
    Model(&'static str),
    Option(Box<Ty>),
    Tuple(Vec<Ty>),
    Set(Box<Ty>),
    Sequence(Box<Ty>),
}

impl Ty {
    fn family(&self) -> &'static str {
        match self {
            Ty::Model(_) => "record",
            Ty::Option(_) => "option",
            Ty::Tuple(_) => "tuple",
            Ty::Set(_) => "set",
            Ty::Sequence(_) => "sequence",
        }
    }

    /// Whether an event type node occurs at any depth (event equality).
    fn contains_event(&self, doc: &BTreeMap<&str, Declared>) -> bool {
        match self {
            Ty::Model(name) => doc[name].form == "event_type",
            Ty::Option(inner) | Ty::Set(inner) | Ty::Sequence(inner) => inner.contains_event(doc),
            Ty::Tuple(parts) => parts.iter().any(|part| part.contains_event(doc)),
        }
    }
}

fn model(name: &'static str) -> Ty {
    Ty::Model(name)
}

fn ancestors<'a>(doc: &'a BTreeMap<&str, Declared>, name: &'a str) -> Vec<&'a str> {
    let mut found = vec![];
    let mut pending = vec![name];
    while let Some(next) = pending.pop() {
        for parent in doc[next].supertypes {
            if !found.contains(parent) {
                found.push(parent);
                pending.push(parent);
            }
        }
    }
    found
}

/// FR-322 "Record conformance": `a` conforms to `p`.
fn conforms(doc: &BTreeMap<&str, Declared>, a: &Ty, p: &Ty) -> bool {
    match (a, p) {
        (Ty::Model(a), Ty::Model(p)) => {
            a == p || (doc[a].form == doc[p].form && ancestors(doc, a).contains(p))
        }
        _ => false,
    }
}

/// A term type satisfies a slot type: equal, or a conforming record node.
fn slot_admits(doc: &BTreeMap<&str, Declared>, slot: &Ty, term: &Ty) -> bool {
    slot == term || conforms(doc, term, slot)
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Admitted,
    /// `ill_typed`/`operator-ineligible`.
    Ineligible,
    /// `ill_typed`/`type-mismatch` (FR-440).
    TypeMismatch,
    /// `missing_declaration`/`missing-selection`.
    MissingSelection,
}

fn catalog() -> Value {
    serde_json::from_str(CHECKED_OPERATION_CATALOG_V1).expect("the catalog is JSON")
}

fn entry(catalog: &Value, identity: &str) -> Value {
    catalog["operations"]
        .as_array()
        .expect("operations")
        .iter()
        .find(|entry| entry["identity"] == identity)
        .unwrap_or_else(|| panic!("{identity} is catalogued"))
        .clone()
}

fn constraint_kinds(entry: &Value) -> Vec<String> {
    entry["constraints"]
        .as_array()
        .expect("constraints")
        .iter()
        .map(|constraint| constraint["kind"].as_str().expect("kind").to_owned())
        .collect()
}

fn group_admits(catalog: &Value, position: &str, family: &str) -> bool {
    match catalog["groups"][position].as_array() {
        Some(members) => members.iter().any(|member| member == family),
        None => position == family,
    }
}

/// The owner join over a `model` node of `form` owned by `owner`.
fn owner_join(doc: &BTreeMap<&str, Declared>, form: &str, owner: &str) -> Outcome {
    match doc.get(owner) {
        Some(declared) if declared.form == form => Outcome::Admitted,
        _ => Outcome::MissingSelection,
    }
}

/// `quire.op.record.project` over `operand` with a `field` member.
fn project(
    catalog: &Value,
    doc: &BTreeMap<&str, Declared>,
    operand: &Ty,
    declaration: &'static str,
    name: &str,
) -> Outcome {
    let entry = entry(catalog, "quire.op.record.project");
    assert_eq!(constraint_kinds(&entry), ["member_of"]);
    let position = entry["operands"][0].as_str().expect("position");
    if !group_admits(catalog, position, operand.family()) {
        return Outcome::Ineligible;
    }
    if !conforms(doc, operand, &model(declaration)) {
        return Outcome::Ineligible;
    }
    let resolved = std::iter::once(declaration)
        .chain(ancestors(doc, declaration))
        .any(|owner| doc[owner].fields.iter().any(|(field, _)| *field == name));
    if resolved {
        Outcome::Admitted
    } else {
        Outcome::Ineligible
    }
}

/// `quire.op.structural.eq` or `.ne` over two operands, with event equality.
fn equality(
    catalog: &Value,
    doc: &BTreeMap<&str, Declared>,
    identity: &str,
    a: &Ty,
    b: &Ty,
) -> Outcome {
    let entry = entry(catalog, identity);
    assert_eq!(
        constraint_kinds(&entry),
        ["same_type"],
        "no event constraint"
    );
    let position = entry["operands"][0].as_str().expect("position");
    if !group_admits(catalog, position, a.family()) || a != b {
        return Outcome::Ineligible;
    }
    if a.contains_event(doc) {
        return Outcome::Ineligible;
    }
    Outcome::Admitted
}

/// The `value`/`parameter` nodes of the package: node key to its
/// `semantic_type`.
fn parameter_nodes() -> BTreeMap<&'static str, Ty> {
    BTreeMap::from([
        ("p-money", model("Money")),
        ("p-taxed", model("TaxedMoney")),
        ("p-seq-money", Ty::Sequence(Box::new(model("Money")))),
        ("p-opt-money", Ty::Option(Box::new(model("Money")))),
    ])
}

/// The `function` nodes of the package as wire bodies (QSL FR-092 "Function
/// nodes"): the first binding of the body is `parameters`, an `aggregate` of
/// `reference` terms to the parameter nodes in parameter order; a function
/// without parameters has no such binding.
fn function_bodies() -> BTreeMap<&'static str, Value> {
    let parameters = |targets: &[&str]| {
        json!({"term": "aggregate", "members": [{
            "term": "binding", "name": "parameters",
            "value": {"term": "aggregate", "members": targets.iter()
                .map(|target| json!({"term": "reference", "target": target}))
                .collect::<Vec<_>>()}
        }]})
    };
    BTreeMap::from([
        ("take_money", parameters(&["p-money"])),
        ("take_taxed", parameters(&["p-taxed"])),
        ("take_seq_money", parameters(&["p-seq-money"])),
        ("take_opt_money", parameters(&["p-opt-money"])),
        ("nullary", json!({"term": "aggregate", "members": []})),
    ])
}

/// The declared parameter types of a function node, read from its body's
/// `parameters` binding; a body without the binding takes none.
fn declared_parameters(body: &Value) -> Vec<Ty> {
    let nodes = parameter_nodes();
    body["members"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|member| member["name"] == "parameters")
        .map(|binding| {
            binding["value"]["members"]
                .as_array()
                .expect("parameters is an aggregate")
                .iter()
                .map(|reference| nodes[reference["target"].as_str().expect("target")].clone())
                .collect()
        })
        .unwrap_or_default()
}

/// The operations of the selected document: parameters by operation name.
fn operations() -> BTreeMap<&'static str, Vec<Ty>> {
    function_bodies()
        .into_iter()
        .map(|(name, body)| (name, declared_parameters(&body)))
        .collect()
}

/// A call to `callee`: the parameters come from the callee's `parameters`
/// member (`function.call`) or the selected document (`dispatch_call`); each
/// argument equals its parameter or conforms to a record one.
fn call(
    catalog: &Value,
    doc: &BTreeMap<&str, Declared>,
    identity: &str,
    callee: &str,
    args: &[Ty],
) -> Outcome {
    let entry = entry(catalog, identity);
    assert!(
        constraint_kinds(&entry).is_empty(),
        "{identity} carries no constraint"
    );
    let params = match identity {
        "quire.op.function.call" => operations(),
        "quire.op.model.dispatch_call" => operations(),
        other => panic!("{other} is not a call"),
    };
    let params = &params[callee];
    if params.len() != args.len() || !params.iter().zip(args).all(|(p, a)| slot_admits(doc, p, a)) {
        return Outcome::Ineligible;
    }
    Outcome::Admitted
}

fn contains(
    catalog: &Value,
    doc: &BTreeMap<&str, Declared>,
    collection: &Ty,
    element: &Ty,
) -> Outcome {
    let entry = entry(catalog, "quire.op.collection.contains");
    assert_eq!(constraint_kinds(&entry), ["inner_type"]);
    let (Ty::Set(inner) | Ty::Sequence(inner)) = collection else {
        return Outcome::Ineligible;
    };
    if slot_admits(doc, inner, element) {
        Outcome::Admitted
    } else {
        Outcome::Ineligible
    }
}

fn construct(
    catalog: &Value,
    doc: &BTreeMap<&str, Declared>,
    identity: &str,
    result: &Ty,
    rest: &[Ty],
) -> Outcome {
    let entry = entry(catalog, identity);
    assert_eq!(constraint_kinds(&entry), ["uniform_rest"]);
    let (Ty::Set(element) | Ty::Sequence(element)) = result else {
        return Outcome::Ineligible;
    };
    let uniform = rest
        .windows(2)
        .all(|pair| pair[0].family() == pair[1].family());
    if uniform && rest.iter().all(|term| slot_admits(doc, element, term)) {
        Outcome::Admitted
    } else {
        Outcome::Ineligible
    }
}

/// A function or expression result: a declared result `R` against the type node of the
/// body root or arm body that produces it.
/// A `case` arm body against the case result type (FR-440): the application's
/// `operator-ineligible` check.
fn arm_admits(doc: &BTreeMap<&str, Declared>, result: &Ty, arm: &Ty) -> Outcome {
    if slot_admits(doc, result, arm) {
        Outcome::Admitted
    } else {
        Outcome::Ineligible
    }
}

fn result_admits(doc: &BTreeMap<&str, Declared>, declared: &Ty, body: &Ty) -> Outcome {
    if slot_admits(doc, declared, body) {
        Outcome::Admitted
    } else {
        Outcome::TypeMismatch
    }
}

/// FR-440 "Union value": a payload term against the member's payload type.
fn union_value(doc: &BTreeMap<&str, Declared>, payload: &Ty, term: &Ty) -> Outcome {
    if slot_admits(doc, payload, term) {
        Outcome::Admitted
    } else {
        Outcome::TypeMismatch
    }
}

#[trace("TC-281", "FR-322-AC-68")]
#[test]
fn tc_281_event_type_joins_its_owner_projects_and_refuses_equality() {
    let (catalog, doc) = (catalog(), document());
    // EV-01, EV-02
    assert_eq!(
        owner_join(&doc, "event_type", "OrderPlaced"),
        Outcome::Admitted
    );
    for owner in ["Money", "Order", "Undeclared"] {
        assert_eq!(
            owner_join(&doc, "event_type", owner),
            Outcome::MissingSelection,
            "{owner}"
        );
    }
    // EV-03
    assert_eq!(
        project(
            &catalog,
            &doc,
            &model("OrderPlaced"),
            "OrderPlaced",
            "amount_minor"
        ),
        Outcome::Admitted
    );
    // EV-04, EV-05
    let order_placed = model("OrderPlaced");
    for identity in ["quire.op.structural.eq", "quire.op.structural.ne"] {
        for ty in [
            order_placed.clone(),
            Ty::Option(Box::new(order_placed.clone())),
            Ty::Tuple(vec![model("Money"), order_placed.clone()]),
            Ty::Set(Box::new(order_placed.clone())),
        ] {
            assert_eq!(
                equality(&catalog, &doc, identity, &ty, &ty),
                Outcome::Ineligible,
                "{identity} over {ty:?}"
            );
        }
        // EV-06
        for ty in [model("Money"), Ty::Option(Box::new(model("Money")))] {
            assert_eq!(
                equality(&catalog, &doc, identity, &ty, &ty),
                Outcome::Admitted,
                "{identity} over {ty:?}"
            );
        }
    }
}

#[trace("TC-281", "FR-322-AC-69")]
#[trace("TC-281", "FR-322-AC-70")]
#[test]
fn tc_281_record_conformance_decides_projection_calls_and_slots() {
    let (catalog, doc) = (catalog(), document());
    let (money, taxed, gift, other) = (
        model("Money"),
        model("TaxedMoney"),
        model("GiftTaxedMoney"),
        model("Other"),
    );
    // RCF-01, RCF-02
    for operand in [&money, &taxed, &gift] {
        assert_eq!(
            project(&catalog, &doc, operand, "Money", "amount_minor"),
            Outcome::Admitted
        );
    }
    for (operand, declaration) in [
        (&taxed, "GiftTaxedMoney"),
        (&money, "TaxedMoney"),
        (&money, "Other"),
    ] {
        assert_eq!(
            project(&catalog, &doc, operand, declaration, "amount_minor"),
            Outcome::Ineligible
        );
    }
    // RCF-03 to RCF-05, for both call forms
    for identity in ["quire.op.function.call", "quire.op.model.dispatch_call"] {
        let call_of = |callee: &str, args: &[Ty]| call(&catalog, &doc, identity, callee, args);
        assert_eq!(
            call_of("take_money", std::slice::from_ref(&taxed)),
            Outcome::Admitted
        );
        assert_eq!(
            call_of("take_money", std::slice::from_ref(&gift)),
            Outcome::Admitted
        );
        assert_eq!(
            call_of("take_taxed", std::slice::from_ref(&money)),
            Outcome::Ineligible
        );
        assert_eq!(
            call_of("take_money", std::slice::from_ref(&other)),
            Outcome::Ineligible
        );
        assert_eq!(
            call_of("take_money", &[taxed.clone(), taxed.clone()]),
            Outcome::Ineligible
        );
        assert_eq!(
            call_of("nullary", std::slice::from_ref(&money)),
            Outcome::Ineligible
        );
        assert_eq!(
            call_of("take_seq_money", &[Ty::Sequence(Box::new(taxed.clone()))]),
            Outcome::Ineligible
        );
        assert_eq!(
            call_of("take_opt_money", &[Ty::Option(Box::new(taxed.clone()))]),
            Outcome::Ineligible
        );
    }
    // RCF-07
    let set_money = Ty::Set(Box::new(money.clone()));
    let set_taxed = Ty::Set(Box::new(taxed.clone()));
    assert_eq!(
        contains(&catalog, &doc, &set_money, &taxed),
        Outcome::Admitted
    );
    assert_eq!(
        contains(&catalog, &doc, &set_taxed, &money),
        Outcome::Ineligible
    );
    // RCF-08
    for identity in ["quire.op.collection.set", "quire.op.collection.sequence"] {
        let result = if identity.ends_with("set") {
            set_money.clone()
        } else {
            Ty::Sequence(Box::new(money.clone()))
        };
        assert_eq!(
            construct(
                &catalog,
                &doc,
                identity,
                &result,
                &[money.clone(), taxed.clone()]
            ),
            Outcome::Admitted
        );
    }
    assert_eq!(
        construct(
            &catalog,
            &doc,
            "quire.op.collection.set",
            &set_taxed,
            std::slice::from_ref(&money)
        ),
        Outcome::Ineligible
    );
    // RCF-13
    let voucher = model("Other");
    assert_eq!(result_admits(&doc, &money, &voucher), Outcome::TypeMismatch);
    // RCF-11, RCF-12
    assert_eq!(result_admits(&doc, &money, &taxed), Outcome::Admitted);
    assert_eq!(result_admits(&doc, &taxed, &money), Outcome::TypeMismatch);
    assert_eq!(result_admits(&doc, &money, &money), Outcome::Admitted);
    assert_eq!(arm_admits(&doc, &money, &other), Outcome::Ineligible);
    assert_eq!(arm_admits(&doc, &money, &taxed), Outcome::Admitted);
    // RCF-09
    assert_eq!(union_value(&doc, &money, &taxed), Outcome::Admitted);
    assert_eq!(union_value(&doc, &taxed, &money), Outcome::TypeMismatch);
    // RCF-10
    for identity in ["quire.op.structural.eq", "quire.op.structural.ne"] {
        assert_eq!(
            equality(&catalog, &doc, identity, &money, &money),
            Outcome::Admitted
        );
        let entry = entry(&catalog, identity);
        assert_eq!(
            entry["leaves"], "operand:0",
            "the leaves are operand 0's walk"
        );
    }
}

#[trace("TC-281", "FR-322-AC-31")]
#[test]
fn tc_281_an_operation_call_takes_a_conforming_record_argument() {
    let (catalog, doc) = (catalog(), document());
    for (callee, arg, expected) in [
        ("take_money", "TaxedMoney", Outcome::Admitted),
        ("take_taxed", "Money", Outcome::Ineligible),
        ("take_money", "Other", Outcome::Ineligible),
    ] {
        assert_eq!(
            call(
                &catalog,
                &doc,
                "quire.op.model.dispatch_call",
                callee,
                &[model(arg)]
            ),
            expected
        );
    }
}
