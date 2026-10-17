---
id: SR-11760
title: "QSL-816 PR 267 spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ed81fddbc60c401f849a11dd73d0c3c9d862873f; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-234-domain-package-model-intake.md, spec/test-cases/TC-945-composed-documents-declare-no-model-node.md, proposals/checked-package-v2/composed-document-vectors.json, proposals/quire-v1/definitions/model-complete.md"
review_set: subset
---
# QSL-816 PR 267 spec review

## Summary

Ticket: QSL-816. One pass over `git diff origin/main...HEAD`. Checked against
FR-154's object id rule and refusal tables, FR-154-AC-8 and AC-10, TC-234
I01 and I09, FR-009 and FR-204, and filament-core-data FR-091, FR-092 and
US-015 at origin/main (FR-154 lists FCD FR-091 as a reference, and K1 is
defined as the package the FCD extraction-frontend lift produces).

Examined:
- FR-154 composed-document paragraph (examined).
- FR-154-AC-14 (examined).
- TC-234 I10 (examined).
- TC-945 C01 to C06 and composed-document-vectors.json (examined; each expected result recomputed against the FR-154 rule).
- model-complete.md object id sentence (examined; consistent with FR-154).
- FR-154 object id rule, declaration refusal table, the "reference to a refused node reports no missing_declaration" rule (context_only; I10/C06 agree with it).
- FR-009, FR-204 (context_only; runtime object identity, no conflict with model-node naming).
- filament-core-data FR-091, FR-092-AC-7, US-015-EX-1 (context_only; source of FND-001).

## Verdict

Changes needed: one high finding. The paragraph, AC-14, I10, C01-C06 and the
model-complete.md sentence agree with each other, with the object id rule and
with the missing_declaration / invalid_model_binding causes. The refusal
choices (missing-name for a reference to an unlifted document,
malformed-declaration at `TypeDefinition.identity` for a frontend-lifted
artifact-type id, no missing-name for a reference to that refused node) are
consistent with FR-154-AC-8 and AC-10. The defect is that the rule contradicts
the FCD lift that FR-154 defines K1 by.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-154-AC-14 and TC-234 I10 require that the lift classifies a document by `type` alone, emits no node for `type: FR` + `object: value_object`, and that B1 plus FR-204/FR-271 yields K1 exactly. K1 is "the domain package that quire-rs extraction and the FCD extraction-frontend lift produce", and FR-154 references FCD FR-091. FCD FR-091 (origin/main) says the opposite in both directions: "For every document whose frontmatter `object` names an object type that one loaded module declares, the frontend SHALL call `extract_semantic`" and "The frontend SHALL skip a document with no `object` frontmatter". FCD FR-092-AC-7 and US-015-EX-1 lift `object: enumeration` / `object: entity` documents. Under the FCD spec, FR-204 would be lifted and C04's `Order` (`type: entity`, no `object:`) would be skipped, so I10's "yields K1 exactly" and C01/C04 are unsatisfiable by the frontend FR-154 names. Also, FR-154's Inputs are package bytes, so "intake classifies a document by its `type`" assigns document classification to a boundary that never sees documents. Either state the rule as a requirement on the FCD frontend and land the matching FCD FR-091 change (and FR-092-AC-7, US-015) with it, or restate the rule against the classifier the FCD frontend actually uses. | spec/functional/type-model/FR-154-admit-domain-package-model.md:180-199,276; spec/test-cases/TC-234-domain-package-model-intake.md:52 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 99a7875 | Round 2, reviewed 99a7875. Planner ruled the rule stays and FCD extraction changes. FR-154 now places the rule on extraction (step 2) and the lift (step 3), says "Intake reads package bytes only and does not classify documents", and AC-14, TC-234 I10, TC-945 and model-complete.md say "extraction skips" consistently. The FCD FR-091 side is not on filament-core-data main and no FCD PR is open at this review, so this PR merges only after that FCD change lands. |
