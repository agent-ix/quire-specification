---
id: TC-367
title: "Clock and timed model declarations"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-415
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-144
    type: verifies
---

# TC-367: Clock and timed model declarations

## Description

Admit and refuse time sources, clocks, resets, clock constraints, time invariants and urgency.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| CK-01 | `time Rpc dense unit ms;` and `time Ctl tick Ctl::tick period rational(1, 2) ms unit ms;` | Both admitted; `Rpc` is a timed model; the tick period is `1/2` ms |
| CK-02 | Two `time` members on one model | `invalid_timed_model`/`duplicate-time-source` |
| CK-03 | A `Clock` field on a `time tick` model | `clock-without-dense-time` |
| CK-04 | `post self.x = 0`; `post self.x = rational(3, 2) ms`; `post self.x = pre(self.x)` | First two admitted; the third `clock-write` |
| CK-05 | Guards `self.x > 1 ms` and `self.x <= 3 ms`; `self.count = self.x`; `self.x <= self.y`; `self.x >= -1 ms` | Guards admitted; `clock-read-outside-constraint`; `clock-comparison`; `negative-clock-constant` |
| CK-06 | `time invariant Rpc when { self.inflight } { self.x <= 3 ms }`; one with body `self.x >= 1 ms` | Admitted; `time-invariant-shape` |
| CK-07 | `urgent Rpc when { self.phase = Done };` `urgent Call::reset;` with `reset` reading no clock; `urgent Call::reply;` | Admitted; admitted; `urgent-guard-reads-clock` |
| CK-08 | A constant `rational(1, 3) ms` with unit `us` | Converted exactly to `1000/3 us` |
| CK-09 | `record Timer { x: Clock; n: Int }` held by object field `t`; guard `self.t.x >= 1 ms`; `post self.t = Timer { x: 0 ms, n: 3 }`; `post self.u = self.t` | Guard and reset admitted over the clock `t.x` of each object; the last `clock-read-outside-constraint` |
| CK-10 | `predicate within(c: Clock, b: Quantity<ms>) { c <= b }` called in a precondition as `within(self.x, 3 ms)`, in a time invariant body, and in an `invariant` clause | The first two expand to the constraint `x <= 3 ms`; the third `clock-read-outside-constraint` |
| CK-11 | `function pick(a: Clock, b: Clock, left: Boolean): Clock { if left then a else b }` used as `pick(self.x, self.y, self.l) >= 1 ms` | Admitted; the guard is the disjunction `(self.l and x >= 1 ms) or (not self.l and y >= 1 ms)` |
| CK-12 | `type Alias = Clock;` and a field `w: Option<Clock>`; guards `present(self.w) and value(self.w) >= 1 ms` and `value(self.w) >= 1 ms`; `post self.w = 0 ms` | The first guard admitted; the second `undefined_expression`/`unproved-presence`; the write makes `w` present with clock 0 |
| CK-13 | Fields `s: Set<Clock>`, `q: Sequence<Timer>`, `b: Bag<Clock>`, `m: Map<Text, Timer>`, `r: Option<Sequence<Clock>>` and `ts: Timer` with multiplicity `[0, 3]`; a field `k: Map<Timer, Int>` | All admitted, `s` holding at most one element and `b` one element for each occurrence; `k` `ill_typed`/`operator-ineligible` |
| CK-14 | Operation `reset(c: Clock)` with `post c = 0` and a frame naming `Call.x`; the same with an empty frame; a predicate with bound parameter `b` called as `within(self.x, self.limit)` | The first admitted, `c` ranging over the places of `Call.x`; the second `clock-write`; the third `clock-bound-not-constant` |
| CK-15 | A model with `ts: Sequence<Timer>[0, 3]` on each of two objects; the initial state, then a delay of `2 ms`, then `self.ts = sequence[Timer { x: 0 ms, n: 1 }]` | No live place initially; after the write the place `ts.0.x` of that object is live at `0`, and after a further `2 ms` delay it is `2` while the other object has no live place |
| CK-16 | `self.ts = filter(e in self.ts: e.n > 1)` and `self.ts = flatten(sequence[sequence[Timer { x: 0 ms, n: 9 }], self.ts])` over three elements with clocks `1`, `2`, `3` ms and `n` of `1`, `2`, `3`; a set literal with two elements of equal `n` and clocks `0` and `5 ms`; an element with `x` copied from a clock | The filter leaves the clocks `2` and `3` at slots 0 and 1; the flatten gives a fresh slot 0 at `0` and moves the old clocks to slots 1 to 3; the set holds one element with clock `0`; the last `clock-write` |
| CK-17 | `self.ts[1].x = 0 ms` with and without `1 < size(self.ts)` in the precondition; `self.m[k].x = 0 ms`; `forall(e in self.ts: e.n > 1 implies e.x = 0 ms)` | Reset of slot 1; `undefined_expression`/`unproved-presence` at `1` without the precondition; reset by key; the quantified reset changes exactly the places with `n > 1` |
| CK-18 | Guards `forall(e in self.ts: e.x <= 3 ms)` and `exists(e in self.ts: e.n = 2 and e.x >= 1 ms)` over the empty sequence and over two elements; the time invariant `forall(e in self.ts: e.x <= 3 ms)`; `count<Int>(e in self.ts: e.x >= 1 ms)` | The first guard true on the empty sequence and the conjunction over two places; the second false on the empty sequence and the disjunction over the places with `n = 2`; the invariant bounds each live place while live; the count `clock-read-outside-constraint` |
| CK-19 | `self.a = self.b` and `self.a != self.b` with `a` and `b` of type `Sequence<Timer>` | `clock-read-outside-constraint` for each; the state key of the two states differing only in clock values is equal |
| CK-20 | `o: Option<Timer>`, `q: Sequence<Option<Clock>>`; `value(self.o).x >= 1 ms` with and without `present(self.o)`; `self.o = Timer { x: 0 ms, n: 1 }`, `self.o = absent`; `arm(c: Option<Clock>)` with `c = 0 ms`; `maybe(o: Timer): Option<Clock>` | Guard admitted with `present(self.o)` and `unproved-presence` without; the write makes `o.x` present at `0` and the second drops it; one instance of `arm` for each guarded place; `maybe` a place expression |
| CK-21 | CK-01's declarations in a unit whose `profile` declaration names `quire.temporal.complete/v1`, then in a unit whose declaration names `quire.model.complete/v1`, then in a unit with no `profile` declaration | Admitted in the first; in the others each `time` member and each `Clock` type refuses `unsupported_construct` naming the unit's selected layers and the required layer `quire.temporal.complete/v1` |
| CK-22 | A clock collection with no declared maximum whose live clocks exceed `model_check.max_clocks`; a request with `limits.max_clocks` `2048`; a request with `limits.max_clocks` `0` | `incomplete` with `limit-reached{limit: max_clocks, value, setting: model_check.max_clocks}` (provider vector `limit-reached-max-clocks`); the raised bound is accepted as a request member (request vector `temporal-request`) and lets the same item finish; the zero is refused by the schema (adverse vector `temporal-limits-max-clocks-zero`) |
| CK-23 | Object `Call` with clocks `x`, `y` and `r: Reference<Clock>`, two calls, `time dense unit ms`; guard `deref(self.r) >= 1 ms`; `post self.r = ref(self.x)`; `ref(self.x) = ref(other.x)`; `ref(self.x)` of `Call1` and of `Call2`; two states differing only in `r`; `post deref(self.r) = 0 ms` with a frame naming only `Call.x`; `deref` of `Option<Reference<Clock>>` with and without `present(self.r)` | Guard is the disjunction over four places, each with the condition `self.r = ref(place)`; the write resets no clock; the comparison is false for two calls; the two `ref(self.x)` have different place ids and the state key holds that id for `r`; the two states differing in `r` have different state keys; `clock-write`; `unproved-presence` without `present`, admitted with it; states differing only in clock values share a state key |
| CK-24 | Constraint `max "3"` with `appliesTo` the alias `Timeout` of `Clock`; the same with `appliesTo` `Clock`; `min "1"`; keyword `minLength`; the first under `time tick` | `x <= 3` on every `Timeout` place only; on every clock place; `time-invariant-shape`; `malformed-declaration` at the constraint; `clock-without-dense-time` |
| CK-25 | The checked-package V2 dense fixture `proposals/checked-package-v2/fixtures/positive-timed-model-nodes.json` carries the clock place, reset, guard, invariant, both urgency variants, and constraint under a `dense` source; the separate `positive-tick-timed-model-nodes.json` fixture carries its `tick` source, model, operation anchor, and period with empty clock-place, reset, guard, invariant, urgency, and constraint collections. Validate both fixtures and apply every mutation from `proposals/checked-package-v2/timed-model-node-vectors.json` to its named fixture, including TM-23's tick source with a clock place and TM-24..TM-29's dense/tick source-unit dimension mismatches. | Both positive fixtures admit with self-typed timed nodes, resolved references, current node keys, and independently recomputed package identities; TM-23 refuses `invalid_package`/`clock-without-dense-time` at the clock place's `model` binding; TM-24..TM-29 refuse `ill_typed`/`invalid-time-quantity` at the source unit binding for `Length`, derived non-Time and derived-Time dimensions; each malformed child is refused by the closed schema or its pinned semantic diagnostic; the timed-model step runs after hybrid and before protocol (FR-322-AC-121, FR-322-AC-126, FR-322-AC-127) |

## Expected Results

Each declaration is admitted or refused exactly as stated, with the FR-272 cause and payload.
