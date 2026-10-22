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
use quire_verification_contracts::{jcs_canonicalize, jcs_sha256};
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

fn registry() -> jsonschema::Registry<'static> {
    jsonschema::Registry::new()
        .add(
            "urn:quire:checked-package:v2",
            parsed(CHECKED_PACKAGE_SCHEMA),
        )
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares")
}

fn validator() -> jsonschema::Validator {
    let schema = parsed(SCHEMA);
    jsonschema::meta::validate(&schema).expect("valid Draft 2020-12 schema");
    let registry = registry();
    jsonschema::options()
        .with_registry(&registry)
        .build(&schema)
        .expect("backend-provider schema compiles")
}

/// A validator for one definition of the backend-provider schema.
fn definition(name: &str) -> jsonschema::Validator {
    let registry = registry();
    jsonschema::options()
        .with_registry(&registry)
        .build(&serde_json::json!({"$ref": format!("urn:quire:backend-provider:v1#/$defs/{name}")}))
        .expect("definition compiles")
}

fn stage_definition(stage: &str) -> &'static str {
    match stage {
        "manifest" => "ManifestStage",
        "request" => "RequestStage",
        "outcome" => "OutcomeStage",
        other => panic!("unknown stage {other}"),
    }
}

/// The ids of every FR artifact under `spec/`, read from the file names.
fn spec_fr_ids() -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut pending = vec![std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/spec"
    ))];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("spec directory reads") {
            let path = entry.expect("spec entry reads").path();
            if path.is_dir() {
                pending.push(path);
            } else if let Some(name) = path.file_name().and_then(|name| name.to_str())
                && let Some(rest) = name.strip_prefix("FR-")
            {
                let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
                if !digits.is_empty() && rest[digits.len()..].starts_with('-') {
                    ids.insert(format!("FR-{digits}"));
                }
            }
        }
    }
    ids
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PatchError {
    InvalidOperation,
    InvalidPath,
    MissingValue,
    MissingParent,
    InvalidParent,
    InvalidArrayIndex,
    IndexOutOfBounds,
    MissingTarget,
    ExistingTarget,
}

fn patch_token(token: &str) -> Result<String, PatchError> {
    let mut decoded = String::new();
    let mut chars = token.chars();
    while let Some(character) = chars.next() {
        decoded.push(match character {
            '~' => match chars.next() {
                Some('0') => '~',
                Some('1') => '/',
                _ => return Err(PatchError::InvalidPath),
            },
            other => other,
        });
    }
    Ok(decoded)
}

fn patch_index(member: &str) -> Result<usize, PatchError> {
    if member.is_empty() || (member.len() > 1 && member.starts_with('0'))
        || !member.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(PatchError::InvalidArrayIndex);
    }
    member.parse().map_err(|_| PatchError::InvalidArrayIndex)
}

/// The fixture protocol supports non-root add/remove/replace JSON pointers;
/// object adds retain the existing new-member-only invariant. Array adds
/// insert at a checked index or append at '-'.
/// Errors are fixture defects, never evidence of schema refusal. No missing
/// value becomes null; invalid operations leave the document unchanged.
fn apply(document: &mut Value, patch: &Value) -> Result<(), PatchError> {
    let op = patch.get("op").and_then(Value::as_str).ok_or(PatchError::InvalidOperation)?;
    if !matches!(op, "add" | "remove" | "replace") {
        return Err(PatchError::InvalidOperation);
    }
    let path = patch.get("path").and_then(Value::as_str).ok_or(PatchError::InvalidPath)?;
    let tokens = path.strip_prefix('/').ok_or(PatchError::InvalidPath)?;
    for token in tokens.split('/') {
        patch_token(token)?;
    }
    let (parent, member) = path.rsplit_once('/').ok_or(PatchError::InvalidPath)?;
    let member = patch_token(member)?;
    let value = if op == "remove" {
        None
    } else {
        Some(patch.get("value").ok_or(PatchError::MissingValue)?.clone())
    };
    match document.pointer_mut(parent).ok_or(PatchError::MissingParent)? {
        Value::Object(object) => match op {
            "remove" => {
                object.remove(&member).ok_or(PatchError::MissingTarget)?;
            }
            "add" => {
                if object.contains_key(&member) {
                    return Err(PatchError::ExistingTarget);
                }
                object.insert(member, value.ok_or(PatchError::MissingValue)?);
            }
            "replace" => {
                *object.get_mut(&member).ok_or(PatchError::MissingTarget)? =
                    value.ok_or(PatchError::MissingValue)?;
            }
            _ => return Err(PatchError::InvalidOperation),
        },
        Value::Array(array) => {
            let index = if op == "add" && member == "-" {
                array.len()
            } else {
                patch_index(&member)?
            };
            match op {
                "remove" => {
                    if index >= array.len() {
                        return Err(PatchError::IndexOutOfBounds);
                    }
                    array.remove(index);
                }
                "add" => {
                    if index > array.len() {
                        return Err(PatchError::IndexOutOfBounds);
                    }
                    array.insert(index, value.ok_or(PatchError::MissingValue)?);
                }
                "replace" => {
                    *array.get_mut(index).ok_or(PatchError::IndexOutOfBounds)? =
                        value.ok_or(PatchError::MissingValue)?;
                }
                _ => return Err(PatchError::InvalidOperation),
            }
        }
        _ => return Err(PatchError::InvalidParent),
    }
    Ok(())
}

fn errors(validator: &jsonschema::Validator, document: &Value) -> Vec<String> {
    validator
        .iter_errors(document)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect()
}

/// Literal outputs are independent of the patch runner and its branches.
#[test]
fn patch_operations_preserve_literal_results() {
    for (document, patch, expected) in [
        (serde_json::json!([10, 20, 30]),
            serde_json::json!({"op": "remove", "path": "/1"}),
            serde_json::json!([10, 30])),
        (serde_json::json!([10, 20, 30]),
            serde_json::json!({"op": "remove", "path": "/0"}),
            serde_json::json!([20, 30])),
        (serde_json::json!([10, 20, 30]),
            serde_json::json!({"op": "remove", "path": "/2"}),
            serde_json::json!([10, 20])),
        (serde_json::json!([10, 30]),
            serde_json::json!({"op": "add", "path": "/1", "value": 20}),
            serde_json::json!([10, 20, 30])),
        (serde_json::json!([20]),
            serde_json::json!({"op": "add", "path": "/0", "value": 10}),
            serde_json::json!([10, 20])),
        (serde_json::json!([10]),
            serde_json::json!({"op": "add", "path": "/1", "value": 20}),
            serde_json::json!([10, 20])),
        (serde_json::json!([]),
            serde_json::json!({"op": "add", "path": "/-", "value": null}),
            serde_json::json!([null])),
        (serde_json::json!([10, 20]),
            serde_json::json!({"op": "replace", "path": "/1", "value": null}),
            serde_json::json!([10, null])),
        (serde_json::json!({"keep": 1, "remove": null}),
            serde_json::json!({"op": "remove", "path": "/remove"}),
            serde_json::json!({"keep": 1})),
        (serde_json::json!({"keep": 1}),
            serde_json::json!({"op": "add", "path": "/new", "value": null}),
            serde_json::json!({"keep": 1, "new": null})),
        (serde_json::json!({"keep": 1}),
            serde_json::json!({"op": "replace", "path": "/keep", "value": 2}),
            serde_json::json!({"keep": 2})),
        (serde_json::json!({"a/b": {"~key": [10, 20]}}),
            serde_json::json!({"op": "remove", "path": "/a~1b/~0key/0"}),
            serde_json::json!({"a/b": {"~key": [20]}})),
        (serde_json::json!({"a/b": 1, "~key": 2}),
            serde_json::json!({"op": "remove", "path": "/a~1b"}),
            serde_json::json!({"~key": 2})),
    ] {
        let mut actual = document;
        assert_eq!(apply(&mut actual, &patch), Ok(()), "literal patch applies");
        assert_eq!(actual, expected, "literal patch result for {patch}");
    }
}

