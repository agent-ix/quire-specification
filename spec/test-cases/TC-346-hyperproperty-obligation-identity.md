---
id: TC-346
title: "Hyperproperty claim body and obligation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-402
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-191
    type: verifies
---

# TC-346: Hyperproperty claim body and obligation identity

## Description

Check the hyperproperty claim node body and that every meaning-changing member of a clause changes its obligation identity.

## Test Procedure

Compile each vector through the real local Rust boundary, read the hyperproperty claim node body, and compute each item's obligation identity; compare identities pairwise.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HI-01 | TC-337 HB-01 | body bindings `domain`, `subjects`, `parameters`, `quantifiers`, `match`, `align_skip`, `body` in order; `subjects` holds `V` once |
| HI-02 | `Opaque` of TC-339 and the same clause with its prefix reversed to `exists trace b of V forall trace a of V` | different identities |
| HI-03 | PF-01 and PF-02 of TC-341 | different identities |
| HI-04 | HB-02 and HB-02 with the aliases of its two variables swapped | different identities |
| HI-05 | HB-01 and HB-01 with a match conjunct removed; HB-01 with and without `align skip` | different identities |
| HI-06 | two instances of HB-02 with different universe keys | different identities |
| HI-07 | HB-01 and HB-01 with different whitespace and comments | the same identity |

## Expected Results

Identity separates every pair of clauses with different meanings and joins clauses that differ only in spelling.
