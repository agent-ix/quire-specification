---
id: SR-16348
title: "spec-review of FR-377-AC-16 timed refinement liveness refutes only on positive-delay loops (quire-specification PR 394)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@add02446; spec/functional/analysis/FR-377-decide-model-refinement.md; spec/test-cases/TC-963-timed-refinement-reductions-and-strong-fairness.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/TC-963
    type: references
---

# Spec review of FR-377-AC-16 timed refinement liveness refutes only on positive-delay loops (quire-specification PR 394)

## Summary

Ticket: STD-351. One spec-review pass over `git diff origin/main...add02446`: standing rule (no drop, narrowing or deferral), testable ACs with hand-derived vectors, id uniqueness across all open PR heads, QSpec name cross-check, mergeability, and the repo gate.

Examined, clean:

- FR-377-AC-16 derived by hand: with `x <= 0` while not sent the `poll` loop cannot let time pass, so total delay is 0 and the weak-fairness lasso over `send` is not time-divergent (proved); without the invariant the same loop can take positive delay, so `Divergence` naming `send` with an empty stem and a one-`poll` loop (refuted, replays).
- The omitted `send` guard `x >= 1` from the ticket sketch is correct to omit: with the guard and the invariant `send` would never be enabled and the vector would not test the delay rule.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable: no defect. Merge-tree against main is clean. Companion QSL #1320 (FR-430-AC-5) is outside this batch.
