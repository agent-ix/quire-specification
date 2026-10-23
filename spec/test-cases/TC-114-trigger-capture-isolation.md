---
id: TC-114
title: "Isolate temporal triggers and immutable captures"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-093
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-479
    type: verifies
---
## Description

Accepted FS04 portable activation and capture controls using two concurrent
order failures and their related payment/refund effects.

## Test Procedure

Activate two instances with equal captured amounts but distinct trigger/payment
identities. Deliver the same trigger receipt twice, then deliver two distinct
triggers with equal payloads. Change mutable order/payment state after
activation. Independently substitute wrong-order refunds, a capture from the
other instance, nullable/wrong-type capture values, stale model/source bindings
and a foreign anchor. Repeat with a false activation guard, an unestablished
guard, and two deliveries for one semantic trigger identity carrying conflicting
payloads. For every instance, retain and mutate independently the qualified
definition, activation relation row, workflow instance, clause subject, origin,
checked activation result and evaluation anchor; offer a binding whose row the
package's activation relation does not hold, a `trigger_event` origin on an
`origin` row, a binding with each required component absent, and the same key
with unequal checked content; attempt a caller-built substitute from a
timestamp, display name, binding coordinate and capture value.

## Expected Results

Duplicate delivery creates no duplicate obligation; distinct semantic triggers
do. Captures remain unchanged and instance-scoped. Wrong, absent, partial,
stale or cross-instance inputs cannot activate or satisfy an obligation and
produce no temporal Boolean. False guards create no instance, unestablished
guards remain incomplete/refused, and conflicting same-trigger deliveries refuse
without replacing any capture.
Every accepted instance has exactly one complete FR-479 binding. A row the
relation does not hold refuses `missing_declaration`/`missing-name`; a
mismatched origin kind and unequal content under one key refuse
`invalid_runtime_input`/`conflicting-identity`; an absent component refuses
`invalid_runtime_input`/`missing-member`; no caller-built substitute produces
an instance.
