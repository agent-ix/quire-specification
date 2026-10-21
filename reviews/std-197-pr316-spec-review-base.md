---
id: SR-13655
title: 'Spec review of QSpec PR #316: replicated role binder and scope region (STD-197 items 1, 4)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@3a4ac58f3; PR #316 diff origin/main...HEAD: shared-grammar.md, FR-428, FR-465, TC-382, TC-947; ticket STD-197'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-428
  type: reviews
- target: ix://agent-ix/quire-specification/FR-465
  type: reviews
- target: ix://agent-ix/quire-specification/TC-382
  type: reviews
- target: ix://agent-ix/quire-specification/TC-947
  type: reviews
---

## Summary

Ticket: STD-197. PR quire-specification#316, head 3a4ac58f31359eb27f2aa0ef477813f50066f11f.

Base lens with EARS and integrity folded in. Gates: cargo tests pass (12 + 113); quire validate 134 failures as on main, touched files clean; merges cleanly into main and alongside #314. The brace refusal is sound: no `primary` alternative starts with `{` (shared-grammar.md:353-365), so `until({w.done})` refuses at the `{`. The scope region agrees with QSL FR-218 Scopes ('the smallest control holding every node `by` that role') and ADR-027 RR-4 (qsl-lane2 origin/main 5c4a96692); the root fallback is consistent with `run` being the root control (ADR-027 PS-2). Items 2 and 3 are left to QSL as the PR states. FR-428-AC-8/9 and RR-07/08 collide with nothing.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| shared-grammar binder paragraph | proposals/quire-v1/shared-grammar.md | examined |
| FR-428 instance binder | spec/functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md | examined |
| FR-428 scope region | spec/functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md | examined |
| FR-428-AC-8 | spec/functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md | examined |
| FR-428-AC-9 | spec/functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md | examined |
| FR-465 binder row | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md | examined |
| TC-382 RR-07 | spec/test-cases/TC-382-replicated-role-lifetimes.md | examined |
| TC-382 RR-08 | spec/test-cases/TC-382-replicated-role-lifetimes.md | examined |
| TC-947 base role | spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md | examined |
| TC-947 RL8 | spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md | examined |
| TC-382 model | spec/test-cases/TC-382-replicated-role-lifetimes.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-382's model puts `done: Int[0, 2]` on `Job`, not on `Worker`; RR-08 says 'over the Worker model with a done field' without giving its type and writes `until(w.done = 1)`, while FR-428-AC-8 writes `until(w.done)` (Boolean). Fix: declare the `Worker.done` field and its type in RR-08 and use the same predicate form in AC-8 and RR-08. | spec/test-cases/TC-382-replicated-role-lifetimes.md:40 |
| FND-002 | low | The rule says the role name `r` names one instance 'inside `until(p)`'. It does not say whether `r` used as a value elsewhere (a choice guard, a node constraint of a `by r` node) refuses or denotes the acting instance, now that the binder and the role share one name. Fix: state that the binder is readable only in `until(p)` (and how another read refuses), or name the other places it is readable. | spec/functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md:74-75 |
| FND-003 | low | The paragraph follows the `verification-step` production (line 623), away from the `role` / `role-lifetime` productions (lines 495-499) it explains. Fix: move it next to the `role` production. | proposals/quire-v1/shared-grammar.md:626-630 |

## Verdict

Mergeable; the three findings are low and can be fixed in this PR. The binder rule and the scope region match QSL FR-218 and ADR-027.
