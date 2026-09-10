---
id: TC-056
title: "Register compensation after effect success"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-056
    type: verifies
---
## Description

Verify effect-dependent, idempotent compensation registration and ordering.

## Test Procedure

Present one successful payment effect, replay its observation, then replace it with failed attempts and duplicate deliveries. Present a refund before the forward effect.

## Expected Results

Exactly one effect-bound registration is created in the valid/replayed case. Attempt/delivery-only cases create none; early compensation reports an ordering violation and cannot repair registration retrospectively.

