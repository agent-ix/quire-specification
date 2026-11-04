// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-091-AC-10 to FR-091-AC-26 and FR-094 (TC-980): the timestamped-event
//! finite-window profile. The evaluator is QSL's; this test applies the stated
//! rule (the established window of FR-091 over the FR-160 possibility sets) to
//! the published vectors, so a vector that the rule does not produce is a defect
//! in the vector or in the rule.

use ix_trace_rs::trace;
use serde_json::Value;
use std::collections::BTreeSet;

const VECTORS: &str = include_str!("../proposals/finite-window/finite-window-vectors.json");
const TICK_COVER: &str = include_str!("../proposals/checked-package-v2/tick-cover-vectors.json");

#[derive(Debug, PartialEq)]
enum Outcome {
    True,
    False {
        at: usize,
    },
    Pending {
        at: usize,
        deadline: i64,
    },
    Indeterminate {
        at: usize,
        reason: &'static str,
        position: Option<usize>,
    },
}

#[derive(Clone, Copy, PartialEq)]
enum Valuation {
    T,
    F,
    Absent,
    Conflict,
}

struct Window {
    op: String,
    a: i64,
    b: i64,
    stamps: Vec<i64>,
    values: Vec<Valuation>,
    w: i64,
    complete: Option<i64>,
    anchors: Vec<usize>,
    history_cutoff: bool,
    /// Counter-cover uncertainty `u`; the establishment margin is `2u`.
    uncertainty: i64,
    scope_closed: bool,
}

/// The valuation of one instant: observations that share a stamp are readings
/// of one instant, so equal valuations are a singleton and differing ones
/// conflict; an observation with no valuation adds nothing.
fn merge(values: &[Valuation]) -> Valuation {
    let has = |v: Valuation| values.contains(&v);
    match (
        has(Valuation::T),
        has(Valuation::F),
        has(Valuation::Conflict),
    ) {
        (true, true, _) | (_, _, true) => Valuation::Conflict,
        (true, false, false) => Valuation::T,
        (false, true, false) => Valuation::F,
        (false, false, false) => Valuation::Absent,
    }
}

fn window(case: &Value) -> Window {
    // Observations are read in canonical order: ascending stamp (the sort is
    // stable, and no case here gives keys to a window).
    let mut positions = case["positions"].as_array().unwrap().clone();
    positions.sort_by_key(|p| p[0].as_i64().unwrap());
    let positions = &positions;
    let valuation = |s: &str| match s {
        "T" => Valuation::T,
        "F" => Valuation::F,
        "absent" => Valuation::Absent,
        "conflict" => Valuation::Conflict,
        other => panic!("unknown valuation {other}"),
    };
    let anchors = match case["activate"].as_str().unwrap() {
        "origin" => vec![0],
        "each" => match case.get("anchor") {
            Some(anchor) => vec![anchor.as_u64().unwrap() as usize],
            None => (0..positions.len()).collect(),
        },
        other => panic!("unknown activation {other}"),
    };
    Window {
        op: case["op"].as_str().unwrap().to_owned(),
        a: case["a"].as_i64().unwrap(),
        b: case["b"].as_i64().unwrap(),
        stamps: positions.iter().map(|p| p[0].as_i64().unwrap()).collect(),
        values: positions
            .iter()
            .map(|p| {
                let stamp = p[0].as_i64().unwrap();
                let readings: Vec<Valuation> = positions
                    .iter()
                    .filter(|q| q[0].as_i64().unwrap() == stamp)
                    .map(|q| valuation(q[1].as_str().unwrap()))
                    .collect();
                merge(&readings)
            })
            .collect(),
        w: case["w"].as_i64().unwrap(),
        complete: case["complete"].as_i64(),
        anchors,
        history_cutoff: case.get("lower").is_some_and(|l| l == "HistoryCutoff"),
        uncertainty: case.get("uncertainty").and_then(Value::as_i64).unwrap_or(0),
        scope_closed: case.get("scope").is_none_or(|s| s == "closed"),
    }
}

/// The reach of an activation: past operators add nothing, future ones their
/// upper bound, and a counter cover adds the margin `2u`.
fn reach(win: &Window, anchor: usize) -> i64 {
    let future = if win.op == "once" { 0 } else { win.b };
    win.stamps[anchor] + future + 2 * win.uncertainty
}

/// Completeness and the watermark both cover the reach.
fn established(win: &Window, anchor: usize) -> bool {
    let reach = reach(win, anchor);
    win.complete.is_some_and(|through| through >= reach) && win.w >= reach
}

