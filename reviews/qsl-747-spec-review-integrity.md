---
id: SR-9038
title: "Integrity review of QSpec PR #250 (QSL-747 owner-nested member identities)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@06d9dbd4bd1256090a4fe7e80b9cbfce24d66179; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/test-cases/TC-235-business-construct-meaning-binding.md, proposals/quire-v1/definitions/model-complete.md, proposals/checked-package-v2/**"
review_set: subset
---

## Summary

Ticket: QSL-747. The IDs are unique. FR-154-AC-12 and TC-235 M30 appear only on
`origin/spec/owner-nested-identities` across every remote branch, and no open
PR adds either ID. PRs #209 and #165 also edit model-complete.md, but neither
adds an ID.

Cross-reference sweep: I searched spec/, proposals/ (including
checked-package-v2 `domain-package-acme-orders.json` and `fixtures/adverse.json`),
tests and schemas for the global-slot spellings. The search found none outside M30's
deliberate negative inputs. The only other hit is adverse.json
`relationship-identity-wrong-slot`, which uses
`clause/Order-order-Order` as a wrong slot for a relationship and is still
correct. The acme/orders document has no clauses, states, transitions or steps.

FR-154-AC-9 and AC-10 concern relationships only. Relationships stay in their
global slot, so both remain correct. The stale statements are listed below.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-154's declaration-refusal row "Two nodes share one identity, including ... two transitions of one state machine with equal `from`, `to` and `trigger`" was true only under the global `transition/<Name>-<from>-<to>-<trigger>` form. Under the nested form, two such transitions have distinct member names and distinct identities. QSL's own M06 run 6 test pushes `OrderLifecycle/transition3`. QSL refuses by shape (`transition_shapes` in normalize.rs), not by identity. The row should state the shape rule separately from shared identity. | spec/functional/type-model/FR-154-admit-domain-package-model.md:88 | wrong-requirement |
| FND-002 | medium | FR-208's Names bullet still says two transitions of one state machine with equal `from`, `to` and `trigger` "have one IR identity" and are therefore `conflicting-binding`. After this PR that premise is false. The refusal stays, but its stated reason is wrong. The PR does not touch FR-208. | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:99-102 | wrong-requirement |
| FND-003 | medium | model-complete.md's identity prose, below the edited Identity row, still says "A member's identity appends its member name to its owner's identity ... The only members are fields, relationships, operations and clauses". The edited row says relationships take a global slot and that states, transitions and steps nest. The paragraph contradicts the row in both directions. | proposals/quire-v1/definitions/model-complete.md:109-114, :162 | wrong-requirement |

## Verdict

Three medium findings. All three are statements this change left stale. ID
allocation and the global-slot spelling sweep are clean.
