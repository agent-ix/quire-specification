---
id: TC-114
title: "Isolate temporal triggers and immutable captures"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-093
    type: verifies
---
## Description

Planned activation and capture controls using two concurrent order failures and
their related payment/refund effects.

## Test Procedure

Activate two instances with equal captured amounts but distinct trigger/payment
identities. Deliver the same trigger receipt twice, then deliver two distinct
triggers with equal payloads. Change mutable order/payment state after
activation. Independently substitute wrong-order refunds, a capture from the
other instance, nullable/wrong-type capture values, stale model/source bindings
and a foreign anchor.

## Expected Results

Duplicate delivery creates no duplicate obligation; distinct semantic triggers
do. Captures remain unchanged and instance-scoped. Wrong, absent, partial,
stale or cross-instance inputs cannot activate or satisfy an obligation and
produce no temporal Boolean.
