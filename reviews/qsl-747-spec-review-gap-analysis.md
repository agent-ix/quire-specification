---
id: SR-9040
title: "Gap analysis and traceability of QSpec PR #250 (QSL-747 owner-nested member identities)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@06d9dbd4bd1256090a4fe7e80b9cbfce24d66179; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-235-business-construct-meaning-binding.md; compared with agent-ix/quire-spec-language@199ada0a67f0215b04cc5b90ae3f9c0af38a7be6"
review_set: subset
---

## Summary

Ticket: QSL-747. Traceability: FR-154-AC-12 is verified by TC-235, and M30 cites
FR-154-AC-12. TC-235 already `verifies` FR-154 in its frontmatter. The ticket
asks for a row with a name that collides under the global form but not under
the nested form; M30 run 1 is that row. Its four refused global spellings cover
the four kinds. The rule text states three obligations that no AC or TC row
exercises. They are listed below.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-154 now requires a clause's `clauseId`, a state's `name` and a step's `name` to equal the member name of its identity. It states no refusal or member path for a mismatch, and AC-12 and M30 do not test one. QSL reports a mismatch at three different paths: `clauses[i].clauseId`, `states[i].name`, and `steps[i].identity` (step identity and name are checked together). Two implementers could pick different paths, and TC rows assert member paths. | spec/functional/type-model/FR-154-admit-domain-package-model.md:180-182, FR-154-AC-12, TC-235 M30 | missing-requirement |
| FND-002 | medium | The nested form newly puts clause, state, transition and step member names in the same `<owner>/<name>` namespace as the owner's fields and operations. A state machine with operation `cancel` and state `cancel` now mints one identity twice, and FR-154 row 88 makes that `conflicting-binding`. This behaviour is new in this change, and no AC or TC row covers it. TC-235's fixture sentence says transitions use member names "distinct within the machine", which is narrower than FR-154's "not used by another member of its owner". | spec/functional/type-model/FR-154-admit-domain-package-model.md:182-183, :88; spec/test-cases/TC-235-business-construct-meaning-binding.md:31 | missing-requirement |
| FND-003 | low | FR-154 refuses a nested identity "whose owner is not the declaring type". An example is an `OrderLifecycle` state with identity `ix://test/orders/Fulfilment/Draft`. This case separates owner-exact nesting from any single-segment nesting. AC-12 and M30 test only the four global-slot spellings. | spec/functional/type-model/FR-154-admit-domain-package-model.md:183-185, FR-154-AC-12, TC-235 M30 | missing-requirement |

## Verdict

Two medium and one low coverage gaps. The ticket's AC 3 (a collision row) is
met by M30 run 1.
