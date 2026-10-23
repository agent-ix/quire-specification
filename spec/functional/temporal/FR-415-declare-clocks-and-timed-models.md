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

### Clock places

A type is **clock-bearing** when it is `Clock`, an alias of a clock-bearing
type, `Option` of one, or a record, tuple or union with a field, element or
case payload of a clock-bearing type. A value of a clock-bearing type is not a
data value: it denotes **clock places**, the clocks it contains, and a place
is the pair of an object of a universe and the path of fields, tuple indexes
and union cases from the object's field to a `Clock`. The clocks of a timed
model are exactly the places of its objects, so their number is fixed by the
universe and the declared types and no state changes it.

- A place under an `Option` or `?` field or a union case is **guarded**: it is
  present at a discrete state exactly when the value on its path holds that
  case (`present(p)` for an optional place `p`), and its clock has a value
  only while it is present. A place that is not guarded is present in every
  state. A guarded place is read in a clock constraint as `value(p)`, which
  carries FR-146's presence obligation: a `value(p)` without `present(p)`
  established by the guard or by the time invariant's predicate refuses
  `undefined_expression`/`unproved-presence` at the read.
- Every present clock is 0 in every initial state. A guarded clock that becomes
  present takes the constant its write gives it (below), and one that becomes
  absent is dropped: only present clocks advance with time.
- A `Clock` parameter is bound by place: the argument of a call is a place, not
  a copy of a value, and the callee reads or resets that same clock. A `Clock`
  result is a place, and a function that returns one returns the place its
  body names.
- A place path that crosses a reference or an `if`/`match` denotes one place
  for each object or branch it can reach, each guarded by the data condition
  that selects it, so a clock constraint over it is the disjunction of those
  guarded constraints.
- A record, tuple or union literal that contains a `Clock` component is a
  write: it is admitted only as the right side of a reset (`self.t =
  Timer { x: 0 ms, n: 3 }`), each of its clock components an exact
  non-negative rational time constant. A write of a constant to a guarded
  place makes it present with that clock, and a write of `absent`, or of
  another union case, drops it.
  Elsewhere it refuses `invalid_timed_model`/`clock-write`.
- A clock-bearing value that is read whole, as an operand of `=`, a function
  argument of data type, an assignment to a data field or a conversion, reads
  its clocks, and refuses `invalid_timed_model`/`clock-read-outside-constraint`.
  The data fields of a clock-bearing record are data values and are read as
  usual.
- A function or predicate with a `Clock` parameter or result, or a parameter or
  result of a clock-bearing type, is a **clock-reading declaration**. Its body
  reads a clock only as the clock operand of an atomic clock constraint or
  passes it to another clock-reading declaration, and a call to it stands only
  where a clock constraint stands (a precondition, a time invariant body or
  the body of another clock-reading declaration); a call anywhere else refuses
  `invalid_timed_model`/`clock-read-outside-constraint`. A clock-reading
  declaration writes no clock: a reset belongs to a postcondition.
- A bound of an atomic clock constraint in a clock-reading declaration is a
  time quantity literal or a quantity parameter whose every call supplies an
  exact non-negative time constant; any other bound refuses
  `invalid_timed_model`/`clock-bound-not-constant`.
- An operation's `Clock` parameter ranges over the clock places of the model,
  as an object parameter ranges over its universe. When the operation's
  postcondition resets the parameter, it ranges over the places of the clock
  fields the operation's frame names; a reset of a clock the frame does not
  name refuses `invalid_timed_model`/`clock-write`.
- A collection (a `Set`, `Bag`, `Sequence`, `OrderedSet`, a map or a field of
  multiplicity other than `[1, 1]`) of a clock-bearing type refuses
  `invalid_timed_model`/`clock-in-collection`, because the number of clocks
  would vary with the state.
- A clock place under `time tick` or in an untimed model refuses
  `invalid_timed_model`/`clock-without-dense-time` at the object field that
  holds it.

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
| FR-415-AC-6 | A record `Timer { x: Clock, n: Int }` held by object field `t`, a `Clock` parameter, a `Clock` result and `type Alias = Clock` each admit the clock rules of the sections above: `t.x`, the argument of a call and the result of a call are read only in atomic clock constraints and reset only by a postcondition reset, `Alias` is `Clock`, and a model with `k` objects and the record field `t` has `k` clocks `t.x`. Reading `t` whole (`self.u = self.t`) refuses `clock-read-outside-constraint`; a record literal with a clock component outside a reset refuses `clock-write`. | Test (TC-367) |
| FR-415-AC-7 | A field `w: Option<Clock>` is a guarded place: it is absent in an initial state whose `w` is absent, `self.w = 0 ms` makes it present with clock 0, writing `absent` drops it, the guard `present(self.w) and value(self.w) >= 1 ms` is false while `w` is absent, the time invariant `when present(self.w) { value(self.w) <= 3 ms }` places no bound while `w` is absent, and `value(self.w) >= 1 ms` with no `present(self.w)` refuses `undefined_expression`/`unproved-presence`. | Test (TC-367) |
| FR-415-AC-8 | Each refuses with its cause: a `Set<Clock>` field and a `Sequence<Timer>` field (`clock-in-collection`); an operation that resets its `Clock` parameter with a frame naming no clock field (`clock-write`); a clock-reading predicate with a bound parameter called with a non-constant argument (`clock-bound-not-constant`); a call to a clock-reading predicate in an `invariant` clause (`clock-read-outside-constraint`); a clock-bearing field under `time tick` (`clock-without-dense-time`). | Test (TC-367) |

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
