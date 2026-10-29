// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Effective-declaration vector qualification.

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_sha256;
use serde_json::Value;
use std::collections::BTreeSet;

const EFFECTIVE_DECLARATION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-effective-declaration-vectors.json");
const EFFECTIVE_DECLARATION_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/model-effective-declaration.schema.json");

const EFFECTIVE_DECLARATION_DOMAIN: &str = "quire.model.effective-declaration/v2";

fn parsed_fixture(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn str_at<'a>(value: &'a Value, pointer: &str) -> &'a str {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("authored member {pointer} is a string"))
}

/// FR-204 key bytes of an identity component: the UTF-8 domain string
/// immediately followed by the 32 digest bytes.
fn identity_key_bytes(component: &Value) -> Vec<u8> {
    let digest = str_at(component, "/digest");
    assert_eq!(digest.len(), 64, "a digest is 32 bytes of lowercase hex");
    let mut bytes = str_at(component, "/domain").as_bytes().to_vec();
    bytes.extend(digest.as_bytes().chunks(2).map(|pair| {
        let pair = std::str::from_utf8(pair).expect("hex digest is ASCII");
        u8::from_str_radix(pair, 16).expect("digest is hexadecimal")
    }));
    bytes
}

fn reference_key(entry: &Value) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    (
        identity_key_bytes(&entry["universe"]),
        identity_key_bytes(&entry["type"]),
        str_at(entry, "/object").as_bytes().to_vec(),
    )
}

/// Every inherit fact links to the one-edge parent it names: a type fact to
/// the type of its direct supertype, a member fact to the same original
/// member in that supertype. A qualify fact links nothing, and no two inherit
/// facts of one declaration share a parent.
#[trace("TC-195", "FR-150-AC-12")]
#[test]
fn tc_195_inherit_links_name_one_edge_parents() {
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let vectors = file["vectors"].as_array().expect("vectors is an array");
    for view in ["n01-view", "n02-view"] {
        let declarations = vector_named(vectors, view)["preimage"]["declarations"]
            .as_array()
            .expect("view declarations are an array");
        let by_id = declarations
            .iter()
            .map(|entry| (str_at(entry, "/effective_id/digest"), &entry["preimage"]))
            .collect::<std::collections::BTreeMap<_, _>>();
        let type_id = |node: &str| {
            declarations
                .iter()
                .find(|entry| {
                    entry["preimage"]["owner_effective_type"].is_null()
                        && str_at(&entry["preimage"], "/original/node") == node
                })
                .map(|entry| str_at(entry, "/effective_id/digest"))
                .unwrap_or_else(|| panic!("{view} holds type {node}"))
        };
        for entry in declarations {
            let preimage = &entry["preimage"];
            let mut parents = BTreeSet::new();
            for fact in preimage["derivation"].as_array().expect("derivation") {
                let rule = str_at(fact, "/rule/identity");
                if rule != "quire.model.normalize.inherit/v2" {
                    assert!(fact["parent"].is_null(), "{view} {rule} links nothing");
                    continue;
                }
                assert_eq!(
                    str_at(fact, "/parent/domain"),
                    EFFECTIVE_DECLARATION_DOMAIN
                );
                let parent_id = str_at(fact, "/parent/digest");
                assert!(parents.insert(parent_id), "{view} parents are distinct");
                let parent = by_id
                    .get(parent_id)
                    .unwrap_or_else(|| panic!("{view} parent is in the view"));
                let inputs = fact["inputs"].as_array().expect("fact inputs");
                let (supertype, original) = (&inputs[0], &inputs[inputs.len() - 1]);
                assert_eq!(&parent["original"], original, "{view} link keeps its original");
                if preimage["owner_effective_type"].is_null() {
                    assert_eq!(inputs.len(), 1, "a type link names one supertype");
                } else {
                    assert_eq!(inputs.len(), 2, "a member link names supertype and member");
                    assert_eq!(
                        str_at(&parent["owner_effective_type"], "/digest"),
                        type_id(str_at(supertype, "/node")),
                        "{view} member link lands in the named supertype"
                    );
                }
            }
        }
    }
}

#[trace("TC-198", "FR-144-AC-11")]
#[test]
fn tc_198_reference_key_order_ascends_by_fr_204_key_bytes() {
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let order = file["reference_key_order"]
        .as_array()
        .expect("reference key order is an array");
    assert!(order.len() >= 2);
    let keys = order.iter().map(reference_key).collect::<Vec<_>>();
    assert!(
        keys.windows(2).all(|pair| pair[0] < pair[1]),
        "published keys ascend strictly by universe, type and object bytes"
    );
    let mut reversed = keys.clone();
    reversed.reverse();
    reversed.sort();
    assert_eq!(reversed, keys, "sorting any permutation restores the order");
}

fn vector_named<'a>(vectors: &'a [Value], name: &str) -> &'a Value {
    vectors
        .iter()
        .find(|vector| vector["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("{name} is published"))
}

/// Every declaration key a declaration preimage names: its original and each
/// derivation fact input.
fn declaration_keys(preimage: &Value) -> Vec<&Value> {
    let mut keys = vec![&preimage["original"]];
    for fact in preimage["derivation"]
        .as_array()
        .expect("derivation is an array")
    {
        keys.extend(fact["inputs"].as_array().expect("fact inputs are an array"));
    }
    keys
}

#[trace("TC-234", "FR-154-AC-4")]
#[test]
fn tc_234_declaration_keys_name_the_domain_package_and_ir_node_only() {
    let schema = jsonschema::validator_for(&parsed_fixture(EFFECTIVE_DECLARATION_SCHEMA))
        .expect("effective declaration schema is valid");
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let vectors = file["vectors"].as_array().expect("vectors is an array");
    for vector in vectors {
        let name = str_at(vector, "/name");
        schema
            .validate(&vector["preimage"])
            .unwrap_or_else(|error| panic!("{name} preimage is schema-valid: {error}"));
    }

    for view in ["n01-view", "n02-view"] {
        let preimage = &vector_named(vectors, view)["preimage"];
        let package = str_at(preimage, "/model_selection/identity");
        for declaration in preimage["declarations"]
            .as_array()
            .expect("view declarations are an array")
        {
            for key in declaration_keys(&declaration["preimage"]) {
                assert_eq!(
                    str_at(key, "/package"),
                    package,
                    "{view} keys name its package"
                );
                assert!(
                    str_at(key, "/node").starts_with("ix://"),
                    "{view} keys name an IR node identity"
                );
                assert!(
                    key.get("digest").is_none(),
                    "a declaration key carries no digest"
                );
            }
        }
    }

}
