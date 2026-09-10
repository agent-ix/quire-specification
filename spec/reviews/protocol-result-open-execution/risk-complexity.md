---
id: SR-205
title: "Risk and complexity review of settled truth on an open subject"
type: SpecReview
analysis: risk-complexity
scope: "FR-061 settlement and result-history semantics"
review_set: all
evaluated_revision: "cb2748d"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
---
## Summary

The change is semantically high risk because an unsound early-settlement rule
can promote incomplete monitoring to success. Complexity remains bounded by
selected finite profiles and exact retained support rather than a generic proof
over arbitrary user formulas.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: settlement requires the selected profile to authorize a closed scope or monotone decisive witness/counterexample; implementation cannot infer decisiveness from seeing a favorable value. | FR-061 result table; FR-061-AC-8 |
| FND-002 | high | Resolved: the exact decision-support set and profile/progress identities make missing-support and stale-rule mutations observable. | FR-061-AC-3, FR-061-AC-4 |
| FND-003 | medium | Retained risk: implementation must encode only the reviewed finite profile rules. General theorem proving, projection and realizability remain outside this result-contract change. | FR-060; protocol-contract.md |

## Risk controls

| Risk | Control |
| --- | --- |
| Optimistic early success | Paired decisive/non-decisive open prefixes and one-axis basis mutations in TC-061. |
| Closure confusion | Separate assessment execution, required-scope closure and whole-execution closure. |
| Stale result repair | New immutable superseding/invalidation relation for contradicted premises. |
| Scope growth | Exact selected profile identity; unsupported analysis remains unavailable or refused. |
