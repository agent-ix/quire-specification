---
id: TC-250
title: "Inductive verification obligation negotiation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-347
    type: verifies
---

# TC-250: Inductive verification obligation negotiation

## Description

Discharge a base case and inductive step over an unbounded collection and a
recursive value type through a registered induction-capable backend.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-347. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-347; a matching base case and
inductive step over an unbounded or recursive domain proves the obligation; a
failing base case or step refutes with its own counterexample; a claim no
registered induction-capable backend can discharge settles `unsupported` with
a warning naming the required capability; with the FR-349 SMT backend
registered, at least one case settles `proved` and at least one settles
`refuted` through it; each mutation returns its exact typed disposition with
source/provenance and preserves independent sibling results.
