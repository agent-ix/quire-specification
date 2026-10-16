---
id: SR-11755
title: "QSL-813 PR 266 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@bfe973f525f6398aaa5bf4d0813d54393dd526b7; FR-152-AC-11, FR-152-AC-12, FR-322-AC-57, TC-197 Y14-Y18, TC-281 RN rows, tests/model_member_types.rs"
review_set: subset
---
# QSL-813 PR 266 gap analysis

## Summary

Ticket: QSL-813. Planless gap analysis, scoped to the PR diff. Plan
completion: not assessed.

Examined:
- FR-322-AC-57 (examined): covered by RN-01 to RN-27, run by the TC-281 test.
- FR-152-AC-11 (examined): covered by TC-197 Y14, Y15 and Y18.
- FR-152-AC-12 (examined): covered by TC-197 Y15, Y16 and Y17, except the below-bound half.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-152-AC-12 says a neighbour count outside the destination bound refuses nothing, but no TC-197 row has a count below the lower bound: Y14 and Y15 use `{0,3}` (lower 0), so only the above-bound case (Y15) is covered | spec/test-cases/TC-197-systems-model-typed-binding.md:96 |

## Verdict

PASS with one medium gap. FR-152-AC-11 and AC-12 are TC-197 rows, as FR-152
AC-1 to AC-10 already are; no test in this repo is tagged to TC-197, which is
the existing pattern for FR-152 and not new here. FR-322-AC-57 is run by the
TC-281 test over all 27 cases.
