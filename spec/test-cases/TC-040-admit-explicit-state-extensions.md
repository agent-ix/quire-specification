---
id: TC-040
title: "Admit state extensions through exact profile dependencies"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-040
    type: verifies
---
## Description

Planned controls for [FR-040](../functional/FR-040-admit-explicit-state-extensions.md).
No executable test or observed coverage is claimed.

## Test Procedure

Offer identical helper/query/graph forms under the first definition and under symbolic exact extension definitions. Import a predicate into a caller with wider support, then alter a callee dependency, select conflicting definitions, and place a prohibited operation inside an unreachable branch.

## Expected Results

Admission follows each declaration's exact closure. No permission leaks from the caller/backend, and no historical source is reinterpreted. Missing/incompatible dependencies refuse their dependents with typed located causes.

