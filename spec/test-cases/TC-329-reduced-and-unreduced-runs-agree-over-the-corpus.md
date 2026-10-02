---
id: TC-329
title: "Reduced and unreduced runs agree over the conformance corpus"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
---

# TC-329: Reduced and unreduced runs agree over the conformance corpus

## Description

Check that no reduction changes a verdict: every model-check case of the
FR-336 conformance corpus is run with each selection the preservation table
admits for it and without any reduction, and the two runs must settle the
same label.

## Test Procedure

1. Enumerate the model-check cases of the FR-336 conformance corpus,
   including every vector of TC-321 to TC-325.
2. For each case, list the selections among symmetry, partial-order
   reduction, and both, whose FR-385 preservation rows all read "yes" for
   the case's claim form and fairness set. Symmetry is listed only for a
   case that declares a `symmetric` population, and partial-order reduction
   only for a subject whose transitions have footprints. Skip a case that
   carries a state constraint.
3. Run the case once with no reduction and once with each listed selection.
   Drop a pair in which either run ends at its search horizon or at a
   resource limit.
4. Compare the FR-341 labels of each remaining pair, and replay every
   counterexample from either run through the model's successor relation,
   with fairness re-established for a lasso.

## Expected Results

Every compared pair settles the same FR-341 label. Every counterexample
replays. A reduced `proved` names proof basis `reduced` and the matching
unreduced `proved` names `exhaustive`. In particular, the partial-order
vectors of TC-323 cases 5 to 10 and the `Gate` vectors of TC-325 cases 2
and 3 settle `refuted` both ways, the `fair weak each` instance of TC-322
case 1 settles `proved` both ways, and the `Mark` protocol of TC-325 case 1
settles `proved` both ways under its default scheduler constraints and
`refuted` both ways under `scheduling adversarial` (FR-383-AC-15).
