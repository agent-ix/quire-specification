---
id: TC-346
title: "Hyperproperty claim body and obligation identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-402
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-191
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-370
    type: verifies
---

# TC-346: Hyperproperty claim body and obligation identity

## Description

Check the hyperproperty claim body and its temporal-node placements, and that
every meaning-changing member of a clause changes its obligation identity.

## Test Procedure

Compile each valid vector through the real local Rust boundary, read the
hyperproperty claim node body, and compute each item's obligation identity;
compare identities pairwise. Pass misplaced-reference and wrong-subject-target
vectors through the reader and record their exact refusals.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HI-01 | TC-337 HB-01 | body bindings `domain`, `subjects`, `parameters`, `quantifiers`, `match`, `align_skip`, `body` in order; `subjects` holds `V` once and references a `model` node; `body` references a `temporal`/`formula` node |
| HI-02 | `Opaque` of TC-339 and the same clause with its prefix reversed to `exists trace b of V forall trace a of V` | different identities |
| HI-03 | PF-01 and PF-02 of TC-341 | different identities |
| HI-04 | HB-02 and HB-02 with the aliases of its two variables swapped | different identities |
| HI-05 | HB-01 and HB-01 with a match conjunct removed; HB-01 with and without `align skip` | different identities |
| HI-06 | two instances of HB-02 with different universe keys | different identities |
| HI-07 | HB-01 and HB-01 with different whitespace and comments | the same identity |
| HI-08 | HB-01 with `fair weak V::Vault::step;` on quantifier `a` | the `subjects` reference targets `V`'s `model` node, the body reference targets a `temporal`/`formula` node, and `a`'s fairness aggregate references a `temporal`/`fairness` node; the reader admits the package under FR-370 |
| HI-09 | `hyper H using inf over t in Q bounded 4 forall trace a { true }` | `subjects` is empty, the fairness set is empty, and `body` references the Boolean `expression` node for the finite-domain formula |
| HI-10 | HI-08 with its formula reference moved from `body` into `match`, and with its fairness reference moved out of `a`'s fairness aggregate | each package is refused `ill_typed`/`operator-ineligible` at its `claim` node (FR-370-AC-14) |
| HI-11 | HI-08 with `subjects.V` referencing an `expression` node instead of `V`'s selected `model` node | `invalid_model_binding`/`malformed-declaration` at the subject target |

## Expected Results

Identity separates every pair of clauses with different meanings and joins
clauses that differ only in spelling. Model-domain temporal and fairness nodes
are referenced only from FR-402's named claim-body positions; a finite-domain
claim carries no model or fairness references and points its body at an
`expression` node.
