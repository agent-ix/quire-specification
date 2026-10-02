---
id: TC-291
title: "Weak and strong fairness on conformance lassos"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-362
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-361
    type: verifies
---

# TC-291: Weak and strong fairness on conformance lassos

## Description

Fairness resolution, admission and the weak and strong lasso rules, with
enabledness computed from a model subject.

## Test Procedure

The **Toggle** subject: object type `T` with `s: Int[0, 2]`; operation `p`
enabled at `s = 0` (to `s = 1`) and at `s = 1` (to `s = 0`); operation `q`
enabled at `s = 0` only (to `s = 2`); operation `r` enabled at `s = 2` only
(to `s = 0`). Universe `{t}`, initial `s = 0`. Lassos are given as loops from
the initial state:

| Lasso | Loop | `q` in the loop |
| --- | --- | --- |
| L1 | `0 -p-> 1 -p-> 0` | enabled at 0, disabled at 1, never taken |
| L2 | `0 -q-> 2 -r-> 0` | taken |
| L3 | `0 -p-> 1 -p-> 0` under a model where `q` is also enabled at `s = 1` (to `s = 2`) | enabled at every loop state, never taken |
| L4 | the terminal stutter loop of a model whose state `s = 2` has no `r` | none enabled |

1. Resolve `fair Cnt::T::q`, `fair weak Cnt::T::q`, `fair whole Cnt::T::q`,
   `fair weak whole Cnt::T::q`, `fair strong Cnt::T::q` and `fair each Cnt::T::q`
   and compare their obligation identities.
2. Author `fair Cnt::T::q;` on a clause under
   `quire.temporal.event-position.false-extension/v1`, and `fair Cnt::T::nope;`
   on an infinite-trace clause.
3. Check each lasso against (`weak`, `q`, `whole`) and (`strong`, `q`, `whole`).
4. Over a universe `{t, u}`, check a loop that takes `q` only with receiver `t`
   while `q` with receiver `u` is enabled at every loop state, under
   `whole` and under `each`.
5. Check the safety claim `always holds(t.s <= 2)` and the bounded claim
   `eventually[0,2] holds(t.s = 2)` with the fairness set empty, weak and
   strong on `q`, and count the admitted behaviours under each set.

## Expected Results

1. The first four resolve to (`weak`, `q`, `whole`) with one obligation
   identity; `fair strong` resolves to (`strong`, `q`, `whole`) and `fair each`
   to (`weak`, `q`, `each`), each a different identity (FR-362-AC-1).
2. The bounded-profile constraint refuses `unsupported_construct` at the
   constraint naming the profile; `nope` refuses `missing_declaration`/
   `missing-name` (FR-362-AC-2).
3. L1 is weakly fair and strongly unfair; L2 is fair under both; L3 is weakly
   and strongly unfair; L4 is fair under both (FR-362-AC-3, FR-362-AC-4,
   FR-361-AC-4).
4. The loop is fair under `whole` and unfair under `each`, which names the
   starved identity with receiver `u` (FR-362-AC-5).
5. Each claim receives the same verdict under all three fairness sets, and
   every set admits at least one infinite behaviour (FR-362-AC-7).
