// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Producer structure and real B canonicalizer parity, not a shared reader.
//! Fixture loading keeps the last value of a raw duplicate key. Strict intake
//! and semantic matching remain IT-003-SC-03.

use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};

use ix_trace_rs::trace;
use quire_verification_contracts::{jcs_canonicalize, jcs_sha256};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const ROOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/proposals/shared-reference-2-draft"
);
/// Caller limits on one qualification fixture, with published defaults
/// (IT-003): bytes read and JSON values (nodes) parsed. Nesting depth is not a
/// limit: the parser keeps its open containers on the heap.
#[derive(Clone, Copy, Debug)]
struct IntakeLimits {
    bytes: usize,
    nodes: usize,
}

impl IntakeLimits {
    const DEFAULT: Self = Self {
        bytes: 1024 * 1024,
        nodes: 65_536,
    };
}

#[derive(Debug, PartialEq, Eq)]
enum IntakeFailure {
    /// The named limit was reached at its configured value; `raise` names the
    /// `IntakeLimits` field a caller raises to admit the input.
    Incomplete {
        limit: &'static str,
        value: usize,
        raise: &'static str,
    },
    Invalid,
}

impl IntakeFailure {
    fn bytes(limits: IntakeLimits) -> Self {
        Self::Incomplete {
            limit: "bytes",
            value: limits.bytes,
            raise: "IntakeLimits::bytes",
        }
    }

    fn nodes(limits: IntakeLimits) -> Self {
        Self::Incomplete {
            limit: "nodes",
            value: limits.nodes,
            raise: "IntakeLimits::nodes",
        }
    }
}

fn intake(bytes: &[u8]) -> Result<Value, IntakeFailure> {
    intake_with(bytes, IntakeLimits::DEFAULT)
}

/// One open container while parsing: an array's items so far, or an object's
/// members so far and the key awaiting its value.
enum Open {
    Array(Vec<Value>),
    Object(serde_json::Map<String, Value>, Option<String>),
}

fn skip_whitespace(bytes: &[u8], at: &mut usize) {
    while bytes
        .get(*at)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
    {
        *at += 1;
    }
}

/// The end of the JSON string token starting at `at` (its opening quote).
fn string_end(bytes: &[u8], at: usize) -> Result<usize, IntakeFailure> {
    let mut index = at + 1;
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'"' => return Ok(index + 1),
            b'\\' => index += 2,
            _ => index += 1,
        }
    }
    Err(IntakeFailure::Invalid)
}

/// The end of the number or literal token starting at `at`.
fn scalar_end(bytes: &[u8], at: usize) -> usize {
    let mut index = at;
    while bytes
        .get(index)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
    {
        index += 1;
    }
    index
}

/// RFC 8259 JSON intake bounded only by the caller's byte and node limits.
/// Strings, numbers and literals are decoded by `serde_json` one token at a
/// time, so a lone surrogate, `NaN` or trailing content is `Invalid`; a raw
/// duplicate key keeps its last value.
///
/// The parse never builds a value it then drops recursively: on refusal every
/// partly built value is released by [`dispose`]. A caller holding a deeply
/// nested result releases it with [`dispose`] too.
fn intake_with(bytes: &[u8], limits: IntakeLimits) -> Result<Value, IntakeFailure> {
    if bytes.len() > limits.bytes {
        return Err(IntakeFailure::bytes(limits));
    }
    let mut stack: Vec<Open> = Vec::new();
    let parsed = parse(bytes, limits, &mut stack);
    for open in stack {
        match open {
            Open::Array(items) => items.into_iter().for_each(dispose),
            Open::Object(fields, _) => fields.into_iter().for_each(|(_, value)| dispose(value)),
        }
    }
    parsed
}

