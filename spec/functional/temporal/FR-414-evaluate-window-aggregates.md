---
id: FR-414
title: "Evaluate window aggregates over observed traces"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-092
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-095
    type: references
---

# FR-414: Evaluate window aggregates over observed traces

## Description

When a temporal clause over a trace subject contains an aggregate atom, the
evaluator SHALL compute the aggregate as a deterministic function of the
positions in its past window and SHALL compare it with its threshold before
temporal evaluation, so that the temporal layer reads a Boolean signal.

## Inputs

A clause under the event-position false-extension or timestamped-event
finite-window profile (FR-090) over a trace subject; its aggregate terms;
the run limit `max_window_values`.

## Outputs

A Boolean value of each aggregate atom at each position, or the kernel's
no value with cause `insufficient-data{count, min_count}` when the window lacks data, or `incomplete`, `limit-reached` naming the limit.

## Behavior

### Aggregate terms and atoms

An aggregate term is one of `count`, `sum`, `min`, `max`, `fraction holds(P)`
and `quantile q`, over a value expression and an optional `where` filter
predicate, over a past window. An aggregate atom compares an aggregate term
with a threshold of its type (`<`, `<=`, `>=`, `>`, `=`), and enters any
bounded-profile formula wherever a `holds` atom may. Aggregate atoms are
admitted over trace subjects; over a model subject the probabilistic measures
of FR-407 serve and an aggregate atom refuses with
`unsupported_construct`/`expression-form`.

### Windows

Under event-position false-extension, `past[a, b]` holds the positions `j`
with `i − b <= j <= i − a` and `j >= 0`. Under timestamped-event
finite-window, it holds the events whose timestamp `t_j` satisfies
`t_i − b <= t_j <= t_i − a`, with `a` and `b` durations converted exactly to
the binding's unit. A window reads no future position, so an aggregate atom
adds nothing to the formula's horizon and needs no lookahead.

### Values

Values are exact integers, exact rationals and quantities with FR-142's
dimension rules. `fraction holds(P)` is the count of filtered positions where
`P` holds over the count of filtered positions. `quantile q` is the
nearest-rank quantile: the `⌈q · n⌉`-th smallest of the `n` filtered values in
the window, `0 < q < 1`. Comparison with the threshold follows FR-142,
including explicit unit conversion within a dimension.

### Insufficient data

An aggregate term declares `min_count >= 1`. A window holding fewer filtered
values gives the atom no value with cause `insufficient-data{count,
min_count}`. This means the trace lacks the data to compute the aggregate; it
is not an undefined evaluation of the claim, and the `undefined-evaluation`
refutation rule for model engines does not apply to it. The evaluator SHALL
settle the monitor clause at that position in the O-16 category `undefined`
with cause `insufficient-data{count, min_count}`, naming the position, the
aggregate term's locus, the filtered count and `min_count`; it is neither a
violation nor a holding result for that position.

### Memory

The evaluator keeps the window's values. `max_window_values` is a run limit
the caller sets per run, with a published default. A window that exceeds it
yields `incomplete`, `limit-reached` naming `max_window_values`, its value
and the request setting that raises it, never a truth value. Evaluation charges
one unit of clause work per value visited.

### Meaning

The truth of an aggregate atom at a position is a deterministic function of
the trace. A violation over an observed trace is a violation of the claim on
that trace; a holding result is `tested`, evidence for that trace only. The
temporal layer stays Boolean over derived signals, so a TL lowering (FR-095)
consumes the derived signal unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-414-AC-1 | `quantile q` returns the `⌈q · n⌉`-th smallest filtered value, and `count`, `sum`, `min`, `max` and `fraction` return their exact values on conformance windows. | Test (TC-366) |
| FR-414-AC-2 | Event-position and timestamped windows select exactly the positions their bounds name, and no future position. | Test (TC-366) |
| FR-414-AC-3 | A window with fewer than `min_count` filtered values gives the atom no value, and the monitor clause at that position settles in the O-16 category `undefined` with `insufficient-data{count, min_count}`, never `refuted` and never `undefined-evaluation`. | Test (TC-366) |
| FR-414-AC-4 | A threshold in another unit of the value's dimension converts exactly; a dimension mismatch refuses `ill_typed`/`unit-mismatch`. | Test (TC-366) |
| FR-414-AC-5 | A window exceeding `max_window_values` yields `incomplete`, `limit-reached` naming the limit, its value and how to raise it. | Test (TC-366) |
| FR-414-AC-6 | An aggregate atom over a model subject refuses `unsupported_construct`/`expression-form`; a holding monitor result is `tested`. | Test (TC-366) |

## Dependencies

- [FR-090](FR-090-select-temporal-profile-and-clock.md) and
  [FR-092](FR-092-evaluate-bounded-past.md) supply the profiles and past
  windows.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  exact values and units.
- [FR-095](FR-095-preserve-native-tl-correspondence.md) lowers the Boolean
  temporal layer.

## References

- QSL ADR-024 WA-1 to WA-8 (WA-4: insufficient data is O-16 undefined).
- Ticket: STD-137 (paired with QSL-371).
