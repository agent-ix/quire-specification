---
id: TC-055
title: "Separate activation from obligation truth"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-055
    type: verifies
---
## Description

Verify exact trigger/capture binding and distinguish inactive from activation-unknown.

## Test Procedure

Assess a failed shipment after E1, a successful shipment under closed complete input, and a missing provider trigger under open/incomplete input. Substitute O2's trigger, captured amount and evaluation anchor independently.

## Expected Results

The first case activates the matching captured refund; the second is inactive; the third is unknown/incomplete. Each substitution refuses and no inactive/unknown case becomes satisfied or demonstrated.
