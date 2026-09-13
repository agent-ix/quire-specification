---
id: TC-051
title: "Discriminate delivery, attempt and effect identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-210
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-224
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-225
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-226
    type: verifies
---
## Description

Verify that transport observations cannot manufacture business effects or activate effect-dependent obligations.

## Test Procedure

Bind one send, two exact delivery keys, one payment attempt and one successful
payment effect. Separately bind two receipt records to one delivery key, replay
one complete effect key, and bind a second exact effect key to the same singleton
effect occurrence and establishing attempt. Relabel a delivery as an effect,
bind an effect to the wrong attempt, rebind one delivery/attempt/effect key to
different relationship facts, remove the effect, substitute its trace ID for
each business identity, permute keys and reverse timestamp/arrival order.

## Expected Results

The valid case retains 1/2/1 message-delivery-effect cardinality; two receipts
for one delivery retain delivery cardinality one. Exact replay creates no new
effect, while the second valid effect key yields the duplicate-effect violation.
Relabeling, wrong-attempt binding, incompatible rebinding or identity
substitution refuses exactly the affected relation; key order creates no FIFO or
causal relation, and the attempt-only case registers no compensation.
