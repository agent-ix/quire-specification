---
id: SR-715
title: "Spec review (failure-domain) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@05f6e31144250e4d2ca5898b2f1284075ced297f; PR #162 diff against origin/main 0d53cf2: proposals/quire-v1/definitions/native-diagnostics.md new rows and 1-draft.8 paragraph, FR-271 and FR-272 new rows; cross-checked against QSL FR-001 (origin/main), FR-140, FR-148"
review_set: subset
---
# Spec review (failure-domain) — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. This pass looks for unstated failure modes and for identity
confusion in the new cause rows. The single-cause codes are unambiguous. The
three `division_pair_out_of_domain` causes partition the two membership
failures completely. `blank-label` fixes a label order, and `White_Space`
pins what counts as whitespace. Refusals retain no value, which matches
kernel `Refusal`'s "never carry the refused value".

Three gaps remain.

## Verdict

**Changes requested.** One medium finding and two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `invalid_source_identity` does not say which cause wins when a label is blank and the path is also empty. The label order decides between labels, but nothing decides between `blank-label` and `empty-path`, so two conforming producers can report different causes for one input. QSL FR-001 does not settle it either. Fix: state the precedence, for example labels before path. | proposals/quire-v1/definitions/native-diagnostics.md:102; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:84 |
| FND-002 | low | The rows fix the payload keys but not how the values are written. A domain `Decimal[lo, hi; smin, smax]`, `Int[lo, hi]` or `Text[min, max; profile]`, a width `binary32`, and the flag set can each be rendered in more than one form. FR-272 fixes only the order of collections. QSL already renders the `cardinality_out_of_bound` bound as `[min, max]`. Field bytes can therefore differ between producers. | proposals/quire-v1/definitions/native-diagnostics.md:102-112 |
| FND-003 | low | The `ieee_not_exact` `flags` row says "the would-be flag set ... the rounded result would have raised". Strict `exact` selects no rounding direction. FR-148 fixes these as the flags the operation would return under `nearest-even`. The row should say so. | proposals/quire-v1/definitions/native-diagnostics.md:110; spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md:55-57 |
