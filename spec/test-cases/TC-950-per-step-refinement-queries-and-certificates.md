---
id: TC-950
title: "Per-step refinement queries and certificates"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-468
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-950: Per-step refinement queries and certificates

## Description

State the per-step queries of a refinement and settle them with and without certificates. Scope: FR-468-AC-1 to FR-468-AC-12 and FR-290-AC-14.

## Test Procedure

1. Take a refinement of a model with seven step rows and no hidden field, history field or `any` row; list its queries; then change one row to `any`, then hide an abstract field, then add a history field.
2. Assemble its certificate; then drop the base part, drop one step part, supply a step part where the base part belongs, and carry a query that differs from the core's.
3. Settle with every part and hypothesis item verified, then with one query's proof carrying no certificate, then one using an unchecked rule.
4. Give one step query a backend counterexample (and one to its definedness requirement), then the initial query a counterexample with and without a reproducing replay.
5. Add a concrete invariant clause and compare the queries and the initial query's identity; add an abstract-model invariant and require no change.
6. Give the concrete model the invariant `self.value < 0` with every step query refuted and certified.
7. Return `unknown`, a timeout, a missing solver and a fault for one query while the others prove; then return one of them for the initial query or step row 1 while step row 3 has a backend counterexample, and again with the counterexample at row 1 and the outcome at row 3; then return one of them for a query while the certificate is rejected.
8. Read the extent of each query item of the step 1 refinement, whose concrete population `counters` has no maximum; then with a proof bound of 2 on `counters`; then with an unread unbounded population `audits` of another object type; then with an unbounded population that only a contract clause of `peek` quantifies over, and one that only the state mapping's `reaches` walks; then in two `peek` variants, one taking `n: Integer` and one returning `Integer`; then with a second population `spare` of the receiver's type. Negotiate the unbounded items against a candidate advertising `operation-contract` only in `bounded` mode with `domains` [`population`], with a finite bound available and then with none; against a bounded-only candidate whose `domains` lack `population`; for each `peek` variant, against bounded-only candidates with `domains` [`population`] and [`population`, `integer`]; then against one advertising `unbounded`.
9. Settle the step 1 refinement's certificate with its base proof using `hole`; with row 2's proof using `lia_generic`; with every part verified; and over the same refinement with an eighth row `bump` that has a `_` argument, with valid parts for the seven other rows and a present part at `bump`'s position. Then repeat the first of these with row 4's carried query changed.
10. Give the refinement a mapping row outside the linear-integer fragment, so that the core refuses to encode the initial query, and check a certificate whose base part carries a query of the initial query's shape, then one whose base part is empty; then check the step 9 `bump` certificate again.

## Expected Results

Step 1: one initial and seven step queries in `steps` order, none for the three excluded declarations. Step 2: the complete certificate is accepted, and each defective one settles `inconclusive`, `certificate-rejected` naming the part. Step 3: `proved` `certified`, then `proved` `uncertified` for each. Step 4: `inconclusive`, `induction-not-closed{depth: 1}` naming the row, never `refuted`; the initial counterexample `refuted` `InitialNotAbstract` once replay reproduces it, `inconclusive` `replay-parity` otherwise. Step 5: the clause is a hypothesis of every query and the initial query's identity changes; the abstract invariant changes nothing. Step 6: `inconclusive`, `hypotheses-not-established`. Step 7: each takes the FR-331 result for the outcome and the half takes the first such query's result in the order initial query, then step rows; with a step counterexample anywhere the half settles `inconclusive`, `induction-not-closed{depth: 1}` naming the counterexample's row whichever position the outcome holds, and with a rejected certificate it settles `certificate-rejected{rule, at}` (FR-468-AC-7, FR-468-AC-11, FR-468-AC-12). Step 8: each of the eight items `unbounded` with the single `counters` population domain, boundable by `cardinality`; each `bounded` under the proof bound; `audits` adds no domain; the population only a contract clause of `peek` ranges over adds its `population` domain to `peek`'s step query alone, and the one only the state mapping walks adds it to the initial query and to every step query; in each `peek` variant, `peek`'s step query gains one `integer` domain (the parameter's in one, the result's in the other), the other seven unchanged; `spare` refuses `ambiguous_declaration`/`ambiguous-name` at each step row naming `counters` and `spare`, with no query; under the bounded-only candidate with `domains` [`population`] the unbounded items settle `requires-bound` with a finite bound available and `unsupported`/`unbounded-extent` with none; under the candidate lacking `population` they settle `unsupported`/`unsupported-requested-capability` naming `population`; `peek`'s step query settles `unsupported-requested-capability` naming `integer` against [`population`] and `requires-bound` against [`population`, `integer`]; every item is decided under the unbounded candidate (FR-468-AC-8, FR-290-AC-14).

Step 9: all verified, every query `proved` `certified`; the `hole` base, the `lia_generic` row and the `bump` certificate each settle the affected query and the half `proved` `uncertified`, none `certified`; with row 4's query changed the half settles `inconclusive`, `certificate-rejected{query-mismatch, step/4}`, the rejection outranking the unverifiable base (FR-468-AC-9). Step 10: both certificates are rejected with the rule `query-mismatch` at `base` and the half settles `inconclusive`, `certificate-rejected{query-mismatch, base}`, never `proved` and never `uncertified`; the `bump` certificate is again `uncertified`, not rejected (FR-468-AC-10).
