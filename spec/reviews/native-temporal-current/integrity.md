---
id: SR-302
title: "Integrity review of native temporal lifecycle correspondence"
type: SpecReview
analysis: integrity
scope: "E temporal requirements, matrix, cases, integration tests and selected definition seams"
review_set: all
evaluated_revision: "2f347b8"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
---
## Summary

The E slice is internally consistent after separating definition admission from
runtime availability and aligning its lifecycle vocabulary with FR-061. Its
requirements, matrix rows and planned cases remain atomic enough to detect each
semantic-axis substitution.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved: FR-090 refusal and FR-091 incompleteness no longer disagree for an absent runtime sample. | FR-090; FR-091; FR-095-AC-6 | wrong-requirement |
| FND-002 | high | Resolved for E: lifecycle/result dimensions named by FR-061 are preserved through FR-094 and FR-095 rather than collapsed into generic closure or formula truth. | FR-061; FR-094; FR-095 | missing-requirement |
| FND-003 | medium | Resolved: the finite exhaustive population closes the quantifier used by FR-095-AC-1 and identifies truthful exhaustion behavior. | FR-095-AC-1; TC-117 | wrong-requirement |
| FND-004 | high | Open external integrity gate: F's consumer artifacts still enumerate the earlier result projection and can erase the dimensions E now supplies. | observation-output-mapping-contract.md; FR-115/116 | wrong-requirement |
| FND-005 | low | Open external integrity gate: A's immutable `1-draft.2` temporal candidate cannot be relabeled with the remediated rule bytes; a successor definition set is required. | definitions/temporal-common.md; definitions/README.md | correct-requirement-no-evidence |

## Integrity checks

| Check | Result |
| --- | --- |
| Unique IDs | SR-300 through SR-307 and new AC-7/AC-8 identifiers are unique in the repository. |
| Matrix closure | FR-094-AC-7/8 and FR-095-AC-7 each have one truthful planned matrix row and named test case. |
| Semantic atomicity | Clock selection, observation availability, activation, assessment execution, both closure axes, settlement and completeness remain independently mutable. |
| Historical identity | Existing profile definition bytes remain immutable; this review does not silently refresh A-owned candidate digests. |
