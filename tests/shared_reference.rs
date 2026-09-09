// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Agent-IX

//! Producer structure and real B canonicalizer parity, not a shared reader.
//! Fixture loading reuses B's legacy bounded parser, which does not reject raw
//! duplicate keys. Strict intake and semantic matching remain IT-003-SC-03.

use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};

use ix_trace_rs::trace;
use quire_verification::contracts::{jcs_canonicalize, jcs_sha256, parse_bounded_json};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const ROOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proposals/shared-reference-2-draft"
);
const MAX_BYTES: usize = 1024 * 1024;
const MAX_DEPTH: usize = 64;

#[derive(Debug, PartialEq, Eq)]
enum IntakeFailure {
    Incomplete,
    Invalid,
}

fn intake(bytes: &[u8]) -> Result<Value, IntakeFailure> {
    if bytes.len() > MAX_BYTES {
        return Err(IntakeFailure::Incomplete);
    }
    let value = parse_bounded_json(bytes).map_err(|_| IntakeFailure::Invalid)?;
    let mut pending = vec![(&value, 0)];
    while let Some((node, depth)) = pending.pop() {
        if depth > MAX_DEPTH {
            return Err(IntakeFailure::Incomplete);
        }
        match node {
            Value::Array(items) => pending.extend(items.iter().map(|item| (item, depth + 1))),
            Value::Object(fields) => {
                pending.extend(fields.values().map(|item| (item, depth + 1)));
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
    Ok(value)
}

fn bytes(name: &str) -> Vec<u8> {
    assert!(
        Path::new(name)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    );
    let file = File::open(Path::new(ROOT).join(name)).expect("required authored fixture");
    let mut bytes = Vec::new();
    file.take(u64::try_from(MAX_BYTES + 1).expect("bounded constant"))
        .read_to_end(&mut bytes)
        .expect("read fixture");
    assert!(
        bytes.len() <= MAX_BYTES,
        "fixture exceeds qualification budget"
    );
    bytes
}

fn load(name: &str) -> Value {
    intake(&bytes(name)).expect("valid bounded authored fixture")
}

fn fixture(name: &str) -> Value {
    load(&format!("fixtures/{name}"))
}

fn validator() -> jsonschema::Validator {
    let schema = load("schema.json");
    jsonschema::meta::validate(&schema).expect("valid Draft 2020-12 schema");
    jsonschema::validator_for(&schema).expect("valid local closed schema")
}

#[test]
#[trace("IT-003-SC-01")]
fn six_role_shapes_and_actual_bytes() {
    let schema = validator();
    let legacy = intake(include_bytes!(
        "../proposals/state-core/schemas/shared-reference-1-draft.schema.json"
    ))
    .expect("unchanged adopted schema");
    let legacy = jsonschema::validator_for(&legacy).expect("valid historical schema");
    let roles = fixture("roles.json");
    let roles = roles.as_array().expect("role inventory");
    assert_eq!(roles.len(), 6);
    let mut observed = std::collections::BTreeSet::new();
    for role in roles {
        let envelope = &role["envelope"];
        schema.validate(envelope).expect("positive role shape");
        let name = role["file"].as_str().expect("fixture filename");
        let actual = bytes(&format!("fixtures/{name}"));
        assert_eq!(
            envelope["value"]["digest"],
            format!("sha256:{:x}", Sha256::digest(actual))
        );
        assert_eq!(envelope["value"]["kind"], role["expectedRole"]);
        assert!(observed.insert(role["expectedRole"].as_str().expect("role")));
        let mut previous = envelope.clone();
        previous["wireVersion"] = json!("ix.shared-reference/1-draft");
        previous["value"]["refVersion"] = json!("ix.artifact-ref/2-draft");
        assert!(
            !legacy.is_valid(&previous),
            "historical vocabulary stays closed"
        );
    }
    assert_eq!(
        observed,
        [
            "trace",
            "fault-model",
            "review-procedure",
            "oracle",
            "observation",
            "review-disposition"
        ]
        .into_iter()
        .collect()
    );
}

#[test]
#[trace("IT-003-SC-01")]
fn structural_negatives_and_semantic_boundary() {
    let schema = validator();
    let semantic = fixture("semantic.json");
    schema.validate(&semantic).expect("positive JCS shape");
    let adverse = fixture("adverse.json");
    for case in adverse["structuralMutations"]
        .as_array()
        .expect("mutations")
    {
        let mut candidate = fixture(case["base"].as_str().expect("base"));
        *candidate
            .pointer_mut(case["pointer"].as_str().expect("pointer"))
            .expect("existing field") = case["replacement"].clone();
        assert!(!schema.is_valid(&candidate), "{case}");
    }
    for field in [
        "artifact",
        "language",
        "semanticProfile",
        "requiredFeatures",
        "canonicalIdentity",
    ] {
        let mut candidate = semantic.clone();
        candidate["value"]
            .as_object_mut()
            .expect("semantic value")
            .remove(field);
        assert!(!schema.is_valid(&candidate), "missing {field}");
    }
    let mut extra = semantic.clone();
    for case in adverse["semanticMutations"]
        .as_array()
        .expect("semantic controls")
    {
        if case["base"] == "semantic.json" {
            let mut candidate = semantic.clone();
            *candidate
                .pointer_mut(case["pointer"].as_str().expect("pointer"))
                .expect("existing field") = case["replacement"].clone();
            assert!(
                schema.is_valid(&candidate),
                "semantic refusals are not schema refusals"
            );
        }
    }
    for features in [
        json!([]),
        json!(["a"]),
        json!(["a", "b"]),
        json!(["b", "a"]),
    ] {
        let mut candidate = semantic.clone();
        candidate["value"]["requiredFeatures"] = features;
        assert!(
            schema.is_valid(&candidate),
            "unique feature sets have valid shape"
        );
    }
    extra["value"]["extra"] = json!(true);
    assert!(!schema.is_valid(&extra));
    let roles = fixture("roles.json");
    for role in roles.as_array().expect("roles") {
        for replacement in adverse["roleSubstitutions"]["replaceKindWith"]
            .as_array()
            .expect("kinds")
        {
            if replacement != &role["expectedRole"] {
                let mut candidate = role["envelope"].clone();
                candidate["value"]["kind"] = replacement.clone();
                assert!(
                    schema.is_valid(&candidate),
                    "a wrong role remains structurally valid"
                );
            }
        }
    }
    for domain in [
        "quire.contract.canonical-json",
        "quire.contract.bound-identity",
    ] {
        let mut candidate = semantic.clone();
        candidate["value"]["canonicalIdentity"] = json!({"domain":domain,"version":"v1","algorithm":"sha256","digest":"0000000000000000000000000000000000000000000000000000000000000000"});
        assert!(schema.is_valid(&candidate));
        candidate["value"]["canonicalIdentity"]["digest"] =
            semantic["value"]["canonicalIdentity"]["digest"].clone();
        assert!(!schema.is_valid(&candidate));
    }
}

#[test]
#[trace("IT-003-SC-02")]
fn independently_authored_canonical_bytes_match_b() {
    let vectors = fixture("canonical-vectors.json");
    for vector in vectors.as_array().expect("canonical vectors") {
        let input =
            intake(vector["input"].as_str().expect("input text").as_bytes()).expect("valid JSON");
        let expected = vector["canonical"]
            .as_str()
            .expect("golden bytes")
            .as_bytes();
        assert_eq!(
            jcs_canonicalize(&input).expect("B canonicalizer"),
            expected,
            "{}",
            vector["id"]
        );
        assert_eq!(
            jcs_sha256(&input).expect("B digest"),
            format!("sha256-jcs:{:x}", Sha256::digest(expected))
        );
    }
    assert_eq!(
        jcs_canonicalize(&json!({"a":1,"b":2})).unwrap(),
        jcs_canonicalize(&intake(br#"{"b":2,"a":1}"#).unwrap()).unwrap()
    );
    assert_ne!(
        jcs_canonicalize(&json!([2, 1])).unwrap(),
        jcs_canonicalize(&json!([1, 2])).unwrap()
    );
}

#[test]
#[trace("IT-003-SC-02")]
fn synthetic_core_is_the_non_circular_preimage() {
    let core = fixture("core.json");
    jsonschema::validator_for(&fixture("core.schema.json"))
        .unwrap()
        .validate(&core)
        .unwrap();
    let semantic = fixture("semantic.json");
    let digest = jcs_sha256(&core).expect("B core digest");
    assert_eq!(
        digest,
        "sha256-jcs:70a89fec8afa6a435d5a92f63ba9ce1d01e86b17791629bb788628ee7bfb81d5"
    );
    assert_eq!(semantic["value"]["canonicalIdentity"]["digest"], digest);
    assert_eq!(
        semantic["value"]["artifact"]["digest"],
        format!("sha256:{:x}", Sha256::digest(bytes("fixtures/core.json")))
    );
    assert_eq!(
        semantic["value"]["semanticProfile"]["definitionDigest"],
        format!(
            "sha256:{:x}",
            Sha256::digest(bytes("fixtures/profile.json"))
        )
    );
    let disposition = fixture("review-disposition.json");
    assert_eq!(disposition["coreDigest"], digest);
    let wrapper = json!({"automatedCore":core,"disposition":disposition});
    assert_eq!(jcs_sha256(&wrapper["automatedCore"]).unwrap(), digest);
    assert_ne!(jcs_sha256(&wrapper).unwrap(), digest);
    let mut changed = wrapper["automatedCore"].clone();
    changed["reportId"] = json!("foreign-core");
    assert_ne!(jcs_sha256(&changed).unwrap(), digest);
}

#[test]
#[trace("IT-003-SC-01")]
fn qualification_fixture_size_depth_and_json_syntax_controls() {
    for raw in [br#""\ud800""#.as_slice(), b"NaN", b"{} {}"] {
        assert_eq!(intake(raw), Err(IntakeFailure::Invalid));
    }
    let mut exact = vec![b' '; MAX_BYTES];
    exact[0] = b'0';
    assert_eq!(intake(&exact), Ok(json!(0)));
    exact.push(b' ');
    assert_eq!(intake(&exact), Err(IntakeFailure::Incomplete));
    for depth in [MAX_DEPTH, MAX_DEPTH + 1] {
        let raw = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        assert_eq!(intake(raw.as_bytes()).is_ok(), depth == MAX_DEPTH);
        if depth > MAX_DEPTH {
            assert_eq!(intake(raw.as_bytes()), Err(IntakeFailure::Incomplete));
        }
    }
}
