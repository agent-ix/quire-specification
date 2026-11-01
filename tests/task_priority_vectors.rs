// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-419/FR-364 optional-priority and scheduler-order contract vectors.
//!
//! These tests qualify the authored vectors against the QSpec wire schema and
//! the policy-order oracle written in those vectors. They do not replace the
//! independent QSL strict-reader/replay tests required by TC-372.

use ix_trace_rs::trace;
use serde_json::{Value, json};

const SCHEMA: &str = include_str!("../proposals/backend-provider-v1/schema.json");
const CHECKED_PACKAGE_SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const VECTORS: &str = include_str!("../proposals/backend-provider-v1/task-priority-vectors.json");

fn parsed(text: &str) -> Value {
    serde_json::from_str(text).expect("authored JSON is valid")
}

fn definition(name: &str) -> jsonschema::Validator {
    let schema = parsed(SCHEMA);
    let registry = jsonschema::Registry::new()
        .add(
            "urn:quire:checked-package:v2",
            parsed(CHECKED_PACKAGE_SCHEMA),
        )
        .expect("checked-package schema registers")
        .add("urn:quire:backend-provider:v1", schema)
        .expect("backend-provider schema registers")
        .prepare()
        .expect("registry prepares");
    jsonschema::options()
        .with_registry(&registry)
        .build(&json!({
            "$ref": format!("urn:quire:backend-provider:v1#/$defs/{name}")
        }))
        .expect("schema definition compiles")
}

fn rational(numerator: &str) -> Value {
    json!({"numerator": numerator, "denominator": "1"})
}

fn affine_count() -> Value {
    json!({"base": 0, "cycle_stride": 0, "cohort_stride": 0})
}

fn source_transition(operation: &str) -> Value {
    json!({
        "type": "transition",
        "operation": operation,
        "receiver": null,
        "arguments": []
    })
}

fn with_priority(mut object: Value, wire_priority: &Value) -> Value {
    match wire_priority {
        Value::String(mode) if mode == "missing" => {}
        other => {
            object["priority"] = other.clone();
        }
    }
    object
}

fn identity_wire(carrier: &str, wire_priority: &Value) -> Value {
    let source = source_transition("Source::release");
    let zero = rational("0");
    let one = rational("1");
    match carrier {
        "TaskTransitionIdentity" => with_priority(
            json!({
                "type": "task-transition",
                "action": "activate",
                "taskset": "TS",
                "task": "a",
                "job": 0,
                "slot": 0,
                "generation": 0,
                "release": source,
                "source_transition": source,
                "demand": one,
                "jitter": zero,
                "deadline": one
            }),
            wire_priority,
        ),
        "QueueGrowthJob" => with_priority(
            json!({
                "ref": "job-a0",
                "taskset": "TS",
                "task": "a",
                "occurrence": affine_count(),
                "generation": affine_count(),
                "queue_position": affine_count(),
                "demand": one,
                "remaining": one,
                "activation_offset": {"base": zero, "cycle_stride": zero, "cohort_stride": zero},
                "deadline_offset": {"base": one, "cycle_stride": zero, "cohort_stride": zero},
                "jitter": zero,
                "allowance": one,
                "status": "ready"
            }),
            wire_priority,
        ),
        "QueueGrowthTransition" => with_priority(
            json!({
                "type": "task-queue-growth-transition",
                "action": "dispatch",
                "taskset": "TS",
                "task": "a",
                "job_ref": "job-a0",
                "occurrence": affine_count(),
                "generation": affine_count(),
                "virtual_position": affine_count(),
                "release_transition": source,
                "source_transition": null,
                "demand": one,
                "jitter": zero,
                "deadline_offset": {"base": one, "cycle_stride": zero, "cohort_stride": zero}
            }),
            wire_priority,
        ),
        other => panic!("unknown priority carrier {other}"),
    }
}

fn checked_priority_agrees(declared: &Value, wire: &Value) -> bool {
    match (declared, wire) {
        (Value::Null, Value::String(mode)) if mode == "missing" => true,
        (Value::Number(expected), Value::Number(actual)) => expected == actual,
        _ => false,
    }
}

/// TC-372 RT-31 / FR-364-AC-15: all three task identity carriers omit
/// priority only for an undeclared task, preserve exact declarations, and
/// reject absent, extra, forged and null values at the appropriate boundary.
#[trace("TC-372", "FR-364-AC-15")]
#[trace("TC-372", "FR-419-AC-11")]
#[test]
fn task_priority_identity_vectors_match_schema_and_checked_declarations() {
    let vectors = parsed(VECTORS);
    for case in vectors["identity_cases"]
        .as_array()
        .expect("identity cases")
    {
        let id = case["id"].as_str().expect("case id");
        let carrier = case["carrier"].as_str().expect("carrier");
        let declared = &case["declared_priority"];
        let wire_priority = &case["wire_priority"];
        let expected = case["expected"].as_str().expect("expected result");
        let wire = identity_wire(carrier, wire_priority);
        let schema_valid = definition(carrier).is_valid(&wire);

        match expected {
            "accept" => {
                assert!(schema_valid, "{id}: valid {carrier} shape");
                assert!(
                    checked_priority_agrees(declared, wire_priority),
                    "{id}: priority agrees with checked declaration"
                );
            }
            "refuse" => {
                assert!(schema_valid, "{id}: semantic mismatch has valid wire shape");
                assert!(
                    !checked_priority_agrees(declared, wire_priority),
                    "{id}: missing, extra or forged priority is refused"
                );
            }
            "malformed" => assert!(!schema_valid, "{id}: null is not a priority value"),
            other => panic!("{id}: unknown expected result {other}"),
        }
    }
}

