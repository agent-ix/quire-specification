---
id: SR-11263
title: "Spec review of quire-specification PR #258: navigate relationship ends by the near-end role"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@76239e3f5c6713da2e583be5183fa337309e5e8d; spec/functional/type-model/FR-152-bind-systems-model-structures.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-197-systems-model-typed-binding.md, spec/test-cases/TC-235-business-construct-meaning-binding.md, proposals/quire-v1/definitions/model-complete.md"
review_set: subset
---
# Spec review of quire-specification PR #258

## Summary

Ticket: QSL-789. One pass of spec-review with the contradiction, ambiguity,
completeness, testability and spec-integrity sub-analyses over
`git diff origin/main...HEAD`. Spec-only PR. Only major issues are reported;
mediums and lows are recorded in the Verdict.

Rule aligned to: QSL FR-363 (merged), FR-322-AC-56/57, TC-281 RN-01 to RN-19.
The source end's role is the forward member, navigable from the source type.
The target end's role is the inverse member, navigable from the target type.
The destination end supplies `[l,u]` and the result type.

Examined:
- FR-152 statement sentence on navigable ends (lines 72-75), "Navigation" (lines 113-141), FR-152-AC-9 (examined)
- FR-322 navigate catalog sentence (lines 1046-1049), FR-322-AC-57 (examined)
- TC-197 fixture Y navigation bullet, population Q, Y07, Y08, Y09 (examined)
- TC-235 M21, all four runs, and the "N charges" definition (examined)
- model-complete.md element-owner paragraph (lines 423-437) and qualify phase table row 2 (examined)
- value-accounting.md model-reference evaluation paragraph (lines 384-397) (examined)
- FR-150 phase (2) qualify; TC-281 RN-01 to RN-19 with `model-member-type-vectors.json`; checked-package-v2 `schema.json` operator enum; `positive-operation-identities.json` and `node-identity-vectors.json` operators (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The rewritten FR-152 sentence makes an end navigable from `T` "exactly when `T` has the relationship member and `r.name` matches the role of the end nearest `T`". model-complete.md says the same: navigable "exactly when `T` has the effective member for the relationship". But qualify (FR-150 phase 2, model-complete phase table row 2) gives a relationship one owner, its owning type. Only connections and allocations are qualified into each end's owner. The target type never has the relationship member, so the inverse member can never be navigable. That contradicts FR-152's own "Navigation" paragraph (the target role is navigable from target-typed receivers), FR-322-AC-56/57 (receiver endpoint conformance, no ownership test), QSL FR-363 "Resolution", and TC-197 Y08 run 3, where `self.pump` from `t1` (a `Tank`, which does not own `Pump/housing`) is expected to check. Fix: state navigability by receiver conformance to the near end's type, and drop the "has the relationship member" condition in both places. | spec/functional/type-model/FR-152-bind-systems-model-structures.md:72-75; proposals/quire-v1/definitions/model-complete.md:429-433 |
| FND-002 | high | Alignment is incomplete in value-accounting.md. `r.name` "takes the end's targets", and the charge schedule branches on "a single-valued end" vs "a multi-valued end". Under the old far-end rule the end `name` named was the destination. Under the near-end rule it is the receiver's own end. Read literally, TC-197 Y07's `self.parts` (near end `Sys` `{1,1}`) is single-valued: one `model.navigate`, no `collection.visit`/`bound`/`result-retain`. Y07 expects the multi-valued schedule of the `Pump` `{0,3}` destination (five work units, three result units; the `4` limit stopping at `collection.result-retain`; six and four work units in the refusal runs). So Y07's charge counts conflict with the accounting text it cites (FR-152-AC-7). Fix: say "the destination end's targets" and "a single-valued/multi-valued destination end". | proposals/quire-v1/definitions/value-accounting.md:390-397; spec/test-cases/TC-197-systems-model-typed-binding.md:84 |
| FND-003 | high | FR-152-AC-9 cannot be tested as cited. Its clause "the far end's role from the receiver finds no candidate" has no outcome in FR-152: no code or cause is stated for a name with no candidate. FR-322-AC-56 uses `missing_declaration`/`missing-name` at package level, and QSL FR-363 uses `ill_typed`/`type-mismatch` at checking. AC-9 cites TC-197, but no TC-197 vector navigates a far-end role (for example `self.owner` from `s1` or `self.parts` from `p1`). A test cannot fail on this clause. Fix: state the no-candidate refusal in "Navigation", and add a Y vector for it. | spec/functional/type-model/FR-152-bind-systems-model-structures.md:156 |

## Verdict

**FAIL: 3 blocking findings (FND-001, FND-002, FND-003).**

Verified clean, step by step against the swapped fixture:
- Y07: `self.parts` from `s1` is the forward source role, `source-to-target` holds, and the destination `Pump` `{0,3}` gives `Set<Reference<M::Pump>>[0,3]`. The 4-target run exceeds `[0,3]`, and `p4` dangling is decided before the bound. All five sub-runs keep their expected results, given FND-002's accounting fix.
- Y08: `self.home` from `p1` is forward, with destination `Tank` `{1,1}`, giving `Reference<M::Tank>` = `t1`. `self.pump` from `t1` is the inverse target role, which is refused `operator-ineligible` under `source-to-target`. Under `undirected` it checks, and with destination `Pump` `{1,1}` the count of 2 (`p1`, `p2`) is `above-maximum`. Field `Pump/home` plus forward `home` from `Pump` is `ambiguous-name`. All hold, given FND-001.
- Y09: the `{0,unbounded}` and ordered target end is the `Pump` destination of `self.parts`, so `expression-form` at `self.parts` holds. `t9` dangling holds. A foreign `Tank` on the target end is refused at intake. All hold.
- M21: "N charges" counts `normalize.record` only. Run 2 = 21 (K2) + 1 relationship + 1 clause `HasOrder` + 2 populations = 25. Multiplicity `{0,1}` adds no declaration, and `model.navigate` is not counted. `self.order` is forward from `Fulfilment`, and the destination `Order` `{0,1}` gives `Option<Reference<Order>>`, so `present` checks. Runs 1, 3 and 4 are unaffected.
- FR-322: operator class `query` is in the v2 `SemanticTerm` operator enum. The `quire.op.model.lookup` and `all_instances` fixtures use it too, and so does QSL FR-093's `Navigate` row. It is consistent.
- TC-281 RN-01 to RN-19 already follow the near-end rule (RN-05 patches `targetEnd` for `order`; RN-03 and RN-10 patch `sourceEnd` for `next` and `followedBy`). No other QSpec text restates the far-end rule. value-accounting.md (FND-002) is the one remaining implicit case.
- No tracking, hash or version ceremony was added. The FR-322 `semantic.record-invalid` failure is pre-existing and was not worsened.

Recorded (medium/low, not blocking):
- Medium: no vector records navigate's operator class (FR-322-AC-57's new sentence). The RN cases carry no `operator`.
- Low: M21 run 2 states no direction for `Fulfilment/order`. This is pre-existing, and forward navigation needs one of three directions.
- Low: FR-152 says "the target end's role, when declared", but model-complete row "Relationships" says "Each end has a role". This tension is pre-existing.