/// Malformed fixture patches fail explicitly, without partial mutation or
/// laundering a runner error into an adverse schema-refusal success.
#[test]
fn invalid_patches_are_errors_and_leave_the_document_unchanged() {
    let original = serde_json::json!({"array": [10, 20], "empty": [],
        "object": {"existing": 1}, "scalar": 7});
    for (patch, expected) in [
        (serde_json::json!({"op": "remove", "path": "/object/missing"}), PatchError::MissingTarget),
        (serde_json::json!({"op": "replace", "path": "/object/missing", "value": 3}), PatchError::MissingTarget),
        (serde_json::json!({"op": "add", "path": "/object/existing", "value": 2}), PatchError::ExistingTarget),
        (serde_json::json!({"op": "remove", "path": "/array/2"}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "replace", "path": "/array/2", "value": 3}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "add", "path": "/array/3", "value": 3}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "remove", "path": "/empty/0"}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "replace", "path": "/empty/0", "value": 3}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "add", "path": "/empty/1", "value": 3}), PatchError::IndexOutOfBounds),
        (serde_json::json!({"op": "move", "path": "/array/0"}), PatchError::InvalidOperation),
        (serde_json::json!({"path": "/array/0"}), PatchError::InvalidOperation),
        (serde_json::json!({"op": 1, "path": "/array/0"}), PatchError::InvalidOperation),
        (serde_json::json!({"op": "remove"}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": 1}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": "array/0"}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": ""}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": "/object/~"}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": "/object/~2"}), PatchError::InvalidPath),
        (serde_json::json!({"op": "remove", "path": "/bad~2/child"}), PatchError::InvalidPath),
    ] {
        let mut actual = original.clone();
        assert_eq!(apply(&mut actual, &patch), Err(expected), "fixture error for {patch}");
        assert_eq!(actual, original, "failed patch cannot mutate {patch}");
    }
    for op in ["add", "remove", "replace"] {
        for member in ["", "01", "+1", "-1", "one", "184467440737095516160"] {
            let patch = serde_json::json!({"op": op, "path": format!("/array/{member}"), "value": 3});
            let mut actual = original.clone();
            assert_eq!(apply(&mut actual, &patch), Err(PatchError::InvalidArrayIndex));
            assert_eq!(actual, original);
        }
        for (path, expected) in [("/missing/child", PatchError::MissingParent),
            ("/scalar/child", PatchError::InvalidParent)] {
            let patch = serde_json::json!({"op": op, "path": path, "value": 3});
            let mut actual = original.clone();
            assert_eq!(apply(&mut actual, &patch), Err(expected));
            assert_eq!(actual, original);
        }
    }
    for op in ["remove", "replace"] {
        let mut actual = original.clone();
        assert_eq!(apply(&mut actual, &serde_json::json!({"op": op,
            "path": "/array/-", "value": 3})), Err(PatchError::InvalidArrayIndex));
        assert_eq!(actual, original);
    }
    for op in ["add", "replace"] {
        let mut actual = original.clone();
        assert_eq!(apply(&mut actual, &serde_json::json!({"op": op,
            "path": "/array/0"})), Err(PatchError::MissingValue));
        assert_eq!(actual, original);
    }
}

/// The real adverse vector retains n=0 and changes m=2 to m=1 only.
#[trace("TC-431", "FR-331-AC-27")]
#[test]
fn hp3_array_removal_is_refused_and_restoration_is_admitted() {
    let base = composed(&positive("certificate-rejected-hyper-identities"));
    let whole = validator();
    assert!(whole.is_valid(&base), "original positive is admitted");
    let adverse = parsed(VECTORS);
    let vector = adverse["adverse"].as_array().expect("adverse vectors").iter()
        .find(|vector| vector["name"] == "hp3-empty-universal-single-existential")
        .expect("original adverse vector retained");
    assert_eq!(vector["base"], "certificate-rejected-hyper-identities");
    assert_eq!(vector["patch"], serde_json::json!([
        {"op": "remove", "path": "/results/4/cause/at/witness_set/0/states/1"}
    ]));
    let mut actual = base.clone();
    assert_eq!(apply(&mut actual, &vector["patch"][0]), Ok(()), "real removal applies");
    assert_eq!(actual["results"][4]["cause"]["at"], serde_json::json!({
        "states": [], "automaton_state": "AQ", "witness_set": [{
            "states": ["1111111111111111111111111111111111111111111111111111111111111111"],
            "automaton_state": "Ag"
        }]
    }));
    assert!(!whole.is_valid(&actual), "n=0, m=1 is a real negative");
    let stage = definition("OutcomeStage");
    assert!(stage.iter_errors(&actual).any(|error| located_at_patch(
        &error.instance_path().to_string(), "/results/4/cause/at/witness_set/0/states/1")),
        "refusal belongs to the changed witness tuple");
    assert_eq!(apply(&mut actual, &serde_json::json!({"op": "add",
        "path": "/results/4/cause/at/witness_set/0/states/1",
        "value": "2222222222222222222222222222222222222222222222222222222222222222"})),
        Ok(()), "restoring insertion applies");
    assert_eq!(actual, base, "restoration changes no other component");
    assert!(whole.is_valid(&actual), "restored n=0, m=2 positive is admitted");
}

