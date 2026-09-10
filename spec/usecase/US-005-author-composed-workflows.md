---
id: US-005
title: "Describe and assess a distributed workflow in one language"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-048
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-036
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-037
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-038
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-039
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-040
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-041
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-042
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-043
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-044
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-045
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-046
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-047
    type: exercises
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-020
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-030
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-031
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-032
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-033
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-034
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-035
    type: exercises
---
## Story

**As a** specification author
**I want** to describe participants, their state conditions, messages, deadlines and recovery obligations together
**So that** I can understand which obligations a distributed workflow met and which remain unproven.

## Context

A payment and fulfillment workflow includes a captured charge amount, a chosen
success/failure branch, parallel work, and an obligation to refund after a
failed effect. A second receipt does not necessarily mean a second charge;
missing observations do not establish that no charge or refund happened.

## Acceptance Examples (Illustrative)

### US-005-EX-1

Given a successful charge and a failed fulfillment branch, the author can follow
the recorded compensation obligation to the matching refund with the captured
amount and its deadline.

### US-005-EX-2

Given an incomplete observation stream or an unobserved branch, the author sees
what is unknown without that result becoming a pass or proof of local realizability.

### US-005-EX-3

Given concurrent orders, split shipments and related payment/refund effects,
the author can state obligations over the declared relationships and a finite
population/window. Unknown membership cannot establish aggregate success.

## Dependencies (Contextual)

- [V1 capability decisions](../../proposals/quire-v1/capabilities.md).
