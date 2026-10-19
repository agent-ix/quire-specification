---
id: SR-13324
title: "Integrity review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/tooling/FR-306-compile-and-cache-exactly.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-373-hybrid-verdict-map.md; ticket STD-208"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-306
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-373
    type: reviews
---

## Summary

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Internal consistency of FR-193, FR-306, FR-331 and TC-373 after the change, and consistency with merged QSL FR-250, FR-292, FR-293 and ADR-029 CA-2/CA-5 (QSL main a2d68af). The step-budget wire spelling matches the backend-provider schema; `timed-out` is already an admitted `IncompleteCause` in proposals/backend-provider-v1/schema.json. The PR claims no measured results (its body states the vectors are derived from text, which is accurate).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-193-row-step | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-row-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-steps | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-identity | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-8 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-9 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-331-incomplete-row | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-hybrid-solver-budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-request-identity | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-29 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-306-outside-key | spec/functional/tooling/FR-306-compile-and-cache-exactly.md | examined |
| TC-373-HY-04 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-07 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-08 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| FR-306-inside-key | spec/functional/tooling/FR-306-compile-and-cache-exactly.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-306's key bullet still puts "the options that affect a result" inside the key, and `max_solver_time` is a hybrid item option that affects a result; the PR moves it outside the key without the reason QSL ADR-029 CA-2 gives (a wall-clock budget can change a result, but every result it limits is time-limited and never stored, so the key never has to bind it). Without that sentence the two bullets read as contradictory. Add the CA-2 reason to FR-306's "Outside the key". | spec/functional/tooling/FR-306-compile-and-cache-exactly.md:94-104 |
| FND-002 | low | FR-331-AC-29 and HY-07 refuse "a time stop spelled `limit-reached`" with `invalid_result`/`unknown-cause`, but `limit-reached` is a closed spelling FR-331 admits and `unknown-cause` is the code for a spelling outside the table; a reader can only recognise a time stop by `limit-reached{limit: max_solver_time}`. State the refused shape (`limit-reached` whose `limit` is `max_solver_time` on a hybrid item) and confirm `unknown-cause` is the intended code for it. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:93,380; spec/test-cases/TC-373-hybrid-verdict-map.md:32 |

## Verdict

Two low findings. Otherwise internally consistent, and consistent with QSL FR-292/FR-293 and ADR-029 CA-2/CA-5 (wall-clock budget outside the key, time-limited result never stored, no wall-time member in a stored record).

## New findings (disposition pass 1)

Against head 5a9fc8d.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | `Hybrid.limits` types both budgets as `Count` (minimum 0) and has no `minProperties`, whereas every sibling limits type (`TemporalLimits`, `StatisticalLimits`, `ExactProbLimits`) uses `PositiveCount` and `minProperties: 1`, and `deadline_ms` is `PositiveCount`. So `max_solver_steps: 0`, `max_solver_time: 0`, and an empty `hybrid: {}` or `limits: {}`, are admitted. FR-193 also says "whole, non-negative". Either use `PositiveCount` and `minProperties: 1` like the siblings, or state what a zero budget settles. | proposals/backend-provider-v1/schema.json:387-401 |

## Dispositions

Round 1, against head 5a9fc8d.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a9fc8d: FR-306 Outside the key gives the CA-2 reason |
| FND-002 | fixed | 5a9fc8d: AC-29 and HY-07 name `limit-reached{limit: max_solver_time}`; `unknown-cause` kept, which the catalog defines as retaining the received spelling |
