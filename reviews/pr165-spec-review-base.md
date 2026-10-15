---
id: SR-11175
title: "Spec review of quire-specification PR 165 (remove file, version and tool tracking), after rebase"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification PR 165 against main; README.md, plan/Plan-001, plan/Plan-007, proposals (adr-0053-handoff, observation-output-mapping-contract, sources-and-rights, state-contract), AD-004, FR-125, NFR-020, TC-020, TC-065, TC-147, US-006, reviews/, spec/reviews/; kept files docs/qualification-dependencies.md, proposals/quire-v1/definitions/complete-model-lock.json, NFR-004"
review_set: subset
---

## Summary

Ticket: none found. The PR body, title and branch name carry no Linear id, so no
tracker comment was posted.

Base spec review and code review of PR 165 after the rebase onto current main.
The diff has 191 files: 175 under reviews/ and spec/reviews/, plus 16 others. It
changes nothing under src/ or tests/.

Units examined: every modified review file (the full hunk set, about 160 files),
the deleted review files (temporal-authority-tracking, capability-ledger-reconciliation x3,
agent-a-rights-publication, qsl117-allocation-sync, tracking-reconciliation, review data
JSON), all 15 non-review spec, plan and proposal edits, TC-065 (deleted), and the three
kept files with every inbound reference. Context only: FR-020, the d07b11f main commit,
model-complete.md "Qualification catalog and package selection".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The review scrubs delete finding and evidence content, not only recorded revisions. spec/reviews/evidence.md loses FND-007 and FND-008 and the NFR-004-M-1/M-2 method rows. spec/reviews/risk-complexity.md loses the NFR-004 risk row and mitigation item 5. The NFR-004 rows are removed from dependency.md (table row, graph node, ordering list), ears-conformance.md, integrity.md and scope-boundary.md. fs06-acceptance/dependency.md rewrites the FND-014 and FND-015 summaries. agent-a-fs01.md rewrites its recommendations. NFR-004 is still in the head tree, so these reviews now leave out a requirement that exists. Restore the removed rows and text. Keep only the evaluated_revision, SHA, fix_sha and data-file-link removals. | spec/reviews/evidence.md; spec/reviews/risk-complexity.md; spec/reviews/dependency.md; spec/reviews/ears-conformance.md; spec/reviews/integrity.md; spec/reviews/scope-boundary.md; spec/reviews/fs06-acceptance/dependency.md; reviews/agent-a-fs01.md |
| FND-002 | high | TC-020 is half of a ruling that the rebase dropped. The PR deletes TC-020's crosswalk paragraph but keeps main's FR-020, which still has FR-020-AC-10 and AC-11 on the inventory. The deleted sentences are real test assertions. A required positive V1 capability cannot disappear because another row shares its family. Later capabilities are not silently promoted. Reserved but absent requirement IDs do not count. These assertions back FR-020's Behavior ("one implemented construct in each family is not the entire capability set") and AC-8/AC-10, and no TC now verifies them. Restore the assertions. The crosswalk link can stay or go. | spec/test-cases/TC-020-report-composed-capabilities.md:19-29; spec/functional/foundation/FR-020-compose-native-profiles.md |
| FND-003 | medium | Two of the three files kept in the last commit are tracking records. docs/qualification-dependencies.md is a hand-kept inventory of the Cargo dev-dependencies and their licences. Its main edit after the fork only resynced one row to Cargo.toml, which is drift maintenance. NFR-004 requires the owner to "record" review and rights, and its only methods are a "rights inventory inspection" and a "requirement and issue audit". Its main edit only dropped the dead FR-016 link. Delete both and fix the inbound references: README.md:79-81, and spec/spec.md:427 (relationship) and :1114 (index row). complete-model-lock.json is content. Keep it. | docs/qualification-dependencies.md; spec/non-functional/NFR-004-retain-review-and-rights.md; README.md:79-81; spec/spec.md:427,1114 |
| FND-004 | medium | The PR body no longer matches the diff. It claims removals that are absent from this diff: the AD-016 Kani pin table and heads manifest; the checksum catalogs; the capability ledger, ticket manifest, adoption record and coverage audit; Plan-008 Task-002/010; NFR-004; qualification dependencies; the complete-model lock and its tests; the operation-catalog digest test; tool-pin recording in the Kani, Verus and SMT specs; and FR-020, FR-311, TC-180, TC-231 and TM-009. Some of these main already did; the others the rebase dropped. "What stays: the ecosystem lock (FR-335, interface_018)" is false because main deleted them. "142 structural failures on both" is stale: both trees now report 145. The title says recorded revisions are removed from reviews, but about 130 review files added after the fork still carry SHAs, as do the std-110 scope strings. Rewrite the body to describe the actual diff. | PR body |

## Verdict

FAIL. FND-001 to FND-003 block the merge, and FND-004 is a body fix.

Clean:

- The 175 review edits are pure deletions of evaluated_revision, reviewed, prior_reviewed, fix_sha and scope SHAs and of data-file links. The exceptions are the hunks listed in FND-001.
- The deleted review files record tracking work: ledger, allocation and manifest reconciliation, and raw tool output.
- No reference outside reviews/ dangles to any file the PR deletes, including TC-065.
- `quire validate` over spec/**, plan/** and reviews/** gives the same 145-document failure set on main and head once line numbers are ignored.
- complete-model-lock.json now holds no digests. It contains the `requires` value-lock edge, the closed selection refusal codes, the role-to-artifact qualification catalog and package_selection. model-complete.md "Qualification catalog and package selection" defines it normatively. TC-195 to TC-198, TC-234 and TC-235 use it as the selection input for `quire.model.complete/v1`. It is content.
- The NFR-020/TC-065 removal is the inventory reconciliation, and the NFR-020 bounding statement stays.
