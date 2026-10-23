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
mathematical-integer obligations against its advertised FR-290 (capability
kind, mode) pairs and its negotiation arm.

## Test Procedure

Use the subject and inputs named by FR-349. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| SMT-01 | Negotiate an item whose required capability kind (e.g. `temporal-satisfaction`) the registered SMT backend does not advertise, while it advertises others (e.g. `value-validity`) | Not a candidate for it; with no other registrant, settles `unsupported` with a warning naming the required kind, unaffected by the backend's other advertised kinds |
| SMT-02 | Run cvc5 and Z3 on the same query, once for each row of FR-349's cross-check table: a certified and an uncertified cvc5 `proved`, a replayed cvc5 counterexample and a cvc5 `inconclusive`, each with Z3 `proved`, `refuted` and `inconclusive` | Each pair settles as its table row states: a certified proof stays `proved`, `certified` with `cross_check` naming Z3's verdict; an agreeing uncertified proof keeps `cross_check` verdict `proved`; every other disagreement settles `inconclusive`, `cross-check-disagreement{tool, primary, cross_check}` naming both verdicts; an agreeing counterexample settles `refuted` with no `cross_check`; two `inconclusive` verdicts keep cvc5's cause; neither verdict is silently preferred |
| SMT-03 | Run a claim within cvc5's advertised subset with Z3 not registered | cvc5's verdict settles the claim on its own, with a warning naming the absent cross-check; the settlement is not held pending Z3's registration |
| SMT-04 | Negotiate a `value-validity` item carrying an obligation form (e.g. induction) its negotiation arm does not discharge, outside the descriptor's declared obligation-form set (e.g. quantification only) | Settles `unsupported` at the SMT negotiation arm with a warning naming the obligation form and cause `unsupported_projection`/`unsupported-requested-capability`; no artifact is emitted |
| SMT-05 | Configure Z3 in the SMT descriptor, then remove its tool from `PATH` before a claim within cvc5's advertised subset | cvc5's verdict settles the claim, with a warning naming the absent cross-check; the item's result is never `unsupported` on Z3's account |

## Expected Results

Every positive and boundary result matches FR-349; the SMT backend registers
under the same exact provider contract as Kani; a claim within its advertised
(capability kind, mode) pairs and discharged obligation forms settles `proved`, `refuted` with a replayable witness or
counterexample, or `inconclusive`; an item whose kind it does not advertise is
not a candidate for it; each cvc5/Z3 pair settles as FR-349's cross-check
table states, a disagreement over anything but a certified proof settling
`inconclusive` with an explicit parity failure naming both verdicts; Z3's absence is a warned outcome, never a hold, on cvc5's own
settlement; a claim no registered backend can discharge settles `unsupported`
with a warning naming the required capability kind; when a registered
quantifier-capable, induction-capable or mathematical-integer SMT backend is
present, this corpus also settles at least one FR-346 `forall`/`exists`
obligation, one FR-347 inductive obligation and one FR-147 mathematical-
integer obligation each `proved` and each `refuted` through that backend;
each mutation returns its exact typed disposition with source/provenance and
preserves independent sibling results.
