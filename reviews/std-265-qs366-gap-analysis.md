---
id: SR-15504
title: "Gap analysis of STD-265 invalid_result causes (QSpec PR #366)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@b34f4d4f25cfcaac5892d8b83f6e7f102232848d; spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md, tests/checked_package_v2.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# SR-15504: Gap analysis of STD-265 invalid_result causes (QSpec PR #366)

## Summary

Ticket: STD-265. PR: quire-specification#366, head b34f4d4 (live head verified after fetch), scoped to `git diff origin/main...b34f4d4` (one commit, 6 files, +66/-4): FR-272 and native-diagnostics `invalid_result` rows (six causes, `document_pointer`/`expected`/`actual`), FR-331 'Result entries' table and validation redirect, FR-331-AC-34, FR-272-AC-23, TC-233 step, TC-431 BP-25, and the new test tc_233_invalid_result_cause_set_is_closed. Gates in a detached temp worktree: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines (grep -ciE 'failed|fail'), identical line-for-line to origin/main 429c59d, none new in a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (119 + 23). quoin 0.28.3, quire 0.36.2 (engine 0.50.2). Planless, scoped to the diff. Plan completion: not assessed. AC-to-test: FR-272-AC-23 -> tc_233_invalid_result_cause_set_is_closed (first clause only, see SR-15501 FND-001). FR-331-AC-34 -> Inspection via TC-431 BP-25, with no executable test (the schema cannot name a cause). No production code changed. The new test has no stub or inflation beyond the AC-23 clause already reported.

Units examined:

- FR-272-AC-23 (examined): The native-diagnostics row and this requirement's row of `invalid_result` each list exactly `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch` and `unknown-cause`, and each names `document_pointer`, `expected` and `actual`; each refused result entry 
- FR-331-AC-34 (examined): A reader refuses each of these before consumption with the `invalid_result` cause given, naming the pointer given: `certification-missing`, `missing-member` at `/certification`; `certification-on-refuted`, `unknown-member` at `/certification`; `certification-other-value`, `invalid-value` at `/certif
- TC-431 (examined): | BP-25 | The reader, given BP-10's adverse vectors `certification-missing`, `certification-on-refuted`, `certification-other-value`, `certification-reduced-certified`, `certification-bounded-solver-certified` and `certification-symmetry-certified`, and a plugin's `proved` entry carrying `certified`
- tc_233_invalid_result_cause_set_is_closed (examined): fn tc_233_invalid_result_cause_set_is_closed() { ... assert_eq!(diagnostic_row_causes(document, "invalid_result"), causes, ...); let keys = diagnostic_row_payload_keys(document, "invalid_result"); for key in ["document_pointer", "expected", "actual"] { assert!(keys.contains(key), ...) } }

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | minor (gap): BP-25 is 'verified by inspection of the reader', but it does not say which reader. QSpec has none, and the only consumer requirement, QSL FR-290, reads plugin entries only and covers just the plugin `certified` case. Nothing implements the five core-proof vectors' causes (`certification-missing`, `-on-refuted`, `-other-value`, `-reduced-certified`, `-bounded-solver-certified`, `-symmetry-certified` on first-party entries). TC-431 also says it 'Runs as tests/backend_provider_v1.rs', which BP-25 does not. Fix: name the reader artifact (the QSL or consumer requirement) that the inspection reads, or record that the core-entry causes are open until such a reader exists. | spec/test-cases/TC-431-backend-provider-wire-vectors.md:52 |

## Verdict

Mergeable for gaps apart from SR-15501 FND-001. One minor: BP-25's inspection names no reader.

## Dispositions

Round 1, reviewed QSpec #366 at ba8287988.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | - |
