---
id: SR-14655
title: 'Gap analysis of PR #344: FR-154-AC-16 and FR-322-AC-30 against TC-280 (STD-104)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@f756c29c6; git diff origin/main...f756c29c6 (PR #344): FR-154-AC-16 and FR-322-AC-30 against TC-280 MT-08 and MA-16..MA-18, proposals/checked-package-v2/model-member-type-vectors.json and tests/model_member_types.rs; ticket STD-104'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
---

## Summary

Ticket: STD-104. Plan completion: not assessed. Both changed ACs have TC-280 rows. FR-154-AC-16 is traced by MT-08, MA-16, MA-17 and MA-18, and FR-322-AC-30 by MT-08 to MT-10 and MA-16. TC-280 says the vectors file records its member cases, "each recorded with its criterion". The PR adds no vector or test for the new rows. The ticket also asks to "update FR-322 and the vectors", and the vectors are not updated.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-154-AC-16 | spec/functional/type-model/FR-154-admit-domain-package-model.md:287 | examined |
| FR-322-AC-30 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1784 | examined |
| TC-280 MT-08, MA-16..MA-18 | spec/test-cases/TC-280-model-owned-member-types.md:130,149-151 | examined |
| member_cases, selection_cases | proposals/checked-package-v2/model-member-type-vectors.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Minor. MA-16, MA-17 and MA-18 have no entry in `model-member-type-vectors.json`: no document-patch selection case for a removed or dangling `Operation.redefines`, and no extension-only case. No test is tagged FR-154-AC-16. TC-280 says the file carries its member and selection cases, each recorded with its criterion, so the three new rows name inputs nothing records. The ticket asks for the vectors too. The PR lists them as remaining work. | spec/test-cases/TC-280-model-owned-member-types.md:149-151; proposals/checked-package-v2/model-member-type-vectors.json |

## Verdict

The TC rows cover every changed AC clause. FR-154-AC-16's admitted case depends on the fixture defect in SR-14653 FND-001, and its three adverse rows have no recorded vectors (FND-001).

## Resolution

FND-001 is closed. `model-member-type-vectors.json` carries `operation_redefinition_cases` MA-16, MA-17a, MA-17b and MA-18, each a document patch with its expected result, and `tests/model_member_types.rs` evaluates them under FR-154-AC-16 and FR-322-AC-30.

## Dispositions

Round 1, reviewed at 0a4daf8d636d4025c77ca40e599ce0e931ecbcc1. `cargo test --test model_member_types` passes 24, including `tc_280_an_operation_redefines_only_through_operation_redefines`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `model-member-type-vectors.json` carries `operation_redefinition_cases` MA-16, MA-17a (`required` false), MA-17b (`required` true) and MA-18, each with `"criterion": "FR-154-AC-16"`, a document patch and its expected result. `tc_280_an_operation_redefines_only_through_operation_redefines` is traced to FR-154-AC-16 and FR-322-AC-30, asserts four cases, and evaluates each over the patched document. |
