---
id: SR-202611141355004
title: "Integrity review of the STD-664 environment-refused cause"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #653 at e410e74e, diff origin/main...HEAD: spec/objects/foundation/FR-272-typed-diagnostic-cause.md (environment_refused row, FR-272-AC-28, FR-272-AC-29), spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md (environment_refused row), proposals/quire-v1/definitions/native-diagnostics.md (environment_refused row), spec/test-cases/TC-047-emit-typed-located-causes.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-272, type: references }
  - { target: ix://agent-ix/quire-specification/FR-271, type: references }
  - { target: ix://agent-ix/quire-specification/TC-047, type: references }
  - { target: ix://agent-ix/quire-specification/TC-233, type: references }
---

## Summary

Ticket: STD-664. Structural checks over the diff: FR-272-AC-28 and FR-272-AC-29 sit inside the
acceptance-criteria table, directly after FR-272-AC-27, with the three-column row shape. They are
the next unused ids on main. Open PR #652 (STD-663) changes FR-322 and adds review files only, and
open PR #640 changes FR-402 and TC-346; neither touches FR-271, FR-272, TC-047, TC-233 or
native-diagnostics.md, and neither mints an FR-272 criterion, so there is no id or file collision.
TC-047 gains a `verifies` edge to FR-272-AC-29 and TC-233 to FR-272-AC-28, matching each
criterion's verification column. `quire validate` on the four changed spec files reports only
pre-existing module warnings.

## Verdict

PASS. The cross-document consistency defect (FR-271 row vs FR-272-AC-28) is recorded in the base
review, FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
