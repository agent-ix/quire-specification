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
/// member it names an owner for exists and names an FR of this specification.
#[trace("TC-431", "FR-331-AC-19")]
#[trace("TC-431", "FR-331-AC-23")]
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
        apply(&mut document, &patch[0]);
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
    let recomputed = jcs_sha256(&package["identity_preimage"]).expect("preimage canonicalizes");
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
    apply(
        &mut missing,
        &serde_json::json!({"op": "remove", "path": "/request/items/0/candidates"}),
    );
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
