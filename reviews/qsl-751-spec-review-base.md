---
id: SR-9025
title: "Base review of QSpec PR #249 (QSL-751 follow-ups) against QSL behaviour"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a8689c3d01eb084ae596c883451da255a2dbca65; proposals/quire-v1/definitions/value-accounting.md, proposals/quire-v1/definitions/native-diagnostics.md, proposals/quire-v1/shared-grammar.md, spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md, spec/functional/type-model/FR-153-query-closed-model-environments.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md, spec/test-cases/TC-198-closed-model-lookup.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md; compared with agent-ix/quire-spec-language@a76965a768512d1202deb0371365eb7b2a53026d (impl/business-model-meanings)"
review_set: subset
---

## Summary

Ticket: QSL-751 (items 2 to 6; item 1 withdrawn). The PR adds no row gating
the `model` declaration, so withdrawn item 1 left no residue. The field
lists now match QSL. `ModelNormalizationLimits` declares
`declaration_records`, `derivation_facts`, `effective_declarations`,
`dispatch_candidates`, `hashed_bytes`, `work_units`, `ancestor_steps` and
`family_steps` in that order, plus an `intake` member that is not a V1
counter. `PopulationAdmissionLimits` declares `population_members`,
`work_units` and `ancestor_steps`. The charge points `model.family-edge` and
`admission.ancestor-edge` match QSL's denial records, and so does the
`model-normalization` stage for every model-meter counter. The checks below
compare QSpec's text with QSL's code (qsl-semantics model/accounting.rs,
model/population.rs, model/dispatch.rs, model/refusal.rs,
check/checked_dispatch.rs) and with QSL's FR-082, FR-083, FR-255, FR-361 and
TC-1015.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | `family_steps` counts every followed `redefines` edge ("a permitted edge increments the count by one"), but QSL's `FamilySteps::charge` charges each distinct edge once, keyed by the redefining member, "however many walks follow it". The effective-precondition walk goes back up the same edges the family walk went down. So for a linear chain of n, QSpec counts 2n and QSL counts n, and FR-150-AC-18 and TC-196 C10 give different n-1 boundaries. Fix: say each distinct edge is charged once per link, and a re-followed edge is neither charged nor denied. | proposals/quire-v1/definitions/value-accounting.md:565-578, FR-150-AC-18, TC-196 C10 | wrong-requirement |
| FND-002 | high | The new model-forms sentence and FR-153-AC-11 say that, without the model profile, `Population<T>[N]` and `allInstances<T>(p)` "refuse under their own typing rule" because `T` "resolves to no model object type". The `model` declaration is not gated (QSL FR-361 item 1, QSL-751 item 1 withdrawn), so `T` can resolve. QSL TC-1015 expects these forms to be admitted or refused only by FR-110's layer rule. TC-198 L13 asserts a refusal QSL does not make. Fix: state only that these forms are outside the `unsupported_construct` gate, and drop the asserted refusal. | proposals/quire-v1/shared-grammar.md:461-467, FR-153-AC-11, TC-198 L13 | wrong-requirement |
| FND-003 | medium | QSpec charges `admission.ancestor_steps` only on the member-type coverage walk. QSL also reads it on the subsetting-field applicability walk before `binding.subset-value`, in the frame check, and in the `allInstances`/`lookup` conformance walks over the admitted binding (population.rs conforms calls with `AdmissionAncestorSteps`). QSpec leaves those walks without a stated ceiling or charge point. | proposals/quire-v1/definitions/value-accounting.md:466-497, FR-153 | missing-requirement |
| FND-004 | medium | The new stage rule takes `stage` from "the stage label the denying accounting contract assigns". `value-accounting.md` assigns a label only to `ModelNormalizationLimitsV1` and none to `PopulationAdmissionLimitsV1`, so an `admission.ancestor-edge` denial has no defined `stage`. QSL emits `population-admission` (ADMISSION_OWNER; QSL FR-082). | proposals/quire-v1/definitions/value-accounting.md:464-506, FR-272-AC-19 | missing-requirement |
| FND-005 | medium | The `runtime_invariant` rows key `kind`, `bound` and `required` to the originating context `population.identity`. QSL raises two faults in that context: `identity-byte-bound-reached`, which carries the encoder limit, and `canonical-encoding-failed`, which carries none (population.rs:1025, :1039). The condition should be the invariant, not the context. Also, FR-272 defines `bound` as "its configured value", but this bound is derived from the members and not configured. | FR-272-AC-18, spec/objects/foundation/FR-272-typed-diagnostic-cause.md:101,229-233, proposals/quire-v1/definitions/native-diagnostics.md:106 | wrong-requirement |
| FND-006 | medium | FR-273 makes the selected producer interface "the only membership authority" for a stage label. The new rule makes the accounting contract assign the cause's `stage`, and does not say whether this cause-payload `stage` is a separate value from the diagnostic's FR-273 producer stage. In the same table, the `cancelled` row still retains "`stage`: the actual stage". Two implementers will fill `stage` differently. | FR-272-AC-19, FR-273-AC-4, spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77-78 | wrong-requirement |
| FND-007 | medium | The FR-271 `stage_limit_exceeded` row now quotes "A maximum fixed by the language is not a caller work budget". That calls stage limits fixed by the language, but stage limits are caller-configured FR-461 settings (FR-323 `stage_limits` carries each "with its configured value"). The planner's wording fits `resource_exhausted` but is wrong for this row. | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:84, spec/objects/interfaces/FR-323-native-runtime-envelope.md:89-92 | wrong-requirement |

