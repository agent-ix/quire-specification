---
id: TC-327
title: "Reduced request identity, negotiation and records"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-382
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-385
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-327: Reduced request identity, negotiation and records

## Description

Check the `reductions` request member, obligation identity under reduction,
negotiation against advertised reductions with no fallback, the result axes
of every reduced verdict, and that simulation and trace replay are
unaffected by reduction.

## Test Procedure

Use TC-321's annotated configs and TC-323's counters.

1. Build request items for one claim: unreduced; with
   `SymmetryDeclaration{config_history, [[a, b, c]]}`; with the partial-order
   selection; with a state constraint only. Compare obligation and request
   identities.
   Submit one with its classes listed out of key order.
2. Register one backend advertising `temporal-satisfaction` with reductions
   `{symmetry}` only, and submit an item selecting partial-order reduction;
   then an item selecting symmetry; then partial-order reduction over a
   transition system that supplies no footprints.
3. Submit an infinite-trace safety claim with an interval operator selecting
   partial-order reduction, and check that no unreduced run follows; then
   submit the same claim without the selection as a new request.
4. For one item of each verdict row of FR-385 (taken from TC-321, TC-323 and
   TC-324 and this case), read the FR-341 label, FR-241 execution and FR-243
   basis.
5. Run FR-181 simulation over the counters subject while a model-check item
   on it selects partial-order reduction; replay every counterexample of
   TC-321 to TC-325 through the native transition relation.

## Expected Results

In case 1 the symmetry and partial-order items each have an obligation
identity different from the unreduced one and from each other, the
constraint-only item has the unreduced obligation identity, a different
request identity, and names its constraint in its terminal record, and the out-of-order classes refuse
`invalid_runtime_input`/`invalid-value` (FR-382-AC-1, FR-382-AC-2). In case
2 the partial-order items settle `unsupported`,
`unsupported_projection`/`unsupported-requested-capability`, naming
partial-order reduction, and the symmetry item runs (FR-382-AC-6). Case 3
settles `inconclusive`, `reduction-not-preserving`, before expansion with no
unreduced run, and the second request is a different obligation that runs
unreduced (FR-385-AC-1). In case 4 each label carries exactly its FR-241 and
FR-243 values from the verdict table (FR-385-AC-7). In case 5 the simulation
reports every one of the 27 reachable states (FR-383-AC-8), and every
counterexample replays with the same terminal disposition and carries no
canonical state, permutation or reduction (FR-181-AC-5).
