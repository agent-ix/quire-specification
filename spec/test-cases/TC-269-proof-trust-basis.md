---
id: TC-269
title: "Proved settlement certification"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-354
    type: verifies
---

# TC-269: Proved settlement certification

## Description

Record `certified` when the core's Alethe checker verifies the cvc5
certificate against the core's own queries, `uncertified` when the
certificate is absent or unverifiable, `inconclusive` with
`certificate-rejected{rule, at}` when it is shown wrong, and `trusted` for a plugin's
`proved` result.

## Test Procedure

Use the subject and inputs named by FR-354. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| TB-01 | A certificate whose queries equal the core's encoding and whose proofs use checked rules | `proved`, `certified` |
| TB-02 | The same proof with no certificate; then the certificate with one step's rule replaced by `hole` | `proved`, `uncertified`, both times |
| TB-03 | The certificate with its query encoded for another depth, internally valid as a proof | `inconclusive`, `certificate-rejected{rule, at}` naming the query mismatch, never `proved` |
| TB-04 | A plugin returns `proved` for one item; the SMT backend returns `proved` for a second item with no certificate | The first settles `proved` with certification `trusted` and the plugin's backend identity, and its rendered outcome shows `trusted` beside the verdict; the second settles `proved`, `uncertified` |
| TB-05 | TB-01's certificate with one step's premise replaced by a later step | `inconclusive`, `certificate-rejected{rule, at}` naming the invalid step |
| TB-06 | TB-01's certificate with its last step removed, so the proof ends short of the empty clause | `inconclusive`, `certificate-rejected{rule, at}` naming the last step |
| TB-07 | A bounded-complete certificate offered for an `inductive` proof | `inconclusive`, `certificate-rejected{rule, at}` naming the shape mismatch |
| TB-08 | TB-01 run with the Z3 cross-check agreeing, then disagreeing | Both settle `proved`, `certified`; the cross-check result is recorded beside the certification and never changes it |
| TB-09 | An inductive certificate whose `Step` proof has an invalid step | `inconclusive`, `certificate-rejected{rule, at}` naming `ProofStepInvalid` at that `Step` step |
| TB-10 | TB-01's certificate with one step's rule replaced by `lia_generic` and no other defect | `proved`, `uncertified` |

## Expected Results

Every positive and boundary result matches FR-354; every SMT `proved`
result carries exactly one of `certified` or `uncertified`, and every plugin
`proved` result carries `trusted` with its backend identity; a certificate
for another query, an invalid step, a short proof or a shape mismatch settles
`inconclusive` with `certificate-rejected{rule, at}`; the Z3
cross-check result is recorded alongside the certification as an
independent signal; each mutation returns its exact typed disposition and
preserves independent sibling results.