## Verdict

Findings: two high and five medium. Items 2, 3 and 6 are directionally
faithful. The field-order decision (`family_steps` 8th, `ancestor_steps` 3rd)
matches QSL's shipped types. The planner accepted that decision, and this
review confirms it. Item 5's ungating is right. Its added refusal rationale is
not (FND-002). QSL-side notes for the QSL owner, which are not QSpec defects:
QSL's `PopulationAdmissionLimits::ancestor_steps` doc says "Not charged by
admission itself", but the coverage walk does charge it. QSL FR-083 says
"every `redefines` edge it follows" while its code dedups (FND-001 needs one
ruling for both repos). QSL's diagnostic.rs still says "a semantic maximum is
not a work budget".

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@b82a677422ff63b1fa8c07e29f809c37887829a5.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-008 | medium | The fix for FND-006 makes the cause's `stage` "the one stage value of the diagnostic (FR-273), whose producer interface lists these labels in its closed vocabulary; the cause retains no second stage". No QSpec producer interface lists `model-normalization` or `population-admission`. FR-273's ruling quotes the native compiler interface's labels (`source` ... `evaluate`), and these two are not among them. FR-273-AC-4 refuses any label outside the selected vocabulary. QSL keeps two stages: FR-302 refuses a `family_steps` denial "at S3" while the cause field carries `model-normalization` (checked_dispatch.rs `assert_family_steps_refusal_with`). This was the planner's ruling, so the fix is either to name the producer interface whose closed vocabulary holds the two labels, or to drop the "one stage, no second value" clause. | proposals/quire-v1/definitions/value-accounting.md:631-634, spec/objects/foundation/FR-272-typed-diagnostic-cause.md:77, proposals/quire-v1/definitions/native-diagnostics.md:82, FR-273-AC-4 | wrong-requirement |
| FND-009 | low | For `identity-byte-bound-reached`, `kind` is "the exceeded canonical-encoding limit kind". The canonical definition says `kind` is "the cause-tag spelling QSL emits for it", but QSL renders no catalog field for this limit: `EncoderLimit` is kept only on the internal fault. So no exact ASCII spelling is fixed, which FR-272-AC-13 requires. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:101,194, FR-272-AC-13 | wrong-requirement |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | b82a677 |
| FND-002 | fixed | 18c2112 |
| FND-003 | fixed | 18c2112 |
| FND-004 | fixed | 18c2112 |
| FND-005 | fixed | 18c2112 |
| FND-006 | fixed | 18c2112 (the ambiguity is resolved; the replacement text raises FND-008) |
| FND-007 | fixed | 18c2112 |
| FND-008 | fixed | bf9cbc3 |
| FND-009 | still-open | Reviewed at bf9cbc3. The `runtime_invariant` rows now fix `kind` to exactly `canonical-bytes`, `input-bytes` or `object-bytes`, one each for quire_canonical's `CanonicalBytes`, `InputBytes` and `ObjectBytes`, so FR-272-AC-13's exact-ASCII requirement is met. The canonical definition at FR-272:194 still says `kind` is "the cause-tag spelling QSL emits for it (for example `input-bytes-exceeded`...)". QSL emits no tag for an encoder limit, and `input-bytes` is not a cause-tag spelling, so the two FR-272 statements still disagree on what `kind` holds. Fix: add to :194 "or, for `identity-byte-bound-reached`, the encoder limit kind as the `runtime_invariant` row spells it". |
