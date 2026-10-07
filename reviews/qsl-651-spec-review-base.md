---
id: SR-1307
title: "Spec review of quire-specification PR #195 (QSL-651): delete authored status from the test matrices"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f76b87ce5f4eae54ec7554f52fc8b49c11f300d2; PR #195 diff against origin/main 65e816f0; spec/complete-v1/tests.md, spec/composed-foundation/tests.md, spec/native-source/tests.md, spec/observation/tests.md, spec/output-mapping/tests.md, spec/protocol/tests.md, spec/temporal/tests.md, spec/verification-reference/tests.md"
review_set: subset
---
# Spec review of quire-specification PR #195

## Summary

Ticket: QSL-651. The PR applies the plan-lead ruling that hand-written test-matrix status is tracking ceremony. In all 8 TestMatrix documents it empties the coverage-table Status cells, cuts each Test Case Summary Status cell to a bare `🚧` marker, renames `Coverage Status` headers to `Status`, drops the Status column from the Atomic Capability and Integration tables, and deletes status prose (Overview disclaimers, `Evidence limits` and `Evidence Limits` sections, the complete-v1 `Coverage gate`).

Examined:
- Overview prose of all 8 tests.md files (examined)
- Every table row in all 8 files, old against new with the Status column removed (examined)
- Deleted sections: verification-reference `Evidence limits`, temporal `Evidence Limits`, complete-v1 `Coverage gate` (examined)
- spec-artifacts-process mappings.yaml TestMatrix entry, lines 240-300 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Overview swaps one status claim for another. On main it said every row "remains Planned until the separate qcir #55/#56/#57 Rust implementations bind real trace evidence". It now says the controls are "implemented by qcir #55/#56/#57", which claims the work is done. The ruling deletes status prose, it does not reverse it, and nothing in the PR shows the claim is true. The Integration Test Matrix row at line 59 already names qcir #55/#56/#57 as the target. Cut the line to "FS06-accepted portable mapping controls." | spec/output-mapping/tests.md:21 |
| FND-002 | low | One status word is left: the Overview still says "Shared-foundation requirements and planned conformance controls". Change "planned conformance controls" to "conformance controls". | spec/composed-foundation/tests.md:65 |

## Verdict

The deletion is right and nothing else was lost. Comparing every table row old against new with the Status column removed: all 2,504 rows across the 8 files are identical, the TC id sets match (225, 32, 7, 11, 6, 21, 28, 4), and no FR, AC, NFR, IT, INT, TC or V1 capability id was dropped or added. The remaining Status values are only empty cells and bare `🚧` markers. No `Planned`, `planned` or other status words remain in table cells. The deleted prose is all status or claim text: "every row remains Planned", "does not claim …", "No hosted workflow was added", and the Coverage gate's promotion rule. The test-design sentences stay, including temporal's "inspected input, not coverage", protocol's "verify claim separation", and composed-foundation's "not automatically composed-profile evidence". IT-003 still owns the structural/JCS distinction that the verification-reference `Evidence limits` section repeated.

Measured: the schema does require the column. In spec-artifacts-process mappings.yaml, the TestMatrix `functionalCoverage` and `testCases` tables require a `Status` column parsed as `status-marker`, so the bare-marker approach is the minimum the schema allows. `quire validate --scope . 'spec/**/*.md'` (quire 0.36.1, engine 0.50.1) drops from 145 to 137 failed documents. The 8 removed failures are exactly the `Coverage Status` column mismatches, no tests.md failure remains, and no new failure appears. `quire matrix` output is byte-identical before and after (3,816 lines), so quire does not read these status cells to compute coverage. No Rust test reads tests.md. `cargo test --locked` passes: 0 + 3 + 88 + 16 + 7 + 0 tests, 0 failed.

Two findings, one medium and one low, both in Overview prose. **Mergeable once FND-001 and FND-002 are fixed.**

## Dispositions

Round 1, reviewed at 07d507ee68b8126825c184cc3669480eb1c9f35e.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 07d507ee |
| FND-002 | fixed | 07d507ee |

Both fixes are in commit 07d507ee, and the commit changes only those two Overview lines (plus the committed SR files in 6ad7abf). At the head, `quire validate --scope . 'spec/**/*.md'` reports 137 failed documents, none in tests.md, and no status words remain in tests.md prose. **Mergeable.**
