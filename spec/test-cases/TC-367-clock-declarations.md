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

## Expected Results

Each declaration is admitted or refused exactly as stated, with the FR-272 cause and payload.
