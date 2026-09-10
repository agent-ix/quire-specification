---
id: SR-210
title: "Failure-domain review of the protocol-result verification projection"
type: SpecReview
analysis: failure-domain
scope: "FR-061, FR-062, TC-061 and TC-064"
review_set: all
evaluated_revision: "07da6927202719c81a348b7c0e09677d8810f1a2"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-062, type: reviews }
---
## Summary

The failure-domain lens exercised invalid source state, digest and identity
substitution, unknown versions/features, unrepresentable target meaning,
dependence-lineage corruption and adequacy cross-wiring. The remediated contract
defines typed refusal before any plausible `TechniqueResult` is emitted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the canonical result initially lacked exact oracle/observation dependence and lineage, so the adapter could invent independence. The source now carries closed relations and refuses missing, self-referential or cyclic lineage. | FR-061-AC-7; TC-061 |
| FND-002 | high | Resolved: unknown source/target revisions and a target unable to represent one FR-007 finding now return stable typed refusals with no output result. | FR-062-AC-4; TC-064 |
| FND-003 | medium | Resolved: source bytes, target binding members and adapter versions are independently mutated; the strict reader recomputes the source digest and the projection identity changes for each semantic dependency. | FR-061-AC-3; FR-062-AC-2 |
| FND-004 | low | No remaining callback, purity or termination gap: predicates stay under the existing pure protocol boundary and NFR-020 bounds caller-controlled collections and traversals. | NFR-020; protocol contract |

## Failure behavior

The adapter is strict rather than resilient: invalid input or projection loss
aborts that projection with a typed refusal. Other independently requested
protocol results remain retained under FR-061; failure never becomes a Boolean
success or a sufficiency decision.

