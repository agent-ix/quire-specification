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

State the per-step queries of a refinement and settle them with and without certificates. Scope: FR-468-AC-1 to FR-468-AC-5.

## Test Procedure

1. Take a refinement of a model with seven step rows and no hidden field, history field or `any` row; list its queries; then change one row to `any`.
2. Assemble its certificate; then drop the base part, drop one step part, and supply a step part where the base part belongs.
3. Settle with every part verified, then with one query's certificate absent, then one unverifiable.
4. Give one step query a backend counterexample, then one step part that fails verification.
5. Add a concrete invariant clause and compare the queries and the initial query's identity.

## Expected Results

Step 1: one initial and seven step queries in `steps` order, none after the `any` row. Step 2: the complete certificate is accepted and each defective one refused. Step 3: `proved` `Certified`, then `proved` `Uncertified` for each. Step 4: `inconclusive`, induction not closed, naming the row or part, never `refuted`. Step 5: the clause appears as a hypothesis of every step query and the initial query's identity changes.
