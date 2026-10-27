// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-336's closed conformance-corpus v1 structural contract.

use serde_json::{json, Value};

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_canonicalize;

const SCHEMA: &str = include_str!("../proposals/conformance-corpus-v1/schema.json");
const FIXTURE: &str = include_str!("../proposals/conformance-corpus-v1/fixtures/complete-v1.json");

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

#[test]
#[trace("TC-969", "FR-336-AC-4", "FR-336-AC-5")]
fn conformance_corpus_v1_fixture_is_admitted_and_invalid_shapes_are_refused() {
    let schema = parsed(SCHEMA);
    jsonschema::meta::validate(&schema).expect("Draft 2020-12 schema is valid");
    assert_eq!(
        schema["$id"], "urn:quire:conformance-corpus:v1",
        "FR-336 contract id is fixed"
    );
    let validator = jsonschema::validator_for(&schema).expect("schema compiles");
    let fixture = parsed(FIXTURE);
    assert!(validator.is_valid(&fixture), "fixture follows the schema");
    assert_eq!(
        jcs_canonicalize(&fixture).expect("fixture canonicalizes"),
        FIXTURE.as_bytes(),
        "fixture bytes use canonical JSON"
    );

    let mut missing_member = fixture.clone();
    missing_member["vectors"][0]
        .as_object_mut()
        .expect("vector is an object")
        .remove("expected");
    assert!(!validator.is_valid(&missing_member));

    let mut unknown_member = fixture.clone();
    unknown_member["vectors"][0]["invented"] = json!(true);
    assert!(!validator.is_valid(&unknown_member));

    let mut invalid_class = fixture.clone();
    invalid_class["vectors"][0]["class"] = json!("outside-domain");
    assert!(!validator.is_valid(&invalid_class));

    let mut invalid_consumer = fixture.clone();
    invalid_consumer["vectors"][0]["subject"]["consumer_type"] = json!("outside-domain");
    assert!(!validator.is_valid(&invalid_consumer));

    let mut invalid_disposition = fixture;
    invalid_disposition["vectors"][0]["expected"]["disposition"] = json!("outside-domain");
    assert!(!validator.is_valid(&invalid_disposition));

    let kinds = [
        "source",
        "model",
        "input",
        "call",
        "object-context",
        "replay-request",
        "bytes",
    ];
    for kind in kinds {
        let mut corpus = parsed(FIXTURE);
        corpus["vectors"][0]["input"]["files"] = json!([{ "kind": kind, "path": "input/example.json" }]);
        assert!(validator.is_valid(&corpus), "input file kind {kind} is admitted");
    }

    let mut invalid_kind = parsed(FIXTURE);
    invalid_kind["vectors"][0]["input"]["files"] =
        json!([{ "kind": "unknown", "path": "input/example.json" }]);
    assert!(!validator.is_valid(&invalid_kind));
}
