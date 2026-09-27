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
