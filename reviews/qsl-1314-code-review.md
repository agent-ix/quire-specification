---
id: SR-2026103005
title: "Code review of quire-specification PR #550 (QSL-1314): restored tc_233 frame_mutations test"
type: SpecReview
analysis: code-review
date: 2026-10-31
scope: "agent-ix/quire-specification@83b8d78db03dc5aac715b49b4b1322d6bb73300d; tests/checked_package_v2.rs (tc_233_frame_mutations_refuse_through_the_full_reader_with_their_recorded_cause)"
review_set: subset
---
# Code review of quire-specification PR #550

## Summary

Ticket: QSL-1314. The PR restores the test that reads every `frame_mutations` vector into the all-families frame node and runs it through `read_package_semantics`. The test asserts the code, the structural locus, and the cause where the vector has one.

Examined:
- tests/checked_package_v2.rs:5353-5454 (examined)
- `FrameRefusal::cause` at tests/checked_package_v2.rs:4280 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `assert_eq!(mutations.len(), 37, ...)` pins how many vectors the file holds. Adding or removing a vector then means editing the test, and the assertion checks no behaviour. The loop already checks every vector, and the cause-set assertion already proves each cause is present. Delete the count assertion. | tests/checked_package_v2.rs:5382-5386 |

## Verdict

The test splices the same members QSL's conformance test splices. The base `deletes_scoped` is `[]`, so leaving it unset when a vector omits it gives the same result as QSL resetting it to `[]`. The test appends `second_frame` and compares the locus as a structural node id. A null cause always comes with the cause-less `invalid_semantic_graph` code, so skipping the cause check there is sound, and `CanonicalOrder.cause()` is never reached. The test passes, 1 passed and 0 failed. It cannot catch SR-2026103004 FND-001, because both orders of a scope conflict pass. That defect is in the data, not the test. One low finding.
