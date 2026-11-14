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

Check the hyperproperty claim node body, its quantifier Groups, subject
roots, fairness references, body wrapper and slot names, the reader's
refusals of a body that breaks them, and that every meaning-changing member
of a clause changes its obligation identity.

## Test Procedure

Compile each admitted vector through the real local Rust boundary, read the
hyperproperty claim node body, and compute each item's obligation identity;
compare identities pairwise. Build each refused vector by editing the
admitted package named in its row, with `dependencies` rebuilt after the
edit, pass it through the checked-package reader, and record its exact
refusal and location.

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
| HI-08 | HB-01 with `fair weak V::Vault::step;` on `a` | `quantifiers` is a Tuple of two Groups: `a`'s binds `variable` `a`, `quantifier` `forall`, `alias` `V` and then holds one `reference` to a `temporal`/`fairness` node whose member is `{kind: "fairness", fairness_kind: "weak", granularity: "whole", target: {kind: "operation", …}}` naming `Vault`'s `step`; `b`'s binds the same three with `variable` `b` and holds no reference; strict wire validation raises no `malformed_wire`; the reader admits the package (FR-402-AC-5, FR-370-AC-21) |
| HI-09 | `hyper H using inf over t in Q bounded 4 forall trace a { true }` | `subjects` is empty; the one Group binds `variable` `a` and `quantifier` `forall` only; `body` references a Boolean `expression` node (FR-402-AC-5) |
| HI-10 | HB-01's `subjects` and HB-02's `subjects`; HB-01 with an added alias `W` selecting a second package, quantified by `exists trace c of W`, with no object parameter of `W` | HB-01's `V` references `V`'s `model`/`model_root` node; HB-02's `S` and `N` reference the roots of their two packages, in that order; `W` references its own package's root (FR-402-AC-6) |
| HI-11 | HI-08 with: (a) `subjects.V` referencing `Vault`'s `model`/`object_type` node; (b) `a`'s `alias` set to `Z`, no subject `Z` | (a) `invalid_model_binding`/`malformed-declaration` at the subject target; (b) `missing_declaration`/`missing-name` at `a`'s `alias` binding (FR-402-AC-6) |
| HI-12 | HI-08 with: (a) the fairness node's target `name` `nope`; (b) HI-08 plus HI-10's alias `W`, whose package's object type declares `step`, with the fairness node's declaring node that object type; (c) the wrapper's law `quire.temporal.event-position.false-extension/v1`; (d) `a`'s `quantifier` `some`; (e) `a`'s `quantifier` binding before its `variable` binding; (f) the fairness node's target a `protocol_root` target | (a) `missing_declaration`/`missing-name`, (b) and (f) `invalid_model_binding`/`malformed-declaration`, and (c) `invalid_package`/`operation-member-mismatch`, each at `/semantic_graph/nodes/{n}/body/members/3/value/members/0/members/3`; (d) `invalid_package`/`invalid-value` at the binding's value; (e) `invalid_semantic_graph` at the first misordered binding (FR-402-AC-7) |
| HI-13 | HI-08 with its fairness reference moved: (a) into `match`; (b) into `body` in place of the wrapper reference; (c) into `a`'s Group as the value of a `binding("fair", …)`; and HI-08 with `body` referencing the wrapper's root `temporal`/`formula` node | each refuses `ill_typed`/`operator-ineligible` at the hyperproperty node (FR-370-AC-21, FR-402-AC-8) |
| HI-14 | HI-08's body wrapper and the request the producer writes for HI-08; HI-08 with the wrapper's `fairness` argument holding a reference to `a`'s fairness node; HI-09 with `body` referencing an integer-typed `expression` node | the wrapper's `formula` references the body's root formula node and its `fairness` is empty; the request holds no FR-368 `formula` item whose `claim.node` is the wrapper; the edited wrapper refuses `invalid_semantic_graph` at `/semantic_graph/nodes/{w}/body/arguments/4`; the integer body refuses `ill_typed`/`non-boolean-root` at the target (FR-402-AC-8) |
| HI-15 | HB-01's slot parameter nodes; the encoder over (`slot`, [`a_b`, `c`]), (`slot`, [`a`, `b_c`]), (`slot`, [`é`, `a`]) and every pair of distinct (tag, field list) over the fields `_`, `s`, `x`, `x41`, `1a`, `a` and `b` | HB-01 holds `slot_sv_sa`, `slot_sv_sb`, `op_sa`, `op_sb`, `arg_sa_sVault_sstep_si` and `arg_sb_sVault_sstep_si`; the three encodings are `slot_sa__b_sc`, `slot_sa_sb__c` and `slot_s_xC3_xA9_sa`; every name matches `[A-Za-z_][A-Za-z0-9_]*`, decodes to its tag and fields, and no two pairs share a name (FR-402-AC-9) |
| HI-16 | HI-08 with: (a) the `align_skip` binding removed (six bindings); (b) an added alias `W` in `subjects` that no Group names; HI-09 with (c) a `subjects` binding, (d) an `alias` binding on its Group, (e) a fairness reference in its Group | each `invalid_semantic_graph`: (a) at the first binding that breaks the layout, (b) at `W`'s `subjects` binding, (c) to (e) at the added member (FR-402-AC-6) |
| HI-17 | HB-01 and HB-01 with trace variable `a` renamed `c` throughout | different node ids and different obligation identities (FR-402-AC-3) |
| HI-18 | HI-08 with `a`'s Group `reference` retargeted to: (a) the clause's body wrapper `temporal`/`temporal_clause` node; (b) a Boolean `expression` node (a match conjunct); (c) `Vault`'s `model`/`object_type` node | each refuses `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/{n}/body/members/3/value/members/0/members/3`; none is admitted (FR-402-AC-10) |

## Expected Results

Identity separates every pair of clauses with different meanings and joins
clauses that differ only in spelling. Each body is grammar-legal: a Tuple of
quantifier Groups whose fairness is a list of references to
`temporal`/`fairness` nodes, subjects that are model roots, and a body
wrapper that carries the formula and no fairness and is never listed as a
single-trace claim. Each refused vector reports exactly the refusal and
location in its row.
