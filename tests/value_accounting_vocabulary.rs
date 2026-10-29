// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! The complete charge-point vocabulary of `quire.value.accounting/v1`
//! (FR-461-AC-9): the published spelling list equals the spellings the
//! definition's ordered table declares, so a one-sided change fails.

use ix_trace_rs::trace;
use quire_specification::VALUE_ACCOUNTING_CHARGE_POINT_VECTORS;
use serde_json::Value;
use std::collections::BTreeSet;

const DEFINITION: &str = include_str!("../proposals/quire-v1/definitions/value-accounting.md");

/// Every backtick-quoted charge-point spelling in `text`: lowercase
/// hyphenated words around one dot (`equality.plan-form`).
fn spellings(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (index, part) in text.split('`').enumerate() {
        if index % 2 == 1 && is_spelling(part) {
            found.push(part.to_owned());
        }
    }
    found
}

fn is_spelling(candidate: &str) -> bool {
    let word = |w: &str| {
        !w.is_empty()
            && !w.starts_with('-')
            && !w.ends_with('-')
            && w.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
    };
    candidate
        .split_once('.')
        .is_some_and(|(a, b)| word(a) && word(b))
}

fn first_appearance(cells: impl Iterator<Item = String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for cell in cells {
        for spelling in spellings(&cell) {
            if seen.insert(spelling.clone()) {
                ordered.push(spelling);
            }
        }
    }
    ordered
}

/// The table body under `heading` whose header row starts with `header`:
/// the rows after the delimiter row, up to the first non-table line.
fn table_rows(definition: &str, heading: Option<&str>, header: &str) -> Vec<Vec<String>> {
    let text = match heading {
        Some(h) => definition.split(h).nth(1).expect("heading is present"),
        None => definition,
    };
    let mut lines = text.lines().skip_while(|l| !l.starts_with(header));
    assert!(lines.next().is_some(), "table header {header:?} is present");
    lines
        .skip(1)
        .take_while(|l| l.starts_with('|'))
        .map(|l| {
            l.trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_owned())
                .collect()
        })
        .collect()
}

fn table_vocabulary(definition: &str) -> Vec<String> {
    let rows = table_rows(
        definition,
        Some("## Ordered charge points"),
        "| Operation family",
    );
    assert!(!rows.is_empty());
    first_appearance(rows.into_iter().map(|r| r[1].clone()))
}

fn amount_vocabulary(definition: &str) -> Vec<String> {
    let rows = table_rows(definition, None, "| Charge point |");
    assert!(!rows.is_empty());
    first_appearance(rows.into_iter().map(|r| r[0].clone()))
}

/// The comparison the first test asserts: the list equals the ordered table
/// of `definition` and the amount table names the same points.
fn compare(published: &[String], definition: &str) -> Result<(), String> {
    let table = table_vocabulary(definition);
    if published != table {
        return Err("published list and ordered table differ".to_owned());
    }
    let amounts: BTreeSet<_> = amount_vocabulary(definition).into_iter().collect();
    let declared: BTreeSet<_> = table.into_iter().collect();
    if amounts != declared {
        return Err("amount table and ordered table differ".to_owned());
    }
    Ok(())
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|v| v.as_str().expect("string").to_owned())
        .collect()
}

#[test]
#[trace("TC-426", "FR-461-AC-9")]
fn published_charge_point_list_equals_the_ordered_table() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_CHARGE_POINT_VECTORS).expect("JSON");
    let table = table_vocabulary(DEFINITION);
    let published = strings(&vectors["charge_points"]);
    assert_eq!(compare(&published, DEFINITION), Ok(()));
    let unique: BTreeSet<_> = table.iter().collect();
    assert_eq!(unique.len(), table.len(), "a spelling is declared once");

    // Both tables name the same points: every charged point has an amount
    // row and every amount row names a declared point.
    let declared: BTreeSet<_> = table.iter().cloned().collect();

    // The points the kernel also carries are declared.
    for required in [
        "equality.plan-form",
        "frame.write",
        "frame.redefines-link",
        "declaration.check",
        "temporal.visit",
    ] {
        assert!(declared.contains(required), "{required} is declared");
    }
}

#[test]
#[trace("TC-426", "FR-461-AC-10")]
fn undeclared_spellings_are_not_in_the_vocabulary() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_CHARGE_POINT_VECTORS).expect("JSON");
    let declared: BTreeSet<_> = table_vocabulary(DEFINITION).into_iter().collect();
    let undeclared = strings(&vectors["undeclared"]);
    assert!(!undeclared.is_empty());
    for spelling in undeclared {
        assert!(
            !declared.contains(&spelling),
            "{spelling:?} must stay undeclared"
        );
    }
}

#[test]
#[trace("TC-426", "FR-461-AC-10")]
fn a_one_sided_change_to_either_list_is_detected() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_CHARGE_POINT_VECTORS).expect("JSON");
    let published = strings(&vectors["charge_points"]);
    assert_eq!(compare(&published, DEFINITION), Ok(()));

    // List side: drop a spelling, add an undeclared one.
    assert!(compare(&published[1..], DEFINITION).is_err());
    let mut added = published.clone();
    added.push("temporal.visits".to_owned());
    assert!(compare(&added, DEFINITION).is_err());

    // Table side: remove one declared spelling from the definition text.
    let victim = format!("`{}`", published[3]);
    let edited = DEFINITION.replacen(&victim, "`edited-away`", 1);
    assert_ne!(edited, DEFINITION);
    assert!(compare(&published, &edited).is_err());
}

// ---- `limit_kind` vocabulary (FR-461-AC-11) ----

