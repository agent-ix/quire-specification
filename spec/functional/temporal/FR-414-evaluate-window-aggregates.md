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
  - target: ix://agent-ix/quire-specification/FR-154
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
no value with cause `insufficient-data{count, min_count}` when the window lacks data, or `Undefined` carrying the cause of the first undefined window position (an `UndefinedEvaluation`), or `incomplete`, `limit-reached` naming the limit.

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

### Check-time refusals

The checker SHALL refuse a malformed aggregate atom at check time, each fault
with its code, cause and locus:

- An aggregate term in a claim over a model subject:
  `unsupported_construct`/`expression-form` at the aggregate term.
- A bound of `past[a, b]` whose kind the profile does not take (a duration
  under event-position, a position count under timestamped-event):
  `ill_typed`/`unit-mismatch` at the first such bound.
- A window with `a > b`: `invalid_package`/`invalid-value` at the window
  `past[a, b]`. Neither bound alone is wrong, so the locus is the whole
  window.
- `quantile q` with `q` outside `0 < q < 1`: `invalid_package`/`invalid-value`
  at the literal `q`.
- `min_count 0` on a `min`, `max`, `fraction` or `quantile` term:
  `invalid_package`/`invalid-value` at the literal.
- A threshold of another dimension than the aggregate's value:
  `ill_typed`/`unit-mismatch` at the threshold.

An aggregate atom (the aggregate term with its comparison and threshold) with
several of these faults SHALL report each, in source order. When a bound of
`past[a, b]` has the wrong kind, the checker SHALL report that refusal and
SHALL NOT compare `a` with `b`, since the two bounds are not of one kind. In a
claim over a model subject, which selects no window profile, the checker SHALL
NOT run the window-kind or `a > b` check; it SHALL report the term's
`expression-form` refusal, then any `quantile`, `min_count` or threshold
fault, in source order.

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

The checker SHALL type the value of a `sum`, `min`, `max` or `quantile` term
as the type of its value expression: `Integer` or `Int[..]`, `Rational` or
`Rational[..]`, and for a value of `Quantity<u>` the type `Quantity<u>` in
that very unit `u`, whichever owner declares `u`. A unit of an admitted domain
package is such a unit: a value of `Quantity<M::ms>` (FR-142 "Namespaces",
FR-142-AC-19, FR-142-AC-20) gives `Quantity<M::ms>` over that package unit's node, and its
threshold converts exactly into `M::ms` from any unit of the package's
dimension, while a unit of another owner's dimension of the same name, with
no dimension equivalence declared, is another dimension and refuses as a
threshold of another dimension does (FR-142-AC-21). A bounded value
`Quantity<u>[min, max]` is read as `Quantity<u>` (FR-154 "Unit bounds"), so
its `sum` is not bounded by the member's bounds.

A carried quantity `Quantity<u, C>` (FR-142 "Carried quantity") is not an
operand of equality or ordering, and every aggregate atom compares its term
with a threshold, so the checker SHALL refuse a value expression or a
threshold of a carried type `ill_typed`/`type-mismatch` at that expression,
before any charge, for each of `sum`, `min`, `max` and `quantile`. No value
is converted implicitly: the exact magnitude is aggregated by writing
`convert<Quantity<u>>(e)` as the value expression, and FR-142 "Carried
quantity sum" serves a `sum<N>` collection query over carried values. The
refused value's threshold is still checked against `Quantity<u>`, so a
threshold of another dimension than `u` adds its `unit-mismatch` refusal in
source order.

### Insufficient data

