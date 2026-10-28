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
  - target: ix://agent-ix/quire-specification/FR-322
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

For a checked tick source, `O` is represented by a `state`/`operation_anchor`
node. Its `context` is the tick source's explicit `model` reference and equals
the timed root's `model` reference. The anchor's operation name resolves to an
operation declared by that model. A function node or an operation anchor for a
different model is not a tick operation.

`u` is a unit of the base dimension declared `Time`; its exact nominal
`dimension_node_id` is copied to the checked timed-model root's explicit
`time_dimension` binding. The root names that base `Time` node, and a checked
package reader requires the source unit's `dimension_node_id` to equal it by
node identity. A unit of base `Length`, a derived non-Time dimension, or a
nominal derived-Time dimension is not a time-source unit, even when its
dimension terms are equivalent. Readers resolve the unit first and follow its
`dimension_node_id`; they do not guess which same-named dimension is meant or
use declaration order to select a node. `p` is a positive exact rational
quantity of `u`. A model with a `time` member is a **timed model**, and its
model subject is a **timed subject**. A second `time` member refuses with
`invalid_timed_model`/`duplicate-time-source`.

### Clocks and resets

Under `time dense`, an object type may declare fields of type `Clock`. Each
object of a universe has its own clock per clock field, so the number of
clocks is the universe's objects times its clock fields. A clock's value is a
non-negative real in unit `u`; every present clock is 0 in every initial state, and a
state snapshot carries no clock value. A `Clock` field under `time tick` or in
an untimed model refuses with `invalid_timed_model`/`clock-without-dense-time`.

An operation's postcondition may reset a clock of its frame to an exact
non-negative rational constant: `self.x = 0` or `self.x = c`. A clock in the
frame that is not reset keeps its value. Reset is the one write to a clock;
any other write refuses with `invalid_timed_model`/`clock-write`.

**No implicit reset on a presence change.** A place becomes present only by a
write of its clock value, for an `Option<Clock>` place and for a union case
alike. Under an operation's frame, an `Option<Clock>` place keeps its presence
bit and its clock unless the postcondition writes it (a constant write makes it
present with that value, `absent` drops it), and a union case that holds a
clock keeps its case and its clock unless the postcondition writes the union
value (a constructor of the case, `Armed { x: c }` or a literal that contains
the clock component, makes it present with the constant it gives; a case
without the clock drops it). A union case has no bit, so a postcondition that
requires only `present(self.state.Armed)` of a predecessor holding `Idle`
does not select an `Armed` successor with a clock of 0: nothing writes the
clock, so there is no successor, exactly as `present(self.v)` alone from an
absent `v` has none. The one rule is that a successor in which a place is
present although it was absent in its predecessor exists only when the
postcondition writes that place's clock value. A case switch that makes a clock
live therefore writes the constructor. A postcondition that requires the
presence without the write has no successor; when the operation's precondition
or an invariant establishes the place absent, the requirement is decided
statically and refuses `invalid_timed_model`/`clock-presence-unwritten` at the
`present(...)` requirement, naming the place. The rule adds no implicit write:
a present place whose frame does not write it is unchanged, so its clock value
is kept.

**The same rule for absence.** A requirement is a constraint, never a write:
`not present(p)` does not write `absent` or `Idle`. An `Option<Clock>` place
keeps its bit unless written, so `not present(self.v)` of a present, unwritten
`v` has no successor. A union value in the frame is not fixed by a bit: the
frame admits any case, subject to the rule above that no clock becomes live
without its write. `not present(self.state.Armed)` of an `Armed` predecessor
therefore has the successor in which `state` is a clockless case (`Idle`) and
the clock is dropped, and an unwritten union of an `Armed` predecessor with no
requirement also admits that successor beside the unchanged one. The two forms
follow the one rule (an unwritten clock keeps its value; a clock is live only
by a write); they differ only because a union can leave a case and an
`Option<Clock>` bit does not change.

### Clock places

