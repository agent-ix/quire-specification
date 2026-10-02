---
id: TC-269
title: "Proved settlement trust basis"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-354
    type: verifies
---

# TC-269: Proved settlement trust basis

## Description

Record `certificate-checked` when Carcara passes the cvc5 Alethe certificate
for the exact settled query, `solver-asserted` otherwise, and `trusted` for a
plugin's `proved` result, without changing the `proved` verdict.

## Test Procedure

Use the subject and inputs named by FR-354. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| TB-01 | Present Carcara with a certificate whose query identity (digest over query bytes plus solver options) does not match the settled claim's query identity, including a certificate that Carcara itself passes as internally valid | Records `solver-asserted`, never `certificate-checked`; the mismatched query identity is named |
| TB-02 | Run a settlement with Carcara not registered | Records `solver-asserted` with a warning naming the absent certificate-checking capability; the `proved` verdict is unaffected |
| TB-03 | A plugin returns `proved` for one item; the SMT backend returns `proved` for a second item with no certificate | The first settles `proved` with basis `trusted` and the plugin's backend identity, and its rendered outcome shows `trusted` beside the verdict; the second records `solver-asserted`, never `trusted` |

## Expected Results

Every positive and boundary result matches FR-354; a `proved` settlement
always carries exactly one of `certificate-checked`, `solver-asserted` or
`trusted`; a plugin's `proved` result records `trusted` with its backend
identity;
`certificate-checked` requires Carcara to have checked the cvc5 Alethe
certificate for the exact settled query identity (a digest over the query
bytes plus solver options) and passed it, and retains the Carcara run
identity; a certificate whose query identity does not match never yields
`certificate-checked`; a missing certificate, a failing Carcara check or a
foreign-query certificate records `solver-asserted` without changing the
`proved` verdict; a recorded basis changes only through a new Carcara check
run producing its own new record, distinguishing a basis never checked from
one checked and rejected; Carcara's absence records `solver-asserted` with a
warning naming the absent capability; the Z3 cross-check result is recorded
alongside the trust basis as an independent signal; each mutation returns its
exact typed disposition with source/provenance and preserves independent
sibling results.
