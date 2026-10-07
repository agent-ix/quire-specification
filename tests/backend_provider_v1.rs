// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-331 `quire.backend-provider/v1` schema and vector qualification.
//!
//! The schema fixes the wire's structure; readers convert from it into their
//! own types (QSL ADR-013 T-7). This test checks that every positive vector is
//! admitted, that every adverse vector is refused, and that the request's
//! `package` member carries exactly the checked package its `package_id`
//! names. Reader behaviour beyond the schema (FR-290 negotiation, replay,
//! certificate checks) is not claimed here.

use std::collections::BTreeSet;

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_canonicalize;
use serde_json::Value;
use sha2::{Digest, Sha256};

const SCHEMA: &str = include_str!("../proposals/backend-provider-v1/schema.json");
const VECTORS: &str = include_str!("../proposals/backend-provider-v1/vectors.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const PACKAGE_FIXTURE: &str =
    include_str!("../proposals/checked-package-v2/fixtures/positive-all-families.json");

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

/// RFC 4648 section 5 base64url with no padding.
fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        let symbols = chunk.len() + 1;
        for i in 0..symbols {
            let index = (n >> (18 - 6 * i)) & 0x3f;
            out.push(char::from(ALPHABET[index as usize]));
        }
    }
    out
}

/// The strict decoder FR-331 requires: base64url alphabet, no padding, no
/// length of 1 modulo 4 and zero unused trailing bits, so each byte string has
/// one spelling.
fn decode(text: &str) -> Option<Vec<u8>> {
    let values = text
        .bytes()
        .map(|byte| ALPHABET.iter().position(|symbol| *symbol == byte))
        .collect::<Option<Vec<_>>>()?;
    if values.len() % 4 == 1 {
        return None;
    }
    let mut out = Vec::with_capacity(values.len() * 3 / 4);
    for chunk in values.chunks(4) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0_u32, |n, (i, value)| n | ((*value as u32) << (18 - 6 * i)));
        let bytes = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        let kept = chunk.len() - 1;
        if bytes[kept..].iter().any(|byte| *byte != 0) && chunk.len() < 4 {
            return None;
        }
        out.extend_from_slice(&bytes[..kept]);
    }
    Some(out)
}

fn package_bytes() -> Vec<u8> {
    jcs_canonicalize(&parsed(PACKAGE_FIXTURE)).expect("fixture canonicalizes")
}