In the checked package, FR-322's `clock_place.resolved_path` table is the
authoritative resolution record. Each row names the declaring object or record,
member identity and declared member type; collection rows also carry collection
kind and bounds; the terminal row marks `Clock`. The reader joins each row to
the corresponding path step and selected model declaration. This table is
carried in checked-package nodes, so timed readers do not reopen source text or
infer clock members from a spelling.

A type is **clock-bearing** when it is `Clock`, an alias of a clock-bearing
type, `Option` of one, a `Set`, `Bag`, `Sequence`, `OrderedSet` or `Map` whose
element (for a map, value) type is clock-bearing, or a record, tuple or union
with a field, element or case payload of a clock-bearing type. A member of a
clock-bearing type whose multiplicity is not `[1, 1]` is the collection of that
type that FR-322 gives the multiplicity. A value of a clock-bearing type is not a
data value: it denotes **clock places**, the clocks it contains, and a place
is the pair of an object of a universe and the path of fields, tuple indexes,
union cases and collection slots (below) from the object's field to a `Clock`.
The clocks of a timed
model are exactly the places of its objects. A place outside every collection
exists in every state, so a model with no clock collection has a number of
clocks fixed by the universe and the declared types; the places of a clock
collection are live only while the collection holds the element (below).

- A place under an `Option` or `?` field or a union case is **guarded**: it is
  present at a discrete state exactly when the value on its path holds that
  case (`present(p)` for an optional place `p`), and its clock has a value
  only while it is present. A place that is not guarded is present in every
  state. A guarded place is read in a clock constraint as `value(p)`, which
  carries FR-146's presence obligation: a `value(p)` without `present(p)`
  established by the guard or by the time invariant's predicate refuses
  `undefined_expression`/`unproved-presence` at the read.
- **Presence is derived.** `present(p)` of a guarded place `p` is a Boolean
  data predicate, never a clock read, whose value at a state is the presence of
  the place that `p` denotes: the instance named by the owner object, the
  template node and the slots (QSL FR-230 "Place identity"). It is read from
  the state's data (the case the value on the path holds, the collection
  holding the slot) or, for an `Option<Clock>` place, from its runtime state bit, which is
  not a package member.
  The model declares no `Option<Boolean>` field, member or type for it, and
  the checked package carries none (FR-322 "Timed-model nodes", Presence). `p`
  may denote a place of another object of the universe or a parameter
  (`present(other.v)` reads `other`'s presence, so two objects of one type have
  independent presences), and `present(e)` in the body of `forall(e in q: B)`
  or `exists(e in q: B)` is read at each live slot `e` binds, so the
  quantifier is the conjunction (`forall`, true on no live slot) or the
  disjunction (`exists`, false on none) of the presences of its live slots.
- A place under several guards (an `Option<Timer>` field, a `Sequence` slot of
  `Option<Clock>`, a union case of an optional field) is present exactly when
  every guard on its path holds, and `value(p)` carries the conjunction of
  those presence tests as its obligation. `Option<Clock>` is a type like any
  other clock-bearing type: a field, parameter, result, record or tuple
  component, union payload, alias target and collection element may be
  `Option<Clock>`. An operation with an `Option<Clock>` parameter has one
  instance for each guarded place of its range, the instance reads `value(c)`
  under the obligation of its place, and a reset of `c` to a constant makes
  the place present. A function whose result is `Option<Clock>` returns the
  guarded place its body names and `absent` where its body gives `absent`,
  whose place is inactive.
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
  place makes it present with that clock, and a write of `absent` (the Option
  literal of FR-143, typed `Option<Clock>` by the place it is written to), or of
  another union case, drops it.
  Elsewhere it refuses `invalid_timed_model`/`clock-write`.
- A clock-bearing value that is read whole, as an operand of `=`, a function
  argument of data type, an assignment to a data field or a conversion, reads
  its clocks, and refuses `invalid_timed_model`/`clock-read-outside-constraint`.
  The data fields of a clock-bearing record are data values and are read as
  usual.