## Dispositions

Round 1, reviewed at 363a893a52e1f4a811087e321ac3805cb4deeff5.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 363a893 | FR-152 lines 72-75 and model-complete.md lines 429-433 now base navigability on `T` conforming (FR-082) to the type of the end whose role `name` is, and state that ownership plays no part. Y08 run 3 (`self.pump` from `t1`): `pump` is the target role, `Tank` conforms to the target end's type, and `undirected` admits the inverse, so it checks. FR-150 phase 2 and phase-table row 2 still qualify a relationship under its owner only, which no longer conflicts. FR-322-AC-56/57 (endpoint conformance) and TC-281 RN-01 to RN-19 agree. |
| FND-002 | fixed 363a893 | value-accounting.md lines 390-397 now take the destination end's targets (the end opposite the named role), and the single-/multi-valued schedule and count bound follow it. Y07's `self.parts` destination is `Pump` `{0,3}`: 1 navigate + 2 visit + bound + retain = 5 work units, 3 result units; the limit-4 run stops at `collection.result-retain`; 4 targets give navigate + 4 visit + bound = 6; `p4` dangling gives navigate + 3 visit = 4. All are consistent. |
| FND-003 | fixed 363a893 | FR-152 Navigation now states the no-candidate refusal by layer: `ill_typed`/`type-mismatch` at the form at checking (agrees with QSL FR-363), and `missing_declaration`/`missing-name` at `member.name` for an IR `relationship_end` naming a role the relationship does not have (FR-322-AC-56). FR-152-AC-9 names `type-mismatch`. New TC-197 Y11, traced against the swapped fixture: `self.owner` from `s1` (`owner` is the target role on the `Pump` end, and `Sys` does not conform to `Pump`), `self.parts` from `p1` (`parts` is the source role on the `Sys` end), and `self.pump` from `p1` (`pump` is the target role on the `Tank` end of `Pump/housing`; `Pump` conforms only to the source end). No `Sys` or `Pump` field has these names, so each has no candidate and is refused `type-mismatch`. The vector is reachable and can fail. |

Recorded medium (no vector for navigate's operator class): resolved at 363a893 by TC-197 Y12. It asserts `query`, member `relationship_end` `Sys/composition`/`parts`, operand `Reference<M::Sys>` and `result_type` `Set<Reference<M::Pump>>[0,3]` on the lowered Y07 navigation. It is reachable through TC-197's compiled source S1 and fails on any other operator class.

New mediums/lows from round 1 (recorded, not blocking):
- Medium: value-accounting.md line 43, the charge-table summary row, still says "a multi-valued end" without "destination". The governing paragraph at lines 390-397 is now explicit.
- Low: Y12 asserts an FR-322-AC-57 fact, but TC-197's relationships verify only FR-152 and FR-208.

M21 is untouched by the fix round (no TC-235 change), so its 25 charges stand.

**Round 1 verdict: PASS. All three findings are fixed and there are no new blocking findings.**
