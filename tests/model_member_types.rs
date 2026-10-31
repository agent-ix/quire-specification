// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-322 "Model-owned members" vector qualification (TC-280): owner recovery
//! by the structural ids of model declaration nodes, member resolution in the
//! declaring type's exposed effective members and member type derivation.
//! TC-281 applies the published operation catalog's `quire.op.reference.eq`,
//! `quire.op.reference.ne`, `quire.op.model.deref` and
//! `quire.op.record.project` entries over the same recovered owners:
//! reference equality over conforming object types and a model field
//! projection over a `deref` result.

use ix_trace_rs::trace;
use quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-member-type-vectors.json");
/// The catalog lives only in `quire-verification-contracts`; it is read from there.
const OPERATION_CATALOG: &str = CHECKED_OPERATION_CATALOG_V1;
/// The selected domain package document: an FCD semantic IR 2.0.0 document.
/// `domain_package` in the vectors is the projection of its declarations that
/// FR-154 intake reads, derived from it by `project_document`.
const DOCUMENT: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-orders.json");
/// The owner-form vectors: a projection that also declares processes, state
/// machines and a persistence interface, with the owner, member, anchor,
/// clause-signature and receiver-frame cases over them.
const OWNER_FORMS: &str =
    include_str!("../proposals/checked-package-v2/model-owner-form-vectors.json");
const ADVERSE: &str = include_str!("../proposals/checked-package-v2/fixtures/adverse.json");
const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");

fn parsed(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn text<'a>(value: &'a Value, member: &str) -> &'a str {
    value[member]
        .as_str()
        .unwrap_or_else(|| panic!("authored member {member} is a string"))
}

fn array<'a>(value: &'a Value, member: &str) -> &'a [Value] {
    value[member]
        .as_array()
        .unwrap_or_else(|| panic!("authored member {member} is an array"))
}

fn refused(code: &str, cause: &str) -> Value {
    json!({"refused": {"code": code, "cause": cause}})
}

/// A refusal with the pointer it is reported at, relative to the application
/// (`member.declaration`, `member.name`, `arguments/0`, `result_type`).
fn refused_at(code: &str, cause: &str, at: &str) -> Value {
    json!({"refused": {"code": code, "cause": cause, "at": at}})
}

fn ineligible() -> Value {
    refused("ill_typed", "operator-ineligible")
}

/// A declaration of the selected package with its node tag and form.
struct Declared<'a> {
    tag: &'static str,
    form: &'static str,
    node: &'a str,
    object_type: Option<&'a Value>,
}

fn declarations(package: &Value) -> Vec<Declared<'_>> {
    let objects = array(package, "object_types").iter().map(|object_type| {
        let form = if object_type["interfaceFeatures"].is_null() {
            "object_type"
        } else {
            "systems_interface"
        };
        Declared {
            tag: "model",
            form,
            node: text(object_type, "node"),
            object_type: Some(object_type),
        }
    });
    let relationships = array(package, "relationships")
        .iter()
        .map(|relationship| Declared {
            tag: "relation",
            form: "relationship",
            node: text(relationship, "node"),
            object_type: None,
        });
    let owned = [
        ("processes", "process"),
        ("state_machines", "state_machine"),
        ("persistence_interfaces", "persistence_interface"),
    ]
    .into_iter()
    .flat_map(|(list, form)| {
        array_or_empty(package, list)
            .iter()
            .map(move |declared| Declared {
                tag: "model",
                form,
                node: text(declared, "node"),
                object_type: Some(declared),
            })
    });
    let populations = array_or_empty(package, "populations")
        .iter()
        .map(|population| Declared {
            tag: "relation",
            form: "population",
            node: text(population, "identity"),
            object_type: None,
        });
    objects
        .chain(relationships)
        .chain(owned)
        .chain(populations)
        .collect()
}

/// The structural node id of a model declaration: the owning package, then
/// the node tag, form and declared node, at ordinal zero.
fn declaration_key(package: &Value, declared: &Declared<'_>) -> Value {
    json!({
        "package": package["identity"],
        "path": [declared.tag, declared.form, declared.node],
        "ordinal": 0
    })
}

/// FR-322 step 2 over a wire node's key, tag and form: the matching
/// declaration of the selected package.
fn recover<'a>(package: &'a Value, key: &Value, tag: &str, form: &str) -> Option<Declared<'a>> {
    declarations(package).into_iter().find(|declared| {
        declared.tag == tag && declared.form == form && declaration_key(package, declared) == *key
    })
}

/// The declaration of an object type, process, state machine or persistence
/// interface with this node identity: the kinds that declare fields,
/// operations and supertypes.
fn object_type<'a>(package: &'a Value, node: &str) -> Option<&'a Value> {
    [
        "object_types",
        "processes",
        "state_machines",
        "persistence_interfaces",
    ]
    .into_iter()
    .flat_map(|list| array_or_empty(package, list))
    .find(|object_type| text(object_type, "node") == node)
}

/// The proper ancestors of `declared` along its declared supertypes.
fn ancestors(package: &Value, declared: &Value) -> BTreeSet<String> {
    let mut reached = BTreeSet::new();
    let mut pending: Vec<&Value> = array(declared, "supertypes").iter().collect();
    while let Some(supertype) = pending.pop() {
        let node = supertype.as_str().expect("a supertype is a node identity");
        if reached.insert(node.to_owned()) {
            let declared = object_type(package, node).expect("a supertype is declared");
            pending.extend(array(declared, "supertypes"));
        }
    }
    reached
}

/// FR-322 step 3: the exposed effective members of `declared` of one member
/// kind, own and inherited, less every member a redefinition hides and every
/// redefining member of a less derived owner (model-complete phase 4).
fn exposed<'a>(package: &'a Value, declared: &'a Value, list: &str) -> Vec<(String, &'a Value)> {
    let mut owners: Vec<&Value> = vec![declared];
    owners.extend(
        ancestors(package, declared)
            .iter()
            .map(|node| object_type(package, node).expect("an ancestor is declared")),
    );
    let mut members: BTreeMap<String, &Value> = BTreeMap::new();
    let mut hidden = BTreeSet::new();
    // (redefined target, redefining member identity, redefining owner)
    let mut redefiners: Vec<(String, String, &Value)> = Vec::new();
    for owner in &owners {
        let node = text(owner, "node");
        for kind in ["fields", "operations"] {
            for member in array(owner, kind) {
                let identity = format!("{node}/{}", text(member, "name"));
                if let Some(target) = member["redefines"].as_str() {
                    hidden.insert(target.to_owned());
                    redefiners.push((target.to_owned(), identity.clone(), owner));
                }
                if kind == list {
                    members.insert(identity, member);
                }
            }
        }
    }
    for (target, identity, owner) in &redefiners {
        let less_derived = redefiners.iter().any(|(other_target, _, other_owner)| {
            other_target == target && ancestors(package, other_owner).contains(text(owner, "node"))
        });
        if less_derived {
            hidden.insert(identity.clone());
        }
    }
    members
        .into_iter()
        .filter(|(identity, _)| !hidden.contains(identity))
        .collect()
}

/// FR-141's six text profiles, the only profiles a `Text[min, max; profile]`
/// member type names.
const TEXT_PROFILES: [&str; 6] = [
    "unicode-scalars",
    "nfc",
    "nfd",
    "nfkc",
    "nfkd",
    "binary-utf8",
];

/// FR-322 step 4's element type table. A reference target is the model
/// declaration node key recomputed from the declaration.
fn element_type(package: &Value, type_ref: &str) -> Option<Value> {
    match type_ref {
        "ix://quire/native/Boolean" => return Some(json!({"type": "Boolean"})),
        "ix://quire/native/Integer" => return Some(json!({"type": "Integer"})),
        _ => {}
    }
    if let Some(value_type) = array(package, "value_types")
        .iter()
        .find(|value_type| text(value_type, "node") == type_ref)
    {
        let bound = |member: &str| text(value_type, member).parse::<i128>().ok();
        let ordered = bound("lower")? <= bound("upper")?;
        return match text(value_type, "native") {
            "ix://quire/native/Integer" => ordered.then(
                || json!({"type": "Int", "min": value_type["lower"], "max": value_type["upper"]}),
            ),
            "ix://quire/native/Text" => {
                (ordered && TEXT_PROFILES.contains(&text(value_type, "profile"))).then(|| {
                    json!({
                        "type": "Text",
                        "min": value_type["lower"],
                        "max": value_type["upper"],
                        "profile": value_type["profile"]
                    })
                })
            }
            _ => None,
        };
    }
    if let Some(name) = type_ref.strip_prefix("ix://quire/unit/") {
        return unit_element_type(package, name);
    }
    let target = declarations(package)
        .into_iter()
        .find(|declared| declared.form == "object_type" && declared.node == type_ref)?;
    Some(json!({"type": "Reference", "target": declaration_key(package, &target)}))
}

/// FR-322 step 4's unit row: a `typeRef` of the form `ix://quire/unit/<name>`
/// names the `scalar_type`/`unit` node of the checked package whose qualified
/// declaration name is the `/`-joined `<name>`. The vectors list the checked
/// package's units as `checked_units`, which the test carries on the package
/// value it passes down. A name that matches no unit is
/// `missing_declaration`/`missing-name`, so it has no element type here.
fn unit_element_type(package: &Value, name: &str) -> Option<Value> {
    array(package, "checked_units")
        .iter()
        .any(|unit| {
            unit.as_array().is_some_and(|segments| {
                segments.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("/") == name
            })
        })
        .then(|| json!({"type": "Quantity", "unit": name}))
}

/// The refusal for a declared type whose unit `typeRef` names no unit of the
/// checked package, or `None` for any other declared type.
fn unresolved_unit(package: &Value, declared: &Value) -> Option<Value> {
    let name = text(declared, "typeRef").strip_prefix("ix://quire/unit/")?;
    unit_element_type(package, name)
        .is_none()
        .then(|| refused("missing_declaration", "missing-name"))
}

/// FR-322 step 4's multiplicity rule.
fn with_multiplicity(element: Value, multiplicity: &Value) -> Option<Value> {
    let lower = text(multiplicity, "lower");
    let upper = text(multiplicity, "upper");
    if (lower, upper) == ("1", "1") {
        return Some(element);
    }
    let kind = match (
        multiplicity["ordered"].as_bool(),
        multiplicity["unique"].as_bool(),
    ) {
        (Some(false), Some(true)) => "Set",
        (Some(false), Some(false)) => "Bag",
        (Some(true), Some(false)) => "Sequence",
        (Some(true), Some(true)) => "OrderedSet",
        _ => return None,
    };
    match (lower, upper) {
        ("0", "unbounded") => Some(json!({"type": kind, "element": element})),
        (_, "unbounded") => None,
        _ => Some(json!({"type": kind, "min": lower, "max": upper, "element": element})),
    }
}