/// Every positive vector of every stage is admitted by the schema, and every
/// member it names an owner for exists and names an FR of this specification.
#[trace("TC-431", "FR-331-AC-19")]
#[trace("TC-431", "FR-331-AC-23")]
#[trace("TC-431", "FR-331-AC-24")]
#[trace("TC-431", "FR-331-AC-25")]
#[trace("TC-431", "FR-331-AC-28")]
#[trace("TC-431", "FR-364-AC-10")]
#[test]
fn tc_431_positive_vectors_are_admitted_with_owned_members() {
    let validator = validator();
    let frs = spec_fr_ids();
    for vector in positives() {
        let name = vector["name"].as_str().expect("vector name");
        let document = composed(&vector);
        assert_eq!(
            errors(&validator, &document),
            Vec::<String>::new(),
            "{name} is admitted"
        );
        // FR-331: exactly one disposition and one accounting record per item.
        if let (Some(dispositions), Some(accounting)) = (
            document["dispositions"].as_array(),
            document["accounting"].as_array(),
        ) {
            if !dispositions.is_empty() && !accounting.is_empty() {
                let indices = |records: &[Value]| -> Vec<u64> {
                    let mut found: Vec<u64> = records
                        .iter()
                        .map(|record| record["request_index"].as_u64().expect("request index"))
                        .collect();
                    found.sort_unstable();
                    found
                };
                let (dispositions, accounting) = (indices(dispositions), indices(accounting));
                let mut unique = accounting.clone();
                unique.dedup();
                assert_eq!(unique, accounting, "{name}: one accounting record per item");
                assert_eq!(
                    accounting, dispositions,
                    "{name}: accounting covers exactly the dispositions' items"
                );
            }
        }
        let owners = vector["owners"].as_object().expect("owners map");
        assert!(!owners.is_empty(), "{name} names member owners");
        for (pointer, fr) in owners {
            assert!(
                document.pointer(pointer).is_some(),
                "{name}: owned member {pointer} exists"
            );
            let fr = fr.as_str().expect("owner is text");
            assert!(
                frs.contains(fr),
                "{name}: {pointer} names {fr}, an FR of spec/"
            );
        }
    }
}

/// Whether a schema error at `error` lies at the patched member: the member
/// itself, a member under it, or an ancestor other than the document root
/// (the root only for a patch of a top-level member).
fn located_at_patch(error: &str, patched: &str) -> bool {
    let top_level = patched.matches('/').count() == 1;
    error == patched
        || error.starts_with(&format!("{patched}/"))
        || ((!error.is_empty() || top_level) && patched.starts_with(&format!("{error}/")))
}

/// Every adverse vector is one patch of one positive vector and is refused
/// by its stage's schema at the patched member, including each Kani row whose
/// cause leaves its result's closed set.
#[trace("TC-431", "FR-331-AC-19")]
#[trace("TC-431", "FR-331-AC-20")]
#[trace("TC-431", "FR-331-AC-23")]
#[trace("TC-431", "FR-331-AC-24")]
#[trace("TC-431", "FR-331-AC-25")]
#[trace("TC-431", "FR-331-AC-28")]
#[trace("TC-431", "FR-368-AC-10")]
#[trace("TC-431", "FR-364-AC-10")]
#[test]
fn tc_431_adverse_vectors_are_refused() {
    let whole = validator();
    let frs = spec_fr_ids();
    let adverse = parsed(VECTORS)["adverse"]
        .as_array()
        .expect("adverse vectors")
        .clone();
    let mut names = BTreeSet::new();
    for vector in adverse {
        let name = vector["name"].as_str().expect("vector name");
        assert!(names.insert(name.to_owned()), "{name} is named once");
        let owner = vector["owner"].as_str().expect("owner");
        assert!(frs.contains(owner), "{name} names {owner}, an FR of spec/");
        let base = positive(vector["base"].as_str().expect("adverse base name"));
        let patch = vector["patch"].as_array().expect("patch list");
        assert_eq!(patch.len(), 1, "{name} changes one member");
        let mut document = composed(&base);
        assert_eq!(apply(&mut document, &patch[0]), Ok(()), "{name}: authored patch applies");
        assert!(
            !whole.is_valid(&document),
            "{name} is refused by the schema"
        );
        let path = patch[0]["path"].as_str().expect("patch path");
        let patched = path.strip_suffix("/-").unwrap_or(path);
        let stage = definition(stage_definition(base["stage"].as_str().expect("stage")));
        let locations: Vec<String> = stage
            .iter_errors(&document)
            .map(|error| error.instance_path().to_string())
            .collect();
        assert!(
            locations
                .iter()
                .any(|location| located_at_patch(location, patched)),
            "{name} is refused at {patched}, not elsewhere: {locations:?}"
        );
    }
}

/// The request's `package` is the unpadded base64url of the checked package's
/// RFC 8785 bytes, and the package identity recomputed from the decoded bytes
/// is the request's `package_id`.
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
    // FR-322: the package-id preimage is the shipped `identity_preimage` with
    // the identity projection derived from the graph, every node in graph
    // order without its source occurrences.
    let mut preimage = package["identity_preimage"].clone();
    preimage["identity_projection"] = Value::Array(
        package["semantic_graph"]["nodes"]
            .as_array()
            .expect("graph nodes are an array")
            .iter()
            .map(|node| {
                let mut projected = node.clone();
                projected
                    .as_object_mut()
                    .expect("node is an object")
                    .remove("occurrences");
                projected
            })
            .collect(),
    );
    let recomputed = jcs_sha256(&preimage).expect("preimage canonicalizes");
    let recomputed = recomputed
        .strip_prefix("sha256-jcs:")
        .expect("canonicalizer retains its prefix");
    let package_id = &request["request"]["package_id"];
    assert_eq!(package_id["domain"], "quire.package.semantic/v2");
    assert_eq!(package_id["algorithm"], "sha256");
    assert_eq!(
        package_id["digest"], recomputed,
        "package_id is the identity recomputed from the carried bytes"
    );
}

