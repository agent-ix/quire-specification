---
id: SR-9005
title: "Gap and traceability review of QSpec PR #247 (resource_exhausted causes)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@5017b5cf6b6d1c731e103b6cb2452145af856d41; FR-272-AC-16, TC-233, spec/complete-v1/tests.md, tests/checked_package_v2.rs"
review_set: subset
---

## Summary

Ticket: QSL-745 item (c). Plan completion: not assessed. `quire matrix
--scope .` at the PR head lists FR-272-AC-16 as `untagged` (no binder),
while its siblings FR-272-AC-10..13, checked against the same two documents,
each have a `#[trace("TC-233", ...)]` test in tests/checked_package_v2.rs.
The criterion id is unique across every origin branch and open PR; the
complete-V1 table row and TC-233 steps and expected result are added.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-272-AC-16 is verified by "Test (TC-233)" but no test carries `#[trace("TC-233", "FR-272-AC-16")]`; the matrix shows it untagged. A sibling of `tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed` over the `resource_exhausted` and `cancelled` rows (plus `requested` in the canonical list) closes it, and would have caught SR-9004 FND-001. | FR-272-AC-16, tests/checked_package_v2.rs:7934-7976 | correct-requirement-no-evidence |

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@9c5d7871bb6f7e22aa7d7489703b2ed30cbd1b78; unchanged by a66831b7893692135acb60a65e0074f2510f5cc4.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-002 | low | `tc_233_resource_exhausted_and_cancelled_rows_agree` compares the union of a row's payload keys, not keys per cause, so moving a key to another cause in either row (for example `requested` under `reservation-size-overflow`) still passes, although FR-272-AC-16 binds each key to its cause. | tests/checked_package_v2.rs:7979-8056, FR-272-AC-16 | correct-requirement-no-evidence |
| FND-003 | low | The same test asserts that both `stage_limit_exceeded` rows name `kind`, `bound`, `actual` and `setting` under the FR-272-AC-16 trace, but FR-272-AC-16 says nothing about `stage_limit_exceeded`, and FR-272-AC-10 names only `setting`. The assertion goes beyond its AC: extend AC-10 and move the assertion to the AC-10 test. | tests/checked_package_v2.rs:8044-8056, FR-272-AC-10 | missing-requirement |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 9c5d787 |
