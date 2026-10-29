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

fn definition_validator(schema: &Value, name: &str) -> jsonschema::Validator {
    let wrapper = json!({
        "$schema": schema["$schema"],
        "$defs": schema["$defs"],
        "$ref": format!("#/$defs/{name}"),
    });
    jsonschema::validator_for(&wrapper).expect("definition compiles")
}

#[test]
#[trace("TC-969", "FR-336-AC-4", "FR-336-AC-5", "FR-336-AC-6")]
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
        "population",
        "replay-request",
        "bytes",
    ];
    for kind in kinds {
        let mut corpus = parsed(FIXTURE);
        corpus["vectors"][0]["input"]["files"] =
            json!([{ "kind": kind, "path": "input/example.json" }]);
        assert!(
            validator.is_valid(&corpus),
            "input file kind {kind} is admitted"
        );
    }

    let mut invalid_kind = parsed(FIXTURE);
    invalid_kind["vectors"][0]["input"]["files"] =
        json!([{ "kind": "unknown", "path": "input/example.json" }]);
    assert!(!validator.is_valid(&invalid_kind));

    let call_validator = definition_validator(&schema, "CallInput");
    let valid_call = json!({
        "function": "check",
        "arguments": [
            {"kind": "boolean", "value": true},
            {"kind": "integer", "decimal": "-17"},
            {
                "kind": "reference",
                "universe": {"package": "orders", "path": ["Orders"], "ordinal": 0},
                "type": {"package": "orders", "path": ["Orders", "Item"], "ordinal": 0},
                "object_identity": "item-1"
            },
            {"kind": "population", "name": "items"}
        ],
        "accounting": {"work_units": "18446744073709551615"}
    });
    assert!(call_validator.is_valid(&valid_call));

    let mut unknown_tag = valid_call.clone();
    unknown_tag["arguments"][0]["kind"] = json!("text");
    assert!(!call_validator.is_valid(&unknown_tag));

    let mut malformed_decimal = valid_call.clone();
    malformed_decimal["arguments"][1]["decimal"] = json!("-0");
    assert!(!call_validator.is_valid(&malformed_decimal));

    let mut malformed_identity = valid_call.clone();
    malformed_identity["arguments"][2]["universe"] = json!("not-a-node-id");
    assert!(!call_validator.is_valid(&malformed_identity));

    let mut unknown_call_member = valid_call.clone();
    unknown_call_member["invented"] = json!(true);
    assert!(!call_validator.is_valid(&unknown_call_member));

    let mut named_argument = valid_call.clone();
    named_argument["arguments"][0] = json!({
        "parameter": "enabled",
        "value": {"kind": "boolean", "value": true}
    });
    assert!(!call_validator.is_valid(&named_argument));

    let mut invalid_limit = valid_call;
    invalid_limit["accounting"]["work_units"] = json!("-1");
    assert!(!call_validator.is_valid(&invalid_limit));

    let replay_provision_validator = definition_validator(&schema, "ReplayProvisionPath");
    let valid_provision_path = json!({
        "authority": "fixture",
        "identity": "source.quire",
        "bytes_path": "inputs/source.quire"
    });
    assert!(replay_provision_validator.is_valid(&valid_provision_path));

    let mut missing_identity = valid_provision_path.clone();
    missing_identity
        .as_object_mut()
        .expect("provision path is an object")
        .remove("identity");
    assert!(!replay_provision_validator.is_valid(&missing_identity));

    let mut extra_provision_member = valid_provision_path;
    extra_provision_member["bytes"] = json!([1, 2, 3]);
    assert!(!replay_provision_validator.is_valid(&extra_provision_member));
}

#[trace("TC-970", "FR-336-AC-8")]
#[test]
fn replay_provision_rows_use_fr001_labels_without_digest_members() {
    let schema = parsed(SCHEMA);
    let validator = definition_validator(&schema, "ReplayProvisionPath");
    let valid = json!({
        "identity": {
            "authority": "example",
            "identity": "source/main",
            "revision_namespace": "git",
            "revision": "main"
        },
        "bytes_path": "inputs/source.quire"
    });
    assert!(validator.is_valid(&valid));

    let mut missing_label = valid.clone();
    missing_label["identity"]
        .as_object_mut()
        .unwrap()
        .remove("revision");
    assert!(!validator.is_valid(&missing_label));

    let mut digest_keyed = valid;
    digest_keyed["digest"] = json!("a".repeat(64));
    assert!(!validator.is_valid(&digest_keyed));

    let object_environment = definition_validator(&schema, "ObjectEnvironmentRef");
    let valid_environment = json!({
        "identity": {
            "authority": "example",
            "identity": "model/main",
            "revision_namespace": "git",
            "revision": "main"
        }
    });
    assert!(object_environment.is_valid(&valid_environment));
    let mut digest_keyed_environment = valid_environment;
    digest_keyed_environment["digest"] = json!(format!("sha256-jcs:{}", "b".repeat(64)));
    assert!(!object_environment.is_valid(&digest_keyed_environment));
}
