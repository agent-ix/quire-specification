---
id: SR-623
title: "Base review of the I2 reader stage-limit causes"
type: SpecReview
analysis: base
scope: "STD-95 branch task/95-i2-limit-causes: native-diagnostics revision 1-draft.7 (`stage_limit_exceeded` causes `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`), the FR-270 revision cell and the tests/checked_package_v2.rs header test"
review_set: base
evaluated_revision: "task/95-i2-limit-causes"
review_date: "2026-09-24"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-270, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-272, type: references }
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

Two wording defects were found and fixed on the branch. Two low findings are
recorded as they are; neither was introduced by this change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-270's revision property quoted the catalog header as `1-draft.5`, two revisions behind. Fixed: it quotes `1-draft.7`. | FR-270; native-diagnostics.md |
| FND-002 | low | The first draft of the row said each I2 kind "carries the cause at the same position in that list". "That list" could refer to the list of kinds instead of the list of causes. Fixed: the row says the kinds carry "the seven causes above, in the same order". | native-diagnostics.md |
| FND-003 | low | FR-272 has no `stage_limit_exceeded` or `duplicate_selection` row, and FR-271's revision history ends at `1-draft.5`. Both gaps date from `1-draft.6`. Revision `1-draft.7` adds no code, so FR-271 needs no change here. FR-272's closed cause sets and a TC backing the seven causes belong with the FR-272 row, in its own change. Recorded, not fixed. | FR-271; FR-272; native-diagnostics.md |
| FND-004 | low | Two other QSL stage limits have no cause at `1-draft.7`: S1's token ceiling (`SyntaxLimit::Tokens`) and I1 domain-package intake limits. This change is scoped to the three I2 kinds. Recorded, not fixed. | native-diagnostics.md |

## Checklist

| Gate | Result |
| --- | --- |
| ID formats | No requirement, AC or TC id minted; SR-623 is this review. |
| Link integrity | No link added. |
| Cause spellings | The three new causes follow the `<noun>-count-exceeded` form of `node-count-exceeded`. Each is distinct from every other cause in the catalog. |
| Revision discipline | The header, the row's revision marker, the `1-draft.7` paragraph, FR-270's revision cell and the header test all name `1-draft.7`. |
| Coverage | `tc_233_native_diagnostics_header_pins_current_revision` pins the header. No AC names the `stage_limit_exceeded` cause set (FND-003). |
| Validation | `quire validate` reports 142 failed documents, the same count as `main` at `e72756f`. FR-270's findings are identical to those on `main`. |
