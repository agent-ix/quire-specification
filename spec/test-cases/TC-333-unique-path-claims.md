---
id: TC-333
title: "Unique-path claims and path counting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
---

# TC-333: Unique-path claims and path counting

## Description

Check `unique path` on object type `Job` with Boolean fields `a`, `b` and `done`, universe `{j}`, initial state all false, and operations `stepA`, `stepB` and `finish` (requires `a and b and not done`).

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay every returned evidence value. Each refutation and each witnessed proof is checked by replaying its evidence, never by comparing it with one expected evidence value; the verdict, basis, settlement method and cause are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| UP-01 | `unique path InOneWay using v over (j: M::Job) from (not j.a and not j.b) to (j.done);` | `refuted`, `decisive-counterexample`; `path-pair` with an empty stem whose two sequences are `stepA, stepB, finish` and `stepB, stepA, finish`; replay checks `done` false before each last node and the sequences differ |
| UP-02 | UP-01 with `stepB` requiring `a and not b` | `proved`, `closed-scope`, exhaustive; the initial state counts 1 |
| UP-03 | UP-02's sequenced model with an operation `undo` (requires `a and b and not done`, clears `b`), so `tff` and `ttf` form a cycle; clause as UP-01 | `refuted`, `decisive-counterexample`; `path-pair` with first sequence `stepA, stepB, finish` and second `stepA, stepB, undo, stepB, finish`; replay reproduces |
| UP-04 | a model in which no `Y` node is reachable from the `X` node | `refuted`, `closed-scope`, with a `trap` at the `X` node |

## Expected Results

Paths are counted per reachable `X` node, a cycle on a path to `Y` gives more than one path, and every refutation replays.
