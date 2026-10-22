---
id: SR-16184
title: "Spec review of quire-specification PR #383: STD-322, FR-352 disposition.temporal"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c19a1e778; PR #383 (git diff origin/main...c19a1e778): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md, spec/test-cases/TC-255-native-run-result-v2-wire.md"
review_set: subset
evaluated_revision: "c19a1e778"
relationships:
  - target: ix://agent-ix/quire-specification/FR-352
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-255
    type: reviews
---
# SR-16184: Spec review of quire-specification PR #383: STD-322, FR-352 disposition.temporal

## Summary

Ticket: STD-322. Reviewed `git diff origin/main...HEAD` at c19a1e778 against QSL FR-267 at main 2c5d82d37 ('Temporal disposition wire', the wire member cross-check rows for `reason` and `limit`, FR-267-AC-14/15) and QSL FR-100's outcome vocabulary (undefined reasons from the kernel and family tables; `incomplete` limit an accounting counter). The ten-row table matches QSL's row for row in members, category, truth, basis and outcome; placement (evaluate stage only, no compile/select/admit, internal failure via `internal_fault`) and the round-trip rule match. The five STD-322 preconditions are each stated. TC-255 NR-12 covers all ten kinds and the `tuple-arity` payload. Lead default (insufficient-data and max_window_values refused on a non-temporal run): sound. FR-100's vocabulary already excludes both (neither is a kernel or family undefined reason, and `max_window_values` is not an accounting counter), so the rule makes explicit what FR-100's closed vocabulary implies, and keeps a state-clause reader from admitting a temporal-only value. Recommend keeping it, with a QSL FR-267 mirror (FND-003). Ids FR-352-AC-10..12 and NR-12..14 are unique across main and every open PR. Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed; TC-255 has no parsed vectors file, so no conformance run applies. origin/main is an ancestor; no textual conflict with the other six PRs.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-352 Temporal disposition` (examined): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md
- `FR-352 basis bullet` (examined): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md
- `FR-352-AC-10` (examined): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md
- `FR-352-AC-11` (examined): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md
- `FR-352-AC-12` (examined): spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md
- `TC-255 NR-07` (examined): spec/test-cases/TC-255-native-run-result-v2-wire.md
- `TC-255 NR-12` (examined): spec/test-cases/TC-255-native-run-result-v2-wire.md
- `TC-255 NR-13` (examined): spec/test-cases/TC-255-native-run-result-v2-wire.md
- `TC-255 NR-14` (examined): spec/test-cases/TC-255-native-run-result-v2-wire.md
- `QSL FR-267 Temporal disposition wire` (context_only): agent-ix/quire-spec-language:spec/functional/FR-267-write-run-results-as-native-run-result-2.md

## Verdict

**MERGEABLE after a fix**: one medium finding, two low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | NR-14 tests FR-352-AC-12's 'a counterexample whose trace_position differs from position' by setting `trace_position` to the string "1". A string is refused by any strict reader as a type error, so the vector passes on a reader that never compares `trace_position` with `position`. Set it to an unsigned integer other than the record's `position` (for example `position + 1`). | spec/test-cases/TC-255-native-run-result-v2-wire.md:41 |
| FND-002 | low | The extension is scoped to 'a temporal selection', not to the two rows that use it. The `undefined` row's outcome is 'the undefined value's FR-100 reason' and the `incomplete` row's is 'FR-100's incomplete with its limit', so whether a temporal `undefined` disposition with reason `insufficient-data`, or an `incomplete` one with limit `max_window_values`, reads or is refused as 'an outcome other than the row's' depends on the reader. Say the reason is admitted only in the `insufficient-data` row and the limit only in the `window-limit-reached` row, as QSL FR-267's rows spell them. | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:152-158 |
| FND-003 | low | Three reader refusals go beyond QSL FR-267's list: a `temporal` on a non-temporal selection, a `temporal` beside a top-level `refusal` (QSL says only that no row writes one), and the two temporal-only values on a non-temporal run. The PR body flags only the last as a lead default. TC-255 NR-07, NR-13 and NR-14 test all three, and QSL-902's TC-255 conformance will run them against a QSL reader whose FR-267 states none of them. Flag the other two as lead defaults and add the matching QSL FR-267 sentences (a QSL follow-up). | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:164-173 |