- A function or predicate with a `Clock` parameter or result, or a parameter or
  result of a clock-bearing type, is a **clock-reading declaration**. Its body
  reads a clock only as the clock operand of an atomic clock constraint,
  passes it to another clock-reading declaration, or returns it as the
  declaration's place result, and a call to it stands only
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
- A clock place under `time tick` or in an untimed model refuses
  `invalid_timed_model`/`clock-without-dense-time` at the object field that
  holds it.
- `Reference<Clock>` (a package `reference` whose `target` is `Clock` or an
  alias of it) is a data value that names one clock place by object, field path
  and slot. It holds no clock value, so a field, parameter, result, record
  component or collection element may hold one, and `Option<Reference<Clock>>`
  is absent or one place. `ref(p)` for a place expression `p` is the data value
  naming the place `p` denotes, and `deref(r)` for a `Reference<Clock>` `r` is a
  place expression that denotes each place the reference can name, each guarded
  by the data condition `r = ref(place)`; a clock constraint over it is the
  disjunction of those guarded constraints, and a reset `deref(r) = c` resets
  each such place under the operation's frame. A `deref` of an
  `Option<Reference<Clock>>` or of a reference to a slot place is admitted where
  `present(r)` and the slot's presence are established, and otherwise refuses
  `undefined_expression`/`unproved-presence` at the `deref`. `=` and `!=` over
  two `Reference<Clock>` compare places, never clock values, and are data reads.
  A reference to a slot place names the slot and keeps naming it when the
  collection moves its element. The state key holds the place id of a
  `Reference<Clock>`, a one-to-one encoding of the place's owner object, place
  and slot indices (QSL FR-230 "Place identity"), and no clock value.
- A package constraint whose `appliesTo` is `Clock` or an alias of it is a
  **clock constraint**. `max c`, `c` an exact non-negative rational number of
  the model's time unit, is the time invariant `x <= c` on every clock place of
  the typed places (the places typed by the alias, or every clock place for
  `Clock` itself), guarded by the place's presence. `min c` refuses
  `invalid_timed_model`/`time-invariant-shape`, because a lower bound on every
  clock is not an upper-bound invariant. Any other keyword refuses
  `invalid_model_binding`/`malformed-declaration` at the constraint, and a
  clock constraint under `time tick` or in an untimed model refuses
  `clock-without-dense-time`.

### Clock collections

