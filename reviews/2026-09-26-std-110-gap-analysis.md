---
id: SR-718
title: "Gap analysis — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@05f6e31144250e4d2ca5898b2f1284075ced297f; PR #162 diff: FR-272-AC-11, FR-270 revision cell, FR-271 new values rows, native-diagnostics header, tests/checked_package_v2.rs"
review_set: subset
---
# Gap analysis — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. Each new criterion and each new row was mapped to a test:

| Item | Test | Status |
| --- | --- | --- |
| FR-272-AC-11: causes of the eleven codes in native-diagnostics and FR-272 | `tc_233_revision_1_draft_8_cause_sets_are_closed` | covered; mutation-verified (4/4 killed) |
| Native-diagnostics header `1-draft.8` | `tc_233_native_diagnostics_header_pins_current_revision` | covered |
| Matrix row FR-272-AC-11 to TC-233 | spec/complete-v1/tests.md:886 | present |
| Payload keys of the eleven rows | none | gap (SR-716 FND-001) |
| FR-270 revision cell `1-draft.8` | none | gap (FND-001) |
| FR-271 values rows for the ten codes | none | gap (FND-001) |

## Verdict

**Pass with a low gap.** The criterion the PR adds is fully covered.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No test pins FR-270's revision cell or checks that FR-271's vocabulary table lists the ten new codes. A stale FR-270 cell, which is the SR-623 FND-001 class of defect, or a code dropped from FR-271 passes every test. No criterion requires either check, which is why this is low. | spec/objects/foundation/FR-270-versioned-diagnostic-catalog.md:35; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:84-93 |
