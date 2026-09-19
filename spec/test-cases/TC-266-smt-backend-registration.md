---
id: TC-266
title: "SMT backend registration and capability advertisement"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-349
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-346
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-347
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-147
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

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| SMT-01 | Negotiate a claim requiring a capability (e.g. induction) the registered SMT backend does not advertise, while it is registered for others (e.g. quantification only) | Not dispatched to it; settles `unsupported` with a warning naming the required capability, unaffected by the backend's other advertised capabilities |
| SMT-02 | Run cvc5 and Z3 on the same query and mutate one solver's verdict to disagree with the other | Settles `inconclusive`, retained as an explicit parity failure naming both verdicts; neither verdict is silently preferred |
| SMT-03 | Run a claim within cvc5's advertised subset with Z3 not registered | cvc5's verdict settles the claim on its own, with a warning naming the absent cross-check; the settlement is not held pending Z3's registration |

## Expected Results

Every positive and boundary result matches FR-349; the SMT backend registers
under the same exact provider contract as Kani; a claim within its advertised
capability subset settles `proved`, `refuted` with a replayable witness or
counterexample, or `inconclusive`; a claim outside the advertised subset is
not dispatched to it; a cvc5/Z3 verdict disagreement settles the claim
`inconclusive` and is retained as an explicit parity failure naming both
verdicts; Z3's absence is a warned outcome, never a hold, on cvc5's own
settlement; a claim no registered backend can discharge settles `unsupported`
with a warning naming the required capability; when a registered
quantifier-capable, induction-capable or mathematical-integer SMT backend is
present, this corpus also settles at least one FR-346 `forall`/`exists`
obligation, one FR-347 inductive obligation and one FR-147 mathematical-
integer obligation each `proved` and each `refuted` through that backend;
each mutation returns its exact typed disposition with source/provenance and
preserves independent sibling results.
