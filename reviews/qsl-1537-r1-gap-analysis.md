---
id: SR-20261103145003
title: "QSL-1537 r1 gap analysis of PR #608 (FR-290-AC-17, FR-272-AC-27, TC-271 NG-21..NG-24)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR #608 head c3904004 vs origin/main b1feb484; FR-290-AC-17, FR-272-AC-27, TC-271 NG-21..NG-24, CK-05; cross-checked against QSL FR-507 and FR-057 (quire-spec-language origin/main)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
---

## Summary

Ticket: QSL-1537. This is a spec-only PR, so the analysis covers how the new ACs trace to the TC-271 vectors, whether those vectors are enough, and whether the new bound is consistent with the producer table it delegates to, QSL FR-507. Plan completion: not assessed.

Traces: FR-290-AC-17 goes to TC-271 NG-21, NG-22 and NG-23, and TC-271 `verifies` FR-290. FR-272-AC-27 goes to TC-271 NG-21 and NG-24, and TC-271 `verifies` FR-272. Both ACs have tagged vectors.

## Verdict

**GAPS.** NG-21, NG-22 and NG-24 cover their clauses: precedence over extent and candidates, absent and unknown kind first, item independence and family-sensitive equality. The "assigned kinds apply" clause of FR-290-AC-17 is tested only on the CK-05 claim forms. Those are exactly the forms on which no producer conflict exists (FND-001).

The delegation to QSL FR-507 conflicts with the bound this PR states (FND-002). The QSpec bound is the correct side. QSL FR-507 must change, and NG-21 relies on FR-507 as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NG-23 checks "for each CK-05 claim form" that the kind CK-05 derives for it is never refused `inapplicable-kind`. CK-05 omits these claim-form rows: the scalar-operation rows; state-graph, `hyper` and model `relation` claims; the `relation` code claim; whole model refinement; per-step simulation; transferred claims; probabilistic, timed and hybrid claims; and `schedulability`. Those are the rows where a claim's declaration family differs from the kind's own Families group, and so the rows where a producer table can refuse an assigned kind (see FND-002). NG-23 also names no declaration family per form. FR-290-AC-17's "an item whose kind is its claim form's assigned kind is never refused this way" is therefore untested where it matters. Fix: run NG-23 over every row of the claim-form assignment table. Name the declaration that holds each claim, and include one declaration that holds two kinds (a `relation` clause with an implementation binding). | spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:39, :69; spec/objects/protocol/FR-290-protocol-claim-kind.md:122-146, :469 |
| FND-002 | high | The producer table this PR delegates to (QSL FR-507, "Owner of the pairs") refuses kinds the bound says always apply. FR-507 gives each kind exactly one declaration family. `operation-contract` gets only `StateModel` and `temporal-satisfaction` gets only `TemporalTrace`, and FR-507-AC-1 enumerates six families with no `Relation` row. (a) A `relation` clause with a bound implementation records both a `temporal-satisfaction` model claim and an `operation-contract` code claim on one node (QSL FR-180-AC-1; FR-290 rows "Model claim"/"Code claim of a `relation` clause"). Whatever family holds that node, FR-507 refuses one of the two assigned kinds. (b) The model-refinement declaration is a QSL `Relation` declaration (ADR-012 §1 as amended by ADR-020 §1). It records the whole refinement as `temporal-satisfaction` and each step as `operation-contract` (ADR-020 "Claim kinds", RE-2, QS-5; FR-290 rows for FR-377 and per-step simulation). FR-507 refuses both. (c) FR-290 assigns `finite-replay` to "Replay of one finite execution against state or protocol semantics", but FR-507 and QSL FR-057:206 ("only a choreography is replayed") admit it on `protocol` only. A state-semantics replay claim held by a state-model declaration would be refused. This is latent, because QSL has no such claim form today. FR-507-AC-2 and NG-21 do not conflict, because a `finite-replay` item on an operation-precondition claim is not that claim's assigned kind. The required change is on the QSL side: FR-507's table and FR-507-AC-1 must admit each assigned kind on the family that holds its claim. That includes `Relation` with `temporal-satisfaction` and `operation-contract`, and the declaration holding `relation` clauses with both. For `finite-replay` on state-model, FR-507 should admit it, or the owner rules that FR-290's row narrows to protocol semantics. Not verified: which QSL family holds state-graph (FR-165), `hyper` (FR-173), time-lock-freedom (FR-417) and FR-407 probabilistic claims. If any of them is not `TemporalTrace`, or not the FR-186 probabilistic claim form, it conflicts too. | spec/objects/protocol/FR-290-protocol-claim-kind.md:131-140, :193-202; spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:67 |
| FND-003 | low | The first half of FR-272-AC-27 is a catalog-row inspection: "The native-diagnostics row and this requirement's row … each list `inapplicable-kind` and say that it retains …". It traces to TC-271, which has no vector that reads either catalog row. The sibling row checks FR-272-AC-24, AC-25 and AC-26 trace to TC-233. Fix: trace that half to TC-233, or add a TC-271 row that compares the two catalog rows. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:205 |

## Dispositions

- FND-001: fixed. TC-271 NG-23 now covers every row of the claim-form assignment table that requires a kind, each with its declaration-family label. It includes one `relation` clause with an implementation binding that holds both a `temporal-satisfaction` and an `operation-contract` claim.
- FND-002: QSpec side fixed; QSL side routed by the coordinator. The QSpec table is now the authority (FR-290 "Declaration-family applicability", FR-290-AC-18). QSL FR-507's table must admit `temporal-satisfaction` and `operation-contract` on the family holding `relation` clauses and model refinements, and `finite-replay` on `state-model`. Because `finite-replay` now applies to `state-model`, QSL FR-507-AC-2's example also conflicts: it expects `finite-replay` on a state claim to be refused. TC-271 NG-21 now uses `global-conformance` on a state-model claim. The QSL repo is not edited here.
- FND-003: fixed. TC-233 now verifies FR-272-AC-27. Its procedure and expected results read the two `invalid_capability` catalog rows for both causes, the payload keys and the canonical names. FR-272-AC-27 traces to TC-233 and TC-271.
