---
id: SR-200
title: "Base review of settled truth on an open subject"
type: SpecReview
analysis: base
scope: "FR-061, TC-061, TM-004 and proposals/quire-v1/protocol-contract.md"
review_set: all
evaluated_revision: "cb2748d"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
---
## Summary

The B-owned result contract was reviewed against the integrated temporal and
observation drafts at `0b0057b`. The original table conflated completion of the
assessment, closure of the assessed subject and completeness of the facts that
support a decision. The change at `cb2748d` separates those concepts, records a
typed settlement basis and makes decisive satisfied and violated results on an
open subject independently testable.

## Verdict

**PASS for the B-owned contribution.** No B-owned blocking finding remains.
The composed baseline is not ready for adoption until A updates its exact
definition/checksum snapshot and F's consumer contract explicitly preserves the
new dimensions. Those are integration findings for their existing editors, not
permission to edit their artifacts here.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: assessment execution, subject closure and completeness were one overloaded condition, which incorrectly delayed decisive temporal truth until whole-execution closure. | FR-061 result table; FR-091-AC-4 | wrong-requirement |
| FND-002 | high | Resolved: settled truth had no typed basis or exact decision-support set, so a reader could not distinguish a valid early witness from optimistic success. | FR-061-AC-3, FR-061-AC-8; TC-061 | missing-requirement |
| FND-003 | medium | Resolved: late contradictions now create linked superseding or invalidating results while preserving prior bytes and identities. | FR-061-AC-9; TC-061; FR-094-AC-5 | missing-requirement |
| FND-004 | high | Open integration gate: A's `protocol-finite` definition and checksum census still pin the earlier FR-061/protocol-contract bytes and explicitly label this seam unresolved. A must mint the next candidate revision and digests after integration. | proposals/quire-v1/definitions/protocol-finite.md; definitions/README.md | correct-requirement-no-evidence |
| FND-005 | high | Open integration gate: F's FR-115 and TC-145 enumerate the old result dimensions and could permit a consumer to erase subject closure or settlement basis. F must extend its owned consumer requirement and case before the composed review. | FR-115; TC-145 | missing-requirement |

## Checklist results

| Check | Result |
| --- | --- |
| IDs and uniqueness | FR-061, TC-061 and FR-061-AC-1 through AC-9 retain valid unique identifiers. |
| Requirement form | FR-061 names its subject, typed inputs and output, strict-reader behavior, failure paths and exact dependencies. Quire reports no EARS or quality warning for the edited artifacts. |
| Test coverage | TM-004 has one truthful planned row for each of nine criteria; TC-061 names positive, negative, one-axis mutation and result-history cases. |
| Cross-family agreement | FR-091/FR-094 now have typed dependency edges. A's definitions and F's explicit consumer list remain named integration gates. |
| Completion honesty | Every case remains planned. The change claims no wire implementation, compiler support, qualification evidence or adopted profile. |

## Review set

| Analysis | Artifact |
| --- | --- |
| base | [base](./base.md) |
| failure-domain | [failure-domain](./failure-domain.md) |
| integrity | [integrity](./integrity.md) |
| dependency | [dependency](./dependency.md) |
| evidence | [evidence](./evidence.md) |
| risk-complexity | [risk-complexity](./risk-complexity.md) |
| scope-boundary | [scope-boundary](./scope-boundary.md) |
| ears-conformance | [ears-conformance](./ears-conformance.md) |
