---
id: TC-299
title: "Strong fairness on the contended mutex"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-362
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
---

# TC-299: Strong fairness on the contended mutex

## Description

The mutex example under `strong each`, `weak each` and `strong whole`, with the
three verdicts and replay of the refutations.

## Test Procedure

The **Mutex** subject: object type `Mutex` with `owner: Int[0, 2]` (0 free),
population `locks`; operation `acquire(p: Int[1, 2])` with precondition
`self.owner = 0` and postcondition `self.owner = p`; operation `release()` with
precondition `self.owner != 0` and postcondition `self.owner = 0`. Universe
`{m}`, initial `owner = 0`. Edges: `0 -acq(1)-> 1`, `0 -acq(2)-> 2`,
`1 -rel-> 0`, `2 -rel-> 0`.

Check `always eventually holds(m.owner = 1)` under
`quire.temporal.infinite-trace/v1`:

1. with `fair strong each M::Mutex::acquire;`
2. with `fair weak each M::Mutex::acquire;`
3. with `fair strong whole M::Mutex::acquire;` and with the unmarked
   `fair strong M::Mutex::acquire;`
4. Replay the step 2 lasso under the step 1 premise.

## Expected Results

1. `proved`, proof basis `exhaustive`: the only accepting component enables
   `acq(1)` at owner 0 and never takes it, and its refinement leaves no fair
   cycle.
2. `refuted` with a lasso such as the loop `0 -acq(2)-> 2 -rel-> 0`, which
   reproduces: `acq(1)` is disabled at owner 2, so it is weakly fair.
3. Both spellings have one obligation identity and settle `refuted` with a
   reproducing lasso: `acq(2)` discharges `acquire` as a whole while process 1
   starves.
4. Refused as an unfair lasso naming (`strong`, `acquire`, `each`) with
   identity `acq(1)` (FR-364-AC-5).

Steps 1 and 2 show a claim refuted under weak fairness and proved under strong,
and the step 2 premise admits every step 1 behaviour (FR-362-AC-6).
