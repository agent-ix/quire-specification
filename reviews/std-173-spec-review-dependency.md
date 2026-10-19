---
id: SR-12823
title: "Dependency review of the FR-331 no-instance wire cause"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification; PR #280 (STD-173) at d9780f2: FR-331, FR-361, FR-363, FR-391, proposals/backend-provider-v1; checked against agent-ix/quire-spec-language main 49bcf9a (FR-125, FR-126, FR-127, FR-165, FR-169, ADR-018)"
review_set: subset
review_date: "2026-10-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-361, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-331, type: reviews }
---
## Summary

Ticket: STD-173. This review checks how the QSpec wire cause depends on QSL's
merged producer side.

What agrees:
- QSL FR-125 Instancing reports `NoInstance` before any state is explored, and
  FR-127 and FR-169 V-6 settle it `inconclusive`/`unsettled`.
- QSL `qsl-replay` spells the sibling `NoInitialState` as `no-initial-state`,
  so `no-instance` follows the same convention.
- FR-165 is the state-graph claim surface that FR-125 instancing covers. That
  matches FR-391 and TC-334 PR-13.
- The FR-363 owner on the vector member is right, because FR-363 holds the
  verdict row.

What does not: the ordering of the zero-instance check against the
`requires-bound` classification. The QSL ADR also lists inconclusive causes
without `NoInstance`.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-361 does not order the zero-instance check against root classification. QSL FR-125 says a population whose member type conforms to `T` and that has no universe returns `RequiresBound` before instantiating. QSpec's new paragraph comes before the `requires-bound` paragraph, and both run "before expansion". Its test, "no object of the universes conforms", is also true when the conforming population has no universe. A QSpec-conformant analyzer could therefore settle `inconclusive`/`no-instance` where QSL returns `requires-bound`: two verdicts for one request. The fix is to say that root classification runs first and that zero instances is decided only over bounded universes. Optionally, also order `no-initial-state` against `no-instance`, which QSL leaves unordered too. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:64-73; spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:147; qsl-lane2 spec/functional/FR-125-read-a-model-subject-s-behaviours-as-temporal-traces.md:110-117 | wrong-requirement |
| FND-002 | low | Out of this PR (QSL side). ADR-018's V-6 row and its "`InconclusiveCause` gains" sentence list `UndecidedSuccessor` and `NoInitialState` but not `NoInstance`, while QSL FR-126, FR-127 and FR-169 carry it. The QS-6 row (line 510) does not list individual causes, so QS-6 itself needs no edit. ADR-018 needs an amendment note on V-6 and on line 164 adding `NoInstance` (FR-125, wire `no-instance`, QSpec FR-331). | qsl-lane2 spec/decisions/ADR-018-temporal-properties-over-every-behaviour.md:151; qsl-lane2 spec/decisions/ADR-018-temporal-properties-over-every-behaviour.md:163-164 | missing-requirement |

## Verdict

No HIGH findings. FND-001 is a one-sentence ordering fix in FR-361 that keeps
it aligned with merged QSL FR-125. FND-002 is a QSL ADR follow-up and does not
block this PR.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (2ab5425). FR-361 now runs `requires-bound` classification first, decides zero instances only over bounded universes, and says a matching population with no universe is `requires-bound`, never `no-instance`. This matches QSL FR-125. FR-361-AC-8 and TC-294 step 7 test both subjects. |
| FND-002 | deferred | QSL-side ADR-018 amendment, tracked as STD-174 (Backlog). |
