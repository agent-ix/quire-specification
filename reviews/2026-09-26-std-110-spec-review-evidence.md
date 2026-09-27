---
id: SR-716
title: "Spec review (evidence) — STD-110 catalog 1-draft.8 value-refusal codes"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-specification@05f6e31144250e4d2ca5898b2f1284075ced297f; PR #162 diff against origin/main 0d53cf2: FR-272-AC-11, TC-233, spec/complete-v1/tests.md, tests/checked_package_v2.rs"
review_set: subset
---
# Spec review (evidence) — STD-110 catalog 1-draft.8 value-refusal codes

## Summary

Ticket: STD-110. This pass checks that each new claim has a verification
method and evidence. FR-272-AC-11 is verified by Test (TC-233), and the
matrix row is present. The test is traced with
`#[trace("TC-233", "FR-272-AC-11")]`.

Four mutations were run in a scratch worktree:

- drop `empty-path` from the native row;
- drop `both-outside-domain` from the FR-272 row;
- add a cause to the native `ieee_not_exact` row;
- rename the FR-272 `modulo_out_of_domain` cause.

Each one failed the test with the named row.

The ticket's deliverable is "a refusal-category code, with the required field
keys". Only the causes are verified.

## Verdict

**Changes requested.** One medium finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No criterion or test verifies the payload keys (`expected`, `actual`, `flags`, `label`) that QSL-245 consumes. `diagnostic_row_causes` reads only the cause cell before the first `;`, so renaming `flags` or `expected` in either document passes every test. No test contains these keys. No criterion ties a code to its FR outcome either: TC-185, TC-186 and TC-192 say "refuses" without a code. Fix: extend FR-272-AC-11, or add a criterion, and extend the TC-233 test to parse each of the eleven rows' payload keys in both documents. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:172; tests/checked_package_v2.rs:4875-4896,6663-6705 |

## Dispositions

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-8156e8a1

<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-716 date=2026-09-26 -->

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | Partly fixed in 6389506c. The payload-key half is fixed: FR-272-AC-11 names each code's keys, and `tc_233_revision_1_draft_8_cause_sets_and_payload_keys_are_closed` checks both documents' rows. The reviewer re-ran the `flags`->`flag_set` rename and it fails the test. The second half of the finding is untouched: "no criterion ties a code to its FR outcome". FR-140, FR-141, FR-147 and TC-185, TC-186, TC-192 still say "refuses" without naming `inexact_decimal`, `decimal_out_of_domain`, `text_length_out_of_domain`, `division_pair_out_of_domain` or `modulo_out_of_domain`, and FR-148/TC-193 do not name `ieee_not_exact`. Only the catalog paragraph maps a code to its FR. Fix: name the code in each owning FR's refusal (as 6389506c already did for the two FR-148 IEEE codes), or defer with a ticket id. |

Delta scope: `git diff 05f6e311..6389506c`. Verified at 6389506c88d3b6eab02efe917afb1adeca7f7d19.

+++ [reviewer data]

```yaml
reviewed: 6389506c88d3b6eab02efe917afb1adeca7f7d19
prior_reviewed: 05f6e311
dispositions:
  - fnd: FND-001
    outcome: still-open
    partial_fix_sha: 6389506c
    reason: "Payload keys: fixed and mutation-verified (rename flags->flag_set in the native-diagnostics ieee_not_exact row fails at tests/checked_package_v2.rs:6784, left {expected} right {expected, flags}). Code-to-FR outcome: open. `grep -rn` over spec/functional and spec/test-cases for inexact_decimal|decimal_out_of_domain|modulo_out_of_domain|division_pair_out_of_domain|text_length_out_of_domain|ieee_not_exact returns nothing at 6389506c."
    path: spec/objects/foundation/FR-272-typed-diagnostic-cause.md
    lines: "172"
    after_excerpt: |-
      Each code's two rows name exactly these payload keys: `label` for `invalid_source_identity`; `expected` and `flags` for `ieee_not_exact`; `expected` and `actual` for `ieee_nan_payload_not_representable`; and `expected` for each other code. `label` and `flags` stand in this requirement's canonical payload-name list.
```

+++

