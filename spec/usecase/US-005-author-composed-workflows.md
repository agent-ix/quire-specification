---
id: US-005
title: "Describe and assess a distributed workflow in one language"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-020
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

## Dependencies (Contextual)

- [V1 capability decisions](../../proposals/quire-v1/capabilities.md).
