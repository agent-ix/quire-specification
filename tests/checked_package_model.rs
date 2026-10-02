// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Complete-model definition lock and effective-declaration vector qualification.

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_sha256;
use serde_json::Value;
use std::collections::BTreeSet;

const EFFECTIVE_DECLARATION_VECTORS: &str =
    include_str!("../proposals/checked-package-v2/model-effective-declaration-vectors.json");
const EFFECTIVE_DECLARATION_SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/model-effective-declaration.schema.json");

const EFFECTIVE_DECLARATION_DOMAIN: &str = "quire.model.effective-declaration/v1";

fn parsed_fixture(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

fn unqualified_jcs_sha256(value: &Value) -> String {
    jcs_sha256(value)
        .expect("authored preimage is canonicalizable")
        .strip_prefix("sha256-jcs:")
        .expect("canonicalizer retains its selected prefix")
        .to_owned()
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

#[trace("TC-195", "FR-150-AC-6")]
#[test]
fn tc_195_effective_declaration_vectors_recompute_their_digests() {
    let file = parsed_fixture(EFFECTIVE_DECLARATION_VECTORS);
    let vectors = file["vectors"].as_array().expect("vectors is an array");
    let mut names = BTreeSet::new();
    for vector in vectors {
        let name = str_at(vector, "/name");
        assert!(names.insert(name), "vector names are unique");
        assert_eq!(
            unqualified_jcs_sha256(&vector["preimage"]),
            str_at(vector, "/sha256"),
            "{name} digest is the JCS SHA-256 of its preimage"
        );
    }

    let digests = vectors
        .iter()
        .map(|vector| str_at(vector, "/sha256"))
        .collect::<BTreeSet<_>>();
    for view in ["n01-view", "n02-view", "n01v2-view"] {
        let vector = vectors
            .iter()
            .find(|vector| vector["name"].as_str() == Some(view))
            .unwrap_or_else(|| panic!("{view} is published"));
        let declarations = vector["preimage"]["declarations"]
            .as_array()
            .expect("view declarations are an array");
        let mut previous: Option<&str> = None;
        for declaration in declarations {
            let effective_id = str_at(declaration, "/effective_id/digest");
            assert_eq!(
                str_at(declaration, "/effective_id/domain"),
                EFFECTIVE_DECLARATION_DOMAIN
            );
            assert_eq!(
                unqualified_jcs_sha256(&declaration["preimage"]),
                effective_id,
                "{view} entry identity is recomputed from its preimage"
            );
            assert!(
                digests.contains(effective_id),
                "{view} entry is a published declaration vector"
            );
            assert!(
                previous < Some(effective_id),
                "{view} declarations ascend strictly by effective identity"
            );
            previous = Some(effective_id);
        }
    }

    for mutation in file["invalid_mutations"]
        .as_array()
        .expect("invalid mutations are an array")
    {
        let name = str_at(mutation, "/name");
        let base_name = str_at(mutation, "/base");
        let base = vector_named(vectors, base_name);
        assert_eq!(
            str_at(mutation, "/retained_sha256"),
            str_at(base, "/sha256"),
            "{name} retains exactly its base vector {base_name}'s digest"
        );
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

    for view in ["n01-view", "n02-view", "n01v2-view"] {
        let preimage = &vector_named(vectors, view)["preimage"];
        let package = str_at(preimage, "/model_selection/identity");
        assert_eq!(
            str_at(preimage, "/model_selection/digest_domain"),
            "sha256-jcs"
        );
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

    let first = &vector_named(vectors, "n01-view")["preimage"];
    let second = &vector_named(vectors, "n01v2-view")["preimage"];
    assert_eq!(
        first["model_selection"]["identity"], second["model_selection"]["identity"],
        "both views select one domain package identity"
    );
    assert_eq!(
        first["declarations"], second["declarations"],
        "equal nodes under a new package digest keep every declaration key and effective declaration"
    );
    assert_ne!(
        str_at(first, "/model_selection/digest"),
        str_at(second, "/model_selection/digest"),
        "the two documents are selected by different digests"
    );
    assert_ne!(
        str_at(vector_named(vectors, "n01-view"), "/sha256"),
        str_at(vector_named(vectors, "n01v2-view"), "/sha256"),
        "the effective view binds the ModelSelection"
    );
    assert_ne!(
        str_at(vector_named(vectors, "n01-universe"), "/sha256"),
        str_at(vector_named(vectors, "n01v2-universe"), "/sha256"),
        "the object universe binds the ModelSelection"
    );
}
