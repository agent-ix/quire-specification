// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-322 "Model-owned members" vector qualification (TC-280): owner recovery
//! by recomputed model declaration node keys, member resolution in the
//! declaring type's exposed effective members and member type derivation.

use ix_trace_rs::trace;
use quire_verification::contracts::jcs_sha256;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-member-type-vectors.json");
const PREIMAGE_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/node-identity-preimage.schema.json");
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
            "version": package["version"],
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
        let selected = preimage["owner"]["version"] == package["version"];
        let recovered = recover(
            package,
            text(vector, "sha256"),
            text(preimage, "node_tag"),
            text(preimage, "semantic_form"),
        );
        assert_eq!(
            recovered.is_some(),
            selected,
            "{name} recovers exactly when its owner is the selected version"
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