A **clock collection** is a `Set`, `Bag`, `Sequence`, `OrderedSet` or `Map` of a
clock-bearing element or value type, and a field, parameter or result of
clock-bearing type whose multiplicity is not `[1, 1]` (FR-322 gives it
`Set`, `Bag`, `Sequence` or `OrderedSet` with the multiplicity's bounds). A
clock collection is admitted in every position a collection of its element type
is admitted, with its clock rules below, including `Option` of a clock
collection, a collection of `Option<Clock>`, a collection of collections and a
record or union that holds one. A clock-bearing `Map` key refuses
`ill_typed`/`operator-ineligible`, because a key is an immutable lookup value
and a clock advances with time.

- **Element places.** An element of a clock collection has one place for each
  clock its element type contains. The place's path has a **slot** step: the
  element's zero-based position in the collection's iteration order, which is
  occurrence order for a `Sequence` and an `OrderedSet`, ascending key for a
  `Map`, and ascending canonical key of the element's data part (FR-144) for a
  `Set` and a `Bag`, copies of one data part in the order they were created.
  That position (or key) is the selector of an **instance** of the place. The
  checked package holds one template `clock_place` node for the path, whose slot
  step carries a symbolic element selector and no index (FR-322 "Timed-model
  nodes").
  The **data part** of a clock-bearing value is the value with every clock
  component removed.
- **Uniqueness.** A `Set`, `OrderedSet` or `Bag` of a clock-bearing element
  type decides uniqueness and multiplicity by equality of data parts, so
  `Set<Clock>` has at most one element, `Bag<Clock>` one element for each
  occurrence and `Set<Timer>` one `Timer` for each distinct `n`. A clock
  collection admits no `=` or `!=`, which would read its clocks: both refuse
  `invalid_timed_model`/`clock-read-outside-constraint`, and the data parts of
  its elements are read through the elements.
- **Liveness.** An element place is a guarded place whose presence test is the
  collection holding the slot: `slot < size` for an ordered kind, a `Set` or a
  `Bag`, and the key being in the domain for a `Map`. A live place has a clock
  value and an inactive place has none: it is unconstrained and takes no part
  in any guard, invariant or delay. The live places of a state are determined
  by its discrete state.
- **Time progression.** A delay advances every live clock by the delay, the
  clocks of element places included.
- **Creation.** A collection value written by a postcondition holds, for each
  occurrence, either an element that comes from a collection of the pre-state
  (named by the expression: the collection itself, an element bound by a
  traversal, and the results of `filter`, `flatten`, `convert`, concatenation,
  slicing and reordering applied to those) or an element built by a literal.
  An element from the pre-state **carries** its clocks and their values to the
  slot it takes in the result, so inserting, removing or reordering other
  elements moves a clock and does not reset it. An element built by a literal is
  **fresh**: its clock components are exact non-negative rational constants and
  its places are live with those constants at insertion. A map entry written
  `self.m[k] = literal` is fresh, and `self.m[k]` for a key already present
  replaces the entry. An element whose clock component is neither carried nor a
  constant refuses `invalid_timed_model`/`clock-write` at the element.
- **Coalescing.** When two occurrences of a `Set`, `OrderedSet` or `Map` result
  are equal by data part (or by key), the result holds the first occurrence in
  the order the expression visits them (FR-145: source order, outer before
  inner, left operand before right), with that occurrence's clocks.
- **Removal.** An element the written value does not hold has no slot in the
  result; its places are inactive and their values are dropped.
- **Reset by selection.** A postcondition conjunct resets element places by a
  selection: `p[i].x = c` or `p[k].x = c` resets the place of the element at
  index `i` or key `k` (a data term evaluated in the pre-state, with the
  collection's index or key obligation: an `i` or `k` with no element refuses
  `undefined_expression`/`unproved-presence` at the selector when the
  precondition does not establish it), and `forall(e in p: P(e) implies e.x = c)`
  resets the place of every live element whose data part satisfies the data
  predicate `P` (with `P` absent, every live element). The selected collection field is named by the operation's
  frame, and an element place not selected keeps its value.
- **Reading.** A guard, a time invariant body and a clock-reading declaration
  read element places by the same selectors and by `forall(e in p: B)`
  and `exists(e in p: B)`, `B` a Boolean combination of atomic clock
  constraints over `e`'s places and data predicates over its data part.
  `forall` is the conjunction of `B` over the live elements and is true when
  there are none; `exists` is the disjunction over them and is false when there
  are none. Each live slot contributes the data predicate "the collection holds
  the slot" to the convex part it appears in, and the data predicates of `B`
  that read only `e`'s data part are evaluated at that slot's element. A time
  invariant body `forall(e in p: e.x <= c)` bounds each live place of `p` only while it is live.
  A query that turns clock values into data (`count`, `fold`, `reduce`, `sum`,
  `map` or `filter` over a clock constraint) reads the clocks as values and
  refuses `invalid_timed_model`/`clock-read-outside-constraint`.
- **Cardinality bound.** The number of live places of a state is its element
  places' collection sizes times the clocks each element holds, and the clock
  dimension of a model is bounded by the sum, over its clock collections, of the
  declared maximum cardinality (FR-144's bound `K<T>[a, b]`, a field's
  multiplicity upper bound) times the clocks each element holds, taken over the
  universe's objects. A clock collection with no declared maximum has no static
  bound: its live places in a state are still the collection's size, and an
  analysis that reaches a state whose live clocks exceed the model checker's
  `max_clocks` limit (setting `model_check.max_clocks`, default 1024) stops
  `incomplete` with the `limit-reached` cause (FR-331) whose `limit` is
  `max_clocks`, whose `value` is the configured count and whose `setting` is
  `model_check.max_clocks` (FR-461). `max_clocks` is a member of the
  temporal item's `limits` (FR-368), a positive count that defaults to 1024
  when the request omits it.

A timed state over clock collections is `(s, v)` with `s` the discrete state,
whose live places it determines, and `v` the valuation of exactly those places.
A step from `(s, v)` to `(s', v')` has `v'` defined on the live places of `s'`:
a carried place takes the value of its source place, a fresh place the constant
of its write, a reset place its constant, and every other live place of `s'` that
`s` also made live at the same path and slot keeps its value. FR-181 states how
the explored state and a timed trace step name these places.

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
| FR-415-AC-2 | `Clock` fields are admitted under `time dense` only, every present clock is 0 initially, and a reset to an exact non-negative rational is the only admitted write. | Test (TC-367) |
| FR-415-AC-3 | Strict and non-strict atomic clock constraints are admitted with exact conversion to `u`; a clock read outside a constraint, a two-clock comparison and a negative constant refuse with their typed causes. | Test (TC-367) |
| FR-415-AC-4 | A time invariant with upper bounds is admitted; a lower bound or equality in its body refuses `invalid_timed_model`/`time-invariant-shape`. | Test (TC-367) |
| FR-415-AC-5 | `urgent when P` is admitted; `urgent O` is admitted only when `O`'s guard reads no clock. | Test (TC-367) |
| FR-415-AC-6 | A record `Timer { x: Clock, n: Int }` held by object field `t`, a `Clock` parameter, a `Clock` result and `type Alias = Clock` each admit the clock rules of the sections above: `t.x`, the argument of a call and the result of a call are read only in atomic clock constraints and reset only by a postcondition reset, `Alias` is `Clock`, and a model with `k` objects and the record field `t` has `k` clocks `t.x`. Reading `t` whole (`self.u = self.t`) refuses `clock-read-outside-constraint`; a record literal with a clock component outside a reset refuses `clock-write`. | Test (TC-367) |
| FR-415-AC-7 | A field `w: Option<Clock>` is a guarded place: it is absent in an initial state whose `w` is absent, `self.w = 0 ms` makes it present with clock 0, writing `absent` drops it, the guard `present(self.w) and value(self.w) >= 1 ms` is false while `w` is absent, the time invariant `when present(self.w) { value(self.w) <= 3 ms }` places no bound while `w` is absent, and `value(self.w) >= 1 ms` with no `present(self.w)` refuses `undefined_expression`/`unproved-presence`. | Test (TC-367) |
| FR-415-AC-8 | Each refuses with its cause: an operation that resets its `Clock` parameter with a frame naming no clock field (`clock-write`); a clock-reading predicate with a bound parameter called with a non-constant argument (`clock-bound-not-constant`); a call to a clock-reading predicate in an `invariant` clause (`clock-read-outside-constraint`); a clock-bearing field under `time tick` (`clock-without-dense-time`). | Test (TC-367) |
| FR-415-AC-9 | A unit whose `profile` declaration names `quire.temporal.complete/v1` admits the forms of AC-1 to AC-5; the same unit whose declaration names `quire.model.complete/v1`, or that has no `profile` declaration, refuses each `time` member, `time invariant`, `urgent` declaration and `Clock` in a type position `unsupported_construct`, naming the unit's selected layers and the required layer `quire.temporal.complete/v1`. | Test (TC-367) |
| FR-415-AC-10 | `Set<Clock>`, `Sequence<Timer>`, `Bag<Clock>`, `OrderedSet<Timer>`, `Map<Text, Timer>`, `Option<Sequence<Clock>>`, `Sequence<Option<Clock>>`, `Sequence<Sequence<Clock>>` and a field `ts: Timer` with multiplicity `[0, 3]` are admitted; the places of a three-element `Sequence<Timer>` held by one object are `ts.0.x`, `ts.1.x` and `ts.2.x`, live exactly while the sequence holds that slot; `Set<Clock>` holds at most one element and `Bag<Clock>` one element for each occurrence; a `Map<Timer, Int>` key refuses `ill_typed`/`operator-ineligible`; `self.a = self.b` and `self.a != self.b` over clock collections refuse `clock-read-outside-constraint`. | Test (TC-367) |
| FR-415-AC-11 | Over a delay of `d`, every live place of a clock collection advances by `d` and an inactive place stays unconstrained. A postcondition `self.ts = sequence[Timer { x: 0 ms, n: 1 }]` makes slot 0 live at 0; `self.ts = filter(e in self.ts: e.n > 1)` drops the places of the removed elements and moves the survivors' clocks to their new slots with their values; `self.ts = flatten(sequence[sequence[Timer { x: 0 ms, n: 9 }], self.ts])` makes a fresh slot 0 at 0 and moves the old slot `j` to `j + 1` with its value; `self.ts = set[Timer { x: 0 ms, n: 1 }, Timer { x: 5 ms, n: 1 }]` holds one element, with the first occurrence's clock `0`; an element with a clock that is neither carried nor a constant refuses `clock-write`. | Test (TC-367) |
| FR-415-AC-12 | `self.ts[i].x = 0 ms` resets the place of slot `i`, and refuses `undefined_expression`/`unproved-presence` at `i` when the precondition does not establish `i < size(self.ts)`; `self.m[k].x = 0 ms` resets by key; `forall(e in self.ts: e.n > 1 implies e.x = 0 ms)` resets exactly the live places whose `n > 1` and leaves the others; the guard `forall(e in self.ts: e.x <= 3 ms)` is true on the empty sequence and, on `k` elements, the conjunction of `x <= 3` over the `k` live places; `exists(e in self.ts: e.n = 2 and e.x >= 1 ms)` is false on a sequence with no `n = 2` and otherwise the disjunction over those places; `forall(e in self.ts: e.x <= 3 ms)` as a time invariant body bounds each live place only while live; `count<Int>(e in self.ts: e.x >= 1 ms)` refuses `clock-read-outside-constraint`. | Test (TC-367) |
| FR-415-AC-13 | A clock collection with declared maximum `3` and two clocks per element adds six to the model's clock dimension for each object that holds it, with no bound beyond the declared maximum; a clock collection with no declared maximum reaches a state whose live clocks exceed `model_check.max_clocks` and the analysis settles `incomplete` with `limit-reached` naming `max_clocks`, its value and `model_check.max_clocks`; a request that sets `limits.max_clocks` to `2048` raises the bound, and `0` is refused by the request schema. | Test (TC-367) |
| FR-415-AC-14 | With `o: Option<Timer>` (`Timer { x: Clock; n: Int }`), `present(self.o) and value(self.o).x >= 1 ms` is a guard on the place `o.x`, and `value(self.o).x >= 1 ms` alone refuses `undefined_expression`/`unproved-presence`; `self.o = Timer { x: 0 ms, n: 1 }` makes `o.x` present at 0 and `self.o = absent` drops it; a `Sequence<Option<Clock>>` slot `j` is present only while the sequence holds slot `j` and the element is present, and `value(self.q[j])` without both tests refuses `unproved-presence`; an operation `arm(c: Option<Clock>)` with `c = 0 ms` has one instance for each guarded place, each making its place present; a function `maybe(o: Timer): Option<Clock>` whose body is `o.x` is a place expression and whose body `absent` is an inactive place. | Test (TC-367) |
| FR-415-AC-15 | With object `Call` holding clocks `x` and `y` and a field `r: Reference<Clock>`, a universe of two calls and `time dense unit ms`, the precondition `deref(self.r) >= 1 ms` is the disjunction of the four guarded constraints on `Call1.x`, `Call1.y`, `Call2.x` and `Call2.y`, each with the condition `self.r = ref(place)`; `self.r = ref(self.x)` writes the place and resets no clock; `ref(self.x) = ref(other.x)` is a data comparison, false for two calls; `deref(self.r) = 0 ms` with a frame naming only `Call.x` refuses `invalid_timed_model`/`clock-write`; `deref(self.r)` over an `Option<Reference<Clock>>` refuses `undefined_expression`/`unproved-presence` without `present(self.r)`; two states that differ only in clock values have one state key and two that differ in `r` have different keys; `ref(self.x)` of `Call1` and of `Call2` have different place ids, and the state key holds that id for `r`. | Test (TC-367) |
| FR-415-AC-16 | A constraint `max "3"` whose `appliesTo` is the alias `Timeout` of `Clock` is the time invariant `x <= 3` on every place typed `Timeout` and on no other clock; with `appliesTo` `Clock` it bounds every clock place; `min "1"` refuses `invalid_timed_model`/`time-invariant-shape`; the keyword `minLength` refuses `invalid_model_binding`/`malformed-declaration`; the same constraint under `time tick` refuses `clock-without-dense-time`. | Test (TC-367) |
| FR-415-AC-17 | A checked package for a dense model with a clock at `Call.t.0.x`, a reset, a guard, an invariant and `urgent when self.ready` emits a `timed_model` root plus source, place, reset, guard, invariant, urgency and normalized clock-constraint nodes. The place path records owner `Call` and the field, collection slot and field steps; no node represents `Clock` as a value or type. Changing in turn the source unit, tick period, place path or guard, reset, invariant, urgency and constraint bound changes the corresponding timed-node key and package identity; reordering source spans without changing semantics changes neither. | Test (TC-367) |
| FR-415-AC-18 | Checked-package V2 read-back admits a source only when its unit's `dimension_node_id` is the exact base `Time` node named by the timed root; source units in `Length`, derived non-Time and nominal derived-Time dimensions refuse `ill_typed`/`invalid-time-quantity` at the `time_source.unit` reference for dense and tick forms. A tick source with any `Clock` place refuses `invalid_package`/`clock-without-dense-time` at the place node's `model` reference; S3 retains the `invalid_timed_model` cause at the source clock declaration. | Test (TC-367; FR-322-AC-126, FR-322-AC-127; TM-23..TM-29) |
| FR-415-AC-19 | The places `ts.0.x`, `ts.1.x` and `ts.2.x` of a three-element `Sequence<Timer>` are three instances of one template `clock_place` in the checked package, whose slot step carries the symbolic element selector, and the template holds no element count, and a collection of another kind or depth changes only its slot steps (FR-322-AC-148..FR-322-AC-150). An empty `guards` aggregate is admitted when the model has no guard, and a guard that exists is listed, never dropped. | Test (TC-367; CK-26) |
| FR-415-AC-20 | With `v: Option<Clock>` and `q: Sequence<Option<Clock>>[0, 2]` on two objects `c1`, `c2`, `present(other.v)` reads the presence of `v` of `other`, so `present(c1.v)` can be true while `present(c2.v)` is false; `forall(e in self.q: present(e))` is true on an empty `q` and otherwise the conjunction of the presences of its live slots, `exists(e in self.q: present(e))` is false on an empty `q` and otherwise their disjunction; none of these declares a field or changes any object's data, and each is a data predicate that never reads a clock value (FR-322-AC-155..FR-322-AC-157). | Test (TC-367; CK-27) |
| FR-415-AC-21 | With `v: Option<Clock>` and `st: Mode` where `Mode` is a union of `Idle` and `Armed { x: Clock }`, both in the frame of an operation: a postcondition that requires `present(self.v)` of an absent `v`, or `present(self.st.Armed)` of an `Idle` `st`, with no write of the clock has no successor (no implicit reset to 0), and refuses `invalid_timed_model`/`clock-presence-unwritten` when the precondition establishes the place absent; with the write `self.v = 5 ms` or the constructor `self.st = Armed { x: 5 ms }` it has one successor in which the place is present with the clock `5 ms`; a write of `absent` or of `Idle` drops the place; a requirement is never a write, so `not present(self.v)` of a present, unwritten `v` has no successor, while `not present(self.st.Armed)` of an `Armed` `st` has the successor with `st` an `Idle`; an unwritten `Armed` `st` with no requirement keeps its clock and also admits the `Idle` successor, and an unwritten present `v` only keeps it. The vectors `FP-01`..`FP-14` of `clock-presence-frame-vectors.json` pair each `Option` vector with its union twin and state which pairs agree and which differ only because a union can leave its case. | Test (TC-367; CK-31; FP-01..FP-14) |

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
