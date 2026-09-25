---
id: SR-623
title: "Base review of the I2 reader stage-limit causes"
type: SpecReview
analysis: base
scope: "STD-95 branch task/95-i2-limit-causes: native-diagnostics revision 1-draft.7 (`stage_limit_exceeded` causes `token-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`; catalog-owned 1-draft.6 spellings), FR-270 revision cell, FR-271 1-draft.6/1-draft.7 codes and history, FR-272 `duplicate_selection` and `stage_limit_exceeded` rows and FR-272-AC-10, TC-233, spec/complete-v1/tests.md, tests/checked_package_v2.rs"
review_set: base
evaluated_revision: "task/95-i2-limit-causes"
review_date: "2026-09-24"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-270, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-271, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-272, type: reviews }
  - { target: ix://agent-ix/quire-specification/TC-233, type: reviews }
---
## Summary

The change gives each of the I2 checked-package reader's seven limit kinds its
own `stage_limit_exceeded` cause. The reader's kinds, from `quire-contract-model`
`CheckedPackageLimit` on `quire-contract-ir` `main`, are `Bytes`, `Depth`,
`Nodes`, `Edges`, `Occurrences`, `Diagnostics` and `Work`. The catalog row now
lists seven causes in that order, names the seven kinds, and marks the three new
causes as revision `1-draft.7`. A revision paragraph states that a producer
claiming `1-draft.6` or earlier emits none of them. No existing cause changes
meaning, and no kind falls back to a broader cause.

The second pass, on the coordinator's decisions, adds `token-count-exceeded`
for S1's token ceiling (QSL `SyntaxLimit::Tokens`) to the same revision. It also
closes FND-003: FR-272 gains `duplicate_selection` and `stage_limit_exceeded`
rows and FR-272-AC-10, FR-271 gains both codes and its history through
`1-draft.7`, and a TC-233 test pins both cause sets in both documents. I1
domain-package intake limits stay out of scope and go to a separate ticket.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-270's revision property quoted the catalog header as `1-draft.5`, two revisions behind. Fixed: it quotes `1-draft.7`. | FR-270; native-diagnostics.md |
| FND-002 | low | The first draft of the row said each I2 kind "carries the cause at the same position in that list". "That list" could refer to the list of kinds instead of the list of causes. Fixed: the row says the kinds carry "the seven causes above, in the same order". | native-diagnostics.md |
| FND-003 | low | FR-272 had no `stage_limit_exceeded` or `duplicate_selection` row, FR-271 listed neither code and its revision history ended at `1-draft.5`, and no test pinned either cause set. All date from `1-draft.6`. Fixed: FR-272 rows and FR-272-AC-10, FR-271 value rows and history through `1-draft.7`, the complete-V1 matrix row, and `tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed`, which requires both documents' rows to equal one closed list. | FR-271; FR-272; TC-233; native-diagnostics.md |
| FND-004 | low | S1's token ceiling (`SyntaxLimit::Tokens`) had no cause. Fixed: `token-count-exceeded`, revision `1-draft.7`. I1 domain-package intake limits (`FamilySteps`, `AncestorSteps`) still have no cause; whether I1 intake is a stage-limit surface or a caller budget is a design question for a separate ticket. | native-diagnostics.md |
| FND-005 | low | `duplicate_selection` and `stage_limit_exceeded` are absent from the pinned compiler inventory (QSL `f444d03` `src/diagnostic.rs`), but the catalog-owned-spellings paragraph named only the `1-draft.3` and `1-draft.5` codes. By the catalog's own rule the two `1-draft.6` codes were then unselected. Fixed: the paragraph names them as catalog-owned spellings of `1-draft.6`. | native-diagnostics.md |
| FND-006 | low | The `1-draft.6` paragraph gives `stage_limit_exceeded` "one cause per closed limit kind". With eight causes, QSL's `LimitKind` (four variants in QSL FR-096) must grow to match before QSL claims `1-draft.7`. That is QSL-side work (QSL-236); no QSpec change. Recorded. | native-diagnostics.md |

## Checklist

| Gate | Result |
| --- | --- |
| ID formats | FR-272-AC-10 is the next free FR-272 AC; SR-623 is this review. No other id minted. |
| Link integrity | No link added. |
| Cause spellings | The four new causes follow the `<noun>-count-exceeded` form of `node-count-exceeded`. Each is distinct from every other cause in the catalog. |
| Revision discipline | The header, the row's revision marker, the `1-draft.7` paragraph, FR-270's revision cell, FR-271's history, FR-272's row and AC-10, and the header test all name `1-draft.7`. |
| Coverage | FR-272-AC-10 maps to TC-233 in `spec/complete-v1/tests.md`; `tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed` traces it. The header test pins `1-draft.7`. |
| Validation | `quire validate` reports 142 failed documents, the same count as `main` at `e72756f`. FR-270's findings are identical to those on `main`. |
