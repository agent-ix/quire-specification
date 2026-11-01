// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Keeps the task-automaton acceptance vectors referenced by the specification
//! matrix. Engine-level replay and settlement are exercised by the TC-372
//! procedure in the task-automaton implementation lane.

use ix_trace_rs::trace;

const TC_371: &str = include_str!("../spec/test-cases/TC-371-zone-certificate-checker.md");
const TC_372: &str = include_str!("../spec/test-cases/TC-372-schedulability.md");

#[trace("TC-372", "FR-419-AC-11")]
#[trace("TC-372", "FR-419-AC-12")]
#[trace("TC-372", "FR-419-AC-13")]
#[trace("TC-372", "FR-419-AC-14")]
#[trace("TC-372", "FR-419-AC-15")]
#[trace("TC-372", "FR-181-AC-31")]
#[trace("TC-372", "FR-181-AC-32")]
#[trace("TC-372", "FR-364-AC-11")]
#[trace("TC-372", "FR-364-AC-12")]
#[trace("TC-372", "FR-415-AC-26")]
#[trace("TC-372", "FR-331-AC-51")]
#[trace("TC-371", "FR-418-AC-11")]
#[test]
fn task_automaton_acceptance_vectors_remain_published() {
    for id in [
        "RT-16", "RT-17", "RT-18", "RT-19", "RT-20", "RT-21", "RT-22", "RT-23",
    ] {
        assert!(TC_372.contains(&format!("| {id} |")), "missing {id}");
    }
    for id in ["ZC-15", "ZC-16", "ZC-17"] {
        assert!(TC_371.contains(&format!("| {id} |")), "missing {id}");
    }
    assert!(TC_372.contains("jittered readiness at 1 retains absolute deadline 4"));
    assert!(TC_372.contains("positive-delay loop"));
    assert!(TC_372.contains("source LO→HI"));
    assert!(TC_372.contains("zero-demand job completes at nominal activation"));
}
