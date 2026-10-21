---
id: SR-13634
title: "Code review (rust-review lane) of STD-155 reference reader (PR #307)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; tests/model_member_types.rs (PR #307 head d586f0d)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
---

# Code review (rust-review lane) of STD-155 reference reader (PR #307)

## Summary

Ticket: STD-155. Reviewed `read_parameter_names` and `tc_280_parameter_names_refuse_each_defect_at_the_selection_row` (rust-review lane folded in). The trace tag `#[trace("TC-280", "FR-154-AC-14")]` binds the test to the AC it asserts. The four requested test targets pass (151 tests). No panics outside the test harness, no unsafe, no integer-conversion surface. gap-analysis not run: the diff has no production code.

Units examined:

- `tests/model_member_types.rs` (examined): tests/model_member_types.rs

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The `conflicting-binding` branch reuses `refused`, which always returns `operation_identity`. FR-154-AC-13 names the shared parameter identity (`text(param, "identity")`). The test therefore agrees with the wrong `parameter-name-repeated` expectation and cannot catch it. The duplicate check also keys on `name`, not identity. That is equivalent only because every name was already checked against its identity, so it hides the AC-13 precedence question (SR-13633 FND-001). | tests/model_member_types.rs:1930-1951 |

## Verdict

Not mergeable while the reader encodes the wrong identity for the repeated-name branch (FND-001, same root as SR-13632 FND-001).
