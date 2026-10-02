// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-322 "Model-owned members" vector qualification (TC-280): owner recovery
//! by recomputed model declaration node keys, member resolution in the
//! declaring type's exposed effective members and member type derivation.
//! TC-281 applies the published operation catalog's `quire.op.reference.eq`,
//! `quire.op.reference.ne`, `quire.op.model.deref` and
//! `quire.op.record.project` entries over the same recovered owners:
//! reference equality over conforming object types and a model field
//! projection over a `deref` result.

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_sha256;
use quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-member-type-vectors.json");
const PREIMAGE_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/node-identity-preimage.schema.json");
/// The catalog lives only in `quire-verification-contracts`; it is read from there.
const OPERATION_CATALOG: &str = CHECKED_OPERATION_CATALOG_V1;
const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

fn parsed(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn digest(value: &Value) -> String {
    jcs_sha256(value)
        .expect("authored value is canonicalizable")
        .strip_prefix("sha256-jcs:")
        .expect("canonicalizer retains its selected prefix")
        .to_owned()
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

fn ineligible() -> Value {
    refused("ill_typed", "operator-ineligible")
}

/// The `ModelDeclarationNode` preimage FR-322 step 2 computes.
fn declaration_preimage(package: &Value, tag: &str, form: &str, node: &str) -> Value {
    json!({
        "version": "quire.structural-node/v1",
        "node_tag": tag,
        "semantic_form": form,
        "semantic_type": null,
        "declaration": null,
        "recursion": null,
        "owner": {
            "kind": "model",
            "identity": package["identity"],
            "node": node
        },
        "body": {"term": "aggregate", "members": []}
    })
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
    objects.chain(relationships).collect()
}

fn declaration_key(package: &Value, declared: &Declared<'_>) -> String {
    digest(&declaration_preimage(
        package,
        declared.tag,
        declared.form,
        declared.node,
    ))
}

/// FR-322 step 2 over a wire node's key, tag and form: the matching
/// declaration of the selected package.
fn recover<'a>(package: &'a Value, key: &str, tag: &str, form: &str) -> Option<Declared<'a>> {
    declarations(package).into_iter().find(|declared| {
        declared.tag == tag && declared.form == form && declaration_key(package, declared) == key
    })
}

/// FR-322 step 2's fixed members of a matched wire node.
fn fixed_members_hold(node: &Value) -> bool {
    node["semantic_type"] == node["node_id"]
        && node.get("declaration").is_none()
        && node.get("recursion_group").is_none()
        && node["body"] == json!({"term": "aggregate", "members": []})
}

fn object_type<'a>(package: &'a Value, node: &str) -> Option<&'a Value> {
    array(package, "object_types")
        .iter()
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
        let integer = text(value_type, "native") == "ix://quire/native/Integer";
        return (integer && bound("lower")? <= bound("upper")?).then(
            || json!({"type": "Int", "min": value_type["lower"], "max": value_type["upper"]}),
        );
    }
    let target = declarations(package)
        .into_iter()
        .find(|declared| declared.form == "object_type" && declared.node == type_ref)?;
    Some(json!({"type": "Reference", "target": declaration_key(package, &target)}))
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

