---
id: TC-322
title: "Annotated quotient each-fairness vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-384
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
---

# TC-322: Annotated quotient each-fairness vectors

## Description

Decide weak and strong `each` fairness under symmetry on the annotated
quotient, build concrete loops from cycle groups, and close a lasso that
needs more than one repetition.

## Test Procedure

Use TC-321's subject and declaration.

1. Check `ReachesTwo` for `c = a` under `fair weak each attemptUpdate`;
   report the cycle group of the SCC with `va = 0` and its witness sets.
2. Check `eventually always holds(c.versionNumber != 2)` for `c = a` under
   `fair weak each attemptUpdate`; replay the counterexample and check each
   identity is taken in its loop.
3. Check case 2's clause under `fair strong each attemptUpdate` on a variant
   where `attemptUpdate` on `a` is disabled at `va = 1`; compare with the
   unreduced verdict.
4. **More than one repetition.** Check `eventually holds(c.versionNumber =
   1)` for `c = a` under `fair weak whole attemptUpdate`, and concretise the
   reduced lasso over the stabiliser quotient whose stem is `(0,0,0)
   -upd(c)-> (0,0,1)` and whose loop is `(0,0,1) -upd(c)-> (0,0,2)
   -upd(b)-> (0,1,2) -upd(c)-> (0,0,1)`, the last edge carrying permutation
   `(b c)`; replay the result.
5. Repeat case 1 with the caller's state limit set below the cycle-group
   computation's needs.

## Expected Results

Case 1: the spanning tree reaches `(0,0,1)` by `upd(b)` with permutation
`(b c)`, the non-tree edge `(0,0,0) -upd(c)-> (0,0,1)` gives the word
`(b c)`, the cycle group is `{id, (b c)}`, `W_taken` lies in one orbit
`{upd(b), upd(c)}` and `W_disabled` is empty; the orbits miss `upd(a)`, the
SCC is rejected, the `va = 1` SCC likewise, and the instance settles
`proved` with proof basis `reduced` over 30 product states, the verdict the
unreduced 135-state product gives (FR-384-AC-1, FR-384-AC-2). Case 2: the
18-state SCC passes; the concrete loop takes `upd(a)`, `upd(b)` and
`upd(c)`, passes `a = 2`, closes at `(0,0,0)` and replays with each identity
taken (FR-384-AC-3). Case 3 gives the unreduced verdict after removing the
quotient states that enable the failing identity and recursing
(FR-384-AC-4). Case 4 concretises to the stem `upd(c)` to `(0,0,1)` and the loop
`upd(c)`, `upd(b)`, `upd(c)`, `upd(b)`, `upd(c)`, `upd(b)` through
`(0,0,2)`, `(0,1,2)`, `(0,1,0)`, `(0,2,0)`, `(0,2,1)` back to `(0,0,1)`: one
pass of the reduced loop ends at `(0,1,0)`, the `(b c)` image of the entry,
so the loop is repeated twice before the concrete state equals the entry;
the lasso keeps `a` at version 0, carries no permutation and replays
(FR-383-AC-7). Case 5
settles `incomplete`, FR-241 `resource-incomplete`, cause `limit-reached{limit, value, setting}`
(FR-384-AC-5).
