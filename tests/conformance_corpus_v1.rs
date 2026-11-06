// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-336's closed conformance-corpus v1 structural contract.

use serde_json::{json, Value};

use ix_trace_rs::trace;
use quire_verification_contracts::jcs_canonicalize;

const SCHEMA: &str = include_str!("../proposals/conformance-corpus-v1/schema.json");
const FIXTURE: &str = include_str!("../proposals/conformance-corpus-v1/fixtures/complete-v1.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

fn definition_validator(schema: &Value, name: &str) -> jsonschema::Validator {
    let wrapper = json!({
        "$schema": schema["$schema"],
        "$defs": schema["$defs"],
        "$ref": format!("#/$defs/{name}"),
    });
    compile(&wrapper)
}

/// A corpus node id is the checked-package `NodeId` (FR-336, FR-322 "Operand
/// index segments"), so the corpus schema compiles against that schema.
fn compile(schema: &Value) -> jsonschema::Validator {
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
        .build(schema)
        .expect("schema compiles")
}

#[test]
#[trace("TC-969", "FR-336-AC-4", "FR-336-AC-5", "FR-336-AC-6", "FR-336-AC-11")]
fn conformance_corpus_v1_fixture_is_admitted_and_invalid_shapes_are_refused() {
    let schema = parsed(SCHEMA);
    jsonschema::meta::validate(&schema).expect("Draft 2020-12 schema is valid");
    assert_eq!(
        schema["$id"], "urn:quire:conformance-corpus:v1",
        "FR-336 contract id is fixed"
    );
    let validator = compile(&schema);
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

    // RP-01 is the positive source-key pair; RP-02..RP-05 prove missing or
    // empty authority/identity has no default or inference.
    let mut missing_authority = valid_provision_path.clone();
    missing_authority
        .as_object_mut()
        .expect("provision path is an object")
        .remove("authority");
    assert!(!replay_provision_validator.is_valid(&missing_authority));

    let mut empty_authority = valid_provision_path.clone();
    empty_authority["authority"] = json!("");
    assert!(!replay_provision_validator.is_valid(&empty_authority));

    let mut empty_identity = valid_provision_path.clone();
    empty_identity["identity"] = json!("");
    assert!(!replay_provision_validator.is_valid(&empty_identity));

    let mut extra_provision_member = valid_provision_path;
    extra_provision_member["bytes"] = json!([1, 2, 3]);
    assert!(!replay_provision_validator.is_valid(&extra_provision_member));

    // A corpus node id is the checked-package NodeId: a derived parameter key
    // holds its operands as index segments into the named package's graph,
    // and a nested operand node id is refused.
    let node_id_validator = definition_validator(&schema, "NodeId");
    assert!(node_id_validator.is_valid(&json!({
        "package": "fixture",
        "path": ["value", "parameter", "name", {"node": 2}, "x"],
        "ordinal": 0
    })));
    assert!(!node_id_validator.is_valid(&json!({
        "package": "fixture",
        "path": ["value", "parameter", "name",
            {"package": "fixture", "path": ["scalar_type", "text"], "ordinal": 0}, "x"],
        "ordinal": 0
    })));
}

/// FR-336-AC-10: an object environment identity is the closed two-member
/// `{authority, identity}`.
#[test]
#[trace("TC-969", "FR-336-AC-10")]
fn object_environment_identity_is_the_closed_two_member_pair() {
    let schema = parsed(SCHEMA);
    let environment = definition_validator(&schema, "ObjectEnvironmentRef");
    let two_member = json!({"identity": {"authority": "agent-ix", "identity": "env"}});
    assert!(
        environment.is_valid(&two_member),
        "AC-10: the environment identity is two-member"
    );
    let extra = json!({"identity": {"authority": "agent-ix", "identity": "env", "other": "x"}});
    assert!(
        !environment.is_valid(&extra),
        "AC-10: no other member on an environment identity"
    );
}

const REPLAY_REQUEST: &str =
    include_str!("../proposals/conformance-corpus-v1/fixtures/replay-request.json");

/// FR-336-AC-7: a replay request carries the proved checked package as its
/// closed top-level `package` member, so its node ids have a graph to resolve
/// against; a request without it, or with a package that is not a checked
/// package, is refused.
#[test]
#[trace("TC-970", "FR-336-AC-7")]
fn replay_request_carries_the_proved_checked_package() {
    let schema = parsed(SCHEMA);
    let request = definition_validator(&schema, "ReplayRequest");
    let fixture = parsed(REPLAY_REQUEST);
    assert!(
        request.is_valid(&fixture),
        "AC-7: the published request validates"
    );
    assert_eq!(fixture.as_object().expect("request is an object").len(), 14);

    let mut without_package = fixture.clone();
    without_package
        .as_object_mut()
        .expect("request is an object")
        .remove("package");
    assert!(
        !request.is_valid(&without_package),
        "AC-7: package is required"
    );

    let mut not_a_package = fixture;
    not_a_package["package"] = json!({"name": "example"});
    assert!(
        !request.is_valid(&not_a_package),
        "AC-7: package is a closed checked package"
    );
}

/// FR-336-AC-7, FR-323-AC-17: a `dependencies` entry may carry its own lock's
/// `refinement_internals` (absent means the empty list); the member is the
/// checked-package `RefinementInternal` list, non-empty when present, and the
/// entry stays closed.
#[test]
#[trace("TC-970", "FR-336-AC-7")]
fn dependency_entry_carries_optional_refinement_internals() {
    let schema = parsed(SCHEMA);
    let request = definition_validator(&schema, "ReplayRequest");
    let fixture = parsed(REPLAY_REQUEST);
    assert!(fixture["dependencies"][0]
        .get("refinement_internals")
        .is_none());
    assert!(request.is_valid(&fixture), "absent member is admitted");

    let mut carried = fixture.clone();
    carried["dependencies"][0]["refinement_internals"] =
        json!([{"refinement": "Spec::Twice", "internal": ["Spec::Twice::finish"]}]);
    assert!(
        request.is_valid(&carried),
        "a refinement_internals list is admitted"
    );

    let mut empty = fixture.clone();
    empty["dependencies"][0]["refinement_internals"] = json!([]);
    assert!(
        !request.is_valid(&empty),
        "an empty list is refused (absent is the empty list)"
    );
    let mut malformed = fixture.clone();
    malformed["dependencies"][0]["refinement_internals"] = json!([{"refinement": "Spec::Twice"}]);
    assert!(
        !request.is_valid(&malformed),
        "a RefinementInternal needs its internal list"
    );
    let mut extra = carried;
    extra["dependencies"][0]["invented"] = json!(true);
    assert!(!request.is_valid(&extra), "the entry stays closed");
}

/// Every NodeId a replay request names, found by shape (`package`, `path`,
/// `ordinal`), outside its embedded `package` member.
fn request_node_ids(value: &Value, found: &mut Vec<Value>) {
    match value {
        Value::Object(members) => {
            if members.len() == 3
                && ["package", "path", "ordinal"]
                    .iter()
                    .all(|key| members.contains_key(*key))
            {
                found.push(value.clone());
            } else {
                members
                    .values()
                    .for_each(|member| request_node_ids(member, found));
            }
        }
        Value::Array(items) => items.iter().for_each(|item| request_node_ids(item, found)),
        _ => {}
    }
}

/// FR-336-AC-7: the request's node ids are ids of the embedded package's
/// graph, and its dependency entries are the lock's selected dependencies.
#[test]
#[trace("TC-970", "FR-336-AC-7")]
fn replay_request_node_ids_resolve_in_its_embedded_package() {
    let fixture = parsed(REPLAY_REQUEST);
    let mut ids = Vec::new();
    for (member, value) in fixture.as_object().expect("request is an object") {
        if member != "package" {
            request_node_ids(value, &mut ids);
        }
    }
    assert_eq!(
        ids.len(),
        3,
        "target, obligation and declared domain name a node each: {ids:?}"
    );
    let graph_ids: Vec<&Value> = fixture["package"]["semantic_graph"]["nodes"]
        .as_array()
        .expect("package graph has nodes")
        .iter()
        .map(|node| &node["node_id"])
        .collect();
    for id in &ids {
        assert!(
            graph_ids.contains(&id),
            "AC-7: {id} resolves in the embedded package"
        );
    }
    let selected: Vec<&Value> = fixture["package"]["lock"]["dependency_selections"]
        .as_array()
        .expect("lock selections")
        .iter()
        .map(|selection| &selection["identity"])
        .collect();
    let entries: Vec<&Value> = fixture["dependencies"]
        .as_array()
        .expect("dependencies array")
        .iter()
        .map(|entry| &entry["identity"])
        .collect();
    assert_eq!(
        entries, selected,
        "AC-7: one dependency entry per selected dependency"
    );
}

/// FR-336-AC-7: unknown members, unknown tags and noncanonical decimals are
/// refused by the closed schema. Stage-limit setting names are FR-255's and
/// the schema admits any non-empty name, so that refusal belongs to the
/// consumer decoder (TC-1325); the schema must not refuse it.
#[test]
#[trace("TC-970", "FR-336-AC-7")]
fn replay_request_mutations_are_refused_one_at_a_time() {
    let schema = parsed(SCHEMA);
    let request = definition_validator(&schema, "ReplayRequest");
    let fixture = parsed(REPLAY_REQUEST);
    assert!(request.is_valid(&fixture));
    let parameter = fixture["obligation_identity"]["node_id"].clone();

    let mut unknown_member = fixture.clone();
    unknown_member["invented"] = json!(true);
    assert!(
        !request.is_valid(&unknown_member),
        "top-level unknown member"
    );
    let mut unknown_nested = fixture.clone();
    unknown_nested["accounting_limits"]["invented"] = json!("0");
    assert!(
        !request.is_valid(&unknown_nested),
        "structural-object unknown member"
    );

    let mut witness = fixture.clone();
    witness["source"]["assignments"] =
        json!([{"parameter": parameter, "value": {"kind": "integer", "decimal": "7"}}]);
    assert!(
        request.is_valid(&witness),
        "an integer assignment is admitted"
    );
    let mut unknown_witness_tag = witness.clone();
    unknown_witness_tag["source"]["assignments"][0]["value"]["kind"] = json!("text-ish");
    assert!(
        !request.is_valid(&unknown_witness_tag),
        "unknown WitnessValue tag"
    );
    let mut json_number = witness.clone();
    json_number["source"]["assignments"][0]["value"]["decimal"] = json!(7);
    assert!(
        !request.is_valid(&json_number),
        "JSON number in place of a decimal string"
    );
    let mut noncanonical = witness;
    noncanonical["source"]["assignments"][0]["value"]["decimal"] = json!("07");
    assert!(
        !request.is_valid(&noncanonical),
        "noncanonical integer decimal"
    );

    let mut unknown_domain_tag = fixture.clone();
    unknown_domain_tag["declared_domains"][0]["domain"]["kind"] = json!("elsewhere");
    assert!(
        !request.is_valid(&unknown_domain_tag),
        "unknown DomainKey tag"
    );

    let mut leading_zero_limit = fixture.clone();
    leading_zero_limit["accounting_limits"]["work_units"] = json!("00");
    assert!(
        !request.is_valid(&leading_zero_limit),
        "noncanonical limit decimal"
    );
    let mut number_limit = fixture.clone();
    number_limit["stage_limits"] = json!({"format.output_bytes": 5});
    assert!(
        !request.is_valid(&number_limit),
        "stage limit is a decimal string"
    );

    let mut outside_setting = fixture;
    outside_setting["stage_limits"] = json!({"not.a.setting": "5"});
    assert!(
        request.is_valid(&outside_setting),
        "an FR-255 setting name is the consumer decoder's refusal, not the schema's"
    );
}
