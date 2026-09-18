// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-208 business-meaning admission fixture (K2): schema conformance and
//! refusal-vocabulary completeness.
//!
//! This qualification check is deliberately data-only. It validates the
//! authored fixture against its schema and checks that every refusal
//! situation FR-208-AC-4 through AC-12 names has at least one vector, by
//! exact vector id. It runs no business-meaning binder of its own: the real
//! binder is a QSL responsibility that does not exist yet (see FR-208's
//! Dependencies and TC-235's Test Procedure), so TC-235 and
//! FR-208-AC-4..12 stay `planned`/unbacked until that binder lands. This
//! file replaces `business_construct_meaning.rs`, which defined its own
//! oracle in the same file it tested and so could never fail against the
//! real requirement.

use ix_trace_rs::trace;
use serde_json::Value;

const SCHEMA: &str =
    include_str!("../proposals/checked-package-v2/business-meanings-k2.schema.json");
const FIXTURE: &str =
    include_str!("../proposals/checked-package-v2/fixtures/business-meanings-k2.json");

fn parsed_fixture(bytes: &str) -> Value {
    serde_json::from_str(bytes).expect("authored fixture is valid JSON")
}

/// Every distinct refusal situation FR-208-AC-4 through AC-12 names, keyed
/// by the fixture vector id that must witness it, together with the
/// `(ac, code, cause)` triple that vector must carry. Kept as one flat list
/// so a missing situation fails by name, not by count.
const EXPECTED_REFUSALS: &[(&str, &str, &str, &str)] = &[
    // AC-4: record-value-type identity field / operation; event-type
    // identity field / operation / occurrence field of another type /
    // occurrence field naming nothing.
    (
        "AC4-record-value-type-identity-field",
        "FR-208-AC-4",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC4-record-value-type-operation",
        "FR-208-AC-4",
        "unsupported_construct",
        "declaration-form",
    ),
    (
        "AC4-event-type-identity-field",
        "FR-208-AC-4",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC4-event-type-operation",
        "FR-208-AC-4",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC4-event-type-occurrence-field-wrong-type",
        "FR-208-AC-4",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC4-event-type-occurrence-field-missing-name",
        "FR-208-AC-4",
        "missing_declaration",
        "missing-name",
    ),
    // AC-5: state machine.
    (
        "AC5-no-operations",
        "FR-208-AC-5",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC5-no-states",
        "FR-208-AC-5",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC5-no-transitions",
        "FR-208-AC-5",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC5-emits-not-event-type",
        "FR-208-AC-5",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC5-guard-not-quire",
        "FR-208-AC-5",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC5-transition-endpoint-missing-name",
        "FR-208-AC-5",
        "missing_declaration",
        "missing-name",
    ),
    (
        "AC5-repeated-state",
        "FR-208-AC-5",
        "invalid_model_binding",
        "conflicting-binding",
    ),
    // AC-6: process.
    (
        "AC6-no-identity-field",
        "FR-208-AC-6",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC6-no-steps",
        "FR-208-AC-6",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC6-consumes-not-event-type",
        "FR-208-AC-6",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC6-step-entry-missing-name",
        "FR-208-AC-6",
        "missing_declaration",
        "missing-name",
    ),
    (
        "AC6-repeated-step-name",
        "FR-208-AC-6",
        "invalid_model_binding",
        "conflicting-binding",
    ),
    // AC-7: persistence interface.
    (
        "AC7-field-present",
        "FR-208-AC-7",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC7-no-operations",
        "FR-208-AC-7",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC7-empty-persists",
        "FR-208-AC-7",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC7-persists-not-object-type",
        "FR-208-AC-7",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC7-persists-missing-name",
        "FR-208-AC-7",
        "missing_declaration",
        "missing-name",
    ),
    (
        "AC7-operation-contract",
        "FR-208-AC-7",
        "unsupported_construct",
        "declaration-form",
    ),
    (
        "AC7-quire-clause",
        "FR-208-AC-7",
        "unsupported_construct",
        "declaration-form",
    ),
    // AC-8: namespace.
    (
        "AC8-field-present",
        "FR-208-AC-8",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC8-operation-present",
        "FR-208-AC-8",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC8-member-is-a-namespace",
        "FR-208-AC-8",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC8-member-already-in-earlier-namespace",
        "FR-208-AC-8",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC8-member-missing-name",
        "FR-208-AC-8",
        "missing_declaration",
        "missing-name",
    ),
    (
        "AC8-quire-clause",
        "FR-208-AC-8",
        "unsupported_construct",
        "declaration-form",
    ),
    // AC-9: cross-meaning member typing.
    (
        "AC9-field-typeref-wrong-meaning",
        "FR-208-AC-9",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC9-population-member-wrong-meaning",
        "FR-208-AC-9",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    // AC-10: invariant evaluation and its refusals.
    (
        "AC10-forbidden-pre-read-in-invariant",
        "FR-208-AC-10",
        "wrong_snapshot",
        "forbidden-pre-read",
    ),
    (
        "AC10-state-machine-invariant-wrong-context",
        "FR-208-AC-10",
        "invalid_model_binding",
        "wrong-context",
    ),
    // AC-11: pre/post condition reads.
    (
        "AC11-forbidden-pre-read-in-precondition",
        "FR-208-AC-11",
        "wrong_snapshot",
        "forbidden-pre-read",
    ),
    // AC-12: specialization.
    (
        "AC12-supertype-of-another-meaning",
        "FR-208-AC-12",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC12-state-or-transition-declared-by-two-owners",
        "FR-208-AC-12",
        "invalid_model_binding",
        "derivation-conflict",
    ),
    (
        "AC12-ancestor-occurrence-field-not-redefined",
        "FR-208-AC-12",
        "invalid_model_binding",
        "derivation-conflict",
    ),
    (
        "AC12-subtype-guard-names-ancestor-invariant",
        "FR-208-AC-12",
        "invalid_model_binding",
        "malformed-declaration",
    ),
    (
        "AC12-persistence-interface-with-supertypes",
        "FR-208-AC-12",
        "unsupported_construct",
        "declaration-form",
    ),
    (
        "AC12-namespace-with-supertypes",
        "FR-208-AC-12",
        "unsupported_construct",
        "declaration-form",
    ),
    (
        "AC12-specialization-cycle",
        "FR-208-AC-12",
        "invalid_model_binding",
        "specialization-cycle",
    ),
];

#[trace("TC-237", "FR-208")]
#[test]
fn k2_fixture_conforms_to_its_schema() {
    let schema = jsonschema::validator_for(&parsed_fixture(SCHEMA)).expect("K2 schema is valid");
    let fixture = parsed_fixture(FIXTURE);
    let errors: Vec<String> = schema
        .iter_errors(&fixture)
        .map(|error| format!("{error} at {}", error.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{errors:#?}");
}

#[trace("TC-237", "FR-208")]
#[test]
fn k2_fixture_covers_every_named_refusal_situation() {
    let fixture = parsed_fixture(FIXTURE);
    let vectors = fixture["vectors"].as_array().expect("vectors");
    for (id, ac, code, cause) in EXPECTED_REFUSALS {
        let vector = vectors
            .iter()
            .find(|vector| vector["id"] == *id)
            .unwrap_or_else(|| panic!("no fixture vector witnesses situation `{id}`"));
        assert_eq!(vector["ac"], *ac, "vector `{id}` is tagged to the wrong AC");
        assert_eq!(
            vector["kind"], "refused",
            "vector `{id}` must be a refused vector"
        );
        assert_eq!(vector["code"], *code, "vector `{id}` has the wrong code");
        assert_eq!(vector["cause"], *cause, "vector `{id}` has the wrong cause");
    }
}

#[trace("TC-237", "FR-208")]
#[test]
fn k2_fixture_names_every_k2_node() {
    let fixture = parsed_fixture(FIXTURE);
    let ids: Vec<&str> = fixture["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .map(|node| node["id"].as_str().expect("node id"))
        .collect();
    for expected in [
        "ix://test/orders/Order",
        "ix://test/orders/Money",
        "ix://test/orders/OrderPlaced",
        "ix://test/orders/OrderLifecycle",
        "ix://test/orders/Fulfilment",
        "ix://test/orders/OrderStore",
        "ix://test/orders/OrderManagement",
    ] {
        assert!(
            ids.contains(&expected),
            "K2 fixture is missing node `{expected}`"
        );
    }
}

#[trace("TC-237", "FR-208")]
#[test]
fn k2_fixture_has_at_least_one_admitted_vector_per_specializable_meaning() {
    let fixture = parsed_fixture(FIXTURE);
    let vectors = fixture["vectors"].as_array().expect("vectors");
    for id in [
        "AC12-record-value-type-inheritance-admitted",
        "AC12-event-type-inheritance-admitted",
        "AC12-state-machine-inheritance-admitted",
        "AC12-process-inherits-no-steps-admitted",
    ] {
        let vector = vectors
            .iter()
            .find(|vector| vector["id"] == id)
            .unwrap_or_else(|| panic!("no fixture vector witnesses `{id}`"));
        assert_eq!(vector["kind"], "admitted");
    }
}
