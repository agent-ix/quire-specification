---
id: TC-033
title: "Admit total typed reusable Boolean predicates"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-033
    type: verifies
---
## Description

Planned acceptance controls for [FR-033](../functional/foundation/FR-033-admit-reusable-predicates.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Define a typed Boolean predicate and two valid callers under the proposed reusable-predicate extension. Independently change argument count/type/unit and root type. Add a self-call and then an indirect two-declaration cycle. Exercise guarded and unguarded optional access. Offer the valid call under the historical no-call definition.

## Expected Results

Valid calls share declaration identity while preserving their own loci. Invalid calls/cycles/partial bodies refuse with typed causes. No monitor or external value is implicitly read. The historical profile still refuses user-defined calls. Concrete syntax is bound to the reviewed L1 grammar before executable tests are implemented.
