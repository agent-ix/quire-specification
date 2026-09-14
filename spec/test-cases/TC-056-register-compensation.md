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

Present one successful payment effect paired with one admitted compensation
definition, replay the same pair, then pair the same effect with a second
definition. Independently remove or change each static-definition and concrete
effect key member, substitute another subject kind, and rebind one complete
registration key to conflicting role, eligibility, retry, commit and recovery
facts. Replace the effect with failed attempts and duplicate deliveries. Present
a refund before the forward effect.

## Expected Results

The exact pair creates one idempotent registration; two definitions paired with
one effect create two registrations. Missing/wrong-kind key members and
conflicting rebinding refuse. Attempt/delivery-only cases create none; early
compensation reports an ordering violation and cannot repair registration
retrospectively.
