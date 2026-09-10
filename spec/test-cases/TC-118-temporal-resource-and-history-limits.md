---
id: TC-118
title: "Refuse temporal resource and retained-state exhaustion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
  - target: ix://agent-ix/quire-specification/NFR-040
    type: verifies
---
## Description

Planned property, boundary and mutation controls for temporal work and required
retained state.

## Test Procedure

Generate nested future/past operators and active trigger populations immediately
below, at and above each supplied work, arithmetic, active-instance, capture and
history limit. Force bound overflow, evict a still-required capture/history
entry, restart without the predecessor prefix and change a restoration/limit
identity. Mutate every exhaustion path to return true or false.

## Expected Results

Within-limit cases match independent oracles. Overflow, exhaustion, required
state loss and missing predecessor return incomplete/refused with exact
obligation/limit identity and no Boolean. Required state remains retained until
settlement or explicit incomplete disposition, and every Boolean-fallback
mutation is detected.
