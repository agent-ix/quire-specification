---
id: TC-366
title: "Window aggregates in monitors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-414
    type: verifies
---

# TC-366: Window aggregates in monitors

## Description

Evaluate aggregate atoms over event-position and timestamped windows with exact nearest-rank quantiles.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| WA-01 | Latencies `5, 1, 4, 2, 3` ms in a 5-event window, `quantile 0.95`, `quantile 0.5` | `5 ms` (`⌈0.95 · 5⌉ = 5`th smallest) and `3 ms` (`3`rd smallest) |
| WA-02 | `count`, `sum`, `min`, `max` and `fraction holds(ok)` over the same window with `ok` true at 3 events | `5`, `15 ms`, `1 ms`, `5 ms`, `3/5` |
| WA-03 | `past[0, 300 s]` under timestamped-event at `t_i = 400 s` with events at 50, 100, 250 and 400 s | Selects the events at 100, 250 and 400 s only |
| WA-04 | `min_count 10` over a 5-value window | The monitor clause at that position settles in the O-16 category `undefined`, `insufficient-data{count: 5, min_count: 10}`; not `refuted`, not `undefined-evaluation` |
| WA-05 | Threshold `5000 us` against ms values; a threshold of a length dimension | Exact conversion; `ill_typed`/`unit-mismatch` |
| WA-06 | A window of 11 values with `max_window_values` 10 | `incomplete`, `limit-reached` naming `max_window_values`, value 10 |
| WA-07 | An aggregate atom over a model subject; a holding monitor result | `unsupported_construct`/`expression-form`; `tested` |

## Expected Results

Every value and outcome is exactly as stated.