/// The strict decoder and the schema's `Base64Url` pattern agree: both refuse
/// each non-canonical spelling and both admit every encoding.
#[trace("TC-431", "FR-331-AC-18")]
#[test]
fn tc_431_noncanonical_package_spellings_refuse() {
    let pattern = definition("Base64Url");
    for spelling in ["eyJhIjoxfQ==", "+/+/", "eyJhIjoxfR", "eyJhI", "eyJhIjoxfQ="] {
        assert_eq!(decode(spelling), None, "the decoder refuses {spelling}");
        assert!(
            !pattern.is_valid(&Value::String(spelling.into())),
            "the schema refuses {spelling}"
        );
    }
    let mut samples: Vec<Vec<u8>> = (0..=255_u8).map(|byte| vec![byte]).collect();
    samples.extend((0..=255_u8).map(|byte| vec![0xff, byte]));
    samples.extend((0..=255_u8).map(|byte| vec![byte, 0x00, 0xff]));
    samples.push(Vec::new());
    samples.push(b"#[kani::proof]".to_vec());
    for bytes in samples {
        let text = encode(&bytes);
        assert_eq!(decode(&text), Some(bytes.clone()), "{text} round-trips");
        assert!(
            pattern.is_valid(&Value::String(text.clone())),
            "the schema admits {text}"
        );
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

/// The row of FR-331's Kani results table a Kani result entry stands for.
fn kani_row(result: &Value, counterexamples: &[Value]) -> String {
    let value = result["result"].as_str().expect("result value");
    if let Some(spelling) = result["basis"]["type"].as_str() {
        return format!("{value}:{spelling}");
    }
    if let Some(position) = result["counterexample"].as_u64() {
        let entry = &counterexamples[usize::try_from(position).expect("position fits")];
        let arm = if entry.get("transcript").is_some() {
            "transcript"
        } else {
            "counterexample"
        };
        return format!("{value}:{arm}");
    }
    let cause = result["cause"]["type"].as_str();
    let diagnostic = result["diagnostic"]["code"].as_str().map(|code| {
        let detail = &result["diagnostic"]["detail"];
        let absent = if detail.get("absent").is_some() {
            "+absent"
        } else {
            ""
        };
        format!(
            "{code}/{}{absent}",
            result["diagnostic"]["cause"]
                .as_str()
                .expect("diagnostic cause")
        )
    });
    match (cause, diagnostic) {
        (Some("limit-reached"), _) => {
            let setting = result["cause"]["setting"].as_str().expect("setting");
            assert!(setting.starts_with("kani."), "Kani setting {setting}");
            format!("{value}:limit-reached")
        }
        (Some(cause), _) => format!("{value}:{cause}"),
        (None, Some(diagnostic)) => format!("{value}:{diagnostic}"),
        (None, None) => value.to_owned(),
    }
}

/// The Kani vectors carry one result for each row of FR-331's Kani results
/// table, each `declined` cause separately.
#[trace("TC-431", "FR-331-AC-20")]
#[test]
fn tc_431_kani_vectors_cover_every_table_row() {
    let mut rows = BTreeSet::new();
    for vector in positives()
        .iter()
        .filter(|vector| vector["stage"] == "outcome")
    {
        let document = &vector["document"];
        let counterexamples = document["counterexamples"]
            .as_array()
            .expect("counterexamples");
        for result in document["results"].as_array().expect("results") {
            if result["backend"] == "kani" {
                rows.insert(kani_row(result, counterexamples));
            }
        }
    }
    let table: BTreeSet<String> = [
        "proved:bounded-proof",
        "inconclusive:kani_vacuous_proof",
        "refuted:transcript",
        "declined:refused",
        "declined:invalid-input",
        "declined:incomplete-input",
        "unsupported:unsupported_projection/tool-unavailable+absent",
        "incomplete:timed-out",
        "incomplete:limit-reached",
        "incomplete:cancelled",
        "inconclusive:replay-parity",
        "tested",
        "failed:tool-failure",
        "failed:unsupported_projection/tool-unavailable",
        "failed:runtime_invariant/established-invariant-broken",
    ]
    .iter()
    .map(|row| (*row).to_owned())
    .collect();
    assert_eq!(rows, table, "one Kani vector result per table row");
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
    assert_eq!(apply(
        &mut missing,
        &serde_json::json!({"op": "remove", "path": "/request/items/0/candidates"}),
    ), Ok(()), "candidate removal applies");
    assert!(!validator.is_valid(&missing));
}

/// The bound kind QSL's `DomainKind::finite_kind` pairs with each ADR-014
/// domain kind, or `None` for a kind no finite bound stands for.
fn finite_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "collection" | "population" => Some("cardinality"),
        "integer" => Some("integer-range"),
        "recursive" => Some("depth"),
        "quantity" | "loop" | "infinite-trace" => None,
        other => panic!("unknown domain kind {other}"),
    }
}

/// Each of the seven ADR-014 domain kinds paired with each of the three
/// bound types: a `ProofBound` is admitted exactly when the bound is the one
/// its kind takes, and a missing or unknown kind is refused.
#[trace("TC-431", "FR-331-AC-21")]
#[test]
fn tc_431_proof_bound_kind_and_bound_pair_as_adr_014_pairs_them() {
    let proof_bound = definition("ProofBound");
    let domain = serde_json::json!({"type": "node", "node": {"domain": "quire.checked-semantic-node/v1", "digest": "1c9ac70521e54c69c310ff337b10364e2c3c80c917efb8b29010c97a9534bd14"}, "path": []});
    let bounds = [
        (
            "cardinality",
            serde_json::json!({"type": "cardinality", "maximum": 4}),
        ),
        (
            "integer-range",
            serde_json::json!({"type": "integer-range", "lower": "0", "upper": "9"}),
        ),
        ("depth", serde_json::json!({"type": "depth", "maximum": 3})),
    ];
    let kinds = [
        "collection",
        "population",
        "integer",
        "recursive",
        "quantity",
        "loop",
        "infinite-trace",
    ];
    let mut admitted = 0;
    for kind in kinds {
        for (bound_type, bound) in &bounds {
            let candidate = serde_json::json!({"domain": domain, "kind": kind, "bound": bound});
            let expected = finite_kind(kind) == Some(*bound_type);
            assert_eq!(
                proof_bound.is_valid(&candidate),
                expected,
                "{kind} with {bound_type}"
            );
            admitted += usize::from(expected);
        }
    }
    assert_eq!(admitted, 4, "exactly four of the 21 pairs are admitted");
    let (_, bound) = &bounds[1];
    assert!(!proof_bound.is_valid(&serde_json::json!({"domain": domain, "bound": bound})));
    assert!(
        !proof_bound
            .is_valid(&serde_json::json!({"domain": domain, "kind": "string", "bound": bound}))
    );
}

/// A descriptor that advertises a `bounded` mode carries a nonempty
/// `domains`; one without it is refused (the FR-290 `invalid-domains`
/// registration refusal), and an unbounded-only descriptor may omit it.
/// A present `domains` that is empty, holds a non-boundable or unknown kind
/// or repeats a kind is refused on every descriptor, unbounded-only included.
#[trace("TC-431", "FR-331-AC-22")]
#[test]
fn tc_431_bounded_descriptors_carry_domains() {
    let descriptor = definition("BackendDescriptor");
    let mut bounded = 0;
    for vector in positives() {
        for entry in vector["document"]["manifest"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let advertises = entry["advertises"].as_array().expect("advertises");
            if advertises.iter().any(|pair| pair["mode"] == "bounded") {
                let domains = entry["domains"]
                    .as_array()
                    .expect("bounded descriptor has domains");
                assert!(!domains.is_empty(), "domains is nonempty");
                bounded += 1;
            }
        }
    }
    assert!(bounded > 0, "a bounded descriptor is exercised");
    let unbounded_only = serde_json::json!({
        "backend": "qsl-model-check",
        "advertises": [{"kind": "temporal-satisfaction", "mode": "unbounded"}],
        "fairness_kinds": ["weak"]
    });
    assert!(
        descriptor.is_valid(&unbounded_only),
        "unbounded-only may omit domains"
    );
    let mut bounded_without = unbounded_only.clone();
    bounded_without["advertises"] = serde_json::json!([
        {"kind": "temporal-satisfaction", "mode": "unbounded"},
        {"kind": "temporal-satisfaction", "mode": "bounded"}
    ]);
    assert!(
        !descriptor.is_valid(&bounded_without),
        "a bounded mode requires domains"
    );
    for malformed in [
        serde_json::json!([]),
        serde_json::json!(["quantity"]),
        serde_json::json!(["size"]),
        serde_json::json!(["integer", "integer"]),
    ] {
        bounded_without["domains"] = malformed.clone();
        assert!(
            !descriptor.is_valid(&bounded_without),
            "bounded descriptor with domains {malformed} is refused"
        );
        let mut unbounded_malformed = unbounded_only.clone();
        unbounded_malformed["domains"] = malformed.clone();
        assert!(
            !descriptor.is_valid(&unbounded_malformed),
            "unbounded-only descriptor with domains {malformed} is refused"
        );
    }
}

