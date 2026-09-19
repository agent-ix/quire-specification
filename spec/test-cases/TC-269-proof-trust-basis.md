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
for the exact settled query, and `solver-asserted` otherwise, without changing
the `proved` verdict.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-354. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-354; a `proved` settlement
always carries exactly one of `certificate-checked` or `solver-asserted`;
`certificate-checked` requires Carcara to have checked the exact query's cvc5
Alethe certificate and passed it, and retains the Carcara run identity; a
missing certificate or a failing Carcara check records `solver-asserted`
without changing the `proved` verdict; neither basis is silently upgraded or
downgraded; the Z3 cross-check result is recorded alongside the trust basis
as an independent signal; each mutation returns its exact typed disposition
with source/provenance and preserves independent sibling results.
