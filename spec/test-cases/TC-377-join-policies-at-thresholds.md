---
id: TC-377
title: "Join policies at zero, below, at and past threshold"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
---

# TC-377: Join policies at zero, below, at and past threshold

## Description

Check that each join policy's `join` step is enabled exactly at its threshold
disposition, and that `outstanding continue` and `outstanding cancel` act as
FR-426 states.

## Test Procedure

Build a `parallel J` with three branches `a`, `b` and `c`, each a single
`event` node whose constraint is true, and the root's `finish` after the
join. For each policy and outstanding rule, explore exhaustively and record,
at every state, the number of `completed` branch threads and whether `join(J)`
is enabled; after the join, record which branch threads still take steps and
their dispositions.

## Vectors

| Vector | Policy | Completed branches 0 / 1 / 2 / 3 | Outstanding rule and expected effect |
| --- | --- | --- | --- |
| JP-01 | `all` | disabled / disabled / disabled / enabled | none needed; no branch is running at the join |
| JP-02 | `any`, `outstanding continue` | disabled / enabled / enabled / enabled | after a join at 1, the two running branches still take their event steps; the root's `finish` waits until both leave `running` |
| JP-03 | `any`, `outstanding cancel` | disabled / enabled / enabled / enabled | after a join at 1, both running branches are `cancelled` in the join step and take no step |
| JP-04 | `quorum(2)`, `outstanding cancel` | disabled / disabled / enabled / enabled | after a join at 2, the remaining branch is `cancelled` in the join step |
| JP-05 | `quorum(2)`, `outstanding continue` | disabled / disabled / enabled / enabled | after a join at 2, the remaining branch keeps running |
| JP-06 | `predicate(f)` with `f` true when `a` and `c` are completed, `outstanding cancel` | disabled at 0; enabled at 1 never; at 2 enabled only when the two are `a` and `c`; enabled at 3 | after a join with `b` running, `b` is `cancelled` |
| JP-07 | `any`, `outstanding continue`, finish constraint `true` | after the join, `finish` is not enabled while either remaining branch is `running`, and is enabled once both have completed |
| JP-08 | `parallel Outer { branch a attempt Fast; branch b sequence { parallel Inner { branch x attempt X1; branch y attempt Y1 } join any [x, y] outstanding continue; attempt After } } join any [a] outstanding cancel`, every attempt always enabled | In the state where `a` has completed, `b` rests at `Inner`'s join and `x` and `y` are `running`, `join(Outer)` makes `b`, `x` and `y` `cancelled`, settles and removes `Inner`'s instance; no later state enables a step of any of them, so `Y1` and `After` are enabled nowhere after the join |
| JP-09 | `predicate(g)` over `[a, b, c]`, `g(p, q, r)` true exactly when `q` is true and `r` is false (`p` for `a`, `q` for `b`, `r` for `c`; `true` for `completed`, `false` for `running`), `outstanding cancel` | enabled at exactly the completed sets `{b}` and `{a, b}`; disabled at `{}`, `{a}`, `{c}`, `{a, c}`, `{b, c}` and `{a, b, c}` | after a join at `{b}`, `a` and `c` are `cancelled` in the join step |

## Expected Results

Each policy's join is enabled exactly where the table states and nowhere
else, a cancelled thread takes no further step, and the parallel instance is
removed only after the join has settled and every branch has left
`running`.
