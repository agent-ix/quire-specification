---
id: TC-304
title: "Strong-fairness hint on a reproduced refutation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-369
    type: verifies
---

# TC-304: Strong-fairness hint on a reproduced refutation

## Description

The `fairness.strong-would-exclude` warning: when it is emitted, which
constraint and granularity it names, and that it changes nothing else.

## Test Procedure

1. Replay the TC-299 `weak each` refutation (loop `0 -acq(2)-> 2 -rel-> 0`).
2. Replay the TC-299 `strong whole` refutation with the same loop.
3. Replay a TC-291 Toggle refutation of `always eventually holds(t.s = 2)`
   with loop L1 under `fair weak Cnt::T::q;`.
4. Inspect the step 2 warnings, whose clause carries (`strong`, `acquire`,
   `whole`), and replay a TC-291 Toggle lasso whose loop
   `0 -q-> 2 -r-> 0 -p-> 1 -p-> 0` takes every enabled identity.
5. Strip the warnings from the step 1 terminal record and compare verdict,
   counterexample, replay identity and obligation identity; replay the step 1
   lasso as returned by two different engines.

## Expected Results

1. One warning `fairness.strong-would-exclude` naming (`strong`, `acquire`,
   `each`), identity `acq(1)`, position 0; no `whole` warning, since `acquire`
   is taken in the loop (FR-369-AC-1).
2. The same warning (FR-369-AC-1).
3. One warning naming (`strong`, `q`, `whole`) with `identity` `null` and the
   first loop position enabling `q` (FR-369-AC-2).
4. The step 2 record names no (`strong`, `acquire`, `whole`) warning, the
   constraint its clause carries, and the Toggle loop carries none
   (FR-369-AC-3).
5. Nothing else changes when the warnings are removed (FR-369-AC-4), and both
   engines' refutations carry the same warnings (FR-369-AC-5).
