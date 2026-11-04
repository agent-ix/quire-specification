---
id: SR-617003
title: 'Gap analysis of PR #617: test mapping for FR-464-AC-2, AC-4, AC-6 and FR-466-AC-1 (QSL-1366)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@a85c7fb; quoin matrix over FR-464, FR-466, TC-946, TC-948 and the manifest tests; QSL FR-383-AC-20 and TC-1063 at origin/main'
review_set: subset
date: '2026-11-04'
relationships: []
---

## Summary

Ticket: QSL-1366. Row counts are unchanged: 85 manifest rows (25 admit, 56 refuse, 4 property) plus 34 layout rows, all refuse, giving 25 admit, 90 refuse, 4 property, so QSL FR-383-AC-20 stays true; QSL TC-1063 row 8a already puts a repeated role entry at the member. quoin matrix: FR-464-AC-2 and FR-464-AC-4 and AC-6 are tagged to the new tests; FR-464-AC-3 is untagged on main and in the PR (not changed here). No run evidence is bound for any of them.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-464-AC-2 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:703 | examined |
| FR-464-AC-4 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:705 | examined |
| FR-464-AC-6 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:707 | examined |
| FR-466-AC-1 | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:341 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The binding gap for M1 is recorded as SR-617002 FND-001.
