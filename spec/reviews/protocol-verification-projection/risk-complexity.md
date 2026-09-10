---
id: SR-214
title: "Risk review of the protocol-result verification projection"
type: SpecReview
analysis: risk-complexity
scope: "FR-061 and FR-062"
review_set: all
evaluated_revision: "07da6927202719c81a348b7c0e09677d8810f1a2"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-062, type: reviews }
---
## Summary

FR-061 is high technical risk because it combines distributed protocol state,
temporal settlement and immutable identity. FR-062 is medium technical risk and
high volatility because its target E02 contract remains provisional. Exact
version pins, strict refusal and discriminating Rust tests are the selected
mitigations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Accepted risk: `quire-verification/e02-draft-2` may evolve. FR-062 refuses unknown target versions and isolates the target mapping so a later contract revision becomes a new reviewed mapping rather than silent drift. | FR-062-AC-1/AC-4 |
| FND-002 | medium | Accepted delivery risk: the intended Rust repository and shared Engineering Assurance interface are not yet available. The scope records them as prerequisites and forbids a local substitute. | FR-062 ownership; IT-021 |

## Risk register

| Requirement | Technical risk | Volatility | Driver | Mitigation |
| --- | --- | --- | --- | --- |
| FR-061 | High | Medium | Distributed/temporal state, dependence lineage, immutable supersession | Closed result algebra, strict reader, property/model tests and exact digests |
| FR-062 | Medium | High | Cross-repository provisional E02 contract | Exact version pin, total table, typed refusal and separate adapter task |

## Top hazards

1. A one-axis mapping loss that changes an FR-007 finding.
2. Treating protocol adequacy as observation adequacy or evidence sufficiency.
3. Reusing a derived E02 result as the canonical protocol result.

