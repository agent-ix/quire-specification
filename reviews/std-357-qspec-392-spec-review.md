---
id: SR-16346
title: "spec-review of replay of a changed carried value refuses content-mismatch (FR-433, TC-387, FR-181, FR-413) (quire-specification PR 392)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0c0f306e; spec/functional/protocol/FR-433-replay-protocol-counterexamples.md; spec/test-cases/TC-387-protocol-counterexample-replay.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-433
    type: references
  - target: ix://agent-ix/quire-specification/TC-387
    type: references
---

# Spec review of replay of a changed carried value refuses content-mismatch (FR-433, TC-387, FR-181, FR-413) (quire-specification PR 392)

## Summary

Ticket: STD-357. One spec-review pass over `git diff origin/main...0c0f306e`: standing rule (no drop, narrowing or deferral), testable ACs with hand-derived vectors, id uniqueness across all open PR heads, QSpec name cross-check, mergeability, and the repo gate.

Examined, clean:

- No `revision-mismatch` remains in spec/ at the head (git grep); main has 7 places. FR-272's `content-mismatch` row now names a replay's recorded and recomputed value.
- TC-387 is prose, no vectors JSON carries the cause: QSL conformance (`cargo test -p qsl-package --lib checked_v2`, QSpec main + this PR) 49 passed.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable: no defect. Merge-tree against main is clean (15 commits behind).
