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
| WA-08 | Event-position `x` values `2, 1, 0, 4`; `sum of 6 / e.x over past[0, 3] <= 20` with its clause result; at position 3 the atoms of that sum, of the guarded `sum of 6 / e.x where holds(e.x != 0) over past[0, 3] <= 20`, of `count where holds(6 / e.x > 1) over past[0, 3] >= 1`, and of the unguarded and guarded sums with `min_count 5` | Clause result `UndefinedEvaluation{where: position 2, cause: division-by-zero}`, category violation. At position 3: the unguarded sum, the undefined filter and the unguarded `min_count 5` sum read `Undefined` with `division-by-zero`, not `10.5` and not `insufficient-data`; the guarded sum is `10.5` and holds; the guarded `min_count 5` sum reads `insufficient-data{count: 3, min_count: 5}` |
| WA-09 | `x` values `1, 0, 2, 3`, `sum of 6 / e.x over past[0, 3]`, `max_window_values` 2 | Clause result `UndefinedEvaluation{where: position 1, cause: division-by-zero}`; the atom at position 3 reads `Undefined` with `division-by-zero`, not `incomplete`, charging 2 position units (a newest-first visit would charge 3) |
| WA-10 | A filter rejecting 10 of 20 window positions | 20 position units charged |
| WA-11 | Each FR-414-AC-9 clause, checked alone: `count over past[0, 9]` over a model subject; `past[0, 300 s]` under event-position; `past[0, 9]` under timestamped-event; `past[5, 2]` under event-position; `quantile 1`, `quantile 0`, `quantile 3/2`; `min_count 0`; threshold `5 m` against a time value; one term with `quantile 1` and `min_count 0` | In order: `unsupported_construct`/`expression-form` at the `count` term; `ill_typed`/`unit-mismatch` at `300 s`; `ill_typed`/`unit-mismatch` at `0`; `invalid_package`/`invalid-value` at `past[5, 2]`; `invalid_package`/`invalid-value` at each `q` literal; `invalid_package`/`invalid-value` at `0`; `ill_typed`/`unit-mismatch` at `5 m`; the two `invalid-value` refusals, the quantile's first |
| WA-12 | Under event-position, `count over past[5, 2 s] >= 1` and `quantile 1 of e.latency over past[0, 9] <= 5 m` against a time value; over a model subject, `count over past[0, 300 s] >= 1` and `quantile 1 of x.v over past[5, 2] <= 1` | Exactly these refusals, in this order and no others: `unit-mismatch` at `2 s`; `invalid-value` at `1`, then `unit-mismatch` at `5 m`; `expression-form` at the `count` term; `expression-form` at the `quantile` term, then `invalid-value` at `1` |
| WA-13 | Under event-position, `count over past[0, 9] >= 1` beside `count over past[0, 9] min_count 1 >= 1`; over `x` values `1, 2`, `count where holds(e.x > 5) over past[0, 9] >= 1` without `min_count` and `count over past[0, 9] min_count 2 >= 1` | One checked term with `min_count` 1 for the first two, and one node id for two clauses that differ only in that spelling, each `true` at position 0 of a one-event trace; `insufficient-data{count: 0, min_count: 1}` at position 1 for the filtered count; `insufficient-data{count: 1, min_count: 2}` at position 0 and `true` at position 1 for `min_count 2` |

## Expected Results

Every value and outcome is exactly as stated.