/// FR-322 step 4's tables over one declared `typeRef` and multiplicity.
fn declared_type_of(package: &Value, declared: &Value) -> Option<Value> {
    element_type(package, text(declared, "typeRef"))
        .and_then(|element| with_multiplicity(element, &declared["multiplicity"]))
}

/// FR-322 step 3 for a `relationship_end` member: the named ends of the
/// recovered relationship are its `sourceEnd.role` and, when the document
/// declares one, its `targetEnd.role`. A role the relationship does not name
/// refuses `missing_declaration`/`missing-name`; no name is synthesized. A
/// named end traversed against the relationship's direction refuses as
/// FR-152 says.
fn relationship_end(package: &Value, recovered: &Declared<'_>, member: &Value) -> Value {
    if recovered.form != "relationship" {
        return refused_at("ill_typed", "operator-ineligible", "member.declaration");
    }
    let Some(declared) = array(package, "relationships")
        .iter()
        .find(|relationship| text(relationship, "node") == recovered.node)
    else {
        return refused_at("missing_declaration", "missing-name", "member.name");
    };
    let named: Vec<&Value> = array(declared, "ends")
        .iter()
        .filter(|end| end["role"] == member["name"])
        .collect();
    let end = match named.as_slice() {
        [] => return refused_at("missing_declaration", "missing-name", "member.name"),
        [end] => end,
        _ => return refused_at("ambiguous_declaration", "ambiguous-name", "member.name"),
    };
    // FR-152: traversal from source to target requires `source-to-target`,
    // `bidirectional` or `undirected`, and from target to source
    // `target-to-source`, `bidirectional` or `undirected`.
    let allowed = match text(end, "end") {
        "source" => "source-to-target",
        _ => "target-to-source",
    };
    if ![allowed, "bidirectional", "undirected"].contains(&text(declared, "direction")) {
        return refused_at("ill_typed", "operator-ineligible", "member.name");
    }
    json!({"resolves_end": {"relationship": recovered.node, "role": member["name"]}})
}

/// FR-322 steps 2 to 4 for one member case, as its expectation spells them.
fn derive(package: &Value, nodes: &[Value], member: &Value) -> Value {
    let declaring = nodes
        .iter()
        .find(|vector| vector["name"] == member["declaration"])
        .expect("a case names a published node vector");
    let Some(recovered) = recover(
        package,
        &declaring["node_id"],
        text(declaring, "node_tag"),
        text(declaring, "semantic_form"),
    ) else {
        return refused("missing_declaration", "missing-selection");
    };
    if text(member, "kind") == "relationship_end" {
        return relationship_end(package, &recovered, member);
    }
    let Some(declared) = recovered.object_type else {
        return ineligible();
    };
    let (list, kind) = match text(member, "kind") {
        "field" => ("fields", "field"),
        "operation" => ("operations", "operation"),
        other => panic!("unexpected member kind {other}"),
    };
    let matches: Vec<_> = exposed(package, declared, list)
        .into_iter()
        .filter(|(_, record)| record["name"] == member["name"])
        .collect();
    let (identity, record) = match matches.as_slice() {
        [] => return ineligible(),
        [one] => one,
        _ => return refused("ambiguous_declaration", "ambiguous-name"),
    };
    let declared_type = if kind == "field" {
        record
    } else {
        if recovered.form != "object_type" {
            return ineligible();
        }
        let receiver = json!({"type": "Reference", "target": declaring["node_id"]});
        if member["receiver"] != receiver {
            return ineligible();
        }
        if let Some(missing) = array(record, "parameters")
            .iter()
            .find_map(|parameter| unresolved_unit(package, parameter))
        {
            return missing;
        }
        let parameters: Option<Vec<Value>> = array(record, "parameters")
            .iter()
            .map(|parameter| declared_type_of(package, parameter))
            .collect();
        if parameters.as_deref() != Some(array(member, "arguments")) {
            return ineligible();
        }
        if record["result"].is_null() {
            return ineligible();
        }
        &record["result"]
    };
    if let Some(missing) = unresolved_unit(package, declared_type) {
        return missing;
    }
    let member_type = declared_type_of(package, declared_type).map(|value| {
        if record["presence"] == "optional" {
            json!({"type": "Option", "inner": value})
        } else {
            value
        }
    });
    match member_type {
        Some(member_type) => json!({"resolves": identity, "member_type": member_type}),
        None => ineligible(),
    }
}

/// Whether the selected document declares the owner's `node` as the kind the
/// wire node stands for: an object type, an interface or a relationship for a
/// model declaration node; an operation member or an object type for a
/// clause function.
fn owner_declares(package: &Value, node: &Value, owner_node: &str) -> bool {
    let object_types = array(package, "object_types");
    let object_type = |interface: bool| {
        object_types.iter().any(|object| {
            object["node"] == owner_node && object["interfaceFeatures"].is_null() != interface
        })
    };
    match (text(node, "node_tag"), text(node, "semantic_form")) {
        ("model", "object_type") => object_type(false),
        ("model", "systems_interface") => object_type(true),
        ("model", "process") => declares_in(package, "processes", owner_node),
        ("model", "state_machine") => declares_in(package, "state_machines", owner_node),
        ("model", "persistence_interface") => {
            declares_in(package, "persistence_interfaces", owner_node)
        }
        ("relation", "relationship") => array(package, "relationships")
            .iter()
            .any(|relationship| relationship["node"] == owner_node),
        // A population is declared by its raw `PopulationRecord`, keyed by
        // `identity`; the reader reads no `node` member of it (FR-322 owner join).
        ("relation", "population") => array_or_empty(package, "populations")
            .iter()
            .any(|population| population["identity"] == owner_node),
        ("function", _) => match clause_kind(node) {
            // An invariant clause is owned by its object type; a
            // precondition or body clause by an operation member of an
            // object type, the operation's context.
            Some("invariant") => object_types.iter().any(|object| {
                object["node"] == owner_node && object["interfaceFeatures"].is_null()
            }),
            Some("precondition" | "body") => object_types.iter().any(|object| {
                let declaring = text(object, "node");
                array(object, "operations").iter().any(|operation| {
                    format!("{declaring}/{}", text(operation, "name")) == owner_node
                })
            }),
            _ => false,
        },
        _ => false,
    }
}

/// Whether the projection's `list` declares the node identity `owner_node`.
fn declares_in(package: &Value, list: &str, owner_node: &str) -> bool {
    array_or_empty(package, list)
        .iter()
        .any(|declared| declared["node"] == owner_node)
}

/// A clause function's clause kind: the text literal of the one `clause`
/// binding of its body (QSL FR-094): `invariant`, `precondition` or `body`.
fn clause_kind(node: &Value) -> Option<&str> {
    let mut bindings = node["body"]["members"]
        .as_array()?
        .iter()
        .filter(|member| member["term"] == "binding" && member["name"] == "clause");
    let binding = bindings.next().filter(|_| bindings.next().is_none())?;
    (binding["value"]["term"] == "literal" && binding["value"]["value_kind"] == "text")
        .then(|| binding["value"]["value"].as_str())
        .flatten()
}

/// FR-322's owner presence rule (the V2 schema), the owner join and step 2
/// over one wire node.
fn read_wire(package: &Value, node: &Value) -> Value {
    let schema = parsed(SCHEMA);
    let validator = jsonschema::validator_for(&json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": "#/$defs/SemanticNode",
        "$defs": schema["$defs"]
    }))
    .expect("SemanticNode is a valid schema definition");
    if !validator.is_valid(node) {
        return json!({"refused": {"code": "malformed_wire"}});
    }
    let owner = &node["owner"];
    if owner["identity"] != package["identity"]
        || !owner_declares(package, node, text(owner, "node"))
    {
        return refused("missing_declaration", "missing-selection");
    }
    if text(node, "node_tag") != "function" {
        json!({"recovers": owner["node"]})
    } else {
        json!({"passes": "owner-join"})
    }
}

/// The member cases recorded for exactly `criterion`.
fn cases_for<'a>(vectors: &'a Value, criterion: &str) -> Vec<&'a Value> {
    let cases: Vec<_> = array(vectors, "member_cases")
        .iter()
        .filter(|case| text(case, "criterion") == criterion)
        .collect();
    assert!(!cases.is_empty(), "{criterion} has member cases");
    cases
}

/// The domain package value with the checked package's units attached, the
/// context FR-322 step 4's unit row resolves a unit `typeRef` against. The
/// unit-typed members live in the `acme/timing` package, which no checked
/// package fixture selects.
fn with_checked_units(vectors: &Value, case: &Value) -> (Value, Value) {
    if case["package"] != "timing" {
        return (
            vectors["domain_package"].clone(),
            vectors["model_declaration_nodes"].clone(),
        );
    }
    let mut package = vectors["timing_package"].clone();
    package["checked_units"] = vectors["checked_units"].clone();
    (package, vectors["timing_declaration_nodes"].clone())
}

