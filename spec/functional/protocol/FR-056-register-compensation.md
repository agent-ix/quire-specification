---
id: FR-056
title: "Register compensation only after a successful effect"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-051
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-055
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-233
    type: references
  - target: ix://agent-ix/quire-specification/FR-234
    type: references
  - target: ix://agent-ix/quire-specification/FR-237
    type: references
  - target: ix://agent-ix/quire-specification/FR-239
    type: references
---
## Description

When an authored forward effect is established, the conformance evaluator SHALL register exactly the compensation obligations paired with that effect and bind each registration to its role, eligibility conditions, retry policy, commit boundary and recovery relation.

## Inputs

An admitted compensation definition and a successfully established forward
effect with exact workflow/effect identity.

## Outputs

Registered compensation obligations or typed violations/refusals; the evaluator
does not invoke the compensating operation.

## Behavior

The evaluator SHALL NOT register compensation from a send, receive, delivery or
attempt that lacks the required successful effect. Repeated observation of one
effect SHALL NOT create additional registrations. Each registration SHALL retain
the source definition and effect that created it.

The registration key SHALL be the pair of the static compensation-definition
key and the concrete forward-business-effect FR-287 key. The static definition
key is the qualified protocol-definition identity plus the exact
declaration-local compensation handle. Replaying the same complete pair is
idempotent. Pairing one effect with two distinct admitted compensation
definitions creates two distinct registrations. Role, eligibility, retry,
commit and recovery facts remain separately validated bindings; reusing one
registration key with conflicting bindings refuses.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-056-AC-1 | One successful payment effect paired with one admitted refund definition registers one obligation after the effect; pairing the same effect with a second distinct definition registers a second distinct obligation. | Test (TC-056) |
| FR-056-AC-2 | Failed payment attempts and duplicate deliveries register no refund obligation. | Test (TC-056) |
| FR-056-AC-3 | Replaying the same effect identity does not create a second registration. | Test (TC-056) |
| FR-056-AC-4 | Compensation observed before its forward effect produces an ordering violation and cannot retroactively validate registration. | Test (TC-056) |
| FR-056-AC-5 | Registration equality uses exactly the qualified compensation-definition key and concrete forward-effect FR-287 key; a missing/wrong-kind component refuses, replay is idempotent, and conflicting relationship rebinding refuses. | Test (TC-056) |

## Dependencies

- [FR-051](FR-051-preserve-communication-identities.md).
- [FR-055](FR-055-activate-protocol-obligations.md).