/// Owner-derived canonical transport bindings, not a substitute for native
/// key-body admission or checker qualification. Expected references and
/// rule pairs are independent of the schema/vector inventories.
#[trace("TC-431", "FR-331-AC-25", "FR-331-AC-27", "FR-412-AC-8")]
#[test]
fn tc_431_canonical_key_bindings_are_complete() {
    let schema = parsed(SCHEMA);
    let certificate = &schema["$defs"]["ProbabilityCertificate"]["properties"];
    for field in ["values", "ranking", "policy"] {
        assert_eq!(certificate[field]["items"]["properties"]["state"]["$ref"],
            "#/$defs/ProbabilityProductKey", "complete key for {field}");
    }
    assert_eq!(certificate["components"]["items"]["properties"]["bias"]
        ["items"]["properties"]["state"]["$ref"], "#/$defs/ProbabilityProductKey");
    assert_eq!(certificate["policy"]["items"]["properties"]["choice"]["$ref"],
        "#/$defs/TransitionIdentity");
    for shape in ["ProductStateLocus", "HyperProductStateLocus"] {
        assert_eq!(schema["$defs"][shape]["properties"]["automaton_state"]["$ref"],
            "#/$defs/AutomatonStateKey");
    }
    assert_eq!(schema["$defs"]["HyperProductStateLocus"]["properties"]["witness_set"]
        ["items"]["properties"]["automaton_state"]["$ref"], "#/$defs/AutomatonStateKey");

    let rejection = definition("CertificateRejected");
    let mut admitted = 0;
    for rule in CERTIFICATE_RULES {
        let expected = matches!(rule,
            "inequality-fails" | "rank-missing" | "rank-broken" | "threshold-missed");
        let candidate = serde_json::json!({"type": "certificate-rejected", "rule": rule,
            "at": {"product_key": "AQ"}});
        assert_eq!(rejection.is_valid(&candidate), expected, "probability locus / {rule}");
        admitted += usize::from(expected);
    }
    assert_eq!(admitted, 4);
    for rule in ["inequality-fails", "threshold-missed"] {
        assert!(!rejection.is_valid(&serde_json::json!({"type": "certificate-rejected",
            "rule": rule, "at": {"domain": "quire.simulation.state-key/v1",
                "digest": "1111111111111111111111111111111111111111111111111111111111111111"}})));
    }
    let locus = definition("ProbabilityProductStateLocus");
    for invalid in [serde_json::json!({}), serde_json::json!({"product_key": "AQ=="}),
        serde_json::json!({"product_key": "AQ", "state": "AQ"})] {
        assert!(!locus.is_valid(&invalid));
    }

    // Independent exact octet oracle: QSL TC-525's parameter-free Counter
    // deadlock-freedom safety key. Codec/JCS checks only, not translation.
    let octets = br#"["quire.automaton-state-key/v1",[["deadlock-freedom"],"quire.temporal.infinite-trace/v1","infinite",[]],["safety",[[[[[],"positive"]],[],[],[]]]]]"#;
    assert_eq!(jcs_canonicalize(&serde_json::from_slice::<Value>(octets)
        .expect("literal canonical key")).expect("JCS"), octets.to_vec());
    let spelling = encode(octets);
    assert_eq!(decode(&spelling), Some(octets.to_vec()));
    assert!(definition("AutomatonStateKey").is_valid(&serde_json::json!(spelling)));
}

// Normative rule inventory, independent of both schema and vector contents.
const CERTIFICATE_RULES: [&str; 24] = [
    "initial-missing",
    "successor-missing",
    "bad-state",
    "not-partition",
    "backward-edge",
    "witness-fails",
    "query-mismatch",
    "shape-mismatch",
    "proof-step-invalid",
    "not-refutation",
    "initial-mismatch",
    "successors-differ",
    "step-fails",
    "post-differs",
    "malformed",
    "undefined-member",
    "violating-member",
    "predicate-undefined",
    "rank-missing",
    "rank-broken",
    "order-broken",
    "count-broken",
    "inequality-fails",
    "threshold-missed",
];

/// The closed rule inventory is authored independently of the schema and
/// fixture data. Removing a rule from both cannot make this check pass.
/// This covers every rule at least once, not every rule/locus pair or an
/// actual checker's context-dependent applicability.
#[trace("TC-431", "FR-331-AC-25")]
#[test]
fn tc_431_certificate_rules_have_positive_coverage() {
    let expected: BTreeSet<&str> = CERTIFICATE_RULES.into_iter().collect();
    let vectors = positives();
    let mut covered = BTreeSet::new();
    for vector in &vectors {
        if let Some(results) = vector["document"]["results"].as_array() {
            for result in results {
                let cause = &result["cause"];
                if cause["type"] == "certificate-rejected" {
                    covered.insert(cause["rule"].as_str().expect("rejection rule"));
                }
            }
        }
    }
    assert_eq!(covered, expected, "every owned rule has a positive vector");

    let schema = parsed(SCHEMA);
    let branches = schema["$defs"]["CertificateRejected"]["oneOf"]
        .as_array()
        .expect("rejection branches");
    let mut admitted = BTreeSet::new();
    for branch in branches {
        let rule = &branch["properties"]["rule"];
        if let Some(values) = rule["enum"].as_array() {
            for value in values {
                admitted.insert(value.as_str().expect("closed rule spelling"));
            }
        } else {
            admitted.insert(rule["const"].as_str().expect("single rule spelling"));
        }
    }
    assert_eq!(admitted, expected, "schema retains the entire owned rule set");
}

