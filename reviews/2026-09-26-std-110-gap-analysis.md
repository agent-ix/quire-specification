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

## Dispositions

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-bbab1dfa

<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-718 date=2026-09-26 -->

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6389506c: added FR-270-AC-5 (the Revision cell quotes the header revision) and FR-271-AC-6 (one values row for each of the eleven codes), both mapped to TC-233 in spec/complete-v1/tests.md. They are traced by `tc_233_catalog_revision_and_code_vocabulary_follow_the_catalog`. The reviewer deleted FR-271's `integer_out_of_domain` row in a scratch worktree and the test failed. |

Delta scope: `git diff 05f6e311..6389506c`. Verified at 6389506c88d3b6eab02efe917afb1adeca7f7d19.

+++ [reviewer data]

```yaml
reviewed: 6389506c88d3b6eab02efe917afb1adeca7f7d19
prior_reviewed: 05f6e311
dispositions:
  - fnd: FND-001
    outcome: fixed
    fix_sha: 6389506c
    path: spec/objects/foundation/FR-270-versioned-diagnostic-catalog.md; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md
    lines: "FR-270:95; FR-271:161"
    after_excerpt: |-
      | FR-270-AC-5 | The Revision property's quoted revision equals the revision in the catalog header's interpretation-identity line. | Test (TC-233) |
      | FR-271-AC-6 | The values table has exactly one row for each of `invalid_source_identity` and the ten codes catalog revision `1-draft.8` adds: ... | Test (TC-233) |
    verification: "Reviewer mutation (delete FR-271 `integer_out_of_domain` row) fails tc_233_catalog_revision_and_code_vocabulary_follow_the_catalog at tests/checked_package_v2.rs:4883 (left 0, right 1)."
```

+++

