---
id: TC-323
title: "Partial-order reduction vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-383-AC-17
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-385
    type: verifies
---

# TC-323: Partial-order reduction vectors

## Description

Reduce three independent counters by partial-order reduction, find the
deadlock the unreduced search finds, refuse a position-counting form before
expansion, and settle a run to its search horizon.

## Test Procedure

Subject: object type `Counter` with `v: Int[0, 2]`, population `counters`,
universe `{a, b, c}`, all at 0. Operation `bump()` with the receiver-scoped
frame `modifies self.v`, precondition `self.v < 2` and postcondition
`self.v = pre(self.v) + 1`. No `terminal` member. Write a state
`(va, vb, vc)`. Request: `partial_order: {"proviso": "breadth-first-revisit"}`.

1. Check `ReachesTwo`, `eventually holds(x.v = 2)` over `(x: Counter)`
   `on origin` under the infinite-trace profile with an empty fairness set,
   for `x = a`, with and without the selection; record the ample set at each
   expanded state and the stored state and transition counts.
2. Read the derived deadlock-freedom item's verdict in both runs and replay
   the reduced run's counterexample.
3. Check `always[0,2] holds(x.v = 0)` under event-position false-extension,
   `on origin`, for `x = a`, with the selection.
4. Repeat case 1 with the search horizon `max_depth` set to 3, with the selection;
   then with a symmetry declaration `[[a, b, c]]` on an annotated variant and
   no partial-order selection.

5. **Enabling through a postcondition.** Object `g` with `armed` (true),
   `alarm` (false) and `cell: Reference<Cell>` set to `a`; Cell `a` with
   `v: Int[0, 1]` (0). `disarm()` on `g`: frame `modifies self.armed`, post
   `not self.armed`. `boom()` on `g`: frame `modifies self.alarm`, pre
   `self.armed`, post `self.alarm and pre(deref(self.cell).v) = 1`. `set()`
   on `a`: frame `modifies self.v`, pre `self.v = 0`, post `self.v = 1`.
   Canonical order `disarm`, `boom`, `set`. Check `always holds(not
   g.alarm)` with and without partial-order reduction; record `boom`'s
   enabling footprint and the ample set at the initial state.
6. **Deleting a receiver.** Population `jobs` of `Job` with fields `armed`,
   `cancellable`, `bad: Boolean` and `peer: Option<Reference<Job>>`, universe
   `{j, k}`: `j.armed` and `j.cancellable` true, `k` neither, `j.peer = k`.
   `boom()`: every-object frame `modifies bad`, pre `self.armed`, post
   `deref(self.peer).bad`. `cancel()`: frame `deletes self`, pre
   `self.cancellable`, deletes its receiver. Check `always holds(not
   k.bad)` with and without partial-order reduction; decide the dependence
   of `cancel(j)` and `boom(j)`.
7. **Deleting an object that is not the receiver.** Case 6 with
   `j.cancellable` false, a field `target: Option<Reference<Job>>`, the
   universe `{j, k, m}` with Job `m` (`cancellable` true, `armed` false,
   `target = j`), and `cancel()` changed to the frame `deletes Job` and
   `modifies self.target`, pre `self.cancellable and present(self.target)`,
   post: the object `pre(self.target)` is no longer a member of `jobs`, every
   other job stays a member, and `self.target` is absent. Check
   `always holds(not k.bad)` with and without
   partial-order reduction; record `W(cancel(m))` and the dependence of
   `cancel(m)` and `boom(j)`.
8. **A delete blocked by a reference.** Population `jobs` of `Job` with
   fields `cancellable: Boolean`, `x: Int[0, 1]` and
   `peer: Option<Reference<Job>>`, universe `{j, k}`: `j.peer = k`,
   `k.cancellable` true, `j.cancellable` false, both `x` 0. `lock()`: frame
   `modifies self.cancellable`, pre `self.cancellable`, post
   `not self.cancellable`. `unlink()`: frame `modifies self.peer`, pre
   `present(self.peer)`, post `not present(self.peer)`. `cancel()`: frame
   `deletes self`, pre `self.cancellable`. Canonical order `lock`, `unlink`,
   `cancel`. Check `always holds(k.x = 0)` with and without partial-order
   reduction; record `cancel(k)`'s enabling footprint and the ample set at
   the initial state.
9. **A navigation through a rewritten reference.** Object `g` with `armed`
   (true), `alarm` (false), `cell: Reference<Cell>` set to `a` and
   `alt: Reference<Cell>` set to `b`; Cells `a` (`v = 0`) and `b`
   (`v = 1`). `disarm()`: frame `modifies self.armed`, post
   `not self.armed`. `point()`: frame `modifies self.cell`, pre
   `self.cell != self.alt`, post `self.cell = pre(self.alt)`. `boom()`:
   frame `modifies self.alarm`, pre `self.armed and deref(self.cell).v = 1`,
   post `self.alarm`. Canonical order `disarm`, `boom`, `point`. Check
   `always holds(not g.alarm)` with and without partial-order reduction;
   record `boom`'s enabling footprint and the ample set at the initial
   state.
10. **A postcondition reading through `result`.** Object `g` with `armed`
    (true) and `alarm` (false); Cell `a` (`v = 0`) in population `cells`.
    `disarm()`: frame `modifies self.armed`, post `not self.armed`. `set()`
    on `a`: frame `modifies self.v`, pre `self.v = 0`, post `self.v = 1`.
    `pick(): Reference<Cell>` on `g`: frame `modifies self.alarm`, pre
    `self.armed`, post `self.alarm and result.v = 1`. Canonical order
    `disarm`, `pick`, `set`. Check `always holds(not g.alarm)` with and
    without partial-order reduction; record `pick`'s enabling footprint and
    the ample set at the initial state.
