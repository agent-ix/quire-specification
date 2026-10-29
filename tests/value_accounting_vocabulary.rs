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
