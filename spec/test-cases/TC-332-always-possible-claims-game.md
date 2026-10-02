---
id: TC-332
title: "Always-possible claims on the game subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
---

# TC-332: Always-possible claims on the game subject

## Description

Check `always possible` on object type `Game` with `phase` in `{Start, Mid, Won, Lost}`, universe `{g}`, initial phase `Start`, and operations `play` (Start to Mid), `win` (Mid to Won) and `lose` (Mid to Lost). The graph has 4 nodes and 3 edges.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay every returned evidence value. Each refutation and each witnessed proof is checked by replaying its evidence, never by comparing it with one expected evidence value; the verdict, basis, settlement method and cause are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| AP-01 | `always possible CanStillWin using v over (x: G::Game) { x.phase = G::Phase::Won }` | `refuted`, `closed-scope`; `trap` whose stem is `play`, `lose`; replay explores the closure `{Lost}` and finds `P` false |
| AP-02 | AP-01 with `from (x.phase != G::Phase::Lost)` | `proved`, `closed-scope`, exhaustive, over 4 nodes |
| AP-03 | AP-01 on the model with `restart` (Lost to Start) added | `proved`, `closed-scope`, exhaustive |
| AP-04 | the deadlock-freedom item of the AP-01 model, no `terminal` member | `refuted` at `Lost`, stem `play`, `lose` |
| AP-05 | AP-01 and the deadlock-freedom item with `terminal when` the phase is `Won` or `Lost` | deadlock-freedom `proved`; `CanStillWin` still `refuted` with the same trap |

## Expected Results

The trap verdict depends only on the trap's closed closure, the `from` predicate restricts the nodes checked, and the `terminal` member changes no state-graph verdict.