fn assert_member_cases(vectors: &Value, criterion: &str) {
    for case in cases_for(vectors, criterion) {
        let (package, nodes) = with_checked_units(vectors, case);
        assert_eq!(
            derive(&package, nodes.as_array().unwrap(), &case["member"]),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

#[trace("TC-280", "FR-322-AC-29")]
#[test]
fn tc_280_model_declaration_nodes_recover_only_selected_owners() {
    let vectors = parsed(VECTORS);
    let package = &vectors["domain_package"];
    for vector in array(&vectors, "model_declaration_nodes") {
        let name = text(vector, "name");
        let selected = vector["owner"]["identity"] == package["identity"];
        let recovered = recover(
            package,
            &vector["node_id"],
            text(vector, "node_tag"),
            text(vector, "semantic_form"),
        );
        assert_eq!(
            recovered.is_some(),
            selected,
            "{name} recovers exactly when its owner is the selected domain package"
        );
    }

    for case in array(&vectors, "wire_cases") {
        assert_eq!(
            read_wire(package, &case["node"]),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
    assert_member_cases(&vectors, "FR-322-AC-29");
}

#[trace("TC-280", "FR-322-AC-30")]
#[test]
fn tc_280_member_names_resolve_in_exposed_effective_members() {
    assert_member_cases(&parsed(VECTORS), "FR-322-AC-30");
}

#[trace("TC-280", "FR-322-AC-31")]
#[test]
fn tc_280_member_types_follow_declared_type_multiplicity_and_presence() {
    assert_member_cases(&parsed(VECTORS), "FR-322-AC-31");
}

/// FR-322-AC-183: a field, an operation parameter or an operation result typed
/// `Rational`, `Decimal` or bare `Text`, or typed by a package value type bound
/// as `Decimal[...]` or `Rational`, is declared by the package and gives the
/// member no type. Each member of `Order` is retyped in turn; the control
/// types it `Boolean`, which resolves, so the refusal is the declared type's
/// and not a missing member.
#[trace("TC-280", "FR-322-AC-183")]
#[test]
fn tc_280_rational_decimal_and_bare_text_members_have_no_type() {
    let vectors = parsed(VECTORS);
    let nodes = array(&vectors, "model_declaration_nodes");
    let cases = array(&vectors, "member_cases");
    let operation_case = |name: &str| {
        cases
            .iter()
            .find(|case| {
                case["member"]["declaration"] == "M1"
                    && case["member"]["kind"] == "operation"
                    && case["member"]["name"] == name
            })
            .unwrap_or_else(|| panic!("a published M1 operation case for {name}"))["member"]
            .clone()
    };
    let field = json!({"kind": "field", "declaration": "M1", "name": "label"});
    let size = operation_case("size");
    let scaled = operation_case("scaled");
    // `typeRef` of the field `label`, the result of `size` and the parameter
    // `n` of `scaled` of `Order`, set to `type_ref`; `argument` replaces
    // the argument list the parameter case applies.
    let retyped = |member: &Value, type_ref: &str, argument: Option<Value>| {
        let mut package = vectors["domain_package"].clone();
        package["value_types"].as_array_mut().expect("value types").extend([
            json!({"node": "ix://acme/orders/Dec", "native": "ix://quire/native/Decimal", "lower": "0", "upper": "9"}),
            json!({"node": "ix://acme/orders/Rat", "native": "ix://quire/native/Rational", "lower": "0", "upper": "9"}),
        ]);
        let order = package["object_types"]
            .as_array_mut()
            .expect("object types")
            .iter_mut()
            .find(|candidate| text(candidate, "node") == "ix://acme/orders/Order")
            .expect("Order");
        let slot = match text(member, "kind") {
            "field" => order["fields"]
                .as_array_mut()
                .expect("fields")
                .iter_mut()
                .find(|candidate| text(candidate, "name") == "label")
                .expect("label"),
            _ if member["name"] == "size" => order["operations"]
                .as_array_mut()
                .expect("operations")
                .iter_mut()
                .find(|candidate| candidate["name"] == "size")
                .map(|operation| &mut operation["result"])
                .expect("size result"),
            _ => order["operations"]
                .as_array_mut()
                .expect("operations")
                .iter_mut()
                .find(|candidate| candidate["name"] == "scaled")
                .map(|operation| &mut operation["parameters"][0])
                .expect("scaled parameter"),
        };
        slot["typeRef"] = json!(type_ref);
        let mut member = member.clone();
        if let Some(argument) = argument {
            member["arguments"] = json!([argument]);
        }
        derive(&package, nodes, &member)
    };
    let refused = json!({"refused": {"code": "ill_typed", "cause": "operator-ineligible"}});
    for native in [
        "ix://quire/native/Rational",
        "ix://quire/native/Decimal",
        "ix://quire/native/Text",
        "ix://acme/orders/Dec",
        "ix://acme/orders/Rat",
    ] {
        assert_eq!(retyped(&field, native, None), refused, "field {native}");
        assert_eq!(retyped(&size, native, None), refused, "result {native}");
        // The parameter's declared type has no type, so no argument matches it.
        assert_eq!(
            retyped(&scaled, native, Some(json!({"type": "Integer"}))),
            refused,
            "parameter {native}"
        );
    }
    assert_eq!(
        retyped(&field, "ix://quire/native/Boolean", None),
        json!({"resolves": "ix://acme/orders/Order/label", "member_type": {"type": "Boolean"}}),
        "the field control resolves"
    );
    assert_eq!(
        retyped(&size, "ix://quire/native/Boolean", None),
        json!({"resolves": "ix://acme/orders/Order/size", "member_type": {"type": "Boolean"}}),
        "the result control resolves"
    );
    assert_eq!(
        retyped(&scaled, "ix://quire/native/Boolean", Some(json!({"type": "Boolean"}))),
        json!({"resolves": "ix://acme/orders/Order/scaled", "member_type": {"type": "Integer"}}),
        "the parameter control resolves"
    );
}

#[trace("TC-280", "FR-322-AC-80")]
#[test]
fn tc_280_unit_type_references_resolve_against_the_checked_packages_units() {
    assert_member_cases(&parsed(VECTORS), "FR-322-AC-80");
}

#[trace("TC-280", "FR-322-AC-44")]
#[test]
fn tc_280_text_value_type_members_carry_their_bounds_and_profile() {
    assert_member_cases(&parsed(VECTORS), "FR-322-AC-44");
}

/// A catalog entry by identity.
fn catalog_entry<'a>(catalog: &'a Value, identity: &str) -> &'a Value {
    array(catalog, "operations")
        .iter()
        .find(|entry| entry["identity"] == identity)
        .unwrap_or_else(|| panic!("{identity} is catalogued"))
}

/// The families a catalog operand position admits: a group's members or the
/// family itself.
fn admitted_families<'a>(catalog: &'a Value, position: &'a str) -> Vec<&'a str> {
    match catalog["groups"][position].as_array() {
        Some(members) => members.iter().filter_map(Value::as_str).collect(),
        None => vec![position],
    }
}

/// The published node vector named `name`.
fn node_vector<'a>(nodes: &'a [Value], name: &str) -> &'a Value {
    nodes
        .iter()
        .find(|vector| vector["name"] == name)
        .unwrap_or_else(|| panic!("{name} is a published node vector"))
}

/// A typed operand of a TC-281 case: `{reference: M}` is the
/// `composite_type`/`reference` node over model declaration node `M`, and
/// `{object: M}` is `M` itself.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Operand<'a> {
    Reference(&'a str),
    Object(&'a str),
    Record(&'a Value),
}

impl<'a> Operand<'a> {
    fn from_case(value: &'a Value) -> Self {
        match (
            value["reference"].as_str(),
            value["object"].as_str(),
            value.get("record"),
        ) {
            (Some(node), None, None) => Self::Reference(node),
            (None, Some(node), None) => Self::Object(node),
            (None, None, Some(record)) => Self::Record(record),
            _ => panic!("an operand names exactly one reference, object or record"),
        }
    }

    /// FR-322's family of the operand's type node: `reference` for a
    /// reference, `object` for an object type or systems interface model
    /// declaration node, and none for a relation node.
    fn family(self, nodes: &[Value]) -> Option<&'static str> {
        match self {
            Self::Reference(_) => Some("reference"),
            Self::Record(_) => Some("record"),
            Self::Object(name) => {
                let vector = node_vector(nodes, name);
                match (text(vector, "node_tag"), text(vector, "semantic_form")) {
                    (
                        "model",
                        "object_type" | "process" | "state_machine" | "systems_interface",
                    ) => Some("object"),
                    _ => None,
                }
            }
        }
    }
}

/// FR-322 step 2 over a named node vector.
fn recover_vector<'a>(package: &'a Value, nodes: &[Value], name: &str) -> Option<Declared<'a>> {
    let vector = node_vector(nodes, name);
    recover(
        package,
        &vector["node_id"],
        text(vector, "node_tag"),
        text(vector, "semantic_form"),
    )
}

/// FR-322 "Reference conformance" over two reference operands: both owners
/// are recovered by step 2, even for one type node.
fn conforming_reference(package: &Value, nodes: &[Value], left: &str, right: &str) -> Value {
    let (Some(a), Some(b)) = (
        recover_vector(package, nodes, left),
        recover_vector(package, nodes, right),
    ) else {
        return refused("missing_declaration", "missing-selection");
    };
    let (Some(a_type), Some(b_type)) = (a.object_type, b.object_type) else {
        return ineligible();
    };
    let conforms = a.node == b.node
        || ancestors(package, a_type).contains(b.node)
        || ancestors(package, b_type).contains(a.node);
    if conforms {
        json!({"admitted": true})
    } else {
        ineligible()
    }
}

/// One reference equality case through its catalog entry: operand families,
/// then each constraint the entry names.
fn reference_equality(catalog: &Value, package: &Value, nodes: &[Value], case: &Value) -> Value {
    let entry = catalog_entry(catalog, text(case, "operation"));
    let positions = string_positions(entry);
    let operands: Vec<Operand<'_>> = array(case, "operands")
        .iter()
        .map(Operand::from_case)
        .collect();
    if operands.len() != positions.len() {
        return ineligible();
    }
    let fits = operands.iter().zip(&positions).all(|(operand, position)| {
        operand
            .family(nodes)
            .is_some_and(|family| admitted_families(catalog, position).contains(&family))
    });
    if !fits {
        return ineligible();
    }
    for constraint in array(entry, "constraints") {
        let slot = |index: usize| match constraint["operands"][index]
            .as_u64()
            .and_then(|slot| usize::try_from(slot).ok())
            .and_then(|slot| operands.get(slot))
        {
            Some(Operand::Reference(node)) => *node,
            _ => panic!("a reference constraint names reference operands"),
        };
        match text(constraint, "kind") {
            "conforming_reference" => {
                let decided = conforming_reference(package, nodes, slot(0), slot(1));
                if decided != json!({"admitted": true}) {
                    return decided;
                }
            }
            other => panic!("TC-281 decides no {other} constraint"),
        }
    }
    json!({"admitted": true})
}

fn string_positions(entry: &Value) -> Vec<&str> {
    array(entry, "operands")
        .iter()
        .map(|position| position.as_str().expect("an operand position is a name"))
        .collect()
}

