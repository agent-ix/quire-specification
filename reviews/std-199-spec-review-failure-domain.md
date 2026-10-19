---
id: SR-13069
title: "Failure-domain review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@4d0b395; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-310-refinement-declaration-admission.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md; QSL FR-147/FR-135 at b41d0a9 (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
---

## Summary

Ticket: STD-199. Looks for unstated precedence, locus and partial-failure cases around the support-set refusal, the list's JSON-pointer loci and per-library lists, and compares the precedence with QSL FR-147/FR-135.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-unbuilt | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-14 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-432-defects | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-375-AC-12 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| TC-310-m14 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| FR-432-AC-9 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Support-set precedence is undefined for a malformed or unresolvable list, and diverges from QSL. FR-375 says the refusal "is decided before the feature's own checks"; QSL FR-147 (merged #1016) resolves the value first (FR-147-AC-6 defects run now) and refuses only a non-empty *resolved* list (FR-147-AC-7). For `[{"refinement": "CasTwice", "internal": ["Spec::Twice::zz"]}]` in a producer without `refinement-internal-list`, QSpec gives `not-yet-implemented` at `Spec` and QSL gives `missing-name` at `/0/internal/0`. QSpec also does not say what a "non-empty internal list for the refinement" is when the value is `{}`, an object lacks `internal`, or `refinement` names no declaration: no refinement exists to locate the refusal at. State whether shape/resolution defects are reported before the support-set check, and the outcome for values attributable to no refinement. | spec/functional/analysis/FR-375-declare-model-refinement.md:158-167; QSL spec/functional/FR-147-check-a-refinement-whose-abstract-side-is-a-protocol.md (The profile's internal list; AC-6, AC-7) |
| FND-002 | medium | The pointer prefix is delegated to FR-375 ("the host prefixes its own location (FR-375)"), but FR-375 states no prefix and no compile-request member path for the unit's value or for a library's value. Only replay has one (FR-323-AC-17, `/package/refinement_internals`). Two producers will report the same compile-time defect at different loci, and the unit's and a library's `/0/refinement` are indistinguishable. State the compile-time prefix (unit and per-library) in FR-375. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:159-162; spec/functional/analysis/FR-375-declare-model-refinement.md:144-156 |
| FND-003 | medium | AC-12's library clause ("a supplied library's list affects only that library's refinements") has only a negative vector: TC-310 mutation 14 supplies a list naming `CasTwice` to a library that does not declare it and expects `missing-name` "in that library" that "does not change the unit". Under FR-307 a supplied library that refuses fails every import of it, and an unimported library may never be compiled, so whether the unit is admitted is unstated. Add a positive vector (a library declaring its own refinement, its list changing only its node and the importer's lock selection) and state whether the unit imports the refusing library. | spec/functional/analysis/FR-375-declare-model-refinement.md:300; spec/test-cases/TC-310-refinement-declaration-admission.md:81-85,134-138 |
| FND-004 | low | With both `protocol-abstract-side` and `refinement-internal-list` unbuilt, a non-empty list under `abstract Spec::Twice` gives two refusals in QSL (FR-147-AC-7: at `Twice` and at `Spec`). QSpec's text implies the same, but AC-14 and TC-310 mutation 16 omit one feature at a time, so the combined count is not pinned. | spec/functional/analysis/FR-375-declare-model-refinement.md:303; spec/test-cases/TC-310-refinement-declaration-admission.md:89-91 |

## Verdict

Not mergeable on this lens until FND-001 and FND-002 are decided, as they leave the refusal order and locus implementation-defined and diverge from the merged QSL text. Clean: the model-subject list rule (support-set refusal when unbuilt, `invalid_model_binding`/`malformed-declaration` at the object pointer when built) is consistent with QSL's interim rule; the three refusal spans (the `history` keyword, the protocol name, the first token after `abstract`) match QSL FR-135/FR-147; absent-is-empty and the list-vs-row conflict match.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The fix round leaves the model-subject list rule contradictory. FR-432 now says that without `refinement-internal-list` "the defects above are still reported", and "above" includes the table's last row (a non-empty list on a model subject refuses `invalid_model_binding`/`malformed-declaration`). FR-375's list of defects still reported (shape, spelling, `missing-name`, `conflicting-binding`) leaves that row out, and FR-375-AC-14 and TC-310 mutation 17 expect only the `refinement-internal-list` refusal at `Spec` for the list under `abstract Spec`. QSL FR-147-AC-7 also expects exactly one `not-yet-implemented` and no `invalid_model_binding`. FR-432 also says FR-375 "states which of these refusals the support set then replaces" and then lists none. State in both FRs that the model-subject row is replaced by the support-set refusal (or that both are reported) and pin it in AC-14. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:172-177 |

## Dispositions

Round 1, against head 986087c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 986087c: precedence now matches QSL FR-147-AC-6/7; values attributable to no refinement are defined; TC-310 mutation 17 tests them. The model-subject row is left inconsistent: see new FND-005 |
| FND-002 | fixed | 986087c: prefixes stated in FR-375 and FR-432 points at them. The compile-request member names are the author's own; see SR-13070 new FND-003 |
| FND-003 | fixed | 986087c: whole-compilation rule stated in FR-375 and FR-307; positive library vector added as TC-310 mutation 16; AC-12 updated |
| FND-004 | fixed | 986087c: combined case stated in FR-375 and AC-14 and tested in TC-310 mutation 17 |
