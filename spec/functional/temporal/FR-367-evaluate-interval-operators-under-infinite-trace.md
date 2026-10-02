---
id: FR-367
title: "Evaluate interval operators inside infinite-trace formulas"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-092
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-367: Evaluate interval operators inside infinite-trace formulas

## Description

Where the infinite-trace profile is selected, the evaluator SHALL give every interval-capable temporal operator written with a closed
interval its offset meaning over the infinite trace, nested in any order with
unbounded operators.

## Inputs

A checked infinite-trace clause, its trace or model subject, and the request's
caller-configurable limits.

## Outputs

The formula's infinite-trace truth on each trace; a located refusal for an
inadmissible interval; or, for an analysis stopped by its automaton-state
limit, the FR-363 stopped verdict naming that limit.

## Behavior

### Admission

Under the infinite-trace profile each of `eventually`, `always`, `until`,
`release`, `once`, `historically`, `since` and `triggered` is written either
with a closed interval `[a,b]`, integers `0 <= a <= b`, making it an
**interval operator**, or with no interval, making it an **unbounded
operator**. The two nest in any order, so
`always (holds(fail) implies eventually always[0,10] holds(healthy))` is
admitted. Reversed bounds `a > b` parse and refuse semantic admission, as under
every profile. An interval's key carries the infinite-trace profile identity
([FR-255](../../objects/temporal/FR-255-bounded-temporal-interval.md)), so it is
never equal to an interval under a bounded profile.

### Meaning

An interval operator has the offset meaning of
[FR-091](FR-091-evaluate-bounded-future.md) for the future operators and of
[FR-092](FR-092-evaluate-bounded-past.md) for the past operators, read over the
infinite trace. Every future position exists, so no closed-boundary rule
applies. Position 0 is the trace's authoritative origin, so a past interval
that reaches before position 0 reads atomic predicates there as false, FR-092's
complete-history rule. An unbounded operator has the meaning
[FR-161](FR-161-analyze-infinite-trace-properties.md) states.

### Previous

Strong previous is `once[1,1]` under every profile, the infinite-trace profile
included: at position `i > 0` it is its operand's truth at `i - 1`, and at
position 0 it is false. It has no separate spelling and no unbounded form; the
unbounded past operators are `once`, `historically`, `since` and `triggered`.

### Safety fragment

The safety fragment admits interval operators: the truth of an interval
operator at a position is decided within `b` positions of it. So
`always (holds(req) implies eventually[0,5] holds(ack))` is a safety formula,
and a formula with an interval operator under an unbounded `eventually` or
`until`, such as the recovery-stability formula above, is a liveness formula
([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)).

### Stutter invariance

An interval operator counts positions, previous included, so a formula that
contains one is not stutter-invariant: inserting a step that repeats a state
can change its truth. A formula built only from atoms, constants, Boolean
connectives and unbounded operators is stutter-invariant. A transfer of a claim
through a refinement mapping or a reduction that relies on stutter invariance
applies only to a stutter-invariant formula.

### Automaton-state limit

An analysis that translates a formula into a property automaton runs under the
caller-configurable limit `max_automaton_states`, with a published default of
1,048,576 (2^20) automaton states, which a request sets with its other limits.
The analysis counts distinct automaton states as it materializes them, with
checked arithmetic. Reaching the limit stops the run and settles the FR-363
stopped verdict: FR-360 `failed`, execution `resource-incomplete`, FR-331
result `incomplete`, cause `resource_exhausted`/`insufficient-next-charge`,
naming `max_automaton_states`, its value and the request member that raises
it. No limit applies to interval length beyond the integer range of its
bounds.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-367-AC-1 | Under the infinite-trace profile, each of the eight operators is admitted with an interval `[a,b]` and with none, nested in any order; `[3,1]` parses and refuses semantic admission. | Test (TC-238, TC-298) |
| FR-367-AC-2 | `always (holds(fail) implies eventually always[0,10] holds(healthy))` is true on a lasso whose every failure is followed by ten consecutive healthy positions and false on a lasso whose loop never holds ten consecutive healthy positions after a failure. | Test (TC-298) |
| FR-367-AC-3 | `once[1,1] holds(p)` is false at position 0 of every trace and equals `holds(p)` at the previous position elsewhere, under the infinite-trace profile. | Test (TC-298) |
| FR-367-AC-4 | Inserting one repeated state into a trace changes the truth of `always (holds(p) implies once[1,1] holds(q))` and of an `eventually[0,1]` formula, and leaves the truth of a formula with only unbounded operators unchanged. | Test (TC-298) |
| FR-367-AC-5 | An analysis whose property automaton exceeds `max_automaton_states` settles FR-331 `incomplete`, FR-360 `failed`, execution `resource-incomplete`, naming the limit, its value and the request member that raises it; the same request with the limit raised completes. | Test (TC-298) |
| FR-367-AC-6 | A past interval reaching before position 0 reads atomic predicates there as false, and an interval operator under infinite-trace never compares equal to the same numeric interval under a bounded profile. | Test (TC-298) |

## Dependencies

- [FR-091](FR-091-evaluate-bounded-future.md) and
  [FR-092](FR-092-evaluate-bounded-past.md) own the offset meaning.
- [FR-161](FR-161-analyze-infinite-trace-properties.md) owns the unbounded
  operators.
- [FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md) owns the
  property forms and the stopped verdict.

## References

- quire-spec-language ADR-018 §11 (IV-1 to IV-7), RU-4 and QS-13; Linear
  STD-131 (paired with QSL-366) and STD-99.
