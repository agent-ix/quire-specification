---
id: SR-713
title: "Spec review (base) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@05f6e31144250e4d2ca5898b2f1284075ced297f; PR #162 diff against origin/main 0d53cf2: proposals/quire-v1/definitions/native-diagnostics.md, FR-270, FR-271, FR-272 (rows and FR-272-AC-11), TC-233, spec/complete-v1/tests.md, tests/checked_package_v2.rs"
review_set: subset
---
# Spec review (base) — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. Base checklist over the PR #162 diff only.

The catalog header moves to `1-draft.8`. It adds ten catalog-owned refusal
codes: `inexact_decimal`, `decimal_out_of_domain`,
`division_pair_out_of_domain`, `modulo_out_of_domain`,
`text_length_out_of_domain`, `integer_out_of_domain`,
`rational_out_of_domain`, `ieee_not_exact`,
`ieee_nan_payload_not_representable` and `ieee_rational_out_of_domain`. It
also closes `invalid_source_identity` as `blank-label` and `empty-path`.
FR-270, FR-271 and FR-272 carry the same rows. FR-272-AC-11 maps to TC-233
and to `tc_233_revision_1_draft_8_cause_sets_are_closed`.

Measured:

- The ten code spellings match the kernel refusals in QSL `quire-exact`
  `Refusal`. The two IEEE codes match FR-148, FR-146, value-accounting.md,
  TC-191 and TC-193.
- FR-140, FR-141, FR-148 and value-accounting.md call every one of these
  outcomes a refusal. FR-147 lists `refused` as an output, and
  value-accounting.md names the bounded-consumer membership refusal.
- QSL FR-001 on origin/main refuses a blank label and an empty path with
  `invalid_source_identity` and no region, so `empty-path` is justified. The
  pinned broad meaning in QSL `docs/native-error-codes.md` at `f444d03`
  ("identity, revision or display path is absent") agrees with the new FR-271
  text. The old FR-271 "digest did not match" wording is gone, and digest
  mismatch stays `source_digest_mismatch` (QSL FR-001-AC-2).
- `quire validate` over the five changed spec files fails on the same 4
  documents on this branch and on origin/main, with byte-identical output. The
  README-scope run fails on 142 documents on both. The only difference is one
  more criterion counted (1427 to 1428).
- No other file names `1-draft.7` as the current revision. See FND-002 for
  the definitions index.

## Verdict

**Changes requested.** One medium finding: two of the new payload keys are
outside FR-272's canonical name list. Two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new rows fix `flags` (`ieee_not_exact`) and `label` (`invalid_source_identity`) as exact payload keys. Neither is in FR-272's canonical cause payload contract, which lists exact ordered names and refuses unknown required fields. The catalog paragraph says only that `expected` and `actual` are canonical and says nothing about `flags` or `label`, so their position in the field order is also undefined. Fix: add both to FR-272's ordered list at a stated position, or use canonical names (`actual` for the would-be flags, `subject` for the label). | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:176-181; proposals/quire-v1/definitions/native-diagnostics.md:102,110,219-223 |
| FND-002 | low | The definitions index still gives `quire.native.diagnostics/v1` draft revision `1-draft.5`, three revisions behind the header this PR sets to `1-draft.8`. This predates the PR but is the one other place that states the current revision. | proposals/quire-v1/definitions/README.md:41 |
| FND-003 | low | "A producer claiming revision `1-draft.7` or earlier emits ... no `invalid_source_identity` cause" can be read as those producers never emitting that code. They did, as a retained code with its upstream structured cause. Say "neither `blank-label` nor `empty-path`". | proposals/quire-v1/definitions/native-diagnostics.md:224-225 |