/// FR-322 steps 2 to 4 for one member case, as its expectation spells them.
fn derive(package: &Value, nodes: &[Value], member: &Value) -> Value {
    let declaring = nodes
        .iter()
        .find(|vector| vector["name"] == member["declaration"])
        .expect("a case names a published node vector");
    let preimage = &declaring["preimage"];
    let Some(recovered) = recover(
        package,
        text(declaring, "sha256"),
        text(preimage, "node_tag"),
        text(preimage, "semantic_form"),
    ) else {
        return refused("missing_declaration", "missing-selection");
    };
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
        let receiver = json!({"type": "Reference", "target": declaring["sha256"]});
        if member["receiver"] != receiver {
            return ineligible();
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

/// FR-322 step 2 over one wire node.
fn recover_wire(package: &Value, node: &Value) -> Value {
    assert_eq!(node["node_id"]["domain"], NODE_DOMAIN);
    let Some(recovered) = recover(
        package,
        text(&node["node_id"], "digest"),
        text(node, "node_tag"),
        text(node, "semantic_form"),
    ) else {
        return refused("missing_declaration", "missing-selection");
    };
    if !fixed_members_hold(node) {
        return refused("invalid_package", "stale-node-key");
    }
    json!({"recovers": recovered.node})
}

/// FR-322 step 1's admission checks over one selection case.
fn select(package: &Value, case: &Value) -> Value {
    let document_digest = digest(package);
    let resolve = |value: &str| {
        if value == "$document" {
            document_digest.clone()
        } else {
            value.to_owned()
        }
    };
    let selection = &case["selection"];
    if text(selection, "digest_domain") != "sha256-jcs" {
        return refused("stale_dependency", "digest-domain-mismatch");
    }
    let selected = resolve(text(selection, "digest"));
    let supplied: BTreeMap<String, Value> = array(case, "supplied")
        .iter()
        .map(|entry| {
            let mut document = package.clone();
            for patch in array(entry, "patch") {
                *document
                    .pointer_mut(text(patch, "path"))
                    .expect("a patch names an existing member") = patch["value"].clone();
            }
            (resolve(text(entry, "under")), document)
        })
        .collect();
    let Some(document) = supplied.get(&selected) else {
        return refused("missing_import", "missing-selection");
    };
    if digest(document) != selected {
        return refused("stale_dependency", "byte-digest-mismatch");
    }
    if document["identity"] != selection["identity"] || document["version"] != selection["version"]
    {
        return refused("invalid_model_binding", "wrong-model-selection");
    }
    json!({"admission": "passed"})
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

fn assert_member_cases(vectors: &Value, criterion: &str) {
    let nodes = array(vectors, "model_declaration_nodes");
    for case in cases_for(vectors, criterion) {
        assert_eq!(
            derive(&vectors["domain_package"], nodes, &case["member"]),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
}

#[trace("TC-280", "FR-322-AC-29")]
#[test]
fn tc_280_model_declaration_node_vectors_recompute_and_recover_only_selected_owners() {
    let vectors = parsed(VECTORS);
    let schema =
        jsonschema::validator_for(&parsed(PREIMAGE_SCHEMA)).expect("node preimage schema is valid");
    let package = &vectors["domain_package"];
    for vector in array(&vectors, "model_declaration_nodes") {
        let name = text(vector, "name");
        let preimage = &vector["preimage"];
        schema
            .validate(preimage)
            .unwrap_or_else(|error| panic!("{name} validates: {error}"));
        assert_eq!(digest(preimage), text(vector, "sha256"), "{name} digest");
        let selected = preimage["owner"]["identity"] == package["identity"];
        let recovered = recover(
            package,
            text(vector, "sha256"),
            text(preimage, "node_tag"),
            text(preimage, "semantic_form"),
        );
        assert_eq!(
            recovered.is_some(),
            selected,
            "{name} recovers exactly when its owner is the selected domain package"
        );

        let mut named_body = preimage.clone();
        named_body["body"]["members"] = json!([{"term": "binding", "name": "total",
            "value": {"term": "aggregate", "members": []}}]);
        assert!(
            !schema.is_valid(&named_body),
            "{name} with a body member is refused"
        );
        let mut declared = preimage.clone();
        declared["declaration"] = json!({"qualified_name": ["M", "Order"]});
        assert!(
            !schema.is_valid(&declared),
            "{name} with a declaration is refused"
        );
    }
    let mut crossed = array(&vectors, "model_declaration_nodes")[3]["preimage"].clone();
    crossed["semantic_form"] = json!("object_type");
    assert!(
        !schema.is_valid(&crossed),
        "a relation node keyed as an object type is refused"
    );

    for case in array(&vectors, "wire_cases") {
        assert_eq!(
            recover_wire(package, &case["node"]),
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

#[trace("TC-280", "FR-322-AC-32")]
#[test]
fn tc_280_selected_domain_package_documents_are_verified_evidence() {
    let vectors = parsed(VECTORS);
    let cases = array(&vectors, "selection_cases");
    let causes: BTreeSet<_> = cases
        .iter()
        .filter_map(|case| case["expected"]["refused"]["cause"].as_str())
        .collect();
    assert_eq!(
        causes,
        BTreeSet::from([
            "digest-domain-mismatch",
            "missing-selection",
            "byte-digest-mismatch",
            "wrong-model-selection"
        ])
    );
    for case in cases {
        assert_eq!(
            select(&vectors["domain_package"], case),
            case["expected"],
            "{}",
            text(case, "name")
        );
    }
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
}

impl<'a> Operand<'a> {
    fn from_case(value: &'a Value) -> Self {
        match (value["reference"].as_str(), value["object"].as_str()) {
            (Some(node), None) => Self::Reference(node),
            (None, Some(node)) => Self::Object(node),
            _ => panic!("an operand names a reference or an object"),
        }
    }

    /// FR-322's family of the operand's type node: `reference` for a
    /// reference, `object` for an object type or systems interface model
    /// declaration node, and none for a relation node.
    fn family(self, nodes: &[Value]) -> Option<&'static str> {
        match self {
            Self::Reference(_) => Some("reference"),
            Self::Object(name) => {
                let preimage = &node_vector(nodes, name)["preimage"];
                match (text(preimage, "node_tag"), text(preimage, "semantic_form")) {
                    ("model", "object_type" | "systems_interface") => Some("object"),
                    _ => None,
                }
            }
        }
    }
}

/// FR-322 step 2 over a named node vector.
fn recover_vector<'a>(package: &'a Value, nodes: &[Value], name: &str) -> Option<Declared<'a>> {
    let vector = node_vector(nodes, name);
    let preimage = &vector["preimage"];
    recover(
        package,
        text(vector, "sha256"),
        text(preimage, "node_tag"),
        text(preimage, "semantic_form"),
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
    let derived = derive(package, nodes, member);
    match derived.get("member_type") {
        Some(member_type) if *member_type != case["result_type"] => ineligible(),
        _ => derived,
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

/// FR-342-AC-4 over one anchor case: the context is an object type, and
/// the operation resolves among its exposed operation members to one the
/// context itself declares.
fn anchor_operation(package: &Value, nodes: &[Value], context: &str, operation: &str) -> Value {
    let form = text(&node_vector(nodes, context)["preimage"], "semantic_form");
    if form != "object_type" {
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

#[trace("TC-280", "FR-341-AC-5")]
#[test]
fn tc_280_a_clause_binds_self_result_and_parameters_of_its_operation() {
    let vectors = parsed(VECTORS);
    let package = &vectors["domain_package"];
    let nodes = array(&vectors, "model_declaration_nodes");
    for case in array(&vectors, "clause_signature_cases") {
        let supplied: Vec<Value> = array(case, "parameters")
            .iter()
            .map(|parameter| match parameter["reference"].as_str() {
                Some(name) => {
                    json!({"type": "Reference", "target": node_vector(nodes, name)["sha256"]})
                }
                None => parameter.clone(),
            })
            .collect();
        let decided = if clause_signature(package, nodes, case).as_ref() == Some(&supplied) {
            json!({"admitted": true})
        } else {
            ineligible()
        };
        assert_eq!(decided, case["expected"], "{}", text(case, "name"));
    }
}
