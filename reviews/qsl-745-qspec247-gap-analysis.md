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
