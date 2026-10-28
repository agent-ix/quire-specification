// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-143 "The `absent` Option literal" (TC-188): the literal is typed by the
//! unique expected type of its position and is `Option<T>` only. The oracle
//! below is the rule as FR-143 states it; the published vectors run through it.

use ix_trace_rs::trace;
use serde_json::{Value, json};

const VECTORS: &str = include_str!("../proposals/quire-v1/option-absent-literal-vectors.json");
const FR143: &str =
    include_str!("../spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md");
const FR153: &str =
    include_str!("../spec/functional/type-model/FR-153-query-closed-model-environments.md");
const GRAMMAR: &str = include_str!("../proposals/quire-v1/shared-grammar.md");
const TC188: &str = include_str!("../spec/test-cases/TC-188-record-tuple-and-recursive-values.md");

fn vectors() -> Value {
    serde_json::from_str(VECTORS).expect("authored vectors are JSON")
}

fn type_name(value: &Value) -> String {
    let kind = value["kind"].as_str().expect("kind");
    match value["of"].as_str() {
        Some(inner) => format!("{kind}<{inner}>"),
        None => kind.to_owned(),
    }
}

/// The typing rule: `absent` takes exactly the `Option<T>` its position
/// expects; no unique expected type is `ambiguous-literal`; any other expected
/// type, a `?` slot included, is `type-mismatch`.
fn type_absent(expected: &Value) -> Value {
    if expected.is_null() {
        return json!({"refused": {"code": "ill_typed", "cause": "ambiguous-literal"}});
    }
    if expected["kind"] == "Option" {
        return json!({"type": type_name(expected)});
    }
    json!({"refused": {"code": "ill_typed", "cause": "type-mismatch"}})
}

/// The tail of a `lookup` after its closing parenthesis: `absent`, then one
/// absence mode. The literal never stands here.
fn lookup_tail(tail: &[&str]) -> Value {
    match tail {
        ["absent", mode @ ("undefined" | "empty" | "refused")] => json!({"mode": mode}),
        _ => json!({"refused": {"code": "invalid_syntax", "cause": "unexpected-token"}}),
    }
}

fn run(vectors: &Value, criterion: &str) -> usize {
    let mut ran = 0;
    for case in vectors["typing_cases"].as_array().expect("typing_cases") {
        if case["criterion"] == criterion {
            assert_eq!(
                type_absent(&case["expected_type"]),
                case["outcome"],
                "{}",
                case["name"]
            );
            ran += 1;
        }
    }
    for case in vectors["lookup_tail_cases"]
        .as_array()
        .expect("lookup_tail_cases")
    {
        if case["criterion"] == criterion {
            let tail: Vec<&str> = case["tail"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t.as_str().unwrap())
                .collect();
            assert_eq!(lookup_tail(&tail), case["outcome"], "{}", case["name"]);
            ran += 1;
        }
    }
    ran
}

fn ac_row<'a>(id: &str) -> &'a str {
    FR143
        .lines()
        .find(|line| line.starts_with(&format!("| {id} |")))
        .unwrap_or_else(|| panic!("{id} row"))
}

#[trace("TC-188", "FR-143-AC-18")]
#[test]
fn tc_188_absent_takes_the_option_its_position_expects() {
    assert_eq!(run(&vectors(), "FR-143-AC-18"), 6);
    let row = ac_row("FR-143-AC-18");
    assert!(row.contains("Option<Clock>") && row.contains("= absent"));
    assert!(FR143.contains("## The `absent` Option literal"));
    assert!(FR153.contains("`none`, the value the literal `absent` denotes"));
    assert!(
        GRAMMAR.contains("| 'none' | 'absent' | 'self'"),
        "primary admits absent"
    );
    assert!(
        TC188.contains("| R15 |") && TC188.contains("(lookup<M::A>(p, r) absent empty) = absent")
    );
}

#[trace("TC-188", "FR-143-AC-19")]
#[test]
fn tc_188_absent_without_an_option_context_is_ill_typed() {
    assert_eq!(run(&vectors(), "FR-143-AC-19"), 6);
    let row = ac_row("FR-143-AC-19");
    assert!(row.contains("ambiguous-literal") && row.contains("type-mismatch"));
    assert!(TC188.contains("| R16 |"));
}

#[trace("TC-188", "FR-143-AC-20")]
#[test]
fn tc_188_an_optional_slot_takes_no_absent_and_the_lookup_mode_keeps_its_meaning() {
    assert_eq!(run(&vectors(), "FR-143-AC-20"), 5);
    let row = ac_row("FR-143-AC-20");
    assert!(row.contains("b: absent") && row.contains("unexpected-token"));
    assert!(TC188.contains("| R17 |"));
}

/// Mutation check for FR-143-AC-18 to FR-143-AC-20: a vector whose outcome is
/// changed, or a rule that defaults the missing context, is detected.
#[trace("TC-188", "FR-143-AC-18", "FR-143-AC-19", "FR-143-AC-20")]
#[test]
fn tc_188_vector_mutations_are_detected() {
    for (from, to) in [
        (
            "\"cause\": \"ambiguous-literal\"",
            "\"cause\": \"type-mismatch\"",
        ),
        (
            "{\"type\": \"Option<Clock>\"}",
            "{\"type\": \"Option<Integer>\"}",
        ),
        (
            "\"tail\": [\"absent\", \"absent\"]",
            "\"tail\": [\"absent\", \"empty\"]",
        ),
        ("\"kind\": \"OptionalSlot\"", "\"kind\": \"Option\""),
    ] {
        assert!(VECTORS.contains(from), "{from}");
        let mutated: Value = serde_json::from_str(&VECTORS.replacen(from, to, 1)).unwrap();
        let caught = ["FR-143-AC-18", "FR-143-AC-19", "FR-143-AC-20"]
            .iter()
            .any(|ac| std::panic::catch_unwind(|| run(&mutated, ac)).is_err());
        assert!(caught, "mutation `{to}` must be detected");
    }
}