/// The typed operand of a projection case: a `deref` application checked
/// against the catalog's `quire.op.model.deref` entry, or a plain operand.
fn projection_operand<'a>(
    catalog: &Value,
    nodes: &[Value],
    operand: &'a Value,
) -> Option<Operand<'a>> {
    if operand.get("deref").is_none() {
        return Some(Operand::from_case(operand));
    }
    let entry = catalog_entry(catalog, "quire.op.model.deref");
    let [position] = string_positions(entry)[..] else {
        panic!("deref takes one operand");
    };
    let source = Operand::from_case(&operand["deref"]);
    let admitted = source
        .family(nodes)
        .is_some_and(|family| admitted_families(catalog, position).contains(&family));
    let Operand::Reference(inner) = source else {
        return None;
    };
    assert_eq!(
        text(entry, "result"),
        "inner:0",
        "deref yields its operand's element type"
    );
    let declared = Operand::Object(text(operand, "result_type"));
    // The declared result names the derived `inner:0` type and has its family.
    (admitted && declared == Operand::Object(inner) && declared.family(nodes).is_some())
        .then_some(declared)
}

/// One projection case through the catalog's `quire.op.record.project`
/// entry, then FR-322 steps 2 to 4 for its `field` member.
fn projection(catalog: &Value, package: &Value, nodes: &[Value], case: &Value) -> Value {
    let entry = catalog_entry(catalog, "quire.op.record.project");
    assert_eq!(entry["member"], "field");
    let [position] = string_positions(entry)[..] else {
        panic!("record.project takes one operand");
    };
    let has_model_deref = case["operand"].get("deref").is_some();
    let read_form = case["member"].get("read_form");
    if has_model_deref
        && !matches!(
            read_form.and_then(Value::as_str),
            Some("bare" | "explicit_deref")
        )
    {
        return json!({"refused": {"code": "malformed_wire"}});
    }
    if !has_model_deref && read_form.is_some() {
        return json!({"refused": {"code": "malformed_wire"}});
    }
    if let Some(record) = case["operand"].get("record") {
        let declaration = text(&case["member"], "declaration");
        if text(record, "type") != declaration {
            return ineligible();
        }
        let Some(field) = record["fields"].as_array().and_then(|fields| {
            fields
                .iter()
                .find(|field| field["name"] == case["member"]["name"])
        }) else {
            return ineligible();
        };
        if field["type"] != case["result_type"] {
            return ineligible();
        }
        return json!({
            "resolves": format!("{declaration}/{}", text(&case["member"], "name")),
            "member_type": field["type"]
        });
    }
    let Some(operand) = projection_operand(catalog, nodes, &case["operand"]) else {
        return ineligible();
    };
    let fits = operand
        .family(nodes)
        .is_some_and(|family| admitted_families(catalog, position).contains(&family));
    if !fits {
        return ineligible();
    }
    let member = &case["member"];
    for constraint in array(entry, "constraints") {
        match text(constraint, "kind") {
            "member_of" => {
                if operand != Operand::Object(text(member, "declaration")) {
                    return ineligible();
                }
            }
            other => panic!("TC-281 decides no {other} constraint"),
        }
    }
    // The `member` result form: the declared `result_type` is the step 4 type.
    let mut derived = derive(package, nodes, member);
    match derived.get("member_type") {
        Some(member_type) if *member_type != case["result_type"] => ineligible(),
        Some(_) => {
            if let Some(read_form) = read_form {
                derived["read_form"] = read_form.clone();
            }
            derived
        }
        None => derived,
    }
}