/// Releases a value without recursion: children move onto a heap stack
/// before their container is dropped, so nesting depth never reaches the
/// call stack.
fn dispose(value: Value) {
    let mut pending = vec![value];
    while let Some(current) = pending.pop() {
        match current {
            Value::Array(items) => pending.extend(items),
            Value::Object(fields) => pending.extend(fields.into_iter().map(|(_, value)| value)),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
}

/// The parse proper; `stack` holds the open containers and is left for the
/// caller to release when the parse refuses.
fn parse(
    bytes: &[u8],
    limits: IntakeLimits,
    stack: &mut Vec<Open>,
) -> Result<Value, IntakeFailure> {
    let mut nodes = 0_usize;
    let mut at = 0;
    let root = loop {
        skip_whitespace(bytes, &mut at);
        if let Some(Open::Object(_, key @ None)) = stack.last_mut() {
            match bytes.get(at) {
                Some(b'}') => {}
                Some(b'"') => {
                    let end = string_end(bytes, at)?;
                    *key = Some(
                        serde_json::from_slice(&bytes[at..end])
                            .map_err(|_| IntakeFailure::Invalid)?,
                    );
                    at = end;
                    skip_whitespace(bytes, &mut at);
                    if bytes.get(at) != Some(&b':') {
                        return Err(IntakeFailure::Invalid);
                    }
                    at += 1;
                    continue;
                }
                _ => return Err(IntakeFailure::Invalid),
            }
        }
        let closes = matches!(
            (stack.last(), bytes.get(at)),
            (Some(Open::Array(_)), Some(b']')) | (Some(Open::Object(_, None)), Some(b'}'))
        );
        let value = if closes {
            at += 1;
            match stack.pop() {
                Some(Open::Array(items)) => Value::Array(items),
                Some(Open::Object(fields, _)) => Value::Object(fields),
                None => return Err(IntakeFailure::Invalid),
            }
        } else {
            nodes += 1;
            if nodes > limits.nodes {
                return Err(IntakeFailure::nodes(limits));
            }
            match bytes.get(at) {
                Some(b'[') => {
                    at += 1;
                    stack.push(Open::Array(Vec::new()));
                    skip_whitespace(bytes, &mut at);
                    continue;
                }
                Some(b'{') => {
                    at += 1;
                    stack.push(Open::Object(serde_json::Map::new(), None));
                    continue;
                }
                Some(b'"') => {
                    let end = string_end(bytes, at)?;
                    let value = serde_json::from_slice(&bytes[at..end])
                        .map_err(|_| IntakeFailure::Invalid)?;
                    at = end;
                    value
                }
                Some(_) => {
                    let end = scalar_end(bytes, at);
                    let value = serde_json::from_slice(&bytes[at..end])
                        .map_err(|_| IntakeFailure::Invalid)?;
                    at = end;
                    value
                }
                None => return Err(IntakeFailure::Invalid),
            }
        };
        skip_whitespace(bytes, &mut at);
        match stack.last_mut() {
            None => break value,
            Some(Open::Array(items)) => {
                items.push(value);
                if bytes.get(at) == Some(&b',') {
                    at += 1;
                    skip_whitespace(bytes, &mut at);
                    if bytes.get(at) == Some(&b']') {
                        return Err(IntakeFailure::Invalid);
                    }
                } else if bytes.get(at) != Some(&b']') {
                    return Err(IntakeFailure::Invalid);
                }
            }
            Some(Open::Object(fields, key)) => {
                let Some(name) = key.take() else {
                    dispose(value);
                    return Err(IntakeFailure::Invalid);
                };
                if let Some(replaced) = fields.insert(name, value) {
                    dispose(replaced);
                }
                if bytes.get(at) == Some(&b',') {
                    at += 1;
                    skip_whitespace(bytes, &mut at);
                    if bytes.get(at) != Some(&b'"') {
                        return Err(IntakeFailure::Invalid);
                    }
                } else if bytes.get(at) != Some(&b'}') {
                    return Err(IntakeFailure::Invalid);
                }
            }
        }
    };
    skip_whitespace(bytes, &mut at);
    if at == bytes.len() {
        Ok(root)
    } else {
        dispose(root);
        Err(IntakeFailure::Invalid)
    }
}

/// The authored bytes of a fixture under `ROOT`, read up to the default byte
/// limit; a larger file is the named `bytes` limit.
fn bytes(name: &str) -> Result<Vec<u8>, IntakeFailure> {
    assert!(
        Path::new(name)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    );
    let limits = IntakeLimits::DEFAULT;
    let file = File::open(Path::new(ROOT).join(name)).expect("required authored fixture");
    let mut bytes = Vec::new();
    file.take(u64::try_from(limits.bytes + 1).expect("bounded constant"))
        .read_to_end(&mut bytes)
        .expect("read fixture");
    if bytes.len() > limits.bytes {
        return Err(IntakeFailure::bytes(limits));
    }
    Ok(bytes)
}

fn load(name: &str) -> Value {
    intake(&bytes(name).expect("fixture within the byte limit"))
        .expect("valid bounded authored fixture")
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
fn six_role_shapes_have_exact_roles() {
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
    assert_eq!(semantic["value"]["canonicalIdentity"]["digest"], digest);
    assert_eq!(
        semantic["value"]["semanticProfile"]["definitionDigest"],
        format!(
            "sha256:{:x}",
            Sha256::digest(bytes("fixtures/profile.json").expect("fixture within the byte limit"))
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
fn qualification_fixture_size_node_and_json_syntax_controls() {
    for raw in [
        br#""\ud800""#.as_slice(),
        b"NaN",
        b"{} {}",
        b"[0,]",
        b"{\"a\":0,}",
        b"01",
        b"[0",
    ] {
        assert_eq!(intake(raw), Err(IntakeFailure::Invalid), "{raw:?}");
    }
    assert_eq!(
        intake(br#"{"a":[true,null,{"b":"c"}],"d":-1.5e3}"#),
        Ok(json!({"a": [true, null, {"b": "c"}], "d": -1.5e3}))
    );
    let defaults = IntakeLimits::DEFAULT;
    let mut exact = vec![b' '; defaults.bytes];
    exact[0] = b'0';
    assert_eq!(intake(&exact), Ok(json!(0)));
    exact.push(b' ');
    assert_eq!(
        intake(&exact),
        Err(IntakeFailure::Incomplete {
            limit: "bytes",
            value: defaults.bytes,
            raise: "IntakeLimits::bytes",
        })
    );
    let limits = IntakeLimits {
        nodes: 3,
        ..defaults
    };
    assert_eq!(intake_with(b"[0,0]", limits), Ok(json!([0, 0])));
    for over in [b"[0,0,0]".as_slice(), b"[[[0]]]"] {
        assert_eq!(
            intake_with(over, limits),
            Err(IntakeFailure::Incomplete {
                limit: "nodes",
                value: 3,
                raise: "IntakeLimits::nodes",
            })
        );
    }
}

/// Nesting is bounded only by the node limit: 1000 levels, far past any
/// fixed parser depth, are admitted under the defaults.
#[test]
#[trace("IT-003-SC-01")]
fn qualification_fixture_nesting_is_not_a_limit() {
    let depth = 1000;
    let raw = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
    let mut value = intake(raw.as_bytes()).expect("deep nesting is admitted");
    for _ in 0..depth {
        value = value
            .as_array_mut()
            .and_then(Vec::pop)
            .expect("each level holds one array");
    }
    assert_eq!(value, json!(0));
}

/// A 40,000-level array, inside the default limits, parses and is released
/// on the default test-thread stack, and so is a refused input that has
/// already built a 40,000-level value.
#[test]
#[trace("IT-003-SC-01")]
fn qualification_fixture_deep_values_are_released_without_recursion() {
    let depth = 40_000;
    let raw = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
    let value = intake(raw.as_bytes()).expect("deep nesting is admitted");
    let mut levels = 0;
    let mut current = &value;
    while let Some(inner) = current.as_array().and_then(|items| items.first()) {
        levels += 1;
        current = inner;
    }
    assert_eq!((levels, current), (depth, &json!(0)));
    dispose(value);

    let trailing = format!("[{raw} x");
    assert_eq!(intake(trailing.as_bytes()), Err(IntakeFailure::Invalid));
    let unclosed = format!("[{raw}");
    assert_eq!(intake(unclosed.as_bytes()), Err(IntakeFailure::Invalid));
}