/// The result of one activation: `None` is a definite true.
fn activation(win: &Window, anchor: usize) -> Option<Outcome> {
    let t0 = win.stamps[anchor];
    let past = win.op == "once";
    let eligible: Vec<usize> = (0..win.stamps.len())
        .filter(|&j| {
            if past {
                j <= anchor && (win.a..=win.b).contains(&(t0 - win.stamps[j]))
            } else {
                j >= anchor && (win.a..=win.b).contains(&(win.stamps[j] - t0))
            }
        })
        .collect();
    let reach = reach(win, anchor);
    let complete = win.complete.is_some_and(|through| through >= reach);
    let history_gap = past && win.history_cutoff && t0 - win.b < win.stamps[0];
    // An unestablished activation (or one with missing history) admits a
    // continuation that adds a further eligible instant of either value.
    let extra = if past {
        history_gap
    } else {
        !established(win, anchor)
    };
    let may_true = |v: Valuation| v != Valuation::F;
    let may_false = |v: Valuation| v != Valuation::T;
    let values: Vec<Valuation> = eligible.iter().map(|&j| win.values[j]).collect();
    let (true_possible, false_possible) = match win.op.as_str() {
        "eventually" | "once" => (
            extra || values.iter().any(|&v| may_true(v)),
            values.iter().all(|&v| may_false(v)),
        ),
        "always" => (
            values.iter().all(|&v| may_true(v)),
            extra || values.iter().any(|&v| may_false(v)),
        ),
        other => panic!("unknown operator {other}"),
    };
    match (true_possible, false_possible) {
        (true, false) => None,
        (false, true) => Some(Outcome::False { at: anchor }),
        (false, false) => panic!("empty possibility set"),
        (true, true) if win.w < reach => Some(Outcome::Pending {
            at: anchor,
            deadline: reach,
        }),
        (true, true) if history_gap => Some(Outcome::Indeterminate {
            at: anchor,
            reason: "MissingHistory",
            position: None,
        }),
        (true, true) if !complete => Some(Outcome::Indeterminate {
            at: anchor,
            reason: "IncompleteInput",
            position: None,
        }),
        (true, true) => {
            let (&j, v) = eligible
                .iter()
                .zip(&values)
                .find(|(_, v)| matches!(v, Valuation::Absent | Valuation::Conflict))
                .expect("a two-valued closed window has an ambiguous valuation");
            let reason = if *v == Valuation::Absent {
                "MissingValuation"
            } else {
                "ConflictingValuation"
            };
            Some(Outcome::Indeterminate {
                at: anchor,
                reason,
                position: Some(j),
            })
        }
    }
}

/// The first definitely false activation, else the first unresolved one, else true.
fn evaluate(win: &Window) -> Outcome {
    let results: Vec<Option<Outcome>> = win.anchors.iter().map(|&a| activation(win, a)).collect();
    if let Some(at) = results
        .iter()
        .position(|r| matches!(r, Some(Outcome::False { .. })))
    {
        return results.into_iter().nth(at).unwrap().unwrap();
    }
    results
        .into_iter()
        .flatten()
        .next()
        .unwrap_or(Outcome::True)
}

fn basis(win: &Window, outcome: &Outcome) -> &'static str {
    match outcome {
        Outcome::Pending { .. } => "unsettled",
        Outcome::Indeterminate { .. } => "unavailable",
        Outcome::True | Outcome::False { .. } => {
            let at = if let Outcome::False { at } = outcome {
                *at
            } else {
                win.anchors[0]
            };
            if established(win, at) && win.scope_closed {
                "closed-scope"
            } else if matches!(outcome, Outcome::True) {
                "decisive-witness"
            } else {
                "decisive-counterexample"
            }
        }
    }
}

/// The instants whose single valuation, flipped, or which, removed and other
/// than the anchor, change the result (activation at the origin).
fn support(win: &Window, base: &Outcome) -> Vec<usize> {
    let rebuild = |values: Vec<Valuation>, stamps: Vec<i64>| Window {
        op: win.op.clone(),
        a: win.a,
        b: win.b,
        stamps,
        values,
        w: win.w,
        complete: win.complete,
        anchors: vec![0],
        history_cutoff: win.history_cutoff,
        uncertainty: win.uncertainty,
        scope_closed: win.scope_closed,
    };
    (0..win.stamps.len())
        .filter(|&j| {
            let mut flipped = win.values.clone();
            flipped[j] = match flipped[j] {
                Valuation::T => Valuation::F,
                Valuation::F => Valuation::T,
                other => other,
            };
            let flip_changes = evaluate(&rebuild(flipped, win.stamps.clone())) != *base;
            let remove_changes = j != 0 && {
                let mut values = win.values.clone();
                let mut stamps = win.stamps.clone();
                values.remove(j);
                stamps.remove(j);
                evaluate(&rebuild(values, stamps)) != *base
            };
            flip_changes || remove_changes
        })
        .collect()
}