An aggregate term has a `min_count`. A `min`, `max`, `fraction` or `quantile`
term has no value over an empty set, so its `min_count` is at least 1. A
`count` or `sum` term may have `min_count 0`: over a window with no filtered
value its value is 0 (for `sum`, 0 in the value's unit), so a claim can state
that a window with no match is acceptable, as
`count where holds(e is Error) over past[0, 9] min_count 0 <= 2`. The term may omit `min_count`, and a
term that omits it has `min_count` 1: its checked term is the term written
`min_count 1`, so the two spellings have one checked meaning, and two clauses
that differ only in this spelling have one node id and one obligation identity. A window holding fewer filtered
values gives the atom no value with cause `insufficient-data{count,
min_count}`. This means the trace lacks the data to compute the aggregate; it
is not an undefined evaluation of the claim, and the `undefined-evaluation`
refutation rule for model engines does not apply to it. The evaluator SHALL
settle the monitor clause at that position in the O-16 category `undefined`
with cause `insufficient-data{count, min_count}`, naming the position, the
aggregate term's locus, the filtered count and `min_count`; it is neither a
violation nor a holding result for that position.

### Undefined positions

The evaluator visits the window's positions oldest first. When the filter is
undefined at a visited position, or the value expression or the `fraction`
predicate is undefined at a position the filter admits (for example a
division by zero), the atom evaluates `Undefined` with the cause of the first
such position, and the evaluator stops there. This is an undefined evaluation
of the clause: the result is `UndefinedEvaluation{where, cause}`, `where` the
first position at which the atom is undefined (QSL FR-327), in the O-16 category violation,
as for the same expression in a plain `holds` atom. The evaluator never skips
the position, so a window result never improves by the loss of a position it
could not evaluate. A value expression is read only at admitted positions.
Later positions are not visited, so a later overflow of `max_window_values`
is not reached. `insufficient-data` is not this case: it stays O-16
`undefined` and is never `UndefinedEvaluation`. When the window is also short
of `min_count`, the undefined position's cause wins, the first fault found.

### Memory

The evaluator keeps the window's values. `max_window_values` is a run limit
the caller sets per run, with a published default. A window that exceeds it
yields `incomplete`, `limit-reached` naming `max_window_values`, its value
and the request setting that raises it, never a truth value. Evaluation charges
one unit of clause work at `temporal.visit` (`quire.value.accounting/v1`) per window position visited, whether or not the filter
admits it, and under a stop at an undefined position, the positions visited
up to and including it.

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
| FR-414-AC-7 | The atom is `Undefined` with the cause of the first undefined window position oldest first, when the filter is undefined at a position or the value expression or `fraction` predicate is undefined at an admitted position; the clause result is `UndefinedEvaluation` in the O-16 category violation, never `undefined` and never a holding result; a position is never skipped, positions after it are not visited, and a guard filter that rejects the position keeps the atom defined. | Test (TC-366) |
| FR-414-AC-8 | One evaluation charges one unit of clause work at `temporal.visit` per window position visited, including positions the filter rejects, and up to and including the first undefined position. | Test (TC-366) |
| FR-414-AC-9 | Each check-time refusal at its locus: `count over past[0, 9]` in a claim over a model subject, `unsupported_construct`/`expression-form` at the `count` term; `past[0, 300 s]` under event-position, `ill_typed`/`unit-mismatch` at `300 s`; `past[0, 9]` under timestamped-event, `ill_typed`/`unit-mismatch` at `0`; `past[5, 2]` under event-position, `invalid_package`/`invalid-value` at `past[5, 2]`; `quantile 1`, `quantile 0` and `quantile 3/2`, `invalid_package`/`invalid-value` at the `q` literal; `max of e.v over past[0, 9] min_count 0 <= 1`, `invalid_package`/`invalid-value` at `0`; a threshold `5 m` against a time value, `ill_typed`/`unit-mismatch` at `5 m`. A term with `quantile 1` and `min_count 0` reports both refusals, the quantile's first. | Test (TC-366) |
| FR-414-AC-10 | Under event-position, `count over past[5, 2 s] >= 1` reports exactly one refusal, `ill_typed`/`unit-mismatch` at `2 s`, and no `invalid-value`. Under event-position, `quantile 1 of e.latency over past[0, 9] <= 5 m` against a time value reports `invalid_package`/`invalid-value` at `1` and then `ill_typed`/`unit-mismatch` at `5 m`. In a claim over a model subject, `count over past[0, 300 s] >= 1` reports only `unsupported_construct`/`expression-form` at the `count` term, and `quantile 1 of x.v over past[5, 2] <= 1` reports `expression-form` at the term and then `invalid-value` at `1`, with no `a > b` refusal. | Test (TC-366) |
| FR-414-AC-11 | Under event-position, `count over past[0, 9] >= 1` and `count over past[0, 9] min_count 1 >= 1` check to one checked term with `min_count` 1, and two clauses `always[0,0] (…)` over them have one node id; at position 0 of a one-event trace each atom is `true`. Over `x` values `1, 2` at positions 0 and 1, the atom `count where holds(e.x > 5) over past[0, 9] >= 1`, written without `min_count`, has no value at position 1, cause `insufficient-data{count: 0, min_count: 1}`; `count over past[0, 9] min_count 2 >= 1` has no value at position 0, cause `insufficient-data{count: 1, min_count: 2}`, and the value `true` at position 1. | Test (TC-366) |
| FR-414-AC-12 | Under event-position over `x` values `1, 2` at positions 0 and 1, at position 1: `count where holds(e.x > 5) over past[0, 9] min_count 0 <= 2` is `true` (count 0), the same term `>= 1` is `false`, and `sum of e.x where holds(e.x > 5) over past[0, 9] min_count 0 = 0` is `true`, none of them `insufficient-data`; without `min_count` the `count` has no value, `insufficient-data{count: 0, min_count: 1}`. `min`, `max`, `fraction holds(e.x > 1)` and `quantile 0.5` of `e.x`, each with `min_count 0`, refuse `invalid_package`/`invalid-value` at `0`. | Test (TC-366) |
| FR-414-AC-13 | With `model M = "acme/rpc"` over a package this criterion defines, declaring base dimension `Time`, its root unit `s` and a unit `ms` of `Time` (`target` `s`, `scale` `1/1000`), and the unit of compilation declaring `dimension Time; unit s : Time = rational(1, 1); unit ms : Time = rational(1, 1000) * s;`, under event-position over a trace whose event field `latency` has type `Quantity<M::ms>` and values `5`, `1` and `4` of `M::ms` at positions 0 to 2, at position 2: `sum`, `min`, `max` and `quantile 0.5` of `e.latency` over `past[0, 2]` have type `Quantity<M::ms>` over the package unit's node and the values `10`, `1`, `5` and `4` of `M::ms`; `sum of e.latency over past[0, 2] <= 1 M::s` is `true`, its threshold converted exactly to `1000` of `M::ms`; `sum of e.latency where holds(e.latency > 9 M::ms) over past[0, 2] min_count 0 = 0 M::ms` is `true`; the thresholds `10 ms` and `10 s` of the source units each refuse `ill_typed`/`unit-mismatch` at the threshold, the source `Time` being a dimension distinct from the package's (FR-142-AC-20). | Test (TC-366) |
| FR-414-AC-14 | With `dimension Length; unit m : Length = rational(1, 1);` and `type Lc = Quantity<m, Int[-5, 5]>;`, under event-position over event field `d` of type `Quantity<m>`, `sum`, `min`, `max` and `quantile 0.5` of `convert<Lc>(e.d)` over `past[0, 2]` with threshold `5 m` each refuse exactly one `ill_typed`/`type-mismatch`, at the value expression `convert<Lc>(e.d)`, before any charge, and with threshold `5 s` (beside `dimension Time; unit s : Time = rational(1, 1);`) refuse that and then `ill_typed`/`unit-mismatch` at `5 s`; `sum of e.d over past[0, 2] <= convert<Lc>(5 m)` refuses `ill_typed`/`type-mismatch` at the threshold; `sum of convert<Quantity<m>>(convert<Lc>(e.d)) over past[0, 2] <= 5 m` is admitted with value type `Quantity<m>`, and over `d` values `2`, `-1` and `3` of `m` its sum at position 2 is `4 m`. | Test (TC-366) |

## Dependencies

- [FR-090](FR-090-select-temporal-profile-and-clock.md) and
  [FR-092](FR-092-evaluate-bounded-past.md) supply the profiles and past
  windows.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  exact values and units.
- [FR-095](FR-095-preserve-native-tl-correspondence.md) lowers the Boolean
  temporal layer.

## References

- QSL FR-194 states the same rule, charge and check-time refusals in the QSL
  half (FR-194-AC-7, FR-194-AC-8).
- QSL ADR-024 WA-1 to WA-8 (WA-4: insufficient data is O-16 undefined).
- Ticket: STD-137 (paired with QSL-371).
