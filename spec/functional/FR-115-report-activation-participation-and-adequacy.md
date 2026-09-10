---
id: FR-115
title: "Report observation adequacy without promoting missing evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When emitting an observation contribution to an assessment result, the result
adapter SHALL preserve requested capability, assessment execution,
subject-progress/closure, truth, settlement basis, exact decision support,
activation, participation, completeness, adequacy, claim strength, provenance,
and immutable definition/profile dependencies as separate facts.

## Behavior

An untriggered compensator, absent external observation, unsupported obligation,
or missing trigger telemetry SHALL NOT count as demonstrated success or as an
adequacy result whose required activation and participation evidence is present.
A confirmed fault and independently observed mitigation SHALL retain failure-mode,
property, binding, and run lineage through the B-owned result contract; this
requirement creates no second evidence store. When the result adapter emits a
settled truth on an open subject, it SHALL retain its `closed-scope`,
`decisive-witness`, `decisive-counterexample`, `unsettled`, or `unavailable`
basis and every exact supporting fact/progress identity. When a later
contradiction is admitted, the result adapter SHALL retain earlier result bytes
and emit only a linked superseding or invalidating result under the corrected
input identity.
The adapter SHALL retain every selected definition/profile identity, revision,
and digest that its contribution depends on; it SHALL NOT replace one with a
current default or a compatible-looking definition.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-115-AC-1 | Healthy, violating, untriggered, missing-data, and unsupported traces remain visibly distinct. | Test (TC-145) |
| FR-115-AC-2 | Dropped trigger telemetry and mis-correlated response cannot improve adequacy. | Test (TC-145) |
| FR-115-AC-3 | Confirmed fault/mitigation lineage retains source, binding, property, and run identities. | Test (TC-145) |
| FR-115-AC-4 | Assessment execution, subject-progress/closure, truth, settlement basis, exact decision support, activation, participation, completeness, adequacy, claim strength, and provenance remain independently readable. | Test (TC-145) |
| FR-115-AC-5 | A settled truth on an open subject retains its decisive basis and exact support; a late contradiction retains prior bytes and emits only a linked superseding or invalidating result. | Test (TC-145) |
| FR-115-AC-6 | Every selected definition/profile identity, revision, and digest remains immutable at the observation-result boundary; drift or substitution is refused. | Test (TC-145) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- [B's reviewed FR-061 result contract](ix://agent-ix/quire-specification/FR-061) owns assessment execution, subject-progress/closure, settlement basis, decision support, and result-combination semantics.
- [B's FR-007 result contract](ix://agent-ix/quire-verification/FR-007) owns independent execution, activation, observation, participation, assumption, logical, and provenance findings; Quoin retains evidence authority.
