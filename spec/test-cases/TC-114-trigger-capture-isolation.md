---
id: TC-114
title: "Isolate temporal triggers and immutable captures"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-093
    type: verifies
---
## Description

Accepted FS04 portable activation and capture controls using two concurrent
order failures and their related payment/refund effects. TM-004 remains Planned.

## Test Procedure

Activate two instances with equal captured amounts but distinct trigger/payment
identities. Deliver the same trigger receipt twice, then deliver two distinct
triggers with equal payloads. Change mutable order/payment state after
activation. Independently substitute wrong-order refunds, a capture from the
other instance, nullable/wrong-type capture values, stale model/source bindings
and a foreign anchor. Repeat with a false activation guard, an unestablished
guard, and two deliveries for one semantic trigger identity carrying conflicting
payloads.

## Expected Results

Duplicate delivery creates no duplicate obligation; distinct semantic triggers
do. Captures remain unchanged and instance-scoped. Wrong, absent, partial,
stale or cross-instance inputs cannot activate or satisfy an obligation and
produce no temporal Boolean. False guards create no instance, unestablished
guards remain incomplete/refused, and conflicting same-trigger deliveries refuse
without replacing any capture.