fn validator() -> jsonschema::Validator {
    let schema = parsed(SCHEMA);
    jsonschema::meta::validate(&schema).expect("valid Draft 2020-12 schema");
    let registry = jsonschema::Registry::new()
        .add(
            "urn:quire:checked-package:v2",
            parsed(CHECKED_PACKAGE_SCHEMA),
        )
        .expect("checked-package schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&schema)
        .expect("backend-provider schema compiles")
}

/// A positive vector's document; a request-stage vector receives the
/// package fixture's v2 bytes and `package_id` (README "Package bytes").
fn composed(vector: &Value) -> Value {
    let mut document = vector["document"].clone();
    if vector["stage"] == "request" {
        let request = document["request"]
            .as_object_mut()
            .expect("request stage carries request");
        request.insert("package".into(), Value::String(encode(&package_bytes())));
        request.insert(
            "package_id".into(),
            parsed(PACKAGE_FIXTURE)["package_id"].clone(),
        );
    }
    document
}

fn positives() -> Vec<Value> {
    parsed(VECTORS)["positive"]
        .as_array()
        .expect("positive vectors")
        .clone()
}

fn positive(name: &str) -> Value {
    positives()
        .into_iter()
        .find(|vector| vector["name"] == name)
        .unwrap_or_else(|| panic!("positive vector {name} exists"))
}

fn apply(document: &mut Value, patch: &Value) {
    let path = patch["path"].as_str().expect("patch path");
    let (parent, member) = path.rsplit_once('/').expect("patch path has a parent");
    match patch["op"].as_str().expect("patch op") {
        "replace" => {
            *document.pointer_mut(path).expect("replace target exists") = patch["value"].clone();
        }
        "remove" => {
            document
                .pointer_mut(parent)
                .and_then(Value::as_object_mut)
                .expect("remove parent is an object")
                .remove(member)
                .expect("remove target exists");
        }
        "add" => match document.pointer_mut(parent).expect("add parent exists") {
            Value::Object(object) => {
                assert!(
                    object
                        .insert(member.to_owned(), patch["value"].clone())
                        .is_none(),
                    "add creates a new member at {path}"
                );
            }
            Value::Array(array) if member == "-" => array.push(patch["value"].clone()),
            _ => panic!("add parent at {path} is an object or an array append"),
        },
        other => panic!("unsupported patch op {other}"),
    }
}

fn errors(validator: &jsonschema::Validator, document: &Value) -> Vec<String> {
    validator
        .iter_errors(document)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect()
}

/// Every positive vector of every stage is admitted by the schema, and every
/// member it names an owner for exists and names a QSpec FR.
#[trace("TC-431", "FR-331-AC-19")]
#[test]
fn tc_431_positive_vectors_are_admitted_with_owned_members() {
    let validator = validator();
    let owner = regex_free_fr_id;
    for vector in positives() {
        let name = vector["name"].as_str().expect("vector name");
        let document = composed(&vector);
        assert_eq!(
            errors(&validator, &document),
            Vec::<String>::new(),
            "{name} is admitted"
        );
        let owners = vector["owners"].as_object().expect("owners map");
        assert!(!owners.is_empty(), "{name} names member owners");
        for (pointer, fr) in owners {
            assert!(
                document.pointer(pointer).is_some(),
                "{name}: owned member {pointer} exists"
            );
            assert!(
                owner(fr.as_str().expect("owner is text")),
                "{name}: {pointer} names an FR"
            );
        }
    }
}

fn regex_free_fr_id(text: &str) -> bool {
    text.strip_prefix("FR-")
        .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
}

/// Every adverse vector, applied to its base, is refused by the schema,
/// including each Kani row whose cause leaves its result's closed set.
#[trace("TC-431", "FR-331-AC-19")]
#[trace("TC-431", "FR-331-AC-20")]
#[test]
fn tc_431_adverse_vectors_are_refused() {
    let validator = validator();
    let adverse = parsed(VECTORS)["adverse"]
        .as_array()
        .expect("adverse vectors")
        .clone();
    let mut names = BTreeSet::new();
    for vector in adverse {
        let name = vector["name"].as_str().expect("vector name");
        assert!(names.insert(name.to_owned()), "{name} is named once");
        assert!(
            regex_free_fr_id(vector["owner"].as_str().expect("owner")),
            "{name} names its owning FR"
        );
        let mut document = composed(&positive(
            vector["base"].as_str().expect("adverse base name"),
        ));
        for patch in vector["patch"].as_array().expect("patch list") {
            apply(&mut document, patch);
        }
        assert!(
            !validator.is_valid(&document),
            "{name} is refused by the schema"
        );
    }
}

/// The request's `package` is the unpadded base64url of the checked package's
/// RFC 8785 bytes, and decoding it yields the package its `package_id` names.
#[trace("TC-431", "FR-331-AC-18")]
#[test]
fn tc_431_package_member_carries_the_v2_bytes_of_its_package_id() {
    let request = composed(&positive("kani-request"));
    let text = request["request"]["package"]
        .as_str()
        .expect("package is a string");
    assert!(!text.contains('='), "no padding");
    let bytes = decode(text).expect("package decodes under the strict rule");
    assert_eq!(bytes, package_bytes(), "package is the v2 RFC 8785 bytes");
    assert_eq!(encode(&bytes), text, "one spelling per byte string");
    let package: Value = serde_json::from_slice(&bytes).expect("v2 bytes parse");
    assert_eq!(
        jcs_canonicalize(&package).expect("canonicalizes"),
        bytes,
        "the bytes are already canonical"
    );
    assert_eq!(package["contract_version"], "quire.checked-package/v2");
    assert_eq!(
        package["package_id"], request["request"]["package_id"],
        "package_id names the carried package"
    );
}

/// The strict decoder and the schema's `Base64Url` pattern agree: each
/// non-canonical spelling of an adverse vector is refused by both.
#[trace("TC-431", "FR-331-AC-18")]
#[test]
fn tc_431_noncanonical_package_spellings_refuse() {
    for spelling in ["eyJhIjoxfQ==", "+/+/", "eyJhIjoxfR", "eyJhI"] {
        assert_eq!(decode(spelling), None, "{spelling} refuses");
    }
    for bytes in [&b""[..], b"{", b"{}", b"{\"a\"", b"#[kani::proof]"] {
        let text = encode(bytes);
        assert_eq!(decode(&text).as_deref(), Some(bytes), "{text} round-trips");
    }
}

/// Every base64url member of every positive vector decodes, and each
/// byte-provision entry's digest is the sha256-jcs digest of its bytes.
#[trace("TC-431", "FR-331-AC-18")]
#[test]
fn tc_431_vector_byte_members_decode_and_provision_digests_bind() {
    for vector in positives() {
        let document = composed(&vector);
        for artifact in document["artifacts"].as_array().into_iter().flatten() {
            decode(artifact["bytes"].as_str().expect("artifact bytes"))
                .expect("artifact bytes decode");
        }
        for entry in document["request"]["provision"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let bytes = decode(entry["bytes"].as_str().expect("provision bytes"))
                .expect("provision bytes decode");
            let digest = Sha256::digest(&bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>();
            assert_eq!(entry["digest"], digest.as_str(), "provision digest binds");
        }
    }
}

/// The Kani vectors carry a result for each row of FR-331's Kani results
/// table: each of the eight results, the `bounded-proof` basis and each
/// cause; `measured` comes from the statistical vector.
#[trace("TC-431", "FR-331-AC-20")]
#[test]
fn tc_431_kani_vectors_cover_every_terminal_value() {
    let mut kani = BTreeSet::new();
    let mut every = BTreeSet::new();
    let mut causes = BTreeSet::new();
    for vector in positives()
        .iter()
        .filter(|vector| vector["stage"] == "outcome")
    {
        for result in vector["document"]["results"].as_array().expect("results") {
            let value = result["result"].as_str().expect("result value").to_owned();
            every.insert(value.clone());
            if result["backend"] == "kani" {
                kani.insert(value);
                for member in ["cause", "basis"] {
                    if let Some(spelling) = result[member]["type"].as_str() {
                        causes.insert(spelling.to_owned());
                    }
                }
            }
        }
    }
    let terminal = [
        "declined",
        "failed",
        "incomplete",
        "inconclusive",
        "proved",
        "refuted",
        "tested",
        "unsupported",
    ];
    assert_eq!(kani, terminal.iter().map(|v| (*v).to_owned()).collect());
    for cause in [
        "bounded-proof",
        "kani_vacuous_proof",
        "limit-reached",
        "timed-out",
        "cancelled",
        "tool-failure",
        "refused",
        "invalid-input",
        "incomplete-input",
    ] {
        assert!(causes.contains(cause), "a Kani vector carries {cause}");
    }
    assert!(every.contains("measured"), "a vector carries measured");
}

/// The item-level defects FR-290 settles per item (absent kind, unknown
/// backend, unknown evidence kind) are admitted on the wire, while an item
/// without `candidates` is refused before consumption.
#[trace("TC-431", "FR-331-AC-4")]
#[test]
fn tc_431_item_level_causes_reach_negotiation() {
    let validator = validator();
    let document = composed(&positive("request-item-level-defects"));
    assert!(validator.is_valid(&document));
    let mut missing = document.clone();
    apply(
        &mut missing,
        &serde_json::json!({"op": "remove", "path": "/request/items/0/candidates"}),
    );
    assert!(!validator.is_valid(&missing));
}
