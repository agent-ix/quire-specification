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
#[trace("TC-431", "FR-331-AC-24")]
#[trace("TC-431", "FR-331-AC-25")]
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
#[trace("TC-431", "FR-331-AC-24")]
#[trace("TC-431", "FR-331-AC-25")]
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
        "#/$defs/ProbabilityWitnessEntry");
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
            "rule": rule, "at": {"state": "1111111111111111111111111111111111111111111111111111111111111111"}})));
    }
    let locus = definition("ProbabilityProductStateLocus");
    for invalid in [serde_json::json!({}), serde_json::json!({"product_key": "AQ=="}),
        serde_json::json!({"product_key": "AQ", "state": "AQ"})] {
        assert!(!locus.is_valid(&invalid));
    }
    let policy = definition("ProbabilityWitnessEntry");
    let deterministic = serde_json::json!({"type": "choice", "choice": "AQ"});
    let randomized = serde_json::json!({"type": "randomized", "choices": [
        {"choice": "AQ", "probability": {"numerator": "3", "denominator": "4"}},
        {"choice": "Ag", "probability": {"numerator": "1", "denominator": "4"}}]});
    for entry in [deterministic, randomized] {
        assert!(policy.is_valid(&entry));
        for field in entry.as_object().expect("closed entry").keys() {
            let mut missing = entry.clone();
            missing.as_object_mut().expect("closed entry").remove(field);
            assert!(!policy.is_valid(&missing), "policy needs {field}");
        }
        let mut extra = entry;
        extra["operation"] = serde_json::json!("invented-observe");
        assert!(!policy.is_valid(&extra));
    }
    assert!(!policy.is_valid(&serde_json::json!({"type": "transition",
        "operation": "inc", "receiver": null, "arguments": []})));
    assert!(!policy.is_valid(&serde_json::json!({"type": "randomized", "choices": []})));

    // Independent exact octet oracle: QSL TC-525's parameter-free Counter
    // deadlock-freedom safety key. Codec/JCS checks only, not translation.
    let octets = br#"["quire.automaton-state-key/v1",[["deadlock-freedom"],"quire.temporal.infinite-trace/v1","infinite",[]],["safety",[[[[[],"positive"]],[],[],[]]]]]"#;
    assert_eq!(jcs_canonicalize(&serde_json::from_slice::<Value>(octets)
        .expect("literal canonical key")).expect("JCS"), octets.to_vec());
    let spelling = encode(octets);
    assert_eq!(decode(&spelling), Some(octets.to_vec()));
    assert!(definition("AutomatonStateKey").is_valid(&serde_json::json!(spelling)));
}

