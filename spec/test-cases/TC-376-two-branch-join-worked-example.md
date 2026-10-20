---
id: TC-376
title: "Two branches and a join: states, deadlock and repair"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-429
    type: verifies
---

# TC-376: Two branches and a join: states, deadlock and repair

## Description

Explore the `Fill` protocol by hand-enumerated expectations: its reachable
states and steps, its deadlock and blocked threads, a liveness claim at the
deadlock, and the repaired protocol.

## Test Procedure

Model: object type `Cell` with `v: Int[0, 3]`, population `cells`, universe
`{c}`, one initial state with `c.v = 0`. Operation `inc()`, frame `modifies
self.v`, precondition `IncPre: self.v < 3`, postcondition `IncPost: self.v =
pre(self.v) + 1`. Operation `setTwo()`, frame `modifies self.v`,
precondition `TwoPre: self.v = 0`, postcondition `TwoPost: self.v = 2`.

```text
protocol Fill using p over (k: M::Cell) on origin {
  role w on M;
  run sequence Main {
    parallel Both {
      branch left  attempt A by w on M::Cell::inc    contracts [IncPre, IncPost] as (a: M::Cell) { true };
      branch right attempt B by w on M::Cell::setTwo contracts [TwoPre, TwoPost] as (b: M::Cell) { true };
    } join all [left, right];
  }
  finish Done as (d: M::Cell) { true };
}
```

Bind `k` to `c`. Explore exhaustively under `sc` through the real local Rust
boundary, recording every state, enabled step and transition identity. Check
the deadlock-freedom item, the claim `eventually holds(k.v = 3)` under
infinite-trace, and the same with `TwoPre: self.v <= 1`.

Parse the protocol text above at S1 under the `quire.value.complete/v1` profile and require it admissible with no diagnostic.

## Vectors

| Vector | Check | Expected result |
| --- | --- | --- |
| FL-01 | Reachable states | Seven: s0 (`v=0`, root at fork) enables only `fork(Both)`; s1 (`v=0`, both branches running) enables `attempt(A)` to s2 and `attempt(B)` to s3; s2 (`v=1`, left completed, right at `B`) enables nothing; s3 (`v=2`, left at `A`, right completed) enables `attempt(A)` to s4; s4 (`v=3`, both completed) enables `join(Both)` to s5; s5 (`v=3`, root at finish, instance `Both` removed) enables `finish` to s6; s6 is finished. Six transitions |
| FL-02 | Transition identities | `fork(Both)` carries `step`, `thread` (root path, instance 0) and `node`; each attempt carries `step`, `thread`, `node`, `operation`, `receiver` `c` and empty `arguments`; no `choice`, `sequence` or branch-end move has an identity |
| FL-03 | Deadlock-freedom item | `refuted`, basis `decisive-counterexample`; prefix s0, s1 (`fork(Both)`), s2 (`attempt(A)` by `left`); blocked threads `right` at `B` with `precondition` `TwoPre`, and the root at `join Both` with `join-waiting` `[right]` |
| FL-04 | `eventually holds(k.v = 3)`, infinite-trace | `refuted` with the lasso whose prefix is s0, s1, s2 and whose loop is s2's terminal stutter step |
| FL-05 | Repaired, `TwoPre: self.v <= 1` | Ten states and nine transitions; both terminal states finished; deadlock-freedom `proved`, basis `exhaustive`; `eventually holds(k.v = 3)` `refuted` by the finished behaviour through `c.v = 2` (`A` then `B`) |
| FL-06 | Repeat and check moves | A variant with a `repeat` of `inc` at `max 2` and a guard that stays true enters `exhausted` at the counter's maximum without a step; a variant with `check C using p { false }` before `A` leaves `left` waiting at `C` with no enabled step |
| FL-07 | Repeat with no maximum | A variant whose `run` is `repeat R by w visible() while { true } attempt F by w on M::Cell::flip …`, with `flip` precondition `self.v <= 1` and postcondition `self.v = 1 - pre(self.v)`, reaches exactly two states (`v 0` and `v 1`, the root at `F`) and two transitions, the second closing the cycle; no state's `bounds` member holds a counter; no state is terminal; `always eventually holds(k.v = 1)` settles `proved` under the default scheduler constraints |
| FL-08 | Undefined constraint | A variant with an empty population `extras` of `Cell` whose attempt `A` has the constraint `{ lookup<M::Cell>(extras, k) absent undefined .v >= 0 }` settles every item, including the deadlock-freedom item, `refuted`, cause `undefined-evaluation`, with the prefix s0, s1 and `where` naming the constraint's locus and reason `absent-key`; `attempt(A)` is not reported as a disabled step |
| FL-09 | Fence step | A variant whose `left` begins with `fence F ordering seq_cst;` has one `fence` step for `left`, which changes no model state and moves `left` to `A`; it is enabled in every state under `sc`; under `tso` the `seq_cst` fence is enabled exactly while `left`'s store buffer is empty and an `acquire` fence is enabled in every state, and in a state with a non-empty buffer the `seq_cst` fence has no enabled step while `left`'s `flush` is enabled, so `left` waits at the fence and that state is no deadlock; the variant with a fence in each branch reaches no deadlock that `Fill` without fences does not reach under `sc` |

## Expected Results

Every count, enabled-step set, verdict and blocked list matches the table,
and every refutation replays to `reproduced-with-evaluated-witness` (FR-433).
