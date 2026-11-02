// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-160-AC-6 and FR-160-AC-7: the half-open counter-tick cover and the endpoint
//! rule that decides `ack - req <= bound` and definite ordering. The evaluator is
//! QSL's and RT's; this test applies the stated rule to the published vectors.

use ix_trace_rs::trace;
use serde_json::Value;

const VECTORS: &str = include_str!("../proposals/checked-package-v2/tick-cover-vectors.json");

/// An interval of ticks: `(lower, lower_open, upper, upper_open)`.
type Interval = (i64, bool, i64, bool);

/// The cover of reading `k` under uncertainty `u`: `[k - u, k + 1 + u)`.
fn cover(k: i64, u: i64) -> Interval {
    (k - u, false, k + 1 + u, true)
}

/// `b - a`: an endpoint is open when either operand endpoint it comes from is.
fn difference(b: Interval, a: Interval) -> Interval {
    (b.0 - a.2, b.1 || a.3, b.2 - a.0, b.3 || a.1)
}

fn within_bound(difference: Interval, bound: i64) -> &'static str {
    if difference.2 <= bound {
        "true"
    } else if difference.0 > bound || (difference.0 == bound && difference.1) {
        "false"
    } else {
        "indeterminate"
    }
}

fn definitely_before(a: Interval, b: Interval) -> bool {
    a.2 < b.0 || (a.2 == b.0 && (a.3 || b.1))
}

#[trace("TC-199", "FR-160-AC-6", "FR-160-AC-7")]
#[test]
fn tc_199_counter_tick_cover_is_half_open() {
    let vectors: Value = serde_json::from_str(VECTORS).expect("vectors parse");
    let mut ids = std::collections::BTreeSet::new();
    for case in vectors["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "{id}: ids are unique");
        let u = case["uncertainty"].as_i64().unwrap();
        match case["kind"].as_str().unwrap() {
            "within_bound" => {
                let diff = difference(
                    cover(case["ack_tick"].as_i64().unwrap(), u),
                    cover(case["request_tick"].as_i64().unwrap(), u),
                );
                assert_eq!(within_bound(diff, case["bound"].as_i64().unwrap()), case["expected"], "{id}");
            }
            "definitely_before" => {
                let before = definitely_before(
                    cover(case["a_tick"].as_i64().unwrap(), u),
                    cover(case["b_tick"].as_i64().unwrap(), u),
                );
                assert_eq!(before, case["expected"].as_bool().unwrap(), "{id}");
            }
            other => panic!("{id}: unknown kind {other}"),
        }
    }
    assert_eq!(ids.len(), 12);
}

/// A closed cover would decide the 3001 boundary indeterminate: the published
/// outcome holds only with the half-open cover.
#[trace("TC-199", "FR-160-AC-7")]
#[test]
fn tc_199_a_closed_cover_would_contradict_the_boundary() {
    let closed = |k: i64| (k, false, k + 1, false);
    let diff = difference(closed(3001), closed(0));
    assert_eq!(within_bound(diff, 3000), "indeterminate");
    assert_eq!(within_bound(difference(cover(3001, 0), cover(0, 0)), 3000), "false");
}
