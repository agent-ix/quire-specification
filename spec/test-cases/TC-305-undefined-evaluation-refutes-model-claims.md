---
id: TC-305
title: "Undefined evaluation refutes a model claim"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-363
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-365
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-361
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-366
    type: verifies
---

# TC-305: Undefined evaluation refutes a model claim

## Description

A claim, guard or `terminal when` predicate that evaluates undefined at a
reachable state settles `refuted` with cause `undefined-evaluation`, and its
counterexample ends at that state and replays to the same undefined value.

## Test Procedure

The **Counter** subject of TC-290, with universe `counters = {c, d}`, both at
value 0, and the operation `drop` that removes `d` from `counters`. `rd` is a
`Reference<Cnt::Counter>` parameter valued `d`. Under
`quire.temporal.infinite-trace/v1`:

1. Check `always holds(lookup<Cnt::Counter>(counters, rd) absent undefined.value <= 3)`
   by explicit-state model checking.
2. Replay the counterexample of step 1.
3. Replay the counterexample of step 1 with `trace_position` 0.
4. Check step 1's claim over the same subject without the operation `drop`.
5. Check `always holds((lookup<Cnt::Counter>(counters, rd) absent undefined).value <= 0)`
   by exhaustive explicit-state model checking: it is false after `inc(d)` and
   undefined after `drop`, both one step from the initial state, and both again
   after longer prefixes.
6. Add an operation `spin` with receiver `c` whose postcondition keeps
   `c.value` unchanged, so the initial state has a self-loop. Check the
   liveness claim `always eventually holds((lookup<Cnt::Counter>(counters, rd) absent undefined).value = 3)`
   by exhaustive explicit-state model checking. The `spin` self-loop at the
   initial state is a fair accepting cycle with an empty stem; `drop` reaches an
   undefined state one step from the initial state.
7. Check `always holds(x.value <= 3)` with `over (x: Cnt::Counter)` bound to
   `d`, reading `x.value` with no guard, over the subject with `drop`.
8. Give `inc` the guard `(lookup<Cnt::Counter>(counters, rd) absent undefined).value < 3`
   and check `always holds(c.value <= 3)` over the subject with `drop`, then
   replay its counterexample.
9. Over the subject with `drop` and no other operation enabled after it, add
   `terminal T using v on Cnt when { (lookup<Cnt::Counter>(counters, rd) absent undefined).value = 0 }`
   and check the derived deadlock-freedom item.
10. Check `always holds(c.value = 1 and (lookup<Cnt::Counter>(counters, rd) absent undefined).value = 0)`
    over a subject whose one operation `inc_drop` increments `c` and removes
    `d` in one step, so the state after it makes the first conjunct false and
    the second undefined.

## Expected Results

1. `refuted`, basis `decisive-counterexample`, FR-331 result `refuted`, cause
   `undefined-evaluation` with `where` naming position 1, the state-key digest
   of the state after `drop` and the locus of the `lookup`, and `cause`
   `absent-key` (FR-363-AC-8). The counterexample's `kind` is the envelope `{"type": "undefined-evaluation", "where", "cause"}` carrying the same `where` and `cause`, a
   prefix of the one `drop` step, an empty `loop` and `trace_position` 1
   (FR-364-AC-8).
2. Replay reads `where` and `cause` from the counterexample and reproduces the
   undefined value at position 1 with the `lookup`'s locus and reason
   `absent-key`: `reproduced-with-evaluated-witness`, item
   `refuted` (FR-364-AC-8).
3. The claim's reads are defined at position 0: `inconclusive`, cause
   `replay-parity` (FR-364-AC-8).
4. `proved`, proof basis `exhaustive`: `d` stays in `counters`, so the read is
   defined at every reachable position and the claim settles by its truth
   values (FR-363-AC-8).
5. Both candidates are first-phase violations. The canonical counterexample has
   a one-step prefix, never a longer one: the one of `drop` (an
   undefined-evaluation envelope as its kind) and `inc(d)` (`kind: formula`) whose
   post-state comes first in FR-181 canonical breadth-first discovery order;
   two runs return byte-identical counterexamples (FR-365-AC-6).
6. `refuted` with cause `undefined-evaluation` and the one-step `drop` prefix.
   The liveness phase never runs, so the empty-stem `spin` lasso is never
   returned (FR-365-AC-6).
7. `refuted`, cause `undefined-evaluation`, with `where` naming the position
   after `drop` and the locus of `x.value`; the same outcome as the `lookup`
   spelling of step 1 (FR-361-AC-6).
8. After `drop`, the guard of `inc(c)` evaluates undefined: every item over the
   subject settles `refuted`, cause `undefined-evaluation`, with `where`
   naming position 1, its state-key digest, the guard's locus and the
   transition identity of `inc(c)`; `inc(c)` is never read as disabled. Replay
   evaluates that guard at the last state and reproduces the undefined value
   (FR-363-AC-9, FR-364-AC-8).
9. The state after `drop` has no enabled transition and `P` evaluates
   undefined there: the deadlock-freedom item settles `refuted`, cause
   `undefined-evaluation`, with the undefined-evaluation counterexample ending
   at that state, never `proved` and never a `deadlock` counterexample
   (FR-363-AC-9, FR-366-AC-8).
10. The reported cause is `undefined-evaluation` and the counterexample is the
    undefined-evaluation one ending at the state after `inc_drop`
    (FR-363-AC-9, FR-365-AC-6).
