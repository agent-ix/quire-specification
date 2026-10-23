---
id: FR-415
title: "Declare clocks and timed models"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-013
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-205
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
---

# FR-415: Declare clocks and timed models

## Description

When a state model declares a time source, the frontend SHALL admit its
clocks, resets, clock constraints, time invariants and urgency only in the
exact forms below, and SHALL keep every clock value out of the value kernel so
that clocks are read only inside atomic clock constraints.

## Inputs

A state model and its operations (FR-013); the shared grammar's timed forms
([shared grammar](../../../proposals/quire-v1/shared-grammar.md)); exact
rational quantities of the time dimension (FR-142, FR-205).

## Outputs

A timed model with its time source, clocks, guards, resets, time invariants
and urgency, or a located typed refusal under `invalid_timed_model`.

## Behavior

### Time source

The timed forms of this requirement (`time`, `time invariant`, `urgent` and
the `Clock` type) belong to the layer `quire.temporal.complete/v1`, which a
`profile` declaration selects (FR-001, [temporal-complete.md](../../../proposals/quire-v1/definitions/temporal-complete.md)).
A unit whose selected layers do not include it, and that writes one of these
forms, refuses `unsupported_construct`, naming the unit's selected layers and
the required layer.

A state model declares at most one `time` member:

- `time M dense unit u` — real-valued clocks in unit `u`;
- `time M tick O period p unit u` — digital time counted by occurrences of
  operation `O` of the model, each occurrence advancing time by `p`.

`u` is a unit of FR-142's time dimension and `p` a positive exact rational
quantity of it. A model with a `time` member is a **timed model**, and its
model subject is a **timed subject**. A second `time` member refuses with
`invalid_timed_model`/`duplicate-time-source`.

### Clocks and resets

Under `time dense`, an object type may declare fields of type `Clock`. Each
object of a universe has its own clock per clock field, so the number of
clocks is the universe's objects times its clock fields. A clock's value is a
non-negative real in unit `u`; every clock is 0 in every initial state, and a
state snapshot carries no clock value. A `Clock` field under `time tick` or in
an untimed model refuses with `invalid_timed_model`/`clock-without-dense-time`.

An operation's postcondition may reset a clock of its frame to an exact
non-negative rational constant: `self.x = 0` or `self.x = c`. A clock in the
frame that is not reset keeps its value. Reset is the one write to a clock;
any other write refuses with `invalid_timed_model`/`clock-write`.

### Clock constraints and guards

An atomic clock constraint is `x ~ c`, with `~` one of `<`, `<=`, `=`, `>=`,
`>` and `c` an exact non-negative rational quantity of the time dimension,
converted exactly to `u`. Strict and non-strict comparisons are both admitted.
A clock is read only inside an atomic clock constraint: a clock read anywhere
else refuses with `invalid_timed_model`/`clock-read-outside-constraint`, and a
constraint that compares two clocks refuses with
`invalid_timed_model`/`clock-comparison`. A negative constant refuses with
`invalid_timed_model`/`negative-clock-constant`.

An operation's precondition is a Boolean combination of data predicates and
atomic clock constraints; its clock part is the operation's **guard**. A
disjunction in a guard is a union of convex constraints.

### Time invariants

A model declares time invariants `time invariant when P { body }`, `P` a data
predicate over the state and `body` a conjunction of upper bounds `x <= c` or
`x < c`. At a state where `P` holds, time may pass only while the body holds.
A body with a lower bound or an equality refuses with
`invalid_timed_model`/`time-invariant-shape`. A time invariant constrains
delays; an `invariant` clause remains a claim.

### Urgency

`urgent when P`, `P` a data predicate, forbids any positive delay at a state
where `P` holds. `urgent O` forbids any positive delay while operation `O` is
enabled, and is admitted only when `O`'s guard reads no clock; otherwise it
refuses with `invalid_timed_model`/`urgent-guard-reads-clock`.

### Atomic steps and resource limits

An operation is one atomic step. Clock values, constants, the clock count and
delay lengths have no language bound; every limit on an analysis is a
caller-set budget with a published default, and reaching it names the limit,
its value and the request setting that raises it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-415-AC-1 | `time M dense unit u` and `time M tick O period p unit u` are admitted with a unit of the time dimension and a positive exact rational period; a second `time` member refuses `invalid_timed_model`/`duplicate-time-source`. | Test (TC-367) |
| FR-415-AC-2 | `Clock` fields are admitted under `time dense` only, every clock is 0 initially, and a reset to an exact non-negative rational is the only admitted write. | Test (TC-367) |
| FR-415-AC-3 | Strict and non-strict atomic clock constraints are admitted with exact conversion to `u`; a clock read outside a constraint, a two-clock comparison and a negative constant refuse with their typed causes. | Test (TC-367) |
| FR-415-AC-4 | A time invariant with upper bounds is admitted; a lower bound or equality in its body refuses `invalid_timed_model`/`time-invariant-shape`. | Test (TC-367) |
| FR-415-AC-5 | `urgent when P` is admitted; `urgent O` is admitted only when `O`'s guard reads no clock. | Test (TC-367) |
| FR-415-AC-9 | A unit whose `profile` declaration names `quire.temporal.complete/v1` admits the forms of AC-1 to AC-5; the same unit whose declaration names `quire.model.complete/v1`, or that has no `profile` declaration, refuses each `time` member, `time invariant`, `urgent` declaration and `Clock` in a type position `unsupported_construct`, naming the unit's selected layers and the required layer `quire.temporal.complete/v1`. | Test (TC-367) |

## Dependencies

- [FR-013](../foundation/FR-013-validate-operation-frames.md) supplies frames
  and operations.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) and
  [FR-205](../../objects/foundation/FR-205-exact-rational-value.md) supply
  exact time quantities.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) gives timed
  subjects their moves; [FR-416](FR-416-evaluate-timed-temporal-claims.md)
  gives timed claims their meaning.

## References

- QSL ADR-026 CK-1 to CK-9, OV-7.
- Owner rulings recorded on QSL-373: dense time is specified, with strict and
  non-strict constraints under timed-automata semantics.
- Ticket: STD-139 (paired with QSL-373).
