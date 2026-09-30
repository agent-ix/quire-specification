---
id: SR-554
title: "Object review of FS06 output mapping"
type: SpecReview
analysis: base
scope: "FR-269 and FR-297–299 identity, absence, ordering and digest contracts"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-269, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-297, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-298, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-299, type: reviews }
---
# Object review of FS06 output mapping

## Summary

PASS. The selected profile, per-obligation record and generated package have
closed identities, explicit absence rules and non-interchangeable digest domains.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | high | Resolved: target family/version and mapping-rule revision were not part of the old loss record. FR-297 is now a complete value and FR-298 includes it in derived record identity. | FR-269; FR-297; FR-298 |
| FND-010 | high | Resolved: absent disposition/condition/cause/region members could admit an implied preservation result. FR-298 defines state-specific invariants and missing required members invalidate assembly. | FR-298; FR-121 |
| FND-011 | medium | Resolved: equal target text could collapse distinct source obligations. Record identity includes source/dependency/profile facts, and package identity binds the complete ordered record set. | FR-298; FR-299 |
| FND-012 | medium | Resolved: target-byte and package digests could be confused. FR-299 uses `sha256-raw` for bytes and `sha256-jcs` over a versioned closed package preimage. | FR-201; FR-299 |

## Object completeness

- FR-297 states identity kind, full key, order, causal order, absence and digest.
- FR-298 states source/dependency/target/disposition members and every
  disposition invariant, including qualified adequacy.
- FR-299 states aggregate members, record containment, region bounds,
  deterministic assembly and failure absence.
