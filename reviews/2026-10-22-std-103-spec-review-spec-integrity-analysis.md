---
id: SR-14650
title: 'Spec integrity analysis of PR #338: reference upcast requires the source to conform to the target (STD-103)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@95cdaac74; git diff origin/main...95cdaac74 (PR #338, branch std-misc1-std-103): FR-322 "Model-owned members" preamble, "Reference conformance", constraint table, FR-322-AC-94, TC-281, model-member-type-vectors.json reference_upcast_cases, proposals/checked-package-v2/README.md; ticket STD-103'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-281
  type: reviews
---

## Summary

Ticket: STD-103. PR: quire-specification#338, head 95cdaac74. The check covers ids, the TC trace, the vector file, consistency with FR-149, FR-151 and the published operation catalog, and statements elsewhere under spec/ and proposals/ (reviews/ excluded) about reference upcasts. FR-322-AC-94 is new and unique: main ends at FR-322-AC-93 and the other open PRs that add FR-322 ACs use AC-95 (#342), AC-96 (#348) and AC-97 (#349). No open PR adds `RU-` rows or a `reference_upcast_cases` array. The vector file still parses, and the PR changes no node key, digest or package_id. A grep for upcast and downcast over spec/ and proposals/ finds FR-149 (the upcast row; "no checked downcast form is selected"), FR-143, FR-151, FR-153, TC-198 L08, interface_005 and model-complete.md. All of them agree with the new constraint. Gates: `quire validate --scope . spec/**/*.md` reports 134 failed documents, as on main, and the diagnostic multiset is identical. FR-322 has 6 diagnostics on both main and the PR, and TC-281 has none. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 117 and 14.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Model-owned members preamble | spec/objects/interfaces/FR-322-checked-package-artifact.md:866-870 | examined |
| FR-322 Reference conformance (upcast paragraph) | spec/objects/interfaces/FR-322-checked-package-artifact.md:1125-1142 | examined |
| FR-322 constraint table `conforming_upcast` | spec/objects/interfaces/FR-322-checked-package-artifact.md:1667 | examined |
| FR-322 type_argument family | spec/objects/interfaces/FR-322-checked-package-artifact.md:1556-1557 | context_only |
| FR-322-AC-94 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1861 | examined |
| TC-281 | spec/test-cases/TC-281-model-reference-equality-and-field-projection.md | examined |
| reference_upcast_cases | proposals/checked-package-v2/model-member-type-vectors.json:511-524 | examined |
| README model-member-type-vectors and Reference conformance | proposals/checked-package-v2/README.md:115-122,335-341 | examined |
| FR-149 reference upcast row | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:157,162-165 | context_only |
| catalog quire.op.reference.upcast | quire-verification-contracts@e4195bd contracts/checked-operation-catalog-v1.json:147 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The constraint is consistent with FR-149's upcast row, FR-151's declared-supertype conformance and the `conforming_reference` text it mirrors. The catalog entry at quire-verification-contracts e4195bd carries only `member_family` for `quire.op.reference.upcast`. Its `constraint_kinds` vocabulary does not yet list `conforming_upcast` either, so the contracts change in the PR's remaining work needs both. That is cross-repo follow-up, not a defect of this PR.

## Resolution

The proposals README now records that the operation catalog's `constraint_kinds` vocabulary lists `conforming_upcast` beside `conforming_reference`.
