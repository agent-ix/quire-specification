---
id: SR-9028
title: "Gap analysis (traceability) of QSpec PR #249 (QSL-751 follow-ups)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@a8689c3d01eb084ae596c883451da255a2dbca65; FR-150-AC-18, FR-153-AC-10, FR-153-AC-11, FR-272-AC-18, FR-272-AC-19, TC-196 C10, TC-198 L12, TC-198 L13, TC-233, tests/checked_package_v2.rs"
review_set: subset
---

## Summary

Ticket: QSL-751. Plan completion: not assessed. FR-150 and FR-153 criteria are
verified by QSL's implementation. Their TC rows exist (TC-196 C10, TC-198
L12/L13), and QSL tests exist for the charge points (checked_dispatch.rs
`model.family-edge`, refusal.rs `admission.ancestor-edge`). FR-272's
catalog-consistency criteria are verified by this repo's own Rust tests in
tests/checked_package_v2.rs (AC-10, AC-11, AC-16/17 each have a `tc_233_*`
test). No build or test was run, because this is a reviewer-only pass.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-272-AC-18 and FR-272-AC-19 are "Test (TC-233)", and the PR extends TC-233's procedure to cover them. No test in tests/checked_package_v2.rs is tagged with either criterion. The existing `tc_233_resource_exhausted_and_cancelled_rows_agree` does not parse the `runtime_invariant` rows or the `stage` rule. Their siblings AC-16/17 have a parser test, so these two have none. | FR-272-AC-18, FR-272-AC-19, tests/checked_package_v2.rs:8018-8105 | correct-requirement-no-evidence |

## Verdict

One medium finding. All other new criteria have a TC row and an owning
verifier.
