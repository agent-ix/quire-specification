---
id: SR-15502
title: "EARS review of STD-265 invalid_result statements (QSpec PR #366)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@b34f4d4f25cfcaac5892d8b83f6e7f102232848d; spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
---
# SR-15502: EARS review of STD-265 invalid_result statements (QSpec PR #366)

## Summary

Ticket: STD-265. PR: quire-specification#366, head b34f4d4 (live head verified after fetch), scoped to `git diff origin/main...b34f4d4` (one commit, 6 files, +66/-4): FR-272 and native-diagnostics `invalid_result` rows (six causes, `document_pointer`/`expected`/`actual`), FR-331 'Result entries' table and validation redirect, FR-331-AC-34, FR-272-AC-23, TC-233 step, TC-431 BP-25, and the new test tc_233_invalid_result_cause_set_is_closed. Gates in a detached temp worktree: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines (grep -ciE 'failed|fail'), identical line-for-line to origin/main 429c59d, none new in a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (119 + 23). quoin 0.28.3, quire 0.36.2 (engine 0.50.2). EARS: the one new SHALL statement ('A reader SHALL refuse a result entry ... before consumption with the FR-272 invalid_result cause that names its defect') is a ubiquitous requirement with a named subject, a single SHALL, and its response (cause, item index, pointer) stated. The unknown_required_feature paragraph that follows is explanation in the requirement's own terms. The validation redirect amends an existing sentence without adding a second SHALL. AC-23 and AC-34 are direct verification statements, not given-when-then shaped. The ambiguity of 'the cause that names its defect' when an entry has more than one defect is a failure-domain matter (SR-15503).

Units examined:

- FR-331 (examined): A reader SHALL refuse a result entry, or the `counterexamples` entry a `refuted` result names, before consumption with the FR-272 `invalid_result` cause that names its defect, retaining the result item index and the offending member's RFC 6901 pointer relative to that entry (the empty pointer names 
- FR-331 (examined): Unknown contract versions, missing required properties, duplicate keys, out-of-domain values and non-canonical encodings refuse before consumption; in a result entry or a `counterexamples` entry, an unknown member and an out-of-domain value refuse with the `invalid_result` causes of "Result entries"
- FR-272-AC-23 (examined): The native-diagnostics row and this requirement's row of `invalid_result` each list exactly `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch` and `unknown-cause`, and each names `document_pointer`, `expected` and `actual`; each refused result entry 
- FR-331-AC-34 (examined): A reader refuses each of these before consumption with the `invalid_result` cause given, naming the pointer given: `certification-missing`, `missing-member` at `/certification`; `certification-on-refuted`, `unknown-member` at `/certification`; `certification-other-value`, `invalid-value` at `/certif

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean: no EARS defects in the changed statements.