/// Independent finite rule/shape and required-field controls. These are
/// schema-only transport samples, not canonical automaton identity evidence
/// or production-reader/checker qualification.
#[trace("TC-431", "FR-331-AC-25", "FR-331-AC-27")]
#[test]
fn tc_431_hyper_loci_have_closed_shapes_and_rule_pairs() {
    let rejection = definition("CertificateRejected");
    let tuple_shape = definition("HyperTransitionTupleLocus");
    let state_shape = definition("HyperProductStateLocus");
    let first = "1111111111111111111111111111111111111111111111111111111111111111";
    let second = "2222222222222222222222222222222222222222222222222222222222222222";
    let tuple = serde_json::json!({"transitions": [
        {"pre_state": first, "transition": {"type": "transition", "operation": "set", "receiver": null, "arguments": [{"type": "integer", "value": "0"}]}, "post_state": first, "result": null},
        {"pre_state": first, "transition": {"type": "transition", "operation": "set", "receiver": null, "arguments": [{"type": "integer", "value": "1"}]}, "post_state": second, "result": null}
    ]});
    let empty_witnesses = serde_json::json!({"states": [], "automaton_state": "AQ", "witness_set": []});
    let nonempty_witnesses = serde_json::json!({"states": [], "automaton_state": "AQ", "witness_set": [
        {"states": [first, second], "automaton_state": "Ag"}
    ]});
    let member = serde_json::json!({"member": "components"});
    for rule in CERTIFICATE_RULES {
        let hyper_state_rule = matches!(
            rule,
            "initial-missing"
                | "successor-missing"
                | "not-partition"
                | "backward-edge"
                | "witness-fails"
                | "malformed"
                | "undefined-member"
                | "violating-member"
        );
        for (at, expected) in [
            (
                &tuple,
                matches!(rule, "undefined-member" | "violating-member"),
            ),
            (&empty_witnesses, hyper_state_rule),
            (&nonempty_witnesses, hyper_state_rule),
            (&member, rule == "malformed"),
        ] {
            let cause = serde_json::json!({"type": "certificate-rejected", "rule": rule, "at": at});
            assert_eq!(rejection.is_valid(&cause), expected, "rule {rule} at {at}");
        }
    }
    assert!(tuple_shape.is_valid(&tuple));
    assert!(!state_shape.is_valid(&tuple), "tuple is not a state locus");
    for at in [&empty_witnesses, &nonempty_witnesses] {
        assert!(state_shape.is_valid(at));
        assert!(!tuple_shape.is_valid(at), "state locus is not a tuple");
        for field in ["states", "automaton_state", "witness_set"] {
            let mut missing = at.clone();
            missing.as_object_mut().expect("state object").remove(field);
            assert!(
                !state_shape.is_valid(&missing),
                "empty universal key requires {field}"
            );
        }
    }
    for edge in 0..2 {
        for field in ["pre_state", "transition", "post_state", "result"] {
            let mut missing = tuple.clone();
            missing["transitions"][edge]
                .as_object_mut()
                .expect("edge object")
                .remove(field);
            assert!(
                !tuple_shape.is_valid(&missing),
                "edge {edge} requires {field}"
            );
        }
        for field in ["type", "operation", "receiver", "arguments"] {
            let mut missing = tuple.clone();
            missing["transitions"][edge]["transition"]
                .as_object_mut()
                .expect("transition object")
                .remove(field);
            assert!(
                !tuple_shape.is_valid(&missing),
                "transition {edge} requires {field}"
            );
        }
    }
    for result in [
        serde_json::json!(null),
        serde_json::json!({"type": "null"}),
        serde_json::json!({"type": "boolean", "value": true}),
    ] {
        let mut with_result = tuple.clone();
        with_result["transitions"][0]["result"] = result;
        assert!(
            tuple_shape.is_valid(&with_result),
            "none and typed results are distinct admitted shapes"
        );
    }
    for result in [
        serde_json::json!(false),
        serde_json::json!(0),
        serde_json::json!({}),
    ] {
        let mut untyped = tuple.clone();
        untyped["transitions"][0]["result"] = result;
        assert!(!tuple_shape.is_valid(&untyped), "untyped result is refused");
    }
    for existential_count in 0..=2 {
        let mut candidate = nonempty_witnesses.clone();
        candidate["witness_set"][0]["states"]
            .as_array_mut()
            .expect("existential tuple")
            .truncate(existential_count);
        assert_eq!(
            state_shape.is_valid(&candidate),
            existential_count == 2,
            "n=0 requires m>=2"
        );
    }
    let mut protocol = tuple.clone();
    protocol["transitions"][0]["transition"] = serde_json::json!({"type": "protocol-transition"});
    assert!(!tuple_shape.is_valid(&protocol));
    let mut extra_edge = tuple.clone();
    extra_edge["transitions"][0]["extra"] = serde_json::json!(true);
    assert!(!tuple_shape.is_valid(&extra_edge));
    let mut extra_locus = tuple.clone();
    extra_locus["automaton_state"] = serde_json::json!("AQ");
    assert!(!tuple_shape.is_valid(&extra_locus));
    let empty_tuple = serde_json::json!({"transitions": []});
    assert!(!tuple_shape.is_valid(&empty_tuple));
    assert!(!tuple_shape.is_valid(&serde_json::json!({})));
    // A state-only projection can match the global state union, but must
    // still be refused for an HP-1 body failure by the context-aware reader.
    // That semantic obligation remains BP-13/14, not asserted by this test.
    let projection = serde_json::json!({"states": [first, first]});
    assert!(state_shape.is_valid(&projection));
    assert!(!tuple_shape.is_valid(&projection));
}

