---
id: SR-8718
title: "QSL-744 base review of TC-235 M10 population member path"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@3ba328881ee062c4e52d9f64f6ba112f03d4d00d; spec/test-cases/TC-235-business-construct-meaning-binding.md row M10"
review_set: subset
---

## Summary

Ticket: QSL-744 (AC4). PR quire-specification#245 changes one cell: TC-235 M10 run 10 now refuses `malformed-declaration` naming `ix://test/orders/PA` at `PopulationDefinition.members[0]` instead of `Population.members[0].type`. The cause, the code, the charge count (22), the run numbering and the FR-208-AC-9 trace are unchanged. The base checklist finds no defect.

## Verdict

**PASS.** Examined: TC-235 M10 (the changed row, all 12 runs), sibling rows M07, M08, M09 and M11 (path root and index convention), FR-208-AC-9 (the criterion M10 verifies). ID format, row id and trace are intact. The new path uses a concrete index, as every sibling row does (`TypeDefinition.members[0]`, `TypeDefinition.steps[3]`, `TypeDefinition.persists[0]`). No hash, digest, pin or tracking record is introduced or touched. A whole-repo grep of QSpec at the PR head finds no remaining `Population.members` spelling and no `members[...].type` spelling anywhere.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
