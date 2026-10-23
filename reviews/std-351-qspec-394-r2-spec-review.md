---
id: SR-16352
title: "spec-review of FR-377-AC-16 timed refinement liveness refutes only on positive-delay loops, head 09162675 (quire-specification PR 394)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@09162675; spec/functional/analysis/FR-377-decide-model-refinement.md; spec/test-cases/TC-963-timed-refinement-reductions-and-strong-fairness.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/TC-963
    type: references
  - target: ix://agent-ix/quire-specification/ADR-026
    type: references
---

# Spec review of FR-377-AC-16 timed refinement liveness refutes only on positive-delay loops, head 09162675 (quire-specification PR 394)

## Summary

Ticket: STD-351. One spec-review pass over the PR's own delta, checked for the no-drop/no-narrowing/no-deferral rule, hand-derived vectors, id uniqueness and mergeability.

Examined, clean:

- An earlier head (add02446) has SR-16348 on this ticket by another reviewer, clean. This pass covers head 09162675, whose own delta from that head states the violating loop delay (1, by ADR-026 EZ-8: the delay set is every positive rational, unbounded, so the lower end 0 plus one unit). EZ-8 says exactly that for an unbounded delay set, and the figure equals QSL #1320 FR-430-AC-5 after its fix.
- Vector hand-check: with `when not self.sent { x <= 0 }` no time passes while `sent` is false, the `poll` loop has total delay 0 and cannot refute, so `proved`; without it the loop admits positive delay and the refinement is `refuted` with `Divergence` naming `send`, empty stem, one-`poll` loop. FR-377-AC-16 and TC-963 TR-08 are unused ids on origin/main and on every other open QSpec head.
- No deferral or narrowing wording.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

Mergeable: no defect in the delta.

Merge-tree clean against origin/main (head already contains it). Gates: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (equal to origin/main); the PR edits no schema or fixture, so no cargo or QSL conformance run was needed.
