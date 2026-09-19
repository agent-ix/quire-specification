---
id: TC-266
title: "SMT backend registration and capability advertisement"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-349
    type: verifies
---

# TC-266: SMT backend registration and capability advertisement

## Description

Register the SMT backend (cvc5, with Z3 cross-check) under the AD-010
provider contract and negotiate quantification, induction, composition and
mathematical-integer claims against its advertised capabilities.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-349. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-349; the SMT backend registers
under the same exact provider contract as Kani; a claim within its advertised
capability subset settles `proved`, `refuted` with a replayable witness or
counterexample, or `inconclusive`; a claim outside the advertised subset is
not dispatched to it; a cvc5/Z3 verdict disagreement is retained as an
explicit parity failure; a claim no registered backend can discharge settles
`unsupported` with a warning naming the required capability; each mutation
returns its exact typed disposition with source/provenance and preserves
independent sibling results.