use quire_specification::VALUE_ACCOUNTING_LIMIT_KIND_VECTORS;

/// The `ScalarLimitsV1` counters: the backtick-quoted field names of the
/// field list in the Limit object, which runs from `limits for` to the
/// sentence end. Fail closed on a missing list, a non-field token or a
/// duplicate.
fn limit_kind_vocabulary(definition: &str) -> Vec<String> {
    let section = definition
        .split("## Limit object")
        .nth(1)
        .expect("Limit object section is present")
        .split("\n## ")
        .next()
        .expect("section body");
    let list = section
        .split_once("limits for")
        .expect("field list follows `limits for`")
        .1
        .split_once(". ")
        .expect("field list ends at its sentence")
        .0;
    let mut found = Vec::new();
    for (index, part) in list.split('`').enumerate() {
        if index % 2 == 1 {
            assert!(
                !part.is_empty()
                    && !part.starts_with('_')
                    && !part.ends_with('_')
                    && part.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'),
                "{part:?} is not a counter field name"
            );
            assert!(
                !found.contains(&part.to_owned()),
                "{part:?} is listed twice"
            );
            found.push(part.to_owned());
        }
    }
    assert!(!found.is_empty(), "the field list names counters");
    found
}

/// Every value the definition gives a `limit_kind`, as `limit_kind: <name>`
/// in a code span or as the first-field wording of another limit object's
/// field list.
fn other_limit_kind_values(definition: &str) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    for chunk in definition.split("limit_kind: ").skip(1) {
        let name: String = chunk
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '_')
            .collect();
        if !name.is_empty() {
            values.insert(name);
        }
    }
    values
}

fn compare_limit_kinds(published: &[String], definition: &str) -> Result<(), String> {
    if published == limit_kind_vocabulary(definition) {
        Ok(())
    } else {
        Err("published list and Limit object differ".to_owned())
    }
}

#[test]
#[trace("TC-426", "FR-461-AC-11")]
fn published_limit_kind_list_equals_the_limit_object() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_LIMIT_KIND_VECTORS).expect("JSON");
    let published = strings(&vectors["limit_kinds"]);
    assert_eq!(compare_limit_kinds(&published, DEFINITION), Ok(()));
    // The count is derived from the definition, never written down.
    let derived = limit_kind_vocabulary(DEFINITION);
    assert!(!derived.is_empty());
    assert_eq!(published.len(), derived.len());
    let unique: BTreeSet<_> = derived.iter().collect();
    assert_eq!(unique.len(), derived.len(), "a counter is named once");
    // Every cumulative or size counter the amount table charges is a kind.
    let kinds: BTreeSet<_> = derived.iter().cloned().collect();
    for required in [
        "work_units",
        "result_units",
        "integer_bits",
        "value_occurrences",
    ] {
        assert!(kinds.contains(required), "{required} is declared");
    }
    assert!(
        vectors.get("count").is_none() && vectors.get("digest").is_none(),
        "no frozen count or digest"
    );
}

/// The spellings of `undeclared` the definition contradicts: a declared
/// `ScalarLimitsV1` counter, a `limit_kind` value any limit object of the
/// definition names, or any backticked word of it.
fn undeclared_violations(definition: &str, undeclared: &[String]) -> Vec<String> {
    let declared: BTreeSet<_> = limit_kind_vocabulary(definition).into_iter().collect();
    let elsewhere = other_limit_kind_values(definition);
    let backticked: BTreeSet<_> = definition
        .split('`')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, w)| w.to_owned())
        .collect();
    undeclared
        .iter()
        .filter(|s| declared.contains(*s) || elsewhere.contains(*s) || backticked.contains(*s))
        .cloned()
        .collect()
}

#[test]
#[trace("TC-426", "FR-461-AC-11")]
fn undeclared_limit_kinds_are_not_in_the_vocabulary() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_LIMIT_KIND_VECTORS).expect("JSON");
    let undeclared = strings(&vectors["undeclared"]);
    assert!(!undeclared.is_empty());
    assert!(
        other_limit_kind_values(DEFINITION).contains("ancestor_steps"),
        "the scan sees other objects"
    );
    assert_eq!(
        undeclared_violations(DEFINITION, &undeclared),
        Vec::<String>::new()
    );
}

#[test]
#[trace("TC-426", "FR-461-AC-11")]
fn a_real_counter_in_undeclared_is_detected() {
    let polluted: Vec<String> = ["unit_work", "work_units", "ancestor_steps", "family_steps"]
        .map(str::to_owned)
        .to_vec();
    assert_eq!(
        undeclared_violations(DEFINITION, &polluted),
        ["work_units", "ancestor_steps", "family_steps"].map(str::to_owned)
    );
}

#[test]
#[trace("TC-426", "FR-461-AC-11")]
fn a_one_sided_limit_kind_change_is_detected() {
    let vectors: Value = serde_json::from_str(VALUE_ACCOUNTING_LIMIT_KIND_VECTORS).expect("JSON");
    let published = strings(&vectors["limit_kinds"]);
    assert!(compare_limit_kinds(&published[1..], DEFINITION).is_err());
    let mut added = published.clone();
    added.push("unit_work".to_owned());
    assert!(compare_limit_kinds(&added, DEFINITION).is_err());
    let mut swapped = published.clone();
    swapped.swap(0, 1);
    assert!(compare_limit_kinds(&swapped, DEFINITION).is_err());
    let victim = format!("`{}`", published[3]);
    let edited = DEFINITION.replacen(&victim, "`edited_away`", 1);
    assert_ne!(edited, DEFINITION);
    assert!(compare_limit_kinds(&published, &edited).is_err());
}