fn ordered_ids(case: &Value) -> Vec<String> {
    let policy = case["policy"].as_str().expect("policy");
    let mut jobs = case["jobs"].as_array().expect("jobs").clone();
    jobs.sort_by(|left, right| {
        if policy == "fixed-priority" {
            left["priority"]
                .as_u64()
                .expect("fixed-priority job declares priority")
                .cmp(&right["priority"].as_u64().expect("declared priority"))
        } else {
            left["deadline"]
                .as_u64()
                .expect("deadline")
                .cmp(&right["deadline"].as_u64().expect("deadline"))
                .then_with(|| {
                    left["task"]
                        .as_str()
                        .unwrap()
                        .cmp(right["task"].as_str().unwrap())
                })
                .then_with(|| {
                    left["activation"]
                        .as_str()
                        .unwrap()
                        .cmp(right["activation"].as_str().unwrap())
                })
                .then_with(|| {
                    left["occurrence"]
                        .as_u64()
                        .unwrap()
                        .cmp(&right["occurrence"].as_u64().unwrap())
                })
        }
    });
    jobs.into_iter()
        .map(|job| job["id"].as_str().unwrap().to_owned())
        .collect()
}

fn queue_growth_word(case: &Value) -> Value {
    let jobs = case["jobs"]
        .as_array()
        .expect("jobs")
        .iter()
        .enumerate()
        .map(|(index, job)| {
            let rational = |member: &str| {
                json!({
                    "numerator": job[member].as_u64().unwrap().to_string(),
                    "denominator": "1"
                })
            };
            let mut wire = json!({
                "ref": job["id"],
                "taskset": "TS",
                "task": job["task"],
                "occurrence": {"base": job["occurrence"], "cycle_stride": 0, "cohort_stride": 0},
                "generation": {"base": 0, "cycle_stride": 0, "cohort_stride": 0},
                "queue_position": {"base": index, "cycle_stride": 0, "cohort_stride": 0},
                "demand": {"numerator": "1", "denominator": "1"},
                "remaining": {"numerator": "1", "denominator": "1"},
                "activation_offset": {"base": {"numerator": "0", "denominator": "1"}, "cycle_stride": {"numerator": "0", "denominator": "1"}, "cohort_stride": {"numerator": "0", "denominator": "1"}},
                "deadline_offset": {"base": rational("deadline"), "cycle_stride": {"numerator": "0", "denominator": "1"}, "cohort_stride": {"numerator": "0", "denominator": "1"}},
                "jitter": {"numerator": "0", "denominator": "1"},
                "allowance": {"numerator": "1", "denominator": "1"},
                "status": "ready"
            });
            if !job["priority"].is_null() {
                wire["priority"] = job["priority"].clone();
            }
            wire
        })
        .collect::<Vec<_>>();
    json!({
        "cycle_offset": 0,
        "block_offset": 0,
        "prefix": [],
        "growth_block": jobs,
        "suffix": []
    })
}

/// TC-372 RT-29/30/32 / FR-419-AC-11: execute EDF priority/tie vectors and
/// preserve fixed-priority order. Task names use UTF-8 lexical order, which
/// preserves Unicode scalar value order for valid strings.
#[trace("TC-372", "FR-419-AC-11")]
#[trace("TC-372", "FR-364-AC-15")]
#[test]
fn scheduler_order_vectors_recompute_edf_ties_and_fixed_priority() {
    let vectors = parsed(VECTORS);
    for case in vectors["order_cases"].as_array().expect("order cases") {
        let id = case["id"].as_str().expect("case id");
        let expected = case["expected_order"]
            .as_array()
            .expect("expected order")
            .iter()
            .map(|job| job.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(ordered_ids(case), expected, "{id}");
        if let Some(provided) = case.get("provided_order") {
            let provided = provided
                .as_array()
                .expect("provided order")
                .iter()
                .map(|job| job.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            assert_eq!(
                case["expected_result"], "refuse",
                "{id}: adversarial ready-order vector is expected to refuse"
            );
            assert_ne!(provided, expected, "{id}: forged order contradicts policy");
        }
        let word = queue_growth_word(case);
        assert!(
            definition("QueueGrowthWord").is_valid(&word),
            "{id}: scheduler vector is a valid queue-growth word"
        );

        let mut source_order = case["source_declaration_order"]
            .as_array()
            .expect("source declaration order")
            .iter()
            .map(|task| task.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        source_order.reverse();
        let mut reversed = case.clone();
        reversed["source_declaration_order"] = json!(source_order);
        assert_eq!(
            ordered_ids(&reversed),
            expected,
            "{id}: source order is irrelevant"
        );
    }
}
