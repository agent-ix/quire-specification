---
id: SR-496002
title: 'Code review of quire-specification PR #462 (STD-496, STD-497, STD-510): tests/checked_package_v2.rs'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@961ddf00; tests/checked_package_v2.rs diff against main 02dfc7dd (trace attributes, case counts, tc_233 mutation expected_code assertion)'
review_set: subset
date: '2026-10-27'
---

## Summary

Ticket: STD-497 (STD-496 and STD-510 trace additions). Rust lane folded in: 16 lines of tests/checked_package_v2.rs. Examined: the `#[trace]` additions (FR-464-AC-20, FR-466-AC-16 on the protocol case-vector test; FR-322-AC-139 on the tc_233 mutation test), the case-count assertion [123, 85, 166] (matches protocol-node-vectors.json: 123, 85, 166), and the new `wire_refused` branch of the expected_code assertion. `cargo test --test checked_package_v2` gives 156 passed and 2 failed on both main and the branch; the failures are K15 (no vector) and the execution-point-initialization case, unchanged.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| protocol case vector test trace and counts | tests/checked_package_v2.rs:2604 | examined |
| tc_233 mutation expected_code assertion | tests/checked_package_v2.rs:6862 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The expected_code assertion derives `malformed_wire` from the mutation name prefix `dimension-equivalence-` combined with `refused_by` `schema`, instead of from `refused_by` alone. It hard-codes the inconsistency of SR-496001 FND-002 (other schema-refused mutations still expect `invalid_semantic_graph`) and will silently apply the wrong code to any later schema-refused mutation outside that prefix. FR-466-AC-16 and FR-464-AC-20 are tagged only to the vector-publication test, which checks row text, not reader behaviour; that is acceptable for a spec PR but means the matrix `tagged` status is not reader evidence. | tests/checked_package_v2.rs:6867 |

## Dispositions

Round 1, reviewed at 7ea036da.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The assertion is now `let wire_refused = mutation["refused_by"].as_str() == Some("schema");` with no name prefix; the protocol case-vector test lost the FR-464-AC-20 and FR-466-AC-16 traces (Analysis rows). |

## Verdict

One low finding. The Rust change is minimal and correct for the vectors as written.
