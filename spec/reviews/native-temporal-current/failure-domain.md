---
id: SR-301
title: "Failure-domain review of native temporal lifecycle correspondence"
type: SpecReview
analysis: failure-domain
scope: "FR-090 through FR-095, NFR-040 and TC-110 through TC-118"
review_set: all
evaluated_revision: "2f347b8"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
---
## Summary

The review exercised identity confusion, premature settlement, absent samples,
open trigger scopes, false closure, regressing progress, missing decision facts,
late contradictions and resource stops. E-owned outcomes are now typed and each
critical boundary has a planned discriminator or one-axis mutation.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: open or incomplete trigger scopes can no longer masquerade as inactive obligations. | FR-093-AC-5; TC-116 | wrong-requirement |
| FND-002 | high | Resolved: open-scope truth requires the matching decisive basis, complete exact support and proof over every admitted continuation; scope closure cannot close the surrounding execution. | FR-091-AC-4; FR-094-AC-3/7; TC-116 | missing-requirement |
| FND-003 | medium | Resolved: runtime sample absence is incomplete, while malformed fixed-sample definition/binding is refused; neither becomes a false atom or removed position. | FR-090; TC-113 | wrong-requirement |
| FND-004 | medium | Resolved: progress regression and revision conflict cannot roll back, restamp or overwrite prior results. | FR-094-AC-8; TC-115 | correct-requirement-no-evidence |
| FND-005 | medium | Retained boundary: late data that contradicts a premise creates a linked superseding/invalidating result; automatic repair and byte rewriting remain forbidden. | FR-094-AC-5; TC-116 | correct-requirement-no-evidence |

## Failure-domain checks

| Domain | Required disposition |
| --- | --- |
| Missing profile/binding | Located refusal before evaluation. |
| Missing runtime valuation/sample/history | Incomplete or unavailable, never Boolean padding. |
| Undecided future | Pending with `unsettled`, not optimistic truth. |
| Open decisive future | Matching decisive basis plus complete exact support; whole execution may remain open. |
| Trigger not observed | Inactive only for closed-complete trigger scope; otherwise activation-unknown. |
| Progress rollback/conflict | Contradiction/refusal preserving earlier progress, closure, identity and bytes. |
| Resource exhaustion | Incomplete/refused with exact limit identity and no Boolean fallback. |
