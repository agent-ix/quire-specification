---
id: SR-11171
title: "base review of quire-specification PR 248 (delete eight test matrices and status prose; type FR-272 Properties)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4797a4ea830b4ffacbf4c211d4b8c1ff8061bd29; spec/{verification-reference,native-source,composed-foundation,protocol,temporal,observation,output-mapping,complete-v1}/tests.md (deleted), spec/spec.md, spec/test-cases/TC-*.md (28), spec/functional/foundation/FR-020..049,130, spec/objects/foundation/FR-272, spec/objects/observation/FR-269, plan/**, proposals/**, docs/v1-capability-inventory.md"
review_set: subset
---

## Summary

Ticket: QSL-749 (extension of 2026-10-13) and QSL-753. Second review of PR 248 at
head 4797a4e, after the extension to all eight hand-maintained matrices. Base
checklist plus a manual walk of the QSL-749 and QSL-753 acceptance criteria. Every
status-prose and plan diff was read in full (28 TC files, 22 FR files, 12 plan
files, 3 proposal files, the inventory doc, FR-269 and FR-272). Every edit removes
only tracking ("TM-00x remains Planned", matrix links); no test procedure, expected
result or requirement text is removed.

Sole-home check, measured against main's eight matrices: all 309 TC/IT ids they
list have their own file at the head. The AC-to-TC rows were compared with each
FR's own Acceptance Criteria table and with every TC/IT file. 516 matrix AC rows
are carried nowhere else at AC level. 303 of them map only to TC-231, a single
FR-311 qualification case whose procedure does not exercise those criteria.
Another 124 map criteria whose own Verification column is Analysis or Inspection
to a TC. Neither group is real trace content, and both are recorded here and not
raised. The remaining 89 are Test-verified criteria whose Verification cell names
no TC, so the matrix row was the only place naming the case (FND-001).

The FR-046-AC-8 row the coder flagged is not lost. FR-046's own table reads
`Test (TC-046)` for AC-8, and TC-046's procedure and expected results exercise the
`invocation`-member refusal. The PR body says otherwise (FND-003).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | 89 Test-verified acceptance criteria lose the only record of which TC verifies them. Their FR Verification cell reads bare `Test`; the deleted matrices named the case. Each TC still has a FR-level `verifies` edge, but not the AC. Fix: write the TC into each Verification cell (`Test (TC-nnn)`, the repo's own form, as in FR-046), one edit per row: FR-001 AC-1/2/4 TC-005; FR-019 AC-1..6 TC-006..TC-011 (one each, in order); FR-153 AC-8 TC-198; FR-240 AC-1..10 TC-061; FR-260 AC-1..4 and FR-261 AC-1..3 TC-140; FR-263 AC-1..5, FR-264 AC-1..5, FR-265 AC-1..5, FR-266 AC-1..4 TC-142; FR-267 AC-1..5, FR-268 AC-1..7, FR-294 AC-1..3 TC-143; FR-322 AC-4/5/6/8/9 TC-217 and AC-15/17/18/19/21/23/24/35/62 TC-233; FR-340 AC-1..12,14,15 TC-233. | spec/functional/foundation/FR-019-admit-native-source-profile.md:66-72; spec/objects/protocol/FR-240-canonical-protocol-result.md; spec/objects/interfaces/FR-322-checked-package-artifact.md:1408; spec/objects/interfaces/FR-340-checked-package-frame-location.md | correct-requirement-no-evidence |
| FND-002 | medium | Twelve TC files still carry the status prose the QSL-749 extension removes. TC-033, TC-034, TC-038 to TC-046 and TC-048 each say "The matrix remains Planned". "The matrix" is TM-003 (spec/composed-foundation/tests.md), which this PR deletes. The sweep only matched literal `TM-00x`, so it missed these. Delete the "The matrix remains Planned;" clause in each, keeping the rest of the sentence. | spec/test-cases/TC-033-admit-reusable-predicates.md:12; spec/test-cases/TC-046-validate-state-invocation-inputs.md:12; spec/test-cases/TC-048-bound-composed-processing.md:12 | wrong-requirement |
| FND-003 | medium | The PR body is wrong about traces. It says the FR-046-AC-8 trace was "the one row only a matrix held" and is dropped. FR-046's AC table carries `Test (TC-046)` for AC-8, and TC-046 exercises it, so nothing is dropped. Meanwhile the 89 criteria in FND-001 really were matrix-only, and the body does not mention them. Correct the body to match whatever FND-001 lands as. | PR 248 body; spec/functional/foundation/FR-046-validate-state-invocation-inputs.md:38 | wrong-requirement |

## Verdict

Not mergeable until FND-001 to FND-003 are fixed. All three are mechanical edits
inside this PR.

What holds:

- All eight matrix files are deleted. `git grep TM-00` outside `reviews/` finds
  nothing, and no live doc links a deleted `tests.md` path. Main's only index row
  and relationship edge (TM-009 in spec/spec.md) are removed together.
  TM-001..TM-008 had no index rows or edges on main.
- 28 TC files, 22 FR dependency bullets and 12 plan, task and log files lose only tracking
  text. No test procedure, expected result, requirement or task content is removed.
- FR-272: the typed Properties table (Field, Type, Multiplicity, Constraints)
  keeps every row. The old Source and Note text moves verbatim into the "Where the
  fields come from" table, with only the capitalisation of "proposed catalog"
  changed. quire validate drops the `fields` required error and the
  legacy-properties warning; only the module-owned `id` error remains. That meets
  the first QSL-753 AC. FR-271 is untouched.
- No new matrix, ledger or count file is added (QSL-749 AC-3).
- SR-9011 FND-001 is addressed by 6795a66, apart from the residue in FND-002.

Lows, recorded and not blocking:

- Eleven TC descriptions end in a double period after the clause was cut:
  TC-004, TC-112, TC-114, TC-115, TC-116, TC-118 and TC-151 to TC-155.
- Every FR-272 field is typed `String`. `code_family` and `location` could
  reference the FR-271 and FR-274 types. No validator rule requires this.
- Completed historical checklist lines still mention matrix statuses
  (Plan-004 plan.md:74, Plan-005 plan.md:70, Plan-006 plan.md:68), and so do
  proposals prose (shared-foundation.md:162, state-contract.md:286). These are
  historical records, not instructions.
- The second QSL-753 AC (the report to the module owner) is outside the PR and
  is not verified here.
