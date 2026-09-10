---
id: TC-062
title: "Enforce every protocol resource boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-020
    type: verifies
---
## Description

Verify exact-boundary acceptance and one-over-boundary resource-incomplete outcomes without executing business actions.

## Test Procedure

For every structural, role, message, branch, iteration, active-instance, observation and result limit, run one case exactly at the declared maximum and one exceeding it by the smallest representable amount. Mutate each comparison in both directions and substitute an action-capable test double.

## Expected Results

Every exact-boundary case remains eligible, every excess case returns a typed resource-incomplete result, both comparison mutants fail, and the action-capable double receives zero calls.