fn expected(case: &Value) -> Outcome {
    let e = &case["expected"];
    let at = || e["where"].as_u64().unwrap() as usize;
    match e["outcome"].as_str().unwrap() {
        "true" => Outcome::True,
        "false" => Outcome::False { at: at() },
        "pending" => Outcome::Pending {
            at: at(),
            deadline: e["deadline"].as_i64().unwrap(),
        },
        "indeterminate" => Outcome::Indeterminate {
            at: at(),
            reason: match e["reason"].as_str().unwrap() {
                "MissingValuation" => "MissingValuation",
                "ConflictingValuation" => "ConflictingValuation",
                "IncompleteInput" => "IncompleteInput",
                "MissingHistory" => "MissingHistory",
                other => panic!("unknown reason {other}"),
            },
            position: e["position"].as_u64().map(|p| p as usize),
        },
        other => panic!("unknown outcome {other}"),
    }
}

fn vectors() -> Value {
    serde_json::from_str(VECTORS).expect("vectors parse")
}

#[trace(
    "TC-980",
    "FR-091-AC-10",
    "FR-091-AC-11",
    "FR-091-AC-12",
    "FR-091-AC-13",
    "FR-091-AC-14",
    "FR-091-AC-15",
    "FR-091-AC-17",
    "FR-091-AC-19",
    "FR-091-AC-20",
    "FR-091-AC-21",
    "FR-091-AC-22",
    "FR-091-AC-23",
    "FR-091-AC-24",
    "FR-091-AC-26"
)]
#[test]
fn tc_980_window_vectors_follow_the_established_window_rule() {
    for case in vectors()["window"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let win = window(case);
        let outcome = evaluate(&win);
        assert_eq!(outcome, expected(case), "{id}");
        if let Some(want) = case["expected"].get("basis") {
            assert_eq!(basis(&win, &outcome), want.as_str().unwrap(), "{id} basis");
        }
        if let Some(want) = case["expected"].get("support") {
            let want: Vec<usize> = want
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p[0].as_u64().unwrap() as usize)
                .collect();
            assert_eq!(support(&win, &outcome), want, "{id} support");
        }
    }
}

type Key = Option<(String, i64)>;

/// Two instants of one stamp have a usable order exactly when both carry an
/// order key, the keys come from one authority and the keys differ.
fn usable(x: &Key, y: &Key) -> bool {
    matches!((x, y), (Some((a, k)), Some((b, l))) if a == b && k != l)
}

/// The order-sensitive operators refuse at the first position that shares a
/// stamp with an earlier one without a usable order.
fn refused_at(positions: &[(i64, Key)]) -> Option<usize> {
    for (j, (stamp, key)) in positions.iter().enumerate() {
        for (earlier_stamp, earlier_key) in &positions[..j] {
            if earlier_stamp == stamp && !usable(key, earlier_key) {
                return Some(j);
            }
        }
    }
    None
}

/// The canonical order: stamp, then authority (a keyless observation after
/// every keyed one), then key. Observation identity, the last step, is not
/// modelled: it fixes indices and never orders a pair for an operator.
fn ordered(positions: &[(i64, Key)]) -> Vec<(i64, Key)> {
    let mut sorted = positions.to_vec();
    sorted.sort_by(|x, y| {
        let rank = |k: &Key| (k.is_none(), k.clone());
        (x.0, rank(&x.1)).cmp(&(y.0, rank(&y.1)))
    });
    sorted
}

fn order_positions(case: &Value) -> Vec<(i64, Key)> {
    case["positions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            (
                p[0].as_i64().unwrap(),
                p[1].as_array()
                    .map(|k| (k[0].as_str().unwrap().to_owned(), k[1].as_i64().unwrap())),
            )
        })
        .collect()
}

