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

/// An `Option<Integer>` operand: the literal `absent`, a held `{"some": n}`, or
/// a name bound by the case.
fn operand(case: &Value, spelling: &Value) -> Option<i64> {
    match spelling {
        Value::String(text) if text == "absent" => None,
        Value::String(name) => operand(case, &case[name.as_str()]),
        held => Some(held["some"].as_i64().expect("a held value")),
    }
}

fn option_value(value: Option<i64>) -> Value {
    value.map_or(json!("none"), |held| json!({"some": held}))
}

/// FR-143-AC-18's forms, evaluated over `Option<Integer>`: `absent` is the
/// `none` value, an `if` takes its chosen branch, a function returns its body,
/// `=` compares option values, and `lookup ... absent empty` is `none` for a
/// key the population does not hold. A literal against a literal has no
/// expected type and is refused (FR-143-AC-19).
fn evaluate(case: &Value) -> Value {
    match case["form"].as_str().expect("form") {
        "if" => {
            let branch = if case["cond"] == true { "then" } else { "else" };
            option_value(operand(case, &case[branch]))
        }
        "function" => {
            let mut bound = case.clone();
            bound["x"] = case["argument"].clone();
            option_value(operand(&bound, &case["body"]))
        }
        "eq" => {
            let literal = |side: &str| case[side] == "absent";
            if literal("left") && literal("right") {
                return json!("refused");
            }
            json!(operand(case, &case["left"]) == operand(case, &case["right"]))
        }
        "lookup_eq" => {
            let held = case["held_keys"].as_array().expect("held_keys");
            let found = held.contains(&case["key"]).then_some(1);
            json!(found.is_none())
        }
        other => panic!("unexpected form {other}"),
    }
}

fn run(vectors: &Value, criterion: &str) -> usize {
    let mut ran = 0;
    for case in vectors["evaluation_cases"].as_array().expect("evaluation_cases") {
        if case["criterion"] == criterion {
            assert_eq!(evaluate(case), case["expected"], "{}", case["name"]);
            ran += 1;
        }
    }
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

/// The cells of the single `| id | statement | evidence |` row of `id` in `doc`,
/// whitespace-normalised.
fn row_cells(doc: &str, id: &str) -> Vec<String> {
    let rows: Vec<&str> = doc
        .lines()
        .filter(|line| line.starts_with(&format!("| {id} |")))
        .collect();
    assert_eq!(rows.len(), 1, "exactly one {id} row");
    rows[0]
        .trim_matches('|')
        .split(" | ")
        .map(|cell| cell.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

fn ac_row(id: &str) -> Vec<String> {
    let cells = row_cells(FR143, id);
    assert_eq!(cells.len(), 3, "{id} has id, statement and evidence cells");
    assert_eq!(cells[2], "Test (TC-188)", "{id} evidence");
    cells
}

/// The alternatives of the shared-grammar `primary` production.
fn primary_alternatives() -> Vec<String> {
    let start = GRAMMAR
        .match_indices("\nprimary ")
        .map(|(at, _)| at + 1)
        .find(|at| GRAMMAR[*at..].lines().next().is_some_and(|line| line.contains("'none'")))
        .expect("the primary production that lists 'none'");
    let production = &GRAMMAR[start..start + GRAMMAR[start..].find(" ;").expect("production end")];
    production
        .split_once('=')
        .expect("production body")
        .1
        .split('|')
        .map(|alternative| alternative.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

#[trace("TC-188", "FR-143-AC-18")]
#[test]
fn tc_188_absent_takes_the_option_its_position_expects() {
    assert_eq!(run(&vectors(), "FR-143-AC-18"), 18);
    let statement = &ac_row("FR-143-AC-18")[1];
    for phrase in [
        "`self.v = absent` for `v: Option<Clock>`",
        "a function result typed `Option<Integer>`",
        "an `if` branch",
        "`(lookup<M::A>(p, r) absent empty) = absent` is true for a key `p` does not hold and false for one it holds",
        "not literals",
        "`absent = absent`",
    ] {
        assert!(statement.contains(phrase), "AC-18 states {phrase}");
    }
    assert!(
        !statement.contains("`absent = absent` of two `Option<Integer>` operands is true"),
        "AC-18 no longer admits the literal against the literal"
    );
    assert!(FR143.lines().any(|line| line == "## The `absent` Option literal"));
    let alternatives = primary_alternatives();
    let none_at = alternatives.iter().position(|a| a == "'none'").expect("'none' primary");
    assert_eq!(alternatives[none_at + 1], "'absent'", "primary admits absent beside none");
    assert!(
        FR153.contains("| `empty` | `Option<Reference<T>>` | the present option | `none`, the value the literal `absent` denotes at that type"),
        "the empty-mode row names the literal"
    );
    let r15 = row_cells(TC188, "R15");
    assert!(r15[1].contains("(lookup<M::A>(p, r) absent empty) = absent"));
    assert!(r15[2].contains("the lookup equality is false for the held key and true for the missing key"));
}

#[trace("TC-188", "FR-143-AC-19")]
#[test]
fn tc_188_absent_without_an_option_context_is_ill_typed() {
    assert_eq!(run(&vectors(), "FR-143-AC-19"), 6);
    let statement = &ac_row("FR-143-AC-19")[1];
    assert!(statement.contains("`let x = absent in x`, `present(absent)`, `value(absent)`) is `refused { code: ill_typed, cause: ambiguous-literal }` before any charge"));
    assert!(statement.contains("an `Integer` field or parameter, a `Reference<T>`, a `Set<Integer>`) is `refused { code: ill_typed, cause: type-mismatch }`"));
    let r16 = row_cells(TC188, "R16");
    assert!(r16[2].contains("ambiguous-literal") && r16[2].contains("type-mismatch"));
}

#[trace("TC-188", "FR-143-AC-20")]
#[test]
fn tc_188_an_optional_slot_takes_no_absent_and_the_lookup_mode_keeps_its_meaning() {
    assert_eq!(run(&vectors(), "FR-143-AC-20"), 5);
    let statement = &ac_row("FR-143-AC-20")[1];
    assert!(statement.contains("`P { a: 1, b: absent }` for `b: Integer?` is `refused { code: ill_typed, cause: type-mismatch }`"));
    assert!(statement.contains("`lookup<M::A>(p, r) absent absent` is not a mode and is `refused { code: invalid_syntax, cause: unexpected-token }`"));
    let r17 = row_cells(TC188, "R17");
    assert!(r17[2].contains("`type-mismatch` (a `?` slot is not an `Option<Integer>` position)"));
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
        ("\"key\": \"k2\", \"expected\": true", "\"key\": \"k2\", \"expected\": false"),
        ("\"cond\": true, \"then\": \"absent\"", "\"cond\": false, \"then\": \"absent\""),
        ("\"body\": \"x\", \"argument\": \"absent\", \"expected\": \"none\"", "\"body\": \"x\", \"argument\": \"absent\", \"expected\": {\"some\": 0}"),
    ] {
        assert!(VECTORS.contains(from), "{from}");
        let mutated: Value = serde_json::from_str(&VECTORS.replacen(from, to, 1)).unwrap();
        let caught = ["FR-143-AC-18", "FR-143-AC-19", "FR-143-AC-20"]
            .iter()
            .any(|ac| std::panic::catch_unwind(|| run(&mutated, ac)).is_err());
        assert!(caught, "mutation `{to}` must be detected");
    }
}
