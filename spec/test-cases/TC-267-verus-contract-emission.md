---
id: TC-267
title: "Verus contract emission from QSL state contracts"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-350
    type: verifies
---

# TC-267: Verus contract emission from QSL state contracts

## Description

Emit Verus `requires`/`ensures`/loop-invariant clauses from a bound QSL state
contract onto a named implementation Rust function and verify them.

## Test Procedure

Use the subject and inputs named by FR-350. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| VER-01 | Request Verus verification of an emitted function whose model-level obligation the registered SMT backend already settled `proved`, with Verus not registered | Keeps the SMT settlement `proved` with its FR-354 certification and records a warning naming the absent Verus verification capability; the item settles `proved` |

## Expected Results

Every positive and boundary result matches FR-350; emitted clauses correspond
exactly to the bound state contract's precondition, postcondition and
invariant; emission targets only a named implementation Rust function; Verus
registers under the same AD-010 provider contract as Kani and the SMT backend;
a refuted clause returns a replayable counterexample; a claim Verus cannot discharge
settles `unsupported` with a warning naming the required capability, or
`requires_bound` when a finite bound is missing; when Verus is not
registered, an already SMT-`proved` claim keeps its settlement and FR-354
certification with a warning naming the absent Verus capability; each mutation returns its exact typed disposition with
source/provenance and preserves independent sibling results.
