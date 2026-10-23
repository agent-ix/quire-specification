---
id: TC-367
title: "Clock and timed model declarations"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-415
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
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
| CK-13 | Fields `s: Set<Clock>` and `q: Sequence<Timer>` | `clock-in-collection` for each |
| CK-14 | Operation `reset(c: Clock)` with `post c = 0` and a frame naming `Call.x`; the same with an empty frame; a predicate with bound parameter `b` called as `within(self.x, self.limit)` | The first admitted, `c` ranging over the places of `Call.x`; the second `clock-write`; the third `clock-bound-not-constant` |

## Expected Results

Each declaration is admitted or refused exactly as stated, with the FR-272 cause and payload.
