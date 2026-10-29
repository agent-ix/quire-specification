// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-483 to FR-485 check-arm vectors (TC-972 to TC-974).
//!
//! Node ids in the vectors are written `$node:<label>`; the harness derives
//! each `NodeId` from its label, so no digest is frozen in the vector file.
//! Each case's outcome stage is validated against the FR-331 schema, its
//! negotiation disposition is recomputed from FR-290's rules, and its arm
//! outcome is checked against FR-483's outcome mapping.

use ix_trace_rs::trace;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const SCHEMA: &str = include_str!("../proposals/backend-provider-v1/schema.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const VECTORS: &str = include_str!("../proposals/backend-check-arm-v1/vectors.json");
const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

fn node_id(label: &str) -> Value {
    let mut hasher = Sha256::new();
    hasher.update(NODE_DOMAIN.as_bytes());
    hasher.update([0]);
    hasher.update(label.as_bytes());
    let digest: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

fn rekey(value: &mut Value) {
    match value {
        Value::String(text) => {
            if let Some(label) = text.strip_prefix("$node:") {
                *value = node_id(label);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(rekey),
        Value::Object(map) => map.values_mut().for_each(rekey),
        _ => {}
    }
}

fn cases() -> Vec<Value> {
    parsed(VECTORS)["cases"].as_array().expect("cases").clone()
}

fn outcome_validator() -> jsonschema::Validator {
    let registry = jsonschema::Registry::new()
        .add("urn:quire:checked-package:v2", parsed(CHECKED_PACKAGE_SCHEMA))
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", parsed(SCHEMA))
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&json!({"$ref": "urn:quire:backend-provider:v1#/$defs/OutcomeStage"}))
        .expect("outcome stage compiles")
}

const BOUNDABLE: [&str; 4] = ["collection", "population", "integer", "recursive"];

fn advertises(descriptor: &Value, kind: &str, mode: &str) -> bool {
    descriptor["advertises"]
        .as_array()
        .expect("advertises")
        .iter()
        .any(|pair| pair["kind"] == kind && pair["mode"] == mode)
}

/// FR-290's candidate set, domain-kind check and advertised-mode table for one item.
fn negotiate(case: &Value) -> (String, Option<String>) {
    let item = &case["item"];
    let kind = item["kind"].as_str().expect("kind");
    let candidates: Vec<&Value> = case["registry"]
        .as_array()
        .expect("registry")
        .iter()
        .filter(|d| {
            (advertises(d, kind, "bounded") || advertises(d, kind, "unbounded"))
                && item.get("backend").is_none_or(|b| *b == d["backend"])
        })
        .collect();
    let [candidate] = candidates.as_slice() else {
        return ("unsupported".into(), Some("unsupported-requested-capability".into()));
    };
    let extent = &item["extent"];
    let bounded = extent["classification"] == "bounded";
    let unbounded_mode = advertises(candidate, kind, "unbounded");
    let texts = |key: &str| -> Vec<&str> {
        extent[key].as_array().expect("kind list").iter().map(|k| k.as_str().expect("kind text")).collect()
    };
    let compared: Vec<&str> = if bounded {
        texts("bound_kinds")
    } else if unbounded_mode {
        vec![]
    } else {
        texts("domain_kinds").into_iter().filter(|k| BOUNDABLE.contains(k)).collect()
    };
    let domains: Vec<&str> = candidate["domains"].as_array().map_or(vec![], |d| {
        d.iter().map(|k| k.as_str().expect("domain")).collect()
    });
    if compared.iter().any(|k| !domains.contains(k)) {
        return ("unsupported".into(), Some("unsupported-requested-capability".into()));
    }
    if bounded || unbounded_mode {
        return ("supported".into(), None);
    }
    if extent["finite_bound_available"] == true {
        ("requires-bound".into(), None)
    } else {
        ("unsupported".into(), Some("unbounded-extent".into()))
    }
}

fn composed(case: &Value) -> Value {
    let mut document = case["outcome"].clone();
    rekey(&mut document);
    document
}

#[trace("TC-972", "FR-483-AC-6")]
#[trace("TC-972", "FR-483-AC-7")]
#[trace("TC-973", "FR-484-AC-8")]
#[trace("TC-974", "FR-485-AC-12")]
#[test]
fn check_arm_negotiation_dispositions_follow_fr_290() {
    for case in cases() {
        let (disposition, cause) = negotiate(&case);
        assert_eq!(disposition, case["negotiation"]["disposition"], "{} disposition", case["id"]);
        assert_eq!(
            cause.as_deref(),
            case["negotiation"]["cause"].as_str(),
            "{} cause",
            case["id"]
        );
    }
}

#[trace("TC-972", "FR-483-AC-1")]
#[trace("TC-972", "FR-483-AC-2")]
#[trace("TC-972", "FR-483-AC-3")]
#[trace("TC-972", "FR-483-AC-4")]
#[trace("TC-972", "FR-483-AC-5")]
#[test]
fn check_arm_outcomes_map_to_the_fr_331_results() {
    let validator = outcome_validator();
    for case in cases() {
        let id = case["id"].as_str().expect("id").to_owned();
        let document = composed(&case);
        let errors: Vec<String> = validator.iter_errors(&document).map(|e| e.to_string()).collect();
        assert_eq!(errors, Vec::<String>::new(), "{id} outcome stage is admitted");
        assert_eq!(
            document["dispositions"][0]["disposition"], case["negotiation"]["disposition"],
            "{id} disposition"
        );
        let results = document["results"].as_array().expect("results");
        // Only an item negotiated supported has a result entry.
        assert_eq!(!results.is_empty(), case["arm"].is_object(), "{id} result iff arm ran");
        let Some(arm) = case["arm"].as_object() else { continue };
        let result = &results[0];
        assert_eq!(result["result"], arm["result"], "{id} result");
        assert_eq!(result["kind"], case["item"]["kind"], "{id} kind");
        let mapped = match arm["outcome"].as_str() {
            Some("supported") => ["proved"].as_slice(),
            Some("violation") => ["refuted"].as_slice(),
            Some("unknown") => ["inconclusive", "incomplete"].as_slice(),
            Some("unsupported") => ["unsupported"].as_slice(),
            None => ["declined"].as_slice(),
            other => panic!("{id}: unknown arm outcome {other:?}"),
        };
        assert!(mapped.contains(&result["result"].as_str().unwrap()), "{id} mapping");
        assert!(
            !["tested", "measured", "failed"].contains(&result["result"].as_str().unwrap()),
            "{id} is no arm result"
        );
        let counterexamples = document["counterexamples"].as_array().expect("counterexamples");
        if result["result"] == "refuted" {
            let entry = &counterexamples[result["counterexample"].as_u64().unwrap() as usize];
            let expected = if case["item"]["kind"] == "value-validity" {
                "assignments"
            } else {
                "contract-counterexample"
            };
            assert_eq!(entry["counterexample"]["type"], expected, "{id} counterexample type");
            assert!(entry.get("transcript").is_none(), "{id} has no transcript");
        } else {
            assert!(counterexamples.is_empty(), "{id} records no counterexample");
        }
        if result["result"] == "proved" {
            let basis = result["basis"]["type"].as_str().unwrap();
            let certification = result["certification"].as_str().unwrap();
            if basis == "exhaustive" {
                assert_eq!(certification, "uncertified", "{id}: enumeration has no core certificate");
            }
        }
        if arm["outcome"].is_null() {
            assert!(
                ["invalid-input", "incomplete-input"].contains(&result["cause"]["type"].as_str().unwrap()),
                "{id} declined cause"
            );
        }
    }
}

#[trace("TC-972", "FR-483-AC-8")]
#[test]
fn value_validity_descriptors_never_list_population() {
    for case in cases() {
        for descriptor in case["registry"].as_array().unwrap() {
            if advertises(descriptor, "value-validity", "bounded") {
                let domains = descriptor["domains"].as_array().expect("bounded carries domains");
                assert!(!domains.iter().any(|d| d == "population"), "{}", case["id"]);
            }
        }
    }
}

#[trace("TC-973", "FR-484-AC-4")]
#[trace("TC-973", "FR-484-AC-7")]
#[trace("TC-974", "FR-485-AC-10")]
#[test]
fn check_arm_vectors_are_rekeyed_from_labels_and_freeze_no_digest() {
    assert!(!VECTORS.contains("\"digest\": \""), "no digest is written in the vectors");
    assert_eq!(node_id("a"), node_id("a"));
    assert_ne!(node_id("a"), node_id("b"));
    let document = composed(&cases()[1]);
    assert_eq!(
        document["counterexamples"][0]["counterexample"]["values"][0]["parameter"],
        node_id("vv.x")
    );
}