/// Independent finite field oracle for FR-413 carriers. AQ/Ag and the
/// reused model-state sample test transport only, never key-body admission.
#[trace("TC-431", "FR-331-AC-27", "FR-413-AC-6", "FR-413-AC-8")]
#[test]
fn tc_431_probability_witness_bindings_are_complete() {
    let schema = parsed(SCHEMA);
    let required = ["transition", "post_state", "random", "action", "post_key", "elapsed"];
    let expected: BTreeSet<_> = required.into_iter().collect();
    let authored: BTreeSet<_> = schema["$defs"]["ProbabilisticStep"]["required"]
        .as_array().expect("required fields").iter()
        .map(|field| field.as_str().expect("field name")).collect();
    assert_eq!(authored, expected);
    assert_eq!(schema["$defs"]["ProbabilisticStep"]["properties"]["action"]["$ref"],
        "#/$defs/ProbabilityProductAction");
    assert_eq!(schema["$defs"]["ProbabilisticStep"]["properties"]["post_key"]["$ref"],
        "#/$defs/ProbabilityProductKey");
    assert_eq!(schema["$defs"]["ProbabilisticCounterexample"]["properties"]["scheduler"]["$ref"],
        "#/$defs/ProbabilityWitnessScheduler");
    let row = &schema["$defs"]["ProbabilityWitnessScheduler"]["properties"]["entries"]["items"];
    assert_eq!(row["properties"]["state"]["$ref"], "#/$defs/ProbabilityProductKey");
    assert_eq!(row["properties"]["choice"]["$ref"], "#/$defs/ProbabilityWitnessEntry");
    let arms = &schema["$defs"]["ProbabilisticCounterexample"]["properties"]["evidence"]["oneOf"];
    assert_eq!(arms[0]["properties"]["paths"]["items"]["items"]["$ref"],
        "#/$defs/ProbabilisticStep");
    assert_eq!(arms[2]["properties"]["path"]["items"]["$ref"],
        "#/$defs/ProbabilisticStep");

    // Reuse an existing structural StateKey; introduce no content digest.
    let old = positive("certificate-rejected-loci");
    let model_state = old["document"]["results"].as_array().expect("results").iter()
        .find_map(|result| result["cause"]["at"]["state"].as_str())
        .expect("existing model-state sample");
    let step = serde_json::json!({"transition": null, "post_state": model_state,
        "random": null, "action": encode(br#"["idle-observe"]"#), "post_key": "AQ",
        "elapsed": {"numerator": "0", "denominator": "1"}});
    let step_reader = definition("ProbabilisticStep");
    assert!(step_reader.is_valid(&step));
    let mut empty_draw = step.clone();
    empty_draw["random"] = serde_json::json!([]);
    assert!(step_reader.is_valid(&empty_draw));
    assert_ne!(empty_draw["random"], step["random"], "drawn empty vector is not None");
    for literal in [br#"["delay"]"#.as_slice(), br#"["enter-idle-delay"]"#.as_slice(),
        br#"["idle-delay"]"#.as_slice(), br#"["terminal-stutter"]"#.as_slice()] {
        let spelling = encode(literal);
        assert_eq!(decode(&spelling), Some(literal.to_vec()));
        assert!(definition("ProbabilityProductAction").is_valid(&serde_json::json!(spelling)));
    }
    let witness_reader = definition("ProbabilisticCounterexample");
    let shapes = positive("probability-witness-shapes");
    let witnesses = shapes["document"]["counterexamples"].as_array().expect("witnesses");
    assert_eq!(witnesses.len(), 2, "path-set and undefined-path independently covered");
    let mut omissions = 0;
    for sample in witnesses {
        let mut filled = sample["counterexample"].clone();
        let path = if filled["evidence"]["type"] == "path-set" {
            "/evidence/paths/0"
        } else {
            assert_eq!(filled["evidence"]["type"], "undefined-path");
            "/evidence/path"
        };
        *filled.pointer_mut(path).expect("actual path") = serde_json::json!([step.clone()]);
        assert!(witness_reader.is_valid(&filled));
        for field in required {
            let mut missing = filled.clone();
            missing.pointer_mut(path).expect("path")[0].as_object_mut()
                .expect("step").remove(field);
            assert!(!witness_reader.is_valid(&missing), "{path} needs {field}");
            omissions += 1;
        }
        for (field, value) in [("elapsed", serde_json::json!({"numerator": "-1", "denominator": "1"})),
            ("post_key", serde_json::json!("AQ==")), ("action", serde_json::json!({"operation": "send_b"})),
            ("random", serde_json::json!({})), ("transition", serde_json::json!({"type": "idle-observe"}))] {
            let mut malformed = filled.clone();
            malformed.pointer_mut(path).expect("path")[0][field] = value;
            assert!(!witness_reader.is_valid(&malformed), "invalid {field} at {path}");
        }
        let mut alias = filled;
        alias.pointer_mut(path).expect("path")[0]["state"] = serde_json::json!("AQ");
        assert!(!witness_reader.is_valid(&alias), "no extra model-key alias");
    }
    assert_eq!(omissions, 12, "six mandatory fields in each of two path arms");
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