#[trace("TC-281", "FR-322-AC-33")]
#[test]
fn tc_281_reference_equality_admits_conforming_object_types() {
    let vectors = parsed(VECTORS);
    let catalog = parsed(OPERATION_CATALOG);
    let nodes = array(&vectors, "model_declaration_nodes");
    let cases = array(&vectors, "reference_equality_cases");
    let operations: BTreeSet<_> = cases.iter().map(|case| text(case, "operation")).collect();
    assert_eq!(
        operations,
        BTreeSet::from(["quire.op.reference.eq", "quire.op.reference.ne"])
    );
    for case in cases {
        assert_eq!(
            reference_equality(&catalog, &vectors["domain_package"], nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

#[trace("TC-281", "FR-322-AC-34")]
#[test]
fn tc_281_record_projection_reads_a_model_field_over_a_deref_result() {
    let vectors = parsed(VECTORS);
    let catalog = parsed(OPERATION_CATALOG);
    let object_groups: BTreeSet<&str> = catalog["groups"]
        .as_object()
        .expect("groups are an object")
        .iter()
        .filter(|(_, members)| {
            members
                .as_array()
                .is_some_and(|m| m.contains(&json!("object")))
        })
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        object_groups,
        BTreeSet::from(["field_owner"]),
        "an object term fits only a field_owner position"
    );
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "projection_cases") {
        assert_eq!(
            projection(&catalog, &vectors["domain_package"], nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }

    let cases = array(&vectors, "projection_cases");
    let case = |name: &str| {
        cases
            .iter()
            .find(|case| case["name"] == name)
            .unwrap_or_else(|| panic!("{name} projection vector"))
    };
    let bare = case("RP-09");
    let explicit = case("RP-10");
    assert_eq!(bare["source_expression"], "view.total");
    assert_eq!(explicit["source_expression"], "deref(view).total");
    assert_eq!(bare["operand"], explicit["operand"]);
    assert_eq!(bare["result_type"], explicit["result_type"]);
    let mut bare_member = bare["member"].clone();
    let mut explicit_member = explicit["member"].clone();
    bare_member
        .as_object_mut()
        .expect("field member is an object")
        .remove("read_form");
    explicit_member
        .as_object_mut()
        .expect("field member is an object")
        .remove("read_form");
    assert_eq!(bare_member, explicit_member, "only read_form differs");

    let schema = parsed(SCHEMA);
    let member_schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": "#/$defs/OperationMember",
        "$defs": schema["$defs"].clone()
    });
    let validator =
        jsonschema::validator_for(&member_schema).expect("the operation-member schema is valid");
    let wire_member = |case: &Value| {
        let mut member = case["member"].clone();
        let declaration = text(&member, "declaration");
        member["declaration"] = array(&vectors, "model_declaration_nodes")
            .iter()
            .find(|node| node["name"] == declaration)
            .unwrap_or_else(|| panic!("{declaration} node"))["node_id"]
            .clone();
        member
    };
    assert!(
        validator.is_valid(&wire_member(bare)),
        "bare mode is in the closed schema"
    );
    assert!(
        validator.is_valid(&wire_member(explicit)),
        "explicit mode is in the closed schema"
    );
    assert!(
        !validator.is_valid(&wire_member(case("RP-11"))),
        "an unrecognized read_form fails the closed schema"
    );
    assert!(
        validator.is_valid(&wire_member(case("RP-12"))),
        "read_form is optional in the member shape; the full reader refuses its omission"
    );
    assert!(
        validator.is_valid(&wire_member(case("RP-13"))),
        "read_form's applicability to the operand is decided by the full reader"
    );
    assert_eq!(case("RP-14")["member"]["read_form"], "bare");
    let mut ordinary_record_member = wire_member(bare);
    ordinary_record_member["read_form"] = case("RP-14")["member"]["read_form"].clone();
    assert!(
        validator.is_valid(&ordinary_record_member),
        "the field member shape admits read_form; the full reader checks applicability"
    );

    let mut ordinary_record = case("RP-14").clone();
    ordinary_record["member"]
        .as_object_mut()
        .expect("ordinary record field member is an object")
        .remove("read_form");
    assert_eq!(
        projection(
            &catalog,
            &vectors["domain_package"],
            nodes,
            &ordinary_record
        ),
        json!({
            "resolves": "Money/amount_minor",
            "member_type": {"type": "Integer"}
        }),
        "an ordinary record projection is valid without model source metadata"
    );
}

/// FR-322 steps 2 and 3 for one member named `name` on the node vector
/// `declaration` names, matched against the `list` member kind (`"fields"`
/// or `"operations"`); `none` is the refusal returned when no member
/// matches. Maps a count of matches: the resolved (owner declaration,
/// identity, record), or the refusal.
fn resolve_member<'a>(
    package: &'a Value,
    nodes: &[Value],
    declaration: &str,
    list: &str,
    name: &str,
    none: Value,
) -> Result<(&'a Value, String, &'a Value), Value> {
    let recovered = recover_vector(package, nodes, declaration)
        .ok_or_else(|| refused("missing_declaration", "missing-selection"))?;
    let declared = recovered.object_type.ok_or_else(ineligible)?;
    let matches: Vec<_> = exposed(package, declared, list)
        .into_iter()
        .filter(|(_, record)| record["name"] == name)
        .collect();
    match matches.as_slice() {
        [] => Err(none),
        [(identity, record)] => Ok((declared, identity.clone(), record)),
        _ => Err(refused("ambiguous_declaration", "ambiguous-name")),
    }
}

/// The owner node identity of a resolved member identity.
fn owner_of(identity: &str) -> &str {
    identity
        .rsplit_once('/')
        .map_or(identity, |(owner, _)| owner)
}

/// The step 4 type of a resolved field record, presence included.
fn field_type(package: &Value, record: &Value) -> Option<Value> {
    declared_type_of(package, record).map(|value| {
        if record["presence"] == "optional" {
            json!({"type": "Option", "inner": value})
        } else {
            value
        }
    })
}

/// FR-322's `reference_edge` constraint over one `reaches_field` case.
fn reference_edge(
    package: &Value,
    nodes: &[Value],
    case: &Value,
    operands: &[Operand<'_>],
) -> Value {
    let member = &case["member"];
    let declaration = text(member, "declaration");
    if operands[0] != Operand::Reference(declaration) {
        return ineligible();
    }
    let (_, identity, record) = match resolve_member(
        package,
        nodes,
        declaration,
        "fields",
        text(member, "name"),
        ineligible(),
    ) {
        Ok(resolved) => resolved,
        Err(refusal) => return refusal,
    };
    let owner = declarations(package)
        .into_iter()
        .find(|declared| declared.form == "object_type" && declared.node == owner_of(&identity))
        .expect("a field owner is a declared object type");
    let edge = json!({"type": "Reference", "target": declaration_key(package, &owner)});
    let admitted = match field_type(package, record) {
        Some(ty) if ty == edge => true,
        Some(ty) if ty["type"] == "Option" => ty["inner"] == edge,
        Some(ty) if ty["type"] == "Sequence" => ty["element"] == edge && ty.get("max").is_some(),
        _ => false,
    };
    if !admitted {
        return ineligible();
    }
    let Operand::Reference(target) = operands[1] else {
        return ineligible();
    };
    let Some(target) = recover_vector(package, nodes, target) else {
        return refused("missing_declaration", "missing-selection");
    };
    let conforms = target.node == owner.node
        || target
            .object_type
            .is_some_and(|declared| ancestors(package, declared).contains(owner.node));
    if conforms {
        json!({"admitted": true})
    } else {
        ineligible()
    }
}

/// One `quire.op.model.reaches_field` case through its catalog entry.
fn reaches_field(catalog: &Value, package: &Value, nodes: &[Value], case: &Value) -> Value {
    let entry = catalog_entry(catalog, "quire.op.model.reaches_field");
    assert_eq!(entry["member"], "field");
    let positions = string_positions(entry);
    let operands: Vec<Operand<'_>> = array(case, "operands")
        .iter()
        .map(Operand::from_case)
        .collect();
    let fits = operands.len() == positions.len()
        && operands.iter().zip(&positions).all(|(operand, position)| {
            operand
                .family(nodes)
                .is_some_and(|family| admitted_families(catalog, position).contains(&family))
        });
    if !fits {
        return ineligible();
    }
    for constraint in array(entry, "constraints") {
        match text(constraint, "kind") {
            "reference_edge" => {
                let decided = reference_edge(package, nodes, case, &operands);
                if decided != json!({"admitted": true}) {
                    return decided;
                }
            }
            other => panic!("TC-281 decides no {other} constraint"),
        }
    }
    json!({"admitted": true})
}

#[trace("TC-281", "FR-322-AC-38")]
#[test]
fn tc_281_reaches_over_a_field_admits_only_reference_edges_of_the_owner() {
    let vectors = parsed(VECTORS);
    let catalog = parsed(OPERATION_CATALOG);
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "reaches_cases") {
        assert_eq!(
            reaches_field(&catalog, &vectors["domain_package"], nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

#[trace("TC-280", "FR-340-AC-13")]
#[test]
fn tc_280_frame_field_entries_resolve_among_exposed_fields() {
    let vectors = parsed(VECTORS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "frame_field_cases") {
        let decided = match resolve_member(
            package,
            nodes,
            text(case, "declaration"),
            "fields",
            text(case, "field"),
            refused("missing_declaration", "missing-name"),
        ) {
            Ok((_, identity, _)) => json!({"resolves": identity}),
            Err(refusal) => refusal,
        };
        assert_eq!(decided, case["expected"], "{}", text(case, "name"));
    }
}

/// FR-342-AC-4 over one anchor case: the context is an object type, a process
/// or a state machine, and the operation resolves among its exposed operation members to one the
/// context itself declares.
fn anchor_operation(package: &Value, nodes: &[Value], context: &str, operation: &str) -> Value {
    let form = text(node_vector(nodes, context), "semantic_form");
    if !matches!(form, "object_type" | "process" | "state_machine") {
        return refused("invalid_model_binding", "malformed-declaration");
    }
    let resolved = resolve_member(
        package,
        nodes,
        context,
        "operations",
        operation,
        refused("missing_declaration", "missing-name"),
    );
    match resolved {
        Err(refusal) => refusal,
        Ok((declared, identity, _)) if owner_of(&identity) != text(declared, "node") => {
            refused("invalid_model_binding", "malformed-declaration")
        }
        Ok((_, identity, _)) => json!({"resolves": identity}),
    }
}

#[trace("TC-280", "FR-342-AC-4")]
#[test]
fn tc_280_an_anchor_operation_resolves_to_one_the_context_declares() {
    let vectors = parsed(VECTORS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "anchor_cases") {
        assert_eq!(
            anchor_operation(
                package,
                nodes,
                text(case, "context"),
                text(case, "operation")
            ),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

/// FR-341-AC-5: the parameter types a clause binds, in order: `self`, then
/// the operation's result in a postcondition of an operation with one, then
/// the operation's parameters.
fn clause_signature(package: &Value, nodes: &[Value], case: &Value) -> Option<Vec<Value>> {
    let context = text(case, "context");
    let owner = recover_vector(package, nodes, context)?;
    let mut signature =
        vec![json!({"type": "Reference", "target": declaration_key(package, &owner)})];
    let Some(operation) = case["operation"].as_str() else {
        return Some(signature);
    };
    let (_, _, record) = resolve_member(
        package,
        nodes,
        context,
        "operations",
        operation,
        ineligible(),
    )
    .ok()?;
    if case["clause"] == "postcondition" && !record["result"].is_null() {
        signature.push(declared_type_of(package, &record["result"])?);
    }
    for parameter in array(record, "parameters") {
        signature.push(declared_type_of(package, parameter)?);
    }
    Some(signature)
}

/// The decision of one clause signature case: admitted exactly when the
/// supplied parameter list is the one FR-341 derives.
fn clause_decision(package: &Value, nodes: &[Value], case: &Value) -> Value {
    let supplied: Vec<Value> = array(case, "parameters")
        .iter()
        .map(|parameter| match parameter["reference"].as_str() {
            Some(name) => {
                json!({"type": "Reference", "target": node_vector(nodes, name)["node_id"]})
            }
            None => parameter.clone(),
        })
        .collect();
    if clause_signature(package, nodes, case).as_ref() == Some(&supplied) {
        json!({"admitted": true})
    } else {
        ineligible()
    }
}

#[trace("TC-280", "FR-341-AC-5")]
#[test]
fn tc_280_a_clause_binds_self_result_and_parameters_of_its_operation() {
    let vectors = parsed(VECTORS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "clause_signature_cases") {
        assert_eq!(
            clause_decision(package, nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

/// A number or text constraint operand as the projection spells it.
fn operand_text(constraint: &Value) -> String {
    match &constraint["operands"]["value"] {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn constraint_of<'a>(declared: &'a Value, keyword: &str) -> Option<&'a Value> {
    declared["constraints"]
        .as_array()?
        .iter()
        .find(|constraint| constraint["keyword"] == keyword)
}

fn projected_multiplicity(multiplicity: &Value) -> Value {
    json!({
        "lower": multiplicity["lower"].to_string(),
        "upper": multiplicity.get("upper").map_or("unbounded".to_owned(), Value::to_string),
        "ordered": multiplicity["ordered"],
        "unique": multiplicity["unique"],
    })
}

fn projected_slot(slot: &Value) -> Value {
    json!({
        "name": slot["name"],
        "typeRef": slot["typeRef"],
        "multiplicity": projected_multiplicity(&slot["multiplicity"]),
    })
}

/// The projection FR-154 intake reads from the selected document: for each
/// object type or interface its supertypes, fields, operations and
/// operation redefinitions; for each value type its native type, bounds and
/// text profile; for each relationship its named ends, `sourceEnd.role` then
/// `targetEnd.role` when present. Lists ascend by node identity.
fn project_document(document: &Value) -> Value {
    let meanings: BTreeMap<(&str, &str), &str> = array(document, "constructs")
        .iter()
        .map(|construct| {
            (
                (
                    text(&construct["kind"], "module"),
                    text(&construct["kind"], "name"),
                ),
                text(&construct["construct"], "meaning"),
            )
        })
        .collect();
    let mut object_types = Vec::new();
    let mut value_types = Vec::new();
    let mut relationships = Vec::new();
    for declared in array(document, "types") {
        let meaning = meanings[&(
            text(&declared["kind"], "module"),
            text(&declared["kind"], "name"),
        )];
        let node = text(declared, "identity");
        match meaning {
            "quire.meaning.model.object-type/v1" | "quire.meaning.systems.interface/v1" => {
                let fields: Vec<Value> = array(declared, "fields")
                    .iter()
                    .map(|field| {
                        let mut projected = projected_slot(field);
                        projected["presence"] = field["presence"].clone();
                        projected["redefines"] =
                            field.get("redefines").cloned().unwrap_or(Value::Null);
                        if let Some(constraints) = field["constraints"].as_array() {
                            projected["constraints"] = Value::Object(
                                constraints
                                    .iter()
                                    .map(|constraint| {
                                        (
                                            text(constraint, "keyword").to_owned(),
                                            json!(operand_text(constraint)),
                                        )
                                    })
                                    .collect(),
                            );
                        }
                        projected
                    })
                    .collect();
                let operations: Vec<Value> = array(declared, "operations")
                    .iter()
                    .map(|operation| {
                        json!({
                            "name": operation["name"],
                            "parameters": array(operation, "params")
                                .iter()
                                .map(projected_slot)
                                .collect::<Vec<_>>(),
                            "result": operation.get("returns").map(|returns| json!({
                                "typeRef": returns["typeRef"],
                                "multiplicity": projected_multiplicity(&returns["multiplicity"]),
                            })),
                            "redefines": operation
                                .get("redefines")
                                .cloned()
                                .unwrap_or(Value::Null),
                        })
                    })
                    .collect();
                for relationship in array(declared, "relationships") {
                    let mut ends = vec![json!({
                        "role": relationship["sourceEnd"]["role"],
                        "type": relationship["sourceEnd"]["type"],
                        "end": "source",
                    })];
                    if let Some(role) = relationship["targetEnd"].get("role") {
                        ends.push(json!({
                            "role": role,
                            "type": relationship["targetEnd"]["type"],
                            "end": "target",
                        }));
                    }
                    relationships.push(json!({
                        "node": relationship["identity"],
                        "direction": relationship["direction"],
                        "ends": ends,
                    }));
                }
                object_types.push(json!({
                    "node": node,
                    "interfaceFeatures": (meaning == "quire.meaning.systems.interface/v1")
                        .then(|| json!([])),
                    "supertypes": declared["supertypes"],
                    "fields": fields,
                    "operations": operations,
                }));
            }
            "quire.meaning.model.value-type/v1" => {
                let (native, low, high) = match text(declared, "scalar") {
                    "integer" => ("Integer", "min", "max"),
                    "string" => ("Text", "minLength", "maxLength"),
                    other => panic!("unexpected value type scalar {other}"),
                };
                let mut projected = json!({
                    "node": node,
                    "native": format!("ix://quire/native/{native}"),
                    "lower": operand_text(constraint_of(declared, low).expect("a lower bound")),
                    "upper": operand_text(constraint_of(declared, high).expect("an upper bound")),
                });
                if let Some(format) = constraint_of(declared, "format") {
                    projected["profile"] = json!(
                        text(&format["operands"], "name")
                            .strip_prefix("text-profile:")
                            .expect("a text profile format")
                    );
                }
                value_types.push(projected);
            }
            other => panic!("unexpected meaning {other}"),
        }
    }
    relationships
        .sort_by(|left: &Value, right: &Value| left["node"].as_str().cmp(&right["node"].as_str()));
    json!({
        "identity": document["package"]["identity"],
        "version": document["package"]["version"],
        "object_types": object_types,
        "value_types": value_types,
        "relationships": relationships,
    })
}

/// FR-322 "Model-owned members": the selected evidence is the semantic IR
/// document, and the vectors' `domain_package` is exactly the projection of it
/// that `project_document` derives.
#[trace("TC-280", "FR-322-AC-32")]
#[test]
fn tc_280_the_domain_package_projection_is_derived_from_the_selected_document() {
    let vectors = parsed(VECTORS);
    let sorted = |mut package: Value| {
        for list in ["object_types", "value_types", "relationships"] {
            package[list]
                .as_array_mut()
                .expect("projection list")
                .sort_by(|left, right| left["node"].as_str().cmp(&right["node"].as_str()));
        }
        package
    };
    assert_eq!(
        sorted(project_document(&parsed(DOCUMENT))),
        sorted(vectors["domain_package"].clone())
    );
}

/// FR-154-AC-16: intake reads an operation's redefinition only from
/// `Operation.redefines`, and a `redefines` naming no node of the package
/// refuses at the operation and the member.
fn read_operation_redefinitions(document: &Value) -> Result<(), Value> {
    let identities: BTreeSet<&str> = array(document, "types")
        .iter()
        .flat_map(|declared| {
            ["fields", "operations"]
                .into_iter()
                .flat_map(move |list| array_or_empty(declared, list))
        })
        .map(|member| text(member, "identity"))
        .collect();
    for declared in array(document, "types") {
        for operation in array_or_empty(declared, "operations") {
            if let Some(target) = operation["redefines"].as_str() {
                if !identities.contains(target) {
                    return Err(json!({"refused": {
                        "code": "missing_declaration",
                        "cause": "missing-name",
                        "at": operation["identity"],
                        "path": "Operation.redefines",
                    }}));
                }
            }
        }
    }
    Ok(())
}

#[trace("TC-280", "FR-154-AC-16")]
#[trace("TC-280", "FR-322-AC-30")]
#[test]
fn tc_280_an_operation_redefines_only_through_operation_redefines() {
    let vectors = parsed(VECTORS);
    let nodes = array(&vectors, "model_declaration_nodes");
    let cases = array(&vectors, "operation_redefinition_cases");
    assert_eq!(cases.len(), 4);
    for case in cases {
        let document = patched_document(Some(&case["patch"]));
        let outcome = match read_operation_redefinitions(&document) {
            Err(refusal) => refusal,
            Ok(()) => derive(&project_document(&document), nodes, &case["member"]),
        };
        assert_eq!(outcome, case["expected"], "{}", text(case, "name"));
    }
}

/// FR-154-AC-9: the document carries filament-core-data FR-095's identity
/// forms. A type is its artifact id; a field, operation and parameter nest
/// under their owner; a relationship is `relationship/<Name>-<verb>-<Target>`
/// and is not owner-nested.
#[trace("TC-280", "FR-154-AC-9")]
#[test]
fn tc_280_the_document_carries_fr_095_identity_forms() {
    let document = parsed(DOCUMENT);
    let members =
        |value: &Value, member: &str| value[member].as_array().cloned().unwrap_or_default();
    let leaf = |identity: &str| identity.rsplit('/').next().unwrap_or_default().to_owned();
    for declared in array(&document, "types") {
        let owner = text(declared, "identity");
        assert_eq!(owner, format!("ix://acme/orders/{}", leaf(owner)));
        for field in &members(declared, "fields") {
            assert_eq!(
                text(field, "identity"),
                format!("{owner}/{}", text(field, "name"))
            );
        }
        for operation in &members(declared, "operations") {
            let identity = format!("{owner}/{}", text(operation, "name"));
            assert_eq!(text(operation, "identity"), identity);
            for param in &members(operation, "params") {
                assert_eq!(
                    text(param, "identity"),
                    format!("{identity}/{}", text(param, "name"))
                );
            }
        }
        for relationship in &members(declared, "relationships") {
            assert_eq!(
                text(relationship, "identity"),
                format!(
                    "ix://acme/orders/relationship/{}-{}-{}",
                    leaf(owner),
                    text(&relationship["sourceEnd"], "role"),
                    leaf(text(&relationship["targetEnd"], "type"))
                )
            );
        }
    }
}

/// Edit `document` at an authored pointer. A segment `@name` selects the
/// array member whose `identity` ends with `/name`.
fn pointer_target<'a>(document: &'a mut Value, segments: &[&str]) -> &'a mut Value {
    let mut current = document;
    for segment in segments {
        current = match segment.strip_prefix('@') {
            Some(name) => {
                let suffix = format!("/{name}");
                current
                    .as_array_mut()
                    .expect("an @ segment indexes an array")
                    .iter_mut()
                    .find(|member| {
                        member["identity"]
                            .as_str()
                            .is_some_and(|i| i.ends_with(&suffix))
                    })
                    .unwrap_or_else(|| panic!("a member named {name}"))
            }
            None if current.is_array() => {
                &mut current[segment.parse::<usize>().expect("an index segment")]
            }
            None => current
                .get_mut(*segment)
                .unwrap_or_else(|| panic!("member {segment}")),
        };
    }
    current
}

fn apply_edit(document: &mut Value, edit: &Value) {
    let path = text(edit, "path");
    let segments: Vec<&str> = path.split('/').skip(1).collect();
    let (last, parents) = segments.split_last().expect("a pointer names a member");
    let parent = pointer_target(document, parents);
    match text(edit, "op") {
        "replace" => {
            if parent.is_array() {
                parent[last.parse::<usize>().expect("an index")] = edit["value"].clone();
            } else {
                // A replace sets the member, adding an optional one such as
                // `upper` when the document leaves it absent.
                parent
                    .as_object_mut()
                    .expect("an object")
                    .insert((*last).to_owned(), edit["value"].clone());
            }
        }
        "remove" => {
            parent.as_object_mut().expect("an object").remove(*last);
        }
        "add" => {
            assert_eq!(*last, "-", "an add appends");
            parent
                .as_array_mut()
                .expect("an array")
                .push(edit["value"].clone());
        }
        other => panic!("unexpected edit {other}"),
    }
}

/// An authored `patch` over the selected document, or a copy when absent.
fn patched_document(patches: Option<&Value>) -> Value {
    let mut document = parsed(DOCUMENT);
    for edit in patches.and_then(Value::as_array).into_iter().flatten() {
        apply_edit(&mut document, edit);
    }
    document
}

const MALFORMED: (&str, &str) = ("invalid_model_binding", "malformed-declaration");

fn non_empty_string(value: &Value) -> bool {
    value.as_str().is_some_and(|text| !text.is_empty())
}

/// A relationship end's multiplicity: non-negative `lower`, optional `upper`
/// and boolean `ordered` and `unique`; the pair `(lower, upper)`.
fn end_multiplicity(end: &Value) -> Option<(u64, Option<u64>)> {
    let multiplicity = end.get("multiplicity")?;
    multiplicity["ordered"].as_bool()?;
    multiplicity["unique"].as_bool()?;
    let lower = multiplicity["lower"].as_u64()?;
    match multiplicity.get("upper") {
        None => Some((lower, None)),
        Some(upper) => Some((lower, Some(upper.as_u64()?))),
    }
}

/// The common-schema origin: exactly one closed branch, `source` with a
/// source locus or `generated` with a generator.
fn origin_is_valid(origin: &Value) -> bool {
    let Some(branches) = origin.as_object().filter(|branches| branches.len() == 1) else {
        return false;
    };
    let positive = |value: &Value| value.as_u64().is_some_and(|number| number >= 1);
    if let Some(source) = branches.get("source") {
        let allowed = [
            "sourceIdentity",
            "path",
            "startLine",
            "startColumn",
            "endLine",
            "endColumn",
        ];
        return source.as_object().is_some_and(|locus| {
            locus.keys().all(|key| allowed.contains(&key.as_str()))
                && non_empty_string(&source["sourceIdentity"])
                && non_empty_string(&source["path"])
                && positive(&source["startLine"])
                && positive(&source["startColumn"])
                && ["endLine", "endColumn"]
                    .iter()
                    .all(|key| source.get(*key).is_none_or(positive))
        });
    }
    branches.get("generated").is_some_and(|generated| {
        generated.as_object().is_some_and(|object| {
            object.len() == 3
                && non_empty_string(&generated["generatorIdentity"])
                && non_empty_string(&generated["generatorVersion"])
                && generated["inputIdentities"]
                    .as_array()
                    .is_some_and(|inputs| !inputs.is_empty() && inputs.iter().all(Value::is_string))
        })
    })
}

/// FR-154 step 1's relationship declaration refusals over the selected
/// document (QSL-638 / IR-661): the refusal, in node order and within a node
/// in the order malformed declaration, unresolved end type, duplicate
/// identity, reversed multiplicity, each reported at the selection row.
fn read_relationships(document: &Value) -> Result<(), (&'static str, &'static str)> {
    read_relationships_retaining(document).map_err(|(code, cause, _, _)| (code, cause))
}

/// A refusal's code, cause and the node's retained identity and `origin`,
/// verbatim; the origin is `None` when the node carries none.
type Retained = (&'static str, &'static str, Option<Value>, Option<Value>);

fn read_relationships_retaining(document: &Value) -> Result<(), Retained> {
    const MISSING_NAME: (&str, &str) = ("missing_declaration", "missing-name");
    let package = text(&document["package"], "identity");
    let slot = format!("ix://{package}/relationship/");
    let mut types: Vec<&Value> = array(document, "types").iter().collect();
    types.sort_by_key(|declared| declared["identity"].as_str().map(str::to_owned));
    let meanings: BTreeMap<(&str, &str), &str> = array(document, "constructs")
        .iter()
        .map(|construct| {
            (
                (
                    text(&construct["kind"], "module"),
                    text(&construct["kind"], "name"),
                ),
                text(&construct["construct"], "meaning"),
            )
        })
        .collect();
    let is_object_type = |declared: &Value| {
        meanings
            .get(&(
                declared["kind"]["module"].as_str().unwrap_or_default(),
                declared["kind"]["name"].as_str().unwrap_or_default(),
            ))
            .is_some_and(|meaning| {
                [
                    "quire.meaning.model.object-type/v1",
                    "quire.meaning.systems.interface/v1",
                ]
                .contains(meaning)
            })
    };
    let objects: BTreeSet<&str> = types
        .iter()
        .filter(|declared| is_object_type(declared))
        .filter_map(|declared| declared["identity"].as_str())
        .collect();
    let others: BTreeSet<&str> = types
        .iter()
        .filter(|declared| !is_object_type(declared))
        .filter_map(|declared| declared["identity"].as_str())
        .collect();
    let relationships: BTreeSet<&str> = types
        .iter()
        .flat_map(|declared| array_or_empty(declared, "relationships"))
        .filter_map(|relationship| relationship["identity"].as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for owner in &types {
        let owner_identity = owner["identity"].as_str().unwrap_or_default();
        for relationship in array_or_empty(owner, "relationships") {
            let identity = relationship["identity"].as_str();
            let retain = |(code, cause): (&'static str, &'static str)| -> Retained {
                (
                    code,
                    cause,
                    relationship.get("identity").cloned(),
                    relationship
                        .get("origin")
                        .filter(|origin| origin_is_valid(origin))
                        .cloned(),
                )
            };
            let source = &relationship["sourceEnd"];
            let target = &relationship["targetEnd"];
            let closed = |value: &Value, members: &[&str]| {
                value
                    .as_object()
                    .is_some_and(|object| object.keys().all(|key| members.contains(&key.as_str())))
            };
            let multiplicity_closed = |end: &Value| {
                closed(
                    &end["multiplicity"],
                    &["lower", "upper", "ordered", "unique"],
                )
            };
            let malformed = !closed(
                relationship,
                &[
                    "identity",
                    "category",
                    "composite",
                    "direction",
                    "sourceEnd",
                    "targetEnd",
                    "origin",
                ],
            ) || !closed(source, &["role", "multiplicity", "type"])
                || !closed(target, &["role", "multiplicity", "type"])
                || !multiplicity_closed(source)
                || !multiplicity_closed(target)
                || !identity.is_some_and(|identity| {
                    identity
                        .strip_prefix(&slot)
                        .is_some_and(|rest| !rest.is_empty() && !rest.contains('/'))
                })
                || !relationship["direction"].as_str().is_some_and(|direction| {
                    [
                        "source-to-target",
                        "target-to-source",
                        "bidirectional",
                        "undirected",
                    ]
                    .contains(&direction)
                })
                || !relationship["category"].as_str().is_some_and(|category| {
                    [
                        "structural",
                        "behavioral",
                        "dataflow",
                        "dependency",
                        "realization",
                        "governance",
                        "traceability",
                    ]
                    .contains(&category)
                })
                || !relationship["composite"].is_boolean()
                || !origin_is_valid(&relationship["origin"])
                || !non_empty_string(&source["role"])
                || target
                    .get("role")
                    .is_some_and(|role| !non_empty_string(role))
                || end_multiplicity(source).is_none()
                || end_multiplicity(target).is_none()
                || source["type"].as_str() != Some(owner_identity)
                || !target["type"].is_string()
                || [source, target].iter().any(|end| {
                    end["type"]
                        .as_str()
                        .is_some_and(|name| relationships.contains(name) || others.contains(name))
                });
            if malformed {
                return Err(retain(MALFORMED));
            }
            let unresolved = [source, target]
                .iter()
                .any(|end| !objects.contains(text(end, "type")));
            if unresolved {
                return Err(retain(MISSING_NAME));
            }
            if !seen.insert(identity.unwrap_or_default()) {
                return Err(retain(("invalid_model_binding", "conflicting-binding")));
            }
            let reversed = [source, target].iter().any(|end| {
                end_multiplicity(end).is_some_and(|(lower, upper)| upper.is_some_and(|u| lower > u))
            });
            if reversed {
                return Err(retain((
                    "invalid_model_binding",
                    "unpreserved-model-meaning",
                )));
            }
        }
    }
    Ok(())
}

fn array_or_empty<'a>(value: &'a Value, member: &str) -> &'a [Value] {
    value[member].as_array().map_or(&[], Vec::as_slice)
}

/// FR-154 relationship declaration refusals and admissions: each published
/// `relationship_document_cases` entry patches the selected document and the
/// reader reports the recorded refusal at the selection row.
#[trace("TC-280", "FR-154-AC-10")]
#[test]
fn tc_280_relationship_declarations_refuse_each_defect_at_the_selection_row() {
    let adverse = parsed(ADVERSE);
    let cases = array(&adverse, "relationship_document_cases");
    assert_eq!(cases.len(), 33);
    for case in cases {
        let id = text(case, "id");
        let document = patched_document(Some(&case["patch"]));
        let outcome = read_relationships(&document);
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ => {
                assert_eq!(text(case, "expected_locus"), "selection_row", "{id}");
                assert_eq!(
                    outcome,
                    Err((text(case, "expected_code"), text(case, "expected_cause"))),
                    "{id}"
                );
            }
        }
    }
}

/// FR-154 operation parameter names at domain selection. Within one operation
/// the repeated-identity check runs first and suppresses the name check of both
/// colliding parameters (a collision is reported alone); then the name check of every parameter, in
/// `params` order. A collision names the shared parameter identity, a name
/// refusal the operation identity; both name the member path.
fn read_parameter_names(
    document: &Value,
) -> Result<(), (&'static str, &'static str, String, String)> {
    let member_name = |name: &str| {
        let mut chars = name.chars();
        chars
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    };
    let mut types: Vec<&Value> = array(document, "types").iter().collect();
    types.sort_by_key(|declared| declared["identity"].as_str().map(str::to_owned));
    for owner in types {
        for (k, operation) in array_or_empty(owner, "operations").iter().enumerate() {
            let operation_identity = text(operation, "identity");
            let params = array_or_empty(operation, "params");
            let path = |s: usize, member: &str| {
                format!("TypeDefinition.operations[{k}].params[{s}]{member}")
            };
            let mut first_seen: BTreeMap<&str, usize> = BTreeMap::new();
            let mut collision = None;
            for (s, param) in params.iter().enumerate() {
                let identity = text(param, "identity");
                match first_seen.get(identity) {
                    Some(_) => {
                        collision.get_or_insert((identity.to_owned(), s));
                    }
                    None => {
                        first_seen.insert(identity, s);
                    }
                }
            }
            if let Some((identity, s)) = collision {
                return Err((
                    "invalid_model_binding",
                    "conflicting-binding",
                    identity,
                    path(s, ""),
                ));
            }
            for (s, param) in params.iter().enumerate() {
                let identity = text(param, "identity");
                let valid = param["name"].as_str().is_some_and(|name| {
                    member_name(name) && identity == format!("{operation_identity}/{name}")
                });
                if !valid {
                    return Err((
                        MALFORMED.0,
                        MALFORMED.1,
                        operation_identity.to_owned(),
                        path(s, ".name"),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// FR-154-AC-14: each `parameter_name_cases` entry patches the selected
/// document and the reader reports the recorded refusal at the selection row.
#[trace("TC-280", "FR-154-AC-14")]
#[test]
fn tc_280_parameter_names_refuse_each_defect_at_the_selection_row() {
    let adverse = parsed(ADVERSE);
    let cases = array(&adverse, "parameter_name_cases");
    for case in cases {
        let id = text(case, "id");
        let outcome = read_parameter_names(&patched_document(Some(&case["patch"])));
        match case["outcome"].as_str() {
            Some("admitted") => assert_eq!(outcome, Ok(()), "{id}"),
            _ => {
                assert_eq!(text(case, "expected_locus"), "selection_row", "{id}");
                assert_eq!(
                    outcome,
                    Err((
                        text(case, "expected_code"),
                        text(case, "expected_cause"),
                        text(case, "expected_identity").to_owned(),
                        text(case, "expected_path").to_owned(),
                    )),
                    "{id}"
                );
            }
        }
    }
}

/// The mapping `derive`'s wire cases name: a model declaration vector's
/// declared node identity.
fn vector_node<'a>(nodes: &'a [Value], name: &str) -> &'a str {
    text(&node_vector(nodes, name)["owner"], "node")
}

fn conforms_to(projection: &Value, node: &str, endpoint: &str) -> bool {
    node == endpoint
        || object_type(projection, node)
            .is_some_and(|declared| ancestors(projection, declared).contains(endpoint))
}

/// FR-152 "Navigation": the destination type of one traversed end.
fn destination_type(end: &Value, destination: &str) -> Result<Value, Value> {
    let multiplicity = &end["multiplicity"];
    let (lower, upper) = end_multiplicity(end).expect("an authored end");
    if multiplicity["ordered"] == true || upper.is_none() {
        return Err(refused_at(
            "unsupported_construct",
            "expression-form",
            "member.name",
        ));
    }
    Ok(match (lower, upper) {
        (1, Some(1)) => json!({"kind": "Reference", "of": destination}),
        (0, Some(1)) => json!({"kind": "Option", "of": destination}),
        (lower, upper) => json!({
            "kind": if multiplicity["unique"] == true { "Set" } else { "Bag" },
            "of": destination,
            "min": lower.to_string(),
            "max": upper.expect("a finite upper bound").to_string(),
        }),
    })
}

/// One `quire.op.model.navigate` or `quire.op.model.reaches` application over
/// a model-owned relationship end, in the operation-step order of FR-322
/// "Model-owned members" step 3: declaration kind, role lookup, receiver
/// endpoint, direction, destination multiplicity, then the operation's own
/// type comparison.
fn relationship_navigation(projection: &Value, nodes: &[Value], case: &Value) -> Value {
    let document = patched_document(case.get("patch"));
    let relationship_vector = node_vector(nodes, text(case, "relationship"));
    if relationship_vector["semantic_form"] != "relationship" {
        return refused_at("ill_typed", "operator-ineligible", "member.declaration");
    }
    let node = vector_node(nodes, text(case, "relationship"));
    let declared = array(&document, "types")
        .iter()
        .flat_map(|owner| array_or_empty(owner, "relationships"))
        .find(|relationship| relationship["identity"] == node)
        .expect("a published relationship");
    let role = text(case, "role");
    let forward = declared["sourceEnd"]["role"] == role;
    let inverse = declared["targetEnd"]["role"] == role;
    let (receiver_end, destination_end, needs) = match (forward, inverse) {
        (false, false) => return refused_at("missing_declaration", "missing-name", "member.name"),
        (true, true) => {
            return refused_at("ambiguous_declaration", "ambiguous-name", "member.name");
        }
        (true, false) => (
            &declared["sourceEnd"],
            &declared["targetEnd"],
            "source-to-target",
        ),
        (false, true) => (
            &declared["targetEnd"],
            &declared["sourceEnd"],
            "target-to-source",
        ),
    };
    let endpoint = text(receiver_end, "type");
    let operands: Vec<&str> = match case.get("receiver") {
        Some(receiver) => vec![receiver.as_str().expect("a receiver vector name")],
        None => array(case, "operands")
            .iter()
            .map(|operand| operand.as_str().expect("an operand vector name"))
            .collect(),
    };
    for (index, operand) in operands.iter().enumerate() {
        if !conforms_to(projection, vector_node(nodes, operand), endpoint) {
            return refused_at(
                "ill_typed",
                "operator-ineligible",
                &format!("arguments/{index}"),
            );
        }
    }
    if ![needs, "bidirectional", "undirected"].contains(&text(declared, "direction")) {
        return refused_at("ill_typed", "operator-ineligible", "member.name");
    }
    let destination = text(destination_end, "type");
    let derived = match destination_type(destination_end, destination) {
        Ok(derived) => derived,
        Err(refusal) => return refusal,
    };
    if text(case, "operation") == "reaches" {
        // Any finite destination multiplicity of the endpoint's own type
        // is reachable; the `Reference`, `Option`, `Set` and `Bag` result
        // shapes all admit.
        if destination != endpoint {
            return refused_at("ill_typed", "operator-ineligible", "member.name");
        }
    } else if derived != case["result"] {
        return refused_at("ill_typed", "operator-ineligible", "result_type");
    }
    json!({"admitted": true})
}

/// FR-322-AC-57: navigation and reachability over a model-owned
/// relationship end, in the recorded operation-step order, each refusal at
/// its recorded application pointer.
#[trace("TC-281", "FR-322-AC-57")]
#[test]
fn tc_281_relationship_ends_navigate_by_role_receiver_direction_and_destination() {
    let vectors = parsed(VECTORS);
    let nodes = array(&vectors, "model_declaration_nodes");
    let cases = array(&vectors, "relationship_navigation_cases");
    assert_eq!(cases.len(), 27);
    for case in cases {
        assert_eq!(
            relationship_navigation(&vectors["domain_package"], nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

/// FR-154: a declaration refusal retains the node's IR identity and `origin`
/// verbatim, a source origin with its locus and a generated origin with its
/// members and no span; nothing is synthesized.
#[trace("TC-280", "FR-154-AC-10")]
#[test]
fn tc_280_a_declaration_refusal_retains_the_nodes_origin_verbatim() {
    let adverse = parsed(ADVERSE);
    let cases = array(&adverse, "declaration_origin_cases");
    assert_eq!(cases.len(), 3);
    for case in cases {
        let id = text(case, "id");
        let document = patched_document(Some(&case["patch"]));
        let (code, cause, identity, origin) =
            read_relationships_retaining(&document).expect_err(id);
        assert_eq!(
            (code, cause),
            (text(case, "expected_code"), text(case, "expected_cause")),
            "{id}"
        );
        assert_eq!(identity, Some(case["expected_identity"].clone()), "{id}");
        assert_eq!(origin, case.get("expected_origin").cloned(), "{id}");
    }
    // A malformed origin retains none, like a missing one.
    let document = patched_document(Some(&json!([{
        "op": "remove",
        "path": "/types/@Order/relationships/@Order-follows-Order/origin"
    }])));
    let (_, _, _, origin) = read_relationships_retaining(&document).expect_err("a missing origin");
    assert_eq!(origin, None);
}

/// A `model` wire node of `form` owned by `owner_node` of the selected
/// package, carrying the structural id of a model declaration node.
fn owner_form_node(package: &Value, case: &Value) -> Value {
    let id = json!({
        "package": package["identity"],
        "path": [case["node_tag"], case["semantic_form"], case["owner_node"]],
        "ordinal": 0
    });
    json!({
        "node_id": id,
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": case["node_tag"],
        "semantic_form": case["semantic_form"],
        "semantic_type": id,
        "dependencies": [],
        "occurrences": [{"role": "generated", "ordinal": 0}],
        "body": {"term": "aggregate", "members": []},
        "owner": {
            "kind": "model",
            "identity": package["identity"],
            "node": case["owner_node"],
        },
    })
}

/// FR-322-AC-72: a process, a state machine and a persistence interface are
/// model declaration nodes. Each recovers its declaration, and the owner
/// join admits a node exactly when the selected document declares its owner as
/// the node's own kind.
#[trace("TC-280", "FR-322-AC-72")]
#[test]
fn tc_280_process_state_machine_and_persistence_owners_join_the_selected_document() {
    let vectors = parsed(OWNER_FORMS);
    let package = &vectors["domain_package"];
    for vector in array(&vectors, "model_declaration_nodes") {
        let name = text(vector, "name");
        assert!(
            recover(
                package,
                &vector["node_id"],
                text(vector, "node_tag"),
                text(vector, "semantic_form"),
            )
            .is_some(),
            "{name} recovers its declaration"
        );
    }
    for case in array(&vectors, "owner_cases") {
        assert_eq!(
            read_wire(package, &owner_form_node(package, case)),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

/// FR-322-AC-82: a declared population is a self-typed `relation`/`population`
/// model declaration node whose `ModelOwner` is the population's declaration
/// id (package, then node). Two populations of one package differ in id, the
/// owner join admits the node exactly when the selected document declares the owner as a
/// population, and an ownerless node fails the V2 schema.
#[trace("TC-280", "FR-322-AC-82")]
#[test]
fn tc_280_a_population_reference_node_is_identified_by_its_declaration_alone() {
    let vectors = parsed(OWNER_FORMS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    let counters = node_vector(nodes, "P7");
    let archive = node_vector(nodes, "P8");
    for vector in [counters, archive] {
        assert_eq!(vector["node_tag"], "relation");
        assert_eq!(vector["semantic_form"], "population");
    }
    assert_ne!(
        counters["node_id"], archive["node_id"],
        "two populations of one package carry different declaration ids"
    );
    let mut ownerless = owner_form_node(
        package,
        &json!({
            "node_tag": "relation", "semantic_form": "population", "owner_node": "ix://acme/runs/counters"
        }),
    );
    assert_eq!(
        read_wire(package, &ownerless),
        json!({"recovers": "ix://acme/runs/counters"})
    );
    ownerless.as_object_mut().expect("a node").remove("owner");
    assert_eq!(
        read_wire(package, &ownerless),
        json!({"refused": {"code": "malformed_wire"}}),
        "a population node without its owner fails the V2 schema"
    );
    for case in array(&vectors, "owner_cases")
        .iter()
        .filter(|case| text(case, "name").starts_with("PR-"))
    {
        assert_eq!(
            read_wire(package, &owner_form_node(package, case)),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
    // Each projected population is an FCD semantic IR 2.0.0 population: the six
    // required members and the optional `symmetric`, and nothing else.
    for population in array(package, "populations") {
        let members: std::collections::BTreeSet<&str> = population
            .as_object()
            .expect("a population is an object")
            .keys()
            .map(String::as_str)
            .collect();
        let required = [
            "displayName",
            "extent",
            "identity",
            "kind",
            "members",
            "origin",
        ];
        assert!(
            required.iter().all(|member| members.contains(member)),
            "a population carries {required:?}"
        );
        assert!(
            members
                .iter()
                .all(|member| required.contains(member) || *member == "symmetric"),
            "a population carries no member beyond the six and `symmetric`: {members:?}"
        );
        assert!(text(population, "identity").starts_with("ix://"));
        assert!(matches!(text(population, "extent"), "closed" | "open"));
        text(&population["kind"], "module");
        text(&population["kind"], "name");
        assert!(array(population, "members").iter().all(Value::is_string));
        let origin = population["origin"].as_object().expect("an origin object");
        assert_eq!(
            origin
                .keys()
                .filter(|key| *key == "source" || *key == "generated")
                .count(),
            1,
            "an origin is exactly one of `source` or `generated`"
        );
    }
}

/// FR-340-AC-19: a frame field entry resolves among the exposed fields of a
/// process or a state machine, own and inherited; a persistence interface and
/// a systems interface are not field-declaring frame entries.
#[trace("TC-280", "FR-340-AC-19")]
#[test]
fn tc_280_frame_field_entries_resolve_among_the_fields_of_a_process_or_state_machine() {
    let vectors = parsed(OWNER_FORMS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "frame_field_cases") {
        let declaration = text(case, "declaration");
        let form = text(node_vector(nodes, declaration), "semantic_form");
        let decided = if matches!(
            form,
            "object_type" | "record_value_type" | "process" | "state_machine"
        ) {
            match resolve_member(
                package,
                nodes,
                declaration,
                "fields",
                text(case, "field"),
                refused("missing_declaration", "missing-name"),
            ) {
                Ok((_, identity, _)) => json!({"resolves": identity}),
                Err(refusal) => refusal,
            }
        } else {
            refused("invalid_model_binding", "malformed-declaration")
        };
        assert_eq!(decided, case["expected"], "{}", text(case, "name"));
    }
}

/// FR-342-AC-7 to FR-342-AC-9: an anchor's operation resolves among the
/// operations its object type, process or state machine declares itself; a
/// persistence interface and a systems interface are not contexts.
#[trace("TC-280", "FR-342-AC-7")]
#[trace("TC-280", "FR-342-AC-8")]
#[trace("TC-280", "FR-342-AC-9")]
#[test]
fn tc_280_an_anchor_context_is_an_object_type_a_process_or_a_state_machine() {
    let vectors = parsed(OWNER_FORMS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "anchor_cases") {
        assert_eq!(
            anchor_operation(
                package,
                nodes,
                text(case, "context"),
                text(case, "operation")
            ),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

/// FR-341-AC-11 and FR-341-AC-12: `self` is `Reference<C>` for a process and a
/// state machine as for an object type, and the clause binds the operation's
/// result and parameters in the same order.
#[trace("TC-280", "FR-341-AC-11")]
#[trace("TC-280", "FR-341-AC-12")]
#[test]
fn tc_280_a_clause_on_a_process_or_state_machine_binds_self_result_and_parameters() {
    let vectors = parsed(OWNER_FORMS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "clause_signature_cases") {
        assert_eq!(
            clause_decision(package, nodes, case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}
