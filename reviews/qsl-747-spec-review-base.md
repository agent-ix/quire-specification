---
id: SR-9037
title: "Base review of QSpec PR #250 (QSL-747 owner-nested member identities) against QSL behaviour"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@06d9dbd4bd1256090a4fe7e80b9cbfce24d66179; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-235-business-construct-meaning-binding.md, proposals/quire-v1/definitions/model-complete.md; compared with agent-ix/quire-spec-language@199ada0a67f0215b04cc5b90ae3f9c0af38a7be6 (impl/business-model-meanings)"
review_set: subset
---

## Summary

Ticket: QSL-747. The planner ruling (nested form wins; FR-154 is amended) is
the decision under review. The FR-154 identity paragraph matches QSL:
`member_identity_name` (qsl-semantics/src/model/intake.rs:1304) accepts exactly
`<owner identity>/<one segment>` where the segment matches
`^[A-Za-z][A-Za-z0-9_]*$` (`is_artifact_segment`, :1280). `read_clauses`
(:3108-3131) refuses a non-member clause identity at `identity`, then a
`clauseId` that is not the member name. `state_machine.rs` `member_key`
(:39-54) refuses state and transition identities that are not members of the
machine. `read_states` also requires `name` to equal the member name. A
transition's member name is otherwise free, because QSL transitions carry no
name. `process.rs` (:59-66) refuses a step whose identity is not
`<process>/<name>`.

The TC-235 M30 oracle was derived independently:

- Member paths. `NodeCtx::malformed_in` builds the path from
  `MemberPath::from_document_location`, so `$.types[i].transitions[3]` +
  `identity` prints `TypeDefinition.transitions[3].identity`. The same holds
  for `clauses[0]` and `states[0]`. The step refusal is built explicitly as
  `steps[0].identity`. `NotYetPlaced` is `clauses[0]`, `Draft` is `states[0]`
  and `receive_order` is `steps[0]` in K2. The refusal names the member's own
  identity (`node_identity_label`), as M08 run 1 does.
- Charges. States, transitions and steps are not counted declarations: K2's
  `OrderLifecycle` has 5 and `Fulfilment` has 2. M06 run 5 (added state) and
  run 6 (added transition) stay at 21, and operation `d` adds one, giving 22. A
  refused member that is a declaration is still charged (M04 run 4, M27 run 4),
  so run 3 (clause) also stays at 22. Runs 2-5 are each one mutation of the run 1
  input, the convention M05 and M27 use.
- Collision. FCD FR-095 `slug` maps every run outside `[A-Za-z0-9]` to `-`, and
  `<Name>` is verbatim. `(a_b, c, d)` and `(a, b_c, d)` therefore both mint
  `transition/OrderLifecycle-a-b-c-d`, and the rationale holds. In run 1 no
  member name of `OrderLifecycle` repeats.

`quire validate` (quire 0.36.2, engine 0.50.2) passes FR-154 and TC-235.
model-complete.md fails only on its missing frontmatter `type`. origin/main has
no frontmatter in that file either, so the failure is not from this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The amended rule and the M30 oracle (refusal member paths, 22 charges,
collision rationale) all match QSL's code and TC-235's conventions. They were
derived from source, not taken from the PR text. The consistency and coverage
defects are recorded by the integrity, gap-analysis and dependency reviews.

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@e8163084787a91ef8e80c45ac07d5584ba4f26f5, the TC-235 M31 row this round adds. M31 runs 1-3 and 5 charge 21, which follows from K2's baseline: no declaration is added, and a refused declaration is still charged (M05, M06 run 5). Run 1's `clauses[0].clauseId`, run 2's `states[0].name` and run 3's `steps[0].identity` match QSL (intake.rs read_clauses, state_machine.rs read_states, process.rs:59-66), and so does run 5's `conflicting-binding` naming `ix://test/orders/OrderLifecycle/cancel`.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-002 | high | TC-235 M31 run 4 gives state `Draft` the identity `ix://test/orders/Fulfilment/Draft` and expects only `malformed-declaration` at `TypeDefinition.states[0].identity`, "and no other refusal". K2's transitions `[0]` and `[1]` still have `from` `ix://test/orders/OrderLifecycle/Draft`, and after the change no node declares that identity. FR-154's missing-name row requires `missing_declaration`/`missing-name` naming `ix://test/orders/OrderLifecycle` at `TypeDefinition.transitions[0].from` and `[1].from`, as M06 run 2 shows. The suppression rule does not apply, because those references name `OrderLifecycle/Draft`, not the refused node `Fulfilment/Draft`. The oracle contradicts FR-154. To fix it, either list the two missing-name refusals in node order or keep the wrong-owner state off every transition (for example, a new unreferenced state). | spec/test-cases/TC-235-business-construct-meaning-binding.md:110 (M31 run 4), FR-154 declaration-refusal table | wrong-requirement |
| FND-003 | low | M31 run 5 names no member path. QSL reports one: the semantic-IR reader raises `DUPLICATE_IDENTITY` at the later declaration in document order, and operations are listed before states. The pointer is therefore `/types/i/states/3/identity`, which `duplicate_process_step_refusal` (intake.rs ~1648-1705) maps to `conflicting-binding` at `TypeDefinition.states[3]`, as M06 run 5 states. The stated reason, normalize.rs ~2491 `member: None`, does not apply: that check runs over records, and a state is not a record. | spec/test-cases/TC-235-business-construct-meaning-binding.md:110 (M31 run 5) | correct-requirement-no-evidence |
| FND-004 | low | M31 run 1 changes `NotYetPlaced`'s `clauseId` to `Other` while transition `[0]` still has guard `NotYetPlaced`. Its "no other refusal" holds only if a guard resolves by member identity (`<owner>/<guard>`, as M05 run 3 spells the absent clause). In that case the guard names the refused clause node and FR-154 suppresses the missing-name. No spec says whether a guard resolves by `clauseId` or by member identity, and the two can now differ. The pinned FCD reader's `DANGLING_CLAUSE_REF` resolves by `clauseId`, and QSL keeps that refusal on node `OrderLifecycle`, so QSL is likely to report an extra missing-name at `TypeDefinition.transitions[0].guard`. | spec/test-cases/TC-235-business-construct-meaning-binding.md:110 (M31 run 1), FR-154:141 | missing-requirement |

## Dispositions

| FND | Outcome | Sha/Reason |
| --- | --- | --- |
| FND-002 | fixed | 86baf67f368a578aa8534dff10317a0a6aa484ff |
| FND-003 | fixed | 86baf67f368a578aa8534dff10317a0a6aa484ff |
| FND-004 | fixed | 86baf67f368a578aa8534dff10317a0a6aa484ff |