11. **A budget under reduction.** Repeat case 1 with partial-order
    reduction selected and `max_states` set to 3.
12. **A read through a pure function.** Case 5's objects with
    `function hot(c: Reference<Cell>): Boolean = deref(c).v = 1`.
    `disarm()` on `g`: frame `modifies self.armed`, post `not self.armed`.
    `boom()` on `g`: frame `modifies self.alarm`, pre
    `self.armed and hot(self.cell)`, post `self.alarm`. `set()` on `a`:
    frame `modifies self.v`, pre `self.v = 0`, post `self.v = 1`. Canonical
    order `disarm`, `boom`, `set`. Check `always holds(not g.alarm)` with and
    without partial-order reduction; record `boom`'s enabling footprint and
    the ample set at the initial state.

## Expected Results

In case 1 the ample sets are `{bump(b)}` at `(0,0,0)` and `(0,1,0)`,
`{bump(c)}` at `(0,2,0)` and `(0,2,1)`, the full enabled set at `(0,2,2)`
and `(1,2,2)`, and none at `(2,2,2)` (C0); the reduced run stores 7 model
states and 6 transitions against 27 and 54, and 6 product states for the
negation against 18; the item settles `proved`, basis `closed-scope`, proof
basis `reduced` listing `PartialOrder{breadth-first-revisit}`, `certification` `uncertified`
(FR-383-AC-4, FR-385-AC-3). In case 2 both runs settle the deadlock-freedom
item `refuted` at `(2,2,2)`; the reduced counterexample is the six-step
prefix of the ample-set path, names no reduction, and replays
(FR-385-AC-4). Case 3 settles `inconclusive`,
`reduction-not-preserving{partial-order, bounded}`, before any expansion
(FR-385-AC-1, FR-385-AC-2). In case 4 the partial-order run settles
`inconclusive`, FR-241 `completed`, cause `reduction-horizon` with horizon 3,
never `failed` or `resource-incomplete`, and the symmetry-only run settles
`inconclusive` with the cause an unreduced run to horizon 3 settles with
(FR-385-AC-6).

In case 5 `boom`'s enabling footprint holds `Field{g, armed}`,
`Field{g, cell}`, `AnyField{Cell, v}` and the membership of `g`, so `set` is in the candidate seeded by `disarm` through the
disabled `boom`; the reduced run reaches the violation through `set`, `boom`
and settles `refuted` with a replaying prefix, as the unreduced run does, both
storing 6 states (FR-383-AC-9). In case 6 `cancel(j)` writes the membership of
`j`, which `boom(j)` reads, so the two are dependent; `{cancel(j)}` is no ample
set, the reduced run reaches `boom(j)` and settles `refuted`, as the unreduced
run does, both storing 4 states (FR-383-AC-10).
In case 7 `W(cancel(m))` holds `AnyMembership{jobs}`, which meets
the membership of `j` that `boom(j)` reads, so the two are dependent and
`{cancel(m)}` is no ample set; the reduced run reaches `boom(j)` and settles
`refuted`, as the unreduced run does, both storing 4 states (FR-383-AC-10).
In case 8 a post-state of `cancel(k)` is admissible only while no reference
names `k`, so its enabling footprint holds `AnyField{Job, peer}` beside
`Field{k, cancellable}` and the membership of `k`; `unlink(j)` writes
`Field{j, peer}`, which meets it, so the candidate seeded by `lock(k)` holds
the disabled `cancel(k)` and `unlink(j)`, the initial state is fully
expanded, and the reduced run reaches `unlink(j)`, `cancel(k)`, where
`k.x = 0` evaluates undefined; both runs settle `refuted`, cause
`undefined-evaluation`, storing 5 states each (FR-383-AC-12). In case 9
`boom`'s enabling footprint holds `Field{g, armed}`, `Field{g, cell}` and
`AnyField{Cell, v}`; `point()` writes `Field{g, cell}`, so it is in the
candidate seeded by `disarm` through the disabled `boom`, the initial state
is fully expanded, and both runs settle `refuted` through `point`, `boom`,
storing 6 states each (FR-383-AC-11).
In case 10 `pick` is enabled only when some Cell has `v = 1`, so its
enabling footprint holds `AnyField{Cell, v}` and `AnyMembership{cells}`
beside `Field{g, armed}`; `set` writes `Field{a, v}`, which meets it, so the
initial state is fully expanded, and both runs settle `refuted` through
`set`, `pick`, storing 6 states each (FR-383-AC-14).

In case 11 the run stops at its third stored state before completing its
method and settles `incomplete`, FR-241 `resource-incomplete`, cause `limit-reached{limit: max_states, value: 3, setting}`, never `failed` (FR-385-AC-9).

In case 12 `boom`'s enabling footprint holds `Field{g, armed}`,
`Field{g, cell}`, `AnyField{Cell, v}` (read through `hot`'s body) and the
membership of `g`; `set` writes `Field{a, v}`, which meets it, so `set` is in
the candidate seeded by `disarm` through the disabled `boom`, the initial
state is fully expanded, and both runs settle `refuted` through `set`, `boom`
with a replaying prefix, storing 6 states each. Without the call rule the
reduced run would expand only `{disarm}` and report `proved` over 3 states
(FR-383-AC-16).

The directional meet (FR-383-AC-17): a field write against a membership read is independent, a written membership against a field read or a membership read is dependent, and two writes meet in either direction; the counter subject's reduced and unreduced verdicts agree.
