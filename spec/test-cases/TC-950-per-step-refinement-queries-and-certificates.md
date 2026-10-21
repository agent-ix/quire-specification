---
id: TC-950
title: "Per-step refinement queries and certificates"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-468
    type: verifies
---

# TC-950: Per-step refinement queries and certificates

## Description

State the per-step queries of a refinement and settle them with and without certificates. Scope: FR-468-AC-1 to FR-468-AC-7.

## Test Procedure

1. Take a refinement of a model with seven step rows and no hidden field, history field or `any` row; list its queries; then change one row to `any`, then hide an abstract field, then add a history field.
2. Assemble its certificate; then drop the base part, drop one step part, supply a step part where the base part belongs, and carry a query that differs from the core's.
3. Settle with every part and hypothesis item verified, then with one query's proof carrying no certificate, then one using an unchecked rule.
4. Give one step query a backend counterexample (and one to its definedness requirement), then the initial query a counterexample with and without a reproducing replay.
5. Add a concrete invariant clause and compare the queries and the initial query's identity; add an abstract-model invariant and require no change.
6. Give the concrete model the invariant `self.value < 0` with every step query refuted and certified.
7. Return `unknown`, a timeout, a missing solver and a fault for one query while the others prove.

## Expected Results

Step 1: one initial and seven step queries in `steps` order, none for the three excluded declarations. Step 2: the complete certificate is accepted, and each defective one settles `inconclusive`, `certificate-rejected` naming the part. Step 3: `proved` `certified`, then `proved` `uncertified` for each. Step 4: `inconclusive`, `induction-not-closed{depth: 1}` naming the row, never `refuted`; the initial counterexample `refuted` `InitialNotAbstract` once replay reproduces it, `inconclusive` `replay-parity` otherwise. Step 5: the clause is a hypothesis of every query and the initial query's identity changes; the abstract invariant changes nothing. Step 6: `inconclusive`, `hypotheses-not-established`. Step 7: each takes the FR-331 result for the outcome and the half takes the first such query's result.