/// The `at` forms FR-331 catalogues, spelled here independently of the schema
/// and the vectors: a query part is `unrolling`, `base`, `step` or
/// `step/<row>` (QSL FR-338's specified spelling); a proof step is `{part,
/// index}`; a product state is `{model_state, automaton_state}`; a model
/// state is its `{domain, digest}` record, in the state-key domain for every
/// rule and in any domain for `malformed`; a state-graph entry is `{entry}`;
/// a simulation step always carries `position`, `edge` and `verdict`, null
/// when absent. Each rule is admitted exactly at the forms listed with it.
#[trace("TC-431", "FR-331-AC-24", "FR-331-AC-25")]
#[test]
fn tc_431_certificate_loci_are_the_catalogued_spellings() {
    let rejection = definition("CertificateRejected");
    let part = definition("QueryPart");
    let digest = "1111111111111111111111111111111111111111111111111111111111111111";
    let model_state = serde_json::json!({"domain": "quire.simulation.state-key/v1", "digest": digest});
    let verdict = serde_json::json!({"kind": "NoAbstractMatch", "position": "1"});
    let foreign_state = serde_json::json!({"domain": "package-fingerprint",
        "digest": "3333333333333333333333333333333333333333333333333333333333333333"});
    let loci: [(Value, &[&str]); 11] = [
        (serde_json::json!("step/3"),
            &["query-mismatch", "shape-mismatch", "malformed", "not-refutation"]),
        (serde_json::json!({"part": "step/3", "index": "7"}),
            &["proof-step-invalid", "not-refutation"]),
        (serde_json::json!({"model_state": model_state, "automaton_state": "AQ"}),
            &["initial-missing", "successor-missing", "bad-state", "not-partition",
              "backward-edge", "witness-fails", "shape-mismatch"]),
        (model_state.clone(),
            &["initial-missing", "successor-missing", "bad-state", "predicate-undefined",
              "rank-missing", "rank-broken", "order-broken", "count-broken", "malformed"]),
        (foreign_state.clone(), &["malformed"]),
        (serde_json::json!({"domain": "", "digest": digest}), &[]),
        (serde_json::json!({"entry": "2"}), &["malformed"]),
        (serde_json::json!({"position": "0", "edge": null, "verdict": null}),
            &["initial-mismatch", "successors-differ", "bad-state", "malformed"]),
        (serde_json::json!({"position": "0", "edge": "1", "verdict": null}), &["post-differs"]),
        (serde_json::json!({"position": "0", "edge": "1", "verdict": verdict}), &["step-fails"]),
        (serde_json::json!("base"),
            &["query-mismatch", "shape-mismatch", "malformed", "not-refutation"]),
    ];
    for (at, rules) in &loci {
        for rule in CERTIFICATE_RULES {
            let cause = serde_json::json!({"type": "certificate-rejected", "rule": rule, "at": at});
            assert_eq!(rejection.is_valid(&cause), rules.contains(&rule), "rule {rule} at {at}");
        }
    }
    for spelling in ["unrolling", "base", "step", "step/0", "step/7", "step/18446744073709551615"] {
        assert!(part.is_valid(&serde_json::json!(spelling)), "{spelling} is a query part");
    }
    for spelling in ["Step", "loop", "step/", "step/01", "step/-1", "step/+1", "step/1.0",
        "step/ 1", "step/1/2", "step/18446744073709551616", "base/0", "unrolling/0", "step3"] {
        assert!(!part.is_valid(&serde_json::json!(spelling)), "{spelling} is not a query part");
    }
    let maximum = u64::MAX.to_string();
    for length in 1..=maximum.len() + 1 {
        for row in [format!("1{}", "0".repeat(length - 1)), "9".repeat(length)] {
            let canonical = row.parse::<u64>().is_ok_and(|value| value.to_string() == row);
            assert_eq!(part.is_valid(&serde_json::json!(format!("step/{row}"))), canonical,
                "step/{row} is admitted exactly when {row} is a canonical u64");
        }
    }
    // The member names and nesting of other spellings are refused at every rule.
    for at in [
        serde_json::json!({"state": digest, "automaton_state": "AQ"}),
        serde_json::json!({"state": digest}),
        serde_json::json!(digest),
        serde_json::json!({"position": "0"}),
        serde_json::json!({"position": "0", "edge": "1"}),
        serde_json::json!({"entry": 2}),
    ] {
        for rule in CERTIFICATE_RULES {
            let cause = serde_json::json!({"type": "certificate-rejected", "rule": rule, "at": at});
            assert!(!rejection.is_valid(&cause), "rule {rule} at {at} is refused");
        }
    }
    // FR-149's verdict is tagged by `kind`, with decimal-string integers.
    let step_verdict = definition("StepVerdict");
    for admitted in [
        serde_json::json!({"kind": "InitialNotAbstract", "initial": "0"}),
        serde_json::json!({"kind": "StutterChanged", "position": "2"}),
        serde_json::json!({"kind": "AbstractStepRejected", "position": "2",
            "transition": {"type": "transition", "operation": "inc", "receiver": null, "arguments": []},
            "cause": {"kind": "Postcondition"}}),
        serde_json::json!({"kind": "UndefinedEvaluation",
            "where": {"position": "1", "state_key": model_state, "locus": "contract"}, "cause": "family"}),
        serde_json::json!({"kind": "Undetermined", "cause": "UndecidedSuccessor",
            "evaluation": {"outcome": "incomplete", "limit": "integer_bits"}}),
    ] {
        assert!(step_verdict.is_valid(&admitted), "{admitted} is a step verdict");
    }
    for refused in [
        serde_json::json!({"kind": "StutterChanged", "position": 2}),
        serde_json::json!({"kind": "Undefined", "position": "1"}),
        serde_json::json!({"kind": "AbstractStepRejected", "position": "2",
            "transition": {"type": "transition", "operation": "inc", "receiver": null, "arguments": []},
            "cause": {"type": "Postcondition"}}),
        serde_json::json!({"type": "replay-parity"}),
        serde_json::json!({"kind": "Undetermined", "cause": "UndecidedSuccessor",
            "evaluation": {"outcome": "incomplete"}}),
    ] {
        assert!(!step_verdict.is_valid(&refused), "{refused} is not a step verdict");
    }
}

