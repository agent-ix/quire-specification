---
id: TC-034
title: "Bind cross-family predicates to explicit evaluation environments"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-034
    type: verifies
---
## Description

Accepted portable controls for [FR-034](../functional/foundation/FR-034-bind-cross-family-predicates.md).
No executable test or completed coverage is claimed
by this artifact.

## Test Procedure

Reference one checked amount predicate from a state clause, temporal obligation and protocol guard. Capture an explicit initial amount, then change later observed state. Independently substitute a same-named foreign declaration, an unrelated operation pre-state, and pending monitor status.

## Expected Results

All valid references retain declaration/source/model/anchor identity. Captured arguments stay at their original value. The three invalid substitutions refuse before evaluation rather than rebinding by name or converting status to Boolean.
