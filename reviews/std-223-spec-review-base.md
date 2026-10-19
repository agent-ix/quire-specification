---
id: SR-13328
title: "Base checklist review of the STD-223 sampler total bound"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/test-cases/TC-354-weighted-sampler-vectors.md, proposals/quire-v1/definitions/simulation-sampler.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: reviews
---

## Summary

Ticket: STD-223. The acceptance bound `n · floor(2^256 / n)` is 0 for `n > 2^256`, so rejection sampling never accepts (re-measured: for `n > 2^256`, `floor(2^256 / n) = 0`). The sampler definition now refuses such a total as `invalid_runtime_input`/`total-above-draw-range` before any draw, FR-181-AC-14 states it, and TC-354 SM-08/SM-09 give the two boundary vectors.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-181-AC-14 | spec/functional/runtime/FR-181-simulate-finite-executions.md | examined |
| TC-354-SM-08 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |
| TC-354-SM-09 | spec/test-cases/TC-354-weighted-sampler-vectors.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Owner decision: refuse (conservative, matches QSL FR-188 / PR #1061) rather than widen the draw; widening would change the recorded sampler identity and every digest sequence. | proposals/quire-v1/definitions/simulation-sampler.md |

## Verdict

Mergeable on this lens. `quire validate` passes on FR-181 and TC-354; the sampler definition has no frontmatter, as on main. AC-14 names TC-354, which carries SM-08 and SM-09.