#[trace("TC-980", "FR-091-AC-16", "FR-091-AC-23")]
#[test]
fn tc_980_equal_distance_instants_need_an_admitted_order_key() {
    let doc = vectors();
    let cases = doc["order"].as_array().unwrap();
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let sensitive = matches!(
            case["op"].as_str().unwrap(),
            "until" | "release" | "since" | "triggered"
        );
        let positions = order_positions(case);
        // The admitted order is the stamp then key order, not insertion order.
        let refused = if sensitive {
            refused_at(&ordered(&positions))
        } else {
            None
        };
        match case["expected"].as_str().unwrap() {
            "refuse" => assert_eq!(
                refused,
                Some(case["refused_at"].as_u64().unwrap() as usize),
                "{id}"
            ),
            "evaluate" => assert_eq!(refused, None, "{id}"),
            other => panic!("{id}: unknown expectation {other}"),
        }
        // The listing order is not consulted: the reversed listing refuses at
        // the same canonical position and reads in the same canonical order.
        let mut reversed = positions.clone();
        reversed.reverse();
        assert_eq!(ordered(&reversed), ordered(&positions), "{id} listing");
        if sensitive {
            assert_eq!(refused_at(&ordered(&reversed)), refused, "{id} reversed");
        }
        if let Some(canonical) = case.get("canonical_order") {
            let want: Vec<Key> = canonical
                .as_array()
                .unwrap()
                .iter()
                .map(|k| {
                    k.as_array()
                        .map(|k| (k[0].as_str().unwrap().to_owned(), k[1].as_i64().unwrap()))
                })
                .collect();
            let got: Vec<Key> = ordered(&positions).into_iter().map(|p| p.1).collect();
            assert_eq!(got, want, "{id} canonical order");
        }
        if let Some(other) = case.get("same_order_as") {
            let twin = cases.iter().find(|c| &c["id"] == other).expect("twin case");
            assert_eq!(ordered(&positions), ordered(&order_positions(twin)), "{id}");
        }
    }
}

fn cover(k: i64, u: i64) -> (i64, bool, i64, bool) {
    (k - u, false, k + 1 + u, true)
}

#[trace("TC-980", "FR-091-AC-18", "FR-091-AC-25")]
#[test]
fn tc_980_counter_cover_straddling_a_bound_is_indeterminate() {
    let doc = vectors();
    for case in doc["counter"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let (bound, req, ack) = (
            case["bound"].as_i64().unwrap(),
            case["request_tick"].as_i64().unwrap(),
            case["ack_tick"].as_i64().unwrap(),
        );
        let verdict = if case["cover"] == "exact" || case["cover"] == "omitted" {
            if ack - req <= bound {
                "true"
            } else {
                "false"
            }
        } else {
            let u = case["uncertainty"].as_i64().unwrap();
            let (b, a) = (cover(ack, u), cover(req, u));
            let diff = (b.0 - a.2, b.1 || a.3, b.2 - a.0, b.3 || a.1);
            if diff.2 <= bound {
                "true"
            } else if diff.0 > bound || (diff.0 == bound && diff.1) {
                "false"
            } else {
                "indeterminate"
            }
        };
        assert_eq!(verdict, case["expected"].as_str().unwrap(), "{id}");
    }
    // Row 21: the neighbouring-tick and same-tick vectors are the published TCV file.
    let cover_file: Value = serde_json::from_str(TICK_COVER).unwrap();
    let have: BTreeSet<&str> = cover_file["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    for id in doc["tick_cover_vectors"]["ids"].as_array().unwrap() {
        assert!(
            have.contains(id.as_str().unwrap()),
            "{id} is in tick-cover-vectors.json"
        );
    }
}

#[trace("TC-980", "FR-094-AC-11")]
#[test]
fn tc_980_a_watermark_covers_its_own_tick_and_not_the_next() {
    for case in vectors()["covered"].as_array().unwrap() {
        let covered = case["stamp"].as_i64().unwrap() <= case["w"].as_i64().unwrap();
        assert_eq!(
            covered,
            case["expected"].as_bool().unwrap(),
            "{}",
            case["id"]
        );
    }
}

/// A trace with no instants is refused on admission; one listed with decreasing
/// stamps is read in canonical order and not refused.
#[trace("TC-980", "FR-091-AC-22")]
#[test]
fn tc_980_an_empty_trace_is_refused_on_admission() {
    for case in vectors()["admit"].as_array().unwrap() {
        let stamps: Vec<i64> = case["stamps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_i64().unwrap())
            .collect();
        let refused = stamps.is_empty();
        assert_eq!(refused, case["expected"] == "refuse", "{}", case["id"]);
    }
}

#[trace("TC-980", "FR-091-AC-10", "FR-091-AC-18")]
#[test]
fn tc_980_vector_ids_are_unique_and_every_origin_row_is_present() {
    let doc = vectors();
    let mut ids = BTreeSet::new();
    let mut rows = BTreeSet::new();
    for group in ["window", "order", "counter", "covered", "admit"] {
        for case in doc[group].as_array().unwrap() {
            assert!(
                ids.insert(case["id"].as_str().unwrap().to_owned()),
                "duplicate id {}",
                case["id"]
            );
            if let Some(row) = case["row"].as_i64() {
                rows.insert(row);
            }
        }
    }
    rows.insert(doc["tick_cover_vectors"]["row"].as_i64().unwrap());
    let want: BTreeSet<i64> = (1..=22)
        .chain([27, 28])
        .chain(31..=33)
        .chain(35..=37)
        .chain([38, 39, 40])
        .collect();
    assert_eq!(rows, want);
}