/// A proof-step locus's `index` admits exactly the canonical decimal
/// spellings of `u64`: the schema's `U64String` agrees with Rust's `u64`
/// parse at every digit of `u64::MAX`, at every length up to one past it,
/// and on each non-canonical spelling.
#[trace("TC-431", "FR-331-AC-24")]
#[test]
fn tc_431_proof_step_index_admits_exactly_the_u64_range() {
    let pattern = definition("U64String");
    let maximum = u64::MAX.to_string();
    let mut samples: Vec<String> = ["", "00", "01", "-0", "-1", "+1", " 1", "1 ", "1.0", "1e3"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    for length in 1..=maximum.len() + 1 {
        samples.push(format!("1{}", "0".repeat(length - 1)));
        samples.push("9".repeat(length));
    }
    for prefix in 0..maximum.len() {
        for digit in '0'..='9' {
            for fill in ['0', '9'] {
                let tail = maximum.len() - prefix - 1;
                samples.push(format!(
                    "{}{digit}{}",
                    &maximum[..prefix],
                    fill.to_string().repeat(tail)
                ));
            }
        }
    }
    let mut admitted = 0;
    for sample in samples {
        let canonical = sample
            .parse::<u64>()
            .is_ok_and(|value| value.to_string() == sample);
        assert_eq!(
            pattern.is_valid(&Value::String(sample.clone())),
            canonical,
            "U64String admits {sample:?} exactly when it is a canonical u64"
        );
        admitted += usize::from(canonical);
    }
    assert!(admitted > 0, "some u64 spellings are exercised");
    assert!(pattern.is_valid(&Value::String(maximum)), "u64::MAX");
    assert!(
        !pattern.is_valid(&Value::String("18446744073709551616".into())),
        "u64::MAX + 1"
    );
}

/// FR-331-AC-30: `memory_models` is a nonempty, duplicate-free subset of
/// `sc`, `tso` and `ra`, carried only by a `temporal-satisfaction` descriptor.
#[trace("TC-431", "FR-331-AC-30")]
#[test]
fn tc_431_memory_models_is_a_nonempty_unique_subset_on_temporal_descriptors() {
    let descriptor = definition("BackendDescriptor");
    let base = serde_json::json!({
        "backend": "qsl-model-check",
        "advertises": [{"kind": "temporal-satisfaction", "mode": "unbounded"}],
        "fairness_kinds": ["weak"]
    });
    let with = |models: Value| {
        let mut entry = base.clone();
        entry["memory_models"] = models;
        entry
    };
    assert!(descriptor.is_valid(&base), "memory_models may be omitted");
    assert!(descriptor.is_valid(&with(serde_json::json!(["sc", "tso", "ra"]))));
    assert!(descriptor.is_valid(&with(serde_json::json!(["ra"]))));
    assert!(!descriptor.is_valid(&with(serde_json::json!([]))), "empty");
    assert!(
        !descriptor.is_valid(&with(serde_json::json!(["sc", "sc"]))),
        "repeated"
    );
    assert!(
        !descriptor.is_valid(&with(serde_json::json!(["arm"]))),
        "unknown"
    );
    let mut other = serde_json::json!({
        "backend": "smt",
        "advertises": [{"kind": "operation-contract", "mode": "unbounded"}]
    });
    assert!(
        descriptor.is_valid(&other),
        "an operation-contract descriptor is valid without memory_models"
    );
    other["memory_models"] = serde_json::json!(["sc"]);
    assert!(
        !descriptor.is_valid(&other),
        "not a temporal-satisfaction descriptor"
    );
    let mut mixed = other.clone();
    mixed["advertises"] = serde_json::json!([
        {"kind": "operation-contract", "mode": "unbounded"},
        {"kind": "temporal-satisfaction", "mode": "unbounded"}
    ]);
    mixed["fairness_kinds"] = serde_json::json!(["weak"]);
    assert!(
        descriptor.is_valid(&mixed),
        "a descriptor that also advertises temporal-satisfaction may carry it"
    );
}

/// FR-331-AC-32: a temporal item's subject may carry a population bound while
/// its extent stays unbounded; the bound moves nothing into the extent.
#[trace("TC-431", "FR-331-AC-32")]
#[test]
fn tc_431_a_subject_bound_leaves_the_extent_unbounded() {
    let validator = validator();
    let vector = positive("temporal-request");
    let document = composed(&vector);
    let item = &document["request"]["items"][0];
    assert_eq!(item["extent"]["classification"], "unbounded");
    assert_eq!(item["extent"]["domains"][0]["kind"], "infinite-trace");
    assert_eq!(item["extent"]["finite_bound_available"], false);
    assert!(item["extent"].get("bounds").is_none());
    let bounds = item["temporal"]["subject"]["proof_bounds"]
        .as_array()
        .expect("subject proof bounds");
    assert!(
        bounds
            .iter()
            .any(|bound| bound["kind"] == "population" && bound["bound"]["type"] == "cardinality")
    );
    assert_eq!(errors(&validator, &document), Vec::<String>::new());
    let mut emptied = document.clone();
    emptied["request"]["items"][0]["temporal"]["subject"]["proof_bounds"] = serde_json::json!([]);
    assert_eq!(errors(&validator, &emptied), Vec::<String>::new());
}

/// FR-331-AC-31: the `memory.load-buffering-shape` warning carries a `load`
/// and a `store` transition identity and nothing else.
#[trace("TC-431", "FR-331-AC-31")]
#[test]
fn tc_431_load_buffering_warning_carries_two_transition_identities() {
    let warning = definition("Warning");
    let identity = |operation: &str| {
        serde_json::json!({
            "type": "transition",
            "operation": operation,
            "receiver": null,
            "arguments": []
        })
    };
    let good = serde_json::json!({
        "code": "memory.load-buffering-shape",
        "load": identity("Reader::read"),
        "store": identity("Writer::write")
    });
    assert!(warning.is_valid(&good), "a well-formed warning is admitted");
    let mut missing = good.clone();
    missing.as_object_mut().unwrap().remove("store");
    assert!(!warning.is_valid(&missing), "store is required");
    let mut extra = good.clone();
    extra["site"] = serde_json::json!("x");
    assert!(!warning.is_valid(&extra), "no extra member");
    let mut bad = good.clone();
    bad["load"] = serde_json::json!("Reader::read");
    assert!(!warning.is_valid(&bad), "load is a transition identity");
}

/// FR-361-AC-13: a universe provision is the RFC 8785 array of keys in list
/// order, each exactly `universe` (64 lowercase hex) and `identity` (lowercase
/// hex), named by its `sha256-jcs` digest.
#[trace("TC-290", "FR-361-AC-13")]
#[test]
fn tc_290_universe_provisions_are_canonical_arrays_of_keys() {
    let is_hex = |text: &str| {
        text.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    };
    let vectors = parsed(VECTORS);
    let cases = vectors["universe_provision_vectors"]
        .as_array()
        .expect("universe provision vectors");
    let mut digests = BTreeSet::new();
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let keys = &case["keys"];
        for key in keys.as_array().expect("keys") {
            let members: BTreeSet<&str> = key
                .as_object()
                .expect("a key is an object")
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(members, BTreeSet::from(["identity", "universe"]), "{name}");
            let universe = key["universe"].as_str().expect("universe");
            let identity = key["identity"].as_str().expect("identity");
            assert!(universe.len() == 64 && is_hex(universe), "{name}: universe");
            assert!(
                !identity.is_empty() && identity.len() % 2 == 0 && is_hex(identity),
                "{name}: identity"
            );
        }
        let canonical = jcs_canonicalize(keys).expect("canonical keys");
        assert_eq!(
            canonical,
            case["jcs"].as_str().expect("jcs").as_bytes(),
            "{name}"
        );
        let digest = jcs_sha256(keys).expect("digest");
        assert_eq!(
            digest.strip_prefix("sha256-jcs:").unwrap_or(&digest),
            case["digest"].as_str().expect("digest"),
            "{name}"
        );
        digests.insert(digest);
    }
    assert_eq!(digests.len(), cases.len(), "list order changes the digest");
}

/// FR-364-AC-10: the observed-arm positive and the coupling adverse vectors
/// exist, by name, so deleting any of them fails this test; their admission
/// and refusal are checked by the positive and adverse vector tests.
#[trace("TC-431", "FR-364-AC-10")]
#[test]
fn tc_431_the_arm_coupling_vectors_are_present() {
    let vectors = parsed(VECTORS);
    let names = |section: &str| -> BTreeSet<String> {
        vectors[section]
            .as_array()
            .expect(section)
            .iter()
            .map(|vector| vector["name"].as_str().expect("name").to_owned())
            .collect()
    };
    let positive = names("positive");
    let adverse = names("adverse");
    for name in [
        "observed-counterexample-outcome",
        "observed-undefined-evaluation-outcome",
    ] {
        assert!(positive.contains(name), "positive vector {name}");
    }
    for name in [
        "observed-counterexample-names-an-initial-state",
        "observed-counterexample-with-a-model-step-in-the-prefix",
        "observed-counterexample-with-a-model-step-in-the-loop",
        "observed-counterexample-deadlock-kind",
        "observed-counterexample-time-lock-with-final-delay",
        "observed-counterexample-over-binding",
        "observed-undefined-evaluation-with-a-state-key",
        "model-counterexample-with-a-document-step-in-the-prefix",
        "model-counterexample-with-a-document-step-in-the-loop",
        "model-counterexample-without-an-initial-state",
        "model-undefined-evaluation-with-a-document",
        "observed-undefined-evaluation-with-state-keys",
    ] {
        assert!(adverse.contains(name), "adverse vector {name}");
    }
}

/// FR-331-AC-26: the global shape union cannot tell which checker a
/// rejection belongs to, so a pairing that is well formed for one checker is
/// admitted by the schema for a claim it does not fit. The reader's context
/// binding is therefore not discharged by the schema: each pair below is
/// schema-admitted, and the reader refuses it for a claim of the wrong form.
#[trace("TC-431", "FR-331-AC-26")]
#[test]
fn tc_431_context_pairs_are_admitted_by_shape_alone() {
    let rejection = definition("CertificateRejected");
    let digest = "1111111111111111111111111111111111111111111111111111111111111111";
    for (rule, at) in [
        ("inequality-fails", serde_json::json!({"product_key": "AQ"})),
        ("successor-missing", serde_json::json!({"domain": "quire.simulation.state-key/v1", "digest": digest})),
    ] {
        let cause = serde_json::json!({"type": "certificate-rejected", "rule": rule, "at": at});
        assert!(rejection.is_valid(&cause), "{rule} at {at} is a well-formed pair");
    }
}
