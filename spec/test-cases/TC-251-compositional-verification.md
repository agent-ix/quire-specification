---
id: TC-251
title: "Compositional verification obligation negotiation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-348
    type: verifies
---

# TC-251: Compositional verification obligation negotiation

## Description

Compose independently verified component obligations into one system-level
verdict, and refute an interface mismatch between a component and the
composition rule.

## Test Procedure

Use the subject and inputs named by FR-348. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-348; matching independently
verified component obligations compose into the system-level verdict without
re-verifying the composed whole; a mismatching interface refutes, naming the
component and interface; a claim no registered composition-capable backend can
discharge settles `unsupported` with a warning naming the required capability;
each mutation returns its exact typed disposition with source/provenance and
preserves independent sibling results.
