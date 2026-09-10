---
id: FR-115
title: "Report observation adequacy without promoting missing evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When emitting an observation contribution to an assessment result, the result adapter SHALL preserve requested capability, execution, truth, activation, participation, completeness, and provenance as separate facts.

## Behavior

An untriggered compensator, absent external observation, unsupported obligation, or missing trigger telemetry SHALL NOT count as demonstrated success or adequate coverage. A confirmed fault and independently observed mitigation SHALL retain failure-mode, property, binding, and run lineage through the B-owned result contract; this requirement creates no second evidence store.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-115-AC-1 | Healthy, violating, untriggered, missing-data, and unsupported traces remain visibly distinct. | Test (TC-145) |
| FR-115-AC-2 | Dropped trigger telemetry and mis-correlated response cannot improve adequacy. | Test (TC-145) |
| FR-115-AC-3 | Confirmed fault/mitigation lineage retains source, binding, property, and run identities. | Test (TC-145) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- B owns the shared result/adequacy contract and Quoin retains evidence authority.
