---
id: SR-215
title: "Scope review of the protocol-result verification projection"
type: SpecReview
analysis: scope-boundary
scope: "FR-061, FR-062 and IT-021"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-062, type: reviews }
---
## Summary

Responsibility is allocated without duplicating the compiler, E02 reader,
Engineering Assurance runner or Quoin evidence system. B owns canonical protocol
result meaning and its outgoing E02 projection; external owners keep their
authoritative interfaces.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the first mapping draft did not name an implementation owner. FR-062 now assigns the future first-party Rust `quire-protocol` library and explicitly prohibits substitute adapters in other repositories. | FR-062 ownership paragraph |
| FND-002 | low | No remaining boundary duplication was found: quire-verification owns E02/assessment, Engineering Assurance owns execution transport, Quoin owns evidence, and A/E/F remain source/profile/temporal/observation authorities. | FR-062; IT-021; protocol contract |

## Allocation

| Responsibility | Owner | Class |
| --- | --- | --- |
| Canonical protocol result and strict reader | future `agent-ix/quire-protocol` Rust library | core |
| Protocol-to-E02 projection | future `agent-ix/quire-protocol` Rust library | core integration |
| E02 schema and evidence assessment | `agent-ix/quire-verification` | external guaranteed contract |
| Plan/task/run execution context and typed transport | Engineering Assurance Rust module | external guaranteed contract |
| Evidence authority and method registry | Quoin | external authority |
| Native language/profile and observation/temporal inputs | A/E/F-owned interfaces | external guaranteed contracts |

Setup authorization and contract adoption are prerequisites. Their absence does
not authorize a local runner, JavaScript/Python adapter, evidence store or E02
fork.
