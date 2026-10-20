---
id: TC-300
title: "Strong-fairness refinement past one level"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-365
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-362
    type: verifies
---

# TC-300: Strong-fairness refinement past one level

## Description

A model whose accepting component fails one strong constraint, whose refined
remainder fails a second, and whose second remainder still holds a fair
accepting cycle.

## Test Procedure

The **Ladder** subject: object type `L` with `pos: Int[0, 4]`, universe `{l}`,
initial `pos = 0`, and these operations, each with frame `modifies [pos]`:

| Operation | Enabled at | Post-state |
| --- | --- | --- |
| `a` | `pos = 0` | `pos = 4` |
| `b` | `pos = 0` or `pos = 1` | `pos = 1` from 0, `pos = 4` from 1 |
| `next` | `pos = 1` or `pos = 2` | `pos + 1` |
| `back` | `pos = 3` | `pos = 2` |
| `loop` | `pos = 3` | `pos = 1` |
| `reset` | `pos = 3` | `pos = 0` |

State 4 is terminal. Check `always eventually holds(l.pos = 4)` under
`quire.temporal.infinite-trace/v1`:

1. with `fair strong Lad::L::a; fair strong Lad::L::b;`
2. with `fair weak Lad::L::a; fair weak Lad::L::b;`

Then pass the synthesized SCC of FR-365-AC-7 (strong constraint, visibility pair and the edges listed there) to the SCC search.

## Expected Results

1. The accepting component `{0, 1, 2, 3}` takes `b` (`0 -b-> 1`) but has no
   `a` edge while `a` is enabled at 0, so it is refined by removing 0. The
   remainder `{1, 2, 3}` has no `b` edge while `b` is enabled at 1, so it is
   refined by removing 1. The remainder `{2, 3}` enables neither and passes.
   The item settles `refuted` with a lasso whose loop is `3 -back-> 2 -next-> 3`
   and whose stem reaches 3, such as `0 -b-> 1 -next-> 2 -next-> 3`; the lasso
   reproduces and is fair under both strong constraints (FR-365-AC-4).
2. The first component passes the weak filter with no refinement, and the item
   settles `refuted` with a reproducing lasso.

The synthesized SCC is refined in one step removing state 0 and the edge `1 → 2`; the first passing component is `{3}` with the loop `3 → 3` (FR-365-AC-7).
