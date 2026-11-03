---
id: FR-487
title: "Checked-package tick-monitor plan root and state-passing entries"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-160
    type: references
  - target: ix://agent-ix/quire-specification/FR-094
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-145
    type: references
  - target: ix://agent-ix/quire-specification/TC-978
    type: verifies
---

# FR-487: Checked-package tick-monitor plan root and state-passing entries

## Description

When a checked package carries a closed tick-monitor program (QSL FR-251) for a
`temporal`/`temporal_clause` declaration, the producer SHALL emit it as
`quire.checked-semantic-graph/v2` nodes over the existing expression contract:
four ordinary `function` nodes (`init`, `on_event`, `on_watermark`, `on_gap`) that pass
the monitor state in and out, and one *plan-root* node, `node_tag: "temporal"`,
`semantic_form: "monitor_plan"`, that names them with the state and result types
and the plan's capacities and counter premises. No statement, loop or assignment
vocabulary exists on the wire: an assignment is a functional update (record
construction), a branch is `quire.op.control.if` or `case`, a loop over a buffer
is a `quire.op.collection.fold` bounded by the buffer's capacity premise, and an
emission is a field of the returned results. The reader SHALL refuse a root
whose rules below do not hold, with the first refusal in the reader order below.

## Properties

- **node** — `node_tag` `temporal`, `semantic_form` `monitor_plan`, no
  `declaration`, no `owner`, `semantic_type` the node of the `state` binding
  (below), occurrences of role `generated` (FR-322 "Source map"). Its
  `node_id` is a derived node's: `path` is `["temporal", "monitor_plan",
  ...operands]` where the operands are, in body order, the index segments of the
  nodes the `clause` to `on_watermark` bindings reference and then the decimal
  text of the two integer premises and the canonical text of `max_event_rate`,
  then for each `buffers` entry the index segment of its subformula and the
  decimal text of its capacity, then the `counter` text, the index segment of
  the `tick_unit` node, the canonical text of `uncertainty` and the index segment of the `on_gap` node, and the decimal text of `activation_capacity`, and last the **call closure**: the
  index segment of the body-root node of every declared `function` node
  (a node with a `declaration`) that the entries call, directly or through the
  body of another closure member, an atom body or the activation guard, in
  ascending node-id order of the called function nodes, each once (FR-322 `path`). As for every index segment, the segment names the body-root node by graph position, and it is that node's structural `node_id` (its content-bearing key, FR-322 "Structural node identity"), not the position, that distinguishes two keys; reordering the graph rewrites positions and changes no key. Like every derived path this is the producer's duty: the reader recomputes no identifier and refuses no path for differing from the derivation (FR-322 "Declaration-less function nodes"), and the call closure is a rule of the key, not a reader check.
  Every premise of the plan, and so the subject, clock binding, uncertainty,
  gap, rate and capacities of QSL FR-251, is part of the key: changing one
  yields a different root key and so a different plan. Changing an entry body changes that entry's own derived `node_id`
  and so the root's: the plan is identified by the content keys of the root and
  its entries, and no other plan identity exists.
  A declared helper function keeps its declaration path as its own `node_id`,
  which is the identity of a call to it (FR-322 "Declaration-less function nodes"; FR-322-AC-309
  derives a path only for a declaration-less function), so a swapped helper body
  changes no call identity. The plan is nevertheless identified by what it
  executes: the call closure puts the body root of every called helper into the
  root's key, so swapping the body of a helper that an entry, an atom or the
  guard calls gives a different root key and so a different plan, although the
  entries' own `node_id`s and the helper's `node_id` are unchanged. A helper stays a declared node
  and is never inlined into an entry to be keyed.
- **body** — `{term: "aggregate", members: [binding("clause", …),
  binding("state", …), binding("inputs", …), binding("scope", …),
  binding("results", …), binding("init", …), binding("on_event", …),
  binding("on_watermark", …), binding("counter_width_bits", …),
  binding("max_reading_gap", …), binding("max_event_rate", …),
  binding("buffers", …), binding("counter", …), binding("tick_unit", …),
  binding("uncertainty", …),
  binding("on_gap", …),
  binding("activation_capacity", …)]}`, in this order:
  - `clause` — a `reference` to the `temporal`/`temporal_clause` node (FR-370)
    the program monitors. Its `temporal_bindings` rows (FR-322 "Temporal clock
    bindings", one per `claim` occurrence of the clause) are the plan's clock
    binding: the plan repeats no clock name, kind or profile, and a consumer
    selects the row of the occurrence it monitors;
  - `state` — a `reference` to the `composite_type`/`record` node of the
    monitor state;
  - `inputs` — a `reference` to the `composite_type`/`record` node of the
    event entry's admitted snapshot and invocation values, one field per
    atom capture binding;
  - `scope` — a `reference` to the `composite_type`/`record` node of the
    admitted scope identities of a progress assertion (clock, subjects, decision
    scope and interval, FR-094);
  - `results` — a `reference` to a `composite_type`/`sequence` node whose
    element is the result record below;
  - `init`, `on_event`, `on_watermark`, `on_gap` — each a `reference` to a
    `function`/`pure_function` node with no `declaration`;
  - `counter_width_bits` — an `integer` literal, 1 to 64;
  - `max_reading_gap` — an `integer` literal, 0 to `2^counter_width_bits - 1`;
  - `max_event_rate` — a `rational` literal in the canonical spelling of FR-322-AC-114, greater than 0, the plan's
    `monitor.max_event_rate` setting in events per tick (QSL FR-251's
    `rate_limit`, an exact rational: the default of one event per microsecond
    converts to a rational number of events per tick);
  - `buffers` — an aggregate of one aggregate per buffered subformula,
    `{binding("subformula", reference), binding("capacity", integer literal)}`,
    ascending by the subformula node's `node_id` with no repeat, each
    `subformula` a `temporal`/`formula` node the clause's formula reaches and
    each `capacity` at least 1 (QSL FR-251: the event rate times the window,
    rounded up, plus one; QSL FR-251 owns the formula and the reader checks only the range). Capacities are premises of the plan; sequence types
    stay ordinary `composite_type`/`sequence` nodes;
  - `counter` — a `text` literal, non-empty: the counter's name, the first
    component of the plan's tick clock binding key (QSL FR-251
    `ClockBindingKey`; the clause's `temporal_bindings` rows keep the clause's
    own clock name and kind, and the plan's counter is a separate premise);
  - `tick_unit` — a `reference` to the `scalar_type`/`unit` node whose `dimension_node_id` equals, by identity, the
    exact base `Time` dimension node (the test of FR-322-AC-127; this reader's
    refusal of a node that fails it is the reference-kind refusal below, not the
    timed root's `invalid-time-quantity`) that names one tick's unit; the unit's
    exact `root_scale` is the tick scale, so no scale is repeated and the width
    is `counter_width_bits`;
  - `uncertainty` — a `rational` literal in the canonical spelling of FR-322-AC-114, at least 0, in ticks: the `u` of the
    widened cover `[k - u, k + 1 + u)` (FR-160 "Counter-tick intervals and
    endpoints");
  - `activation_capacity` — an `integer` literal, at least 0 (member 16, after `on_gap`): the plan's bound on the live obligation instances of the unbounded operators in the state's `activations`. An instance that would exceed it is not started: the body returns the `Incomplete` outcome with `LimitReached{limit: activation, value: activation_capacity, setting: monitor.max_activations}`, never a silent drop. The `limit` kind `activation` names the resource counter (as `event_buffer` does) and the setting `monitor.max_activations` follows FR-461's rule for an engine limit (the engine's name, a dot and the member, like `monitor.max_event_rate`).

  The tick intervals of QSL FR-251 are not bindings. Each is the clause's
  interval bounds (in the clause's clock unit, FR-322 "Temporal clock
  bindings") multiplied by the scale of that unit and divided by the scale of
  `tick_unit`, an exact rational; the plan carries no rounded value, and the evaluator's sound rounding of a bound to a tick (QSL FR-251) acts on it with the cover widened by `u`; the clause and `tick_unit`
  are in the root's key, so the interval association is as identified as the
  clause.
- **entry signatures** — a `function`'s parameters are its `parameters`
  binding (FR-322 "Calls"); each entry's `semantic_type` is the
  `composite_type`/`tuple` node of the two positions `(State, Results)`, the
  `state` and `results` nodes, except `init`, whose `semantic_type` is the
  `state` node:
  - `init` — no parameters;
  - `on_event` — three parameters, in order: the state (`state` node); the
    gap-checked tick delta, an `integer` (the runtime computes the modular
    difference of the counter readings and raises the reading-gap fault, so a
    body never sees a raw wrapping reading and keeps the unwrapped time in the
    state); and the admitted values (`inputs` node);
  - `on_watermark` — three parameters, in order: the state; the unwrapped
    history-boundary tick of the admitted progress assertion, an `integer`
    that covers every tick up to and including the boundary (FR-094); and the
    scope identities (`scope` node);
  - `on_gap` — two parameters, in order: the state, and the checked gap, an
    `integer` (the modular difference in ticks of the two counter readings,
    greater than `max_reading_gap`: the `gap` of `ReadingGapExceeded` below).
    It is called by the runtime when the reading-gap fault occurs and in place
    of `on_event` for that reading; its result is the `(State, Results)` tuple.
- **state** — the `state` record has exactly these fields, in this order: `now`,
  an `integer`, the unwrapped tick of the latest admitted reading; `history_start`,
  an `integer`, the unwrapped tick at which the retained history begins (the
  faulting reading after a counter-gap, see below); for each `buffers` entry a
  field named `buffer_<n>` (`<n>` the entry's position in `buffers`, from 0)
  typed by a `composite_type`/`sequence` node; and `activations`, a
  `composite_type`/`sequence` node of records with the fields `subformula`
  (`integer`), `origin` (`integer`, the unwrapped tick of the activating event)
  and `status`, whose type is any type node the producer declares for the
  field (a scalar, or a `composite_type`/`record` such as one holding the
  activation position of an "on each" activation as an `integer` field
  `position`): the reader checks the field names, order and the `integer` types of
  `subformula` and `origin`, and imposes no rule on the type of `status`; and `event_position`, an `integer`, the unwrapped monotone count
  of admitted events (the event position of FR-091 and FR-092; `init` returns 0
  and `on_event` adds 1 per admitted event, the first reading's `on_event(0)`
  included; `on_watermark` and `on_gap` leave it unchanged, since a watermark and
  a faulting reading are not admitted events, and a clock poll calls no entry). A body keeps each `buffer_<n>` at no more than its capacity, except that the `buffer_<n>` of an unbounded past operator (`once`, `historically`, `since` or `triggered` with no upper bound: no interval, or `[a, *]`) additionally holds one **summary cell** as its first element, which is not counted in the capacity: the cell stores the accumulator that folds the entries as they age out (QSL FR-251 "Unbounded past operators"), the element type of such a buffer is the producer's and the reader imposes no rule on it, and `State` keeps exactly the fields of this list; and
  `activations` at no more than `activation_capacity`; an event that would
  exceed either returns the `Incomplete` outcome below instead of appending.
  An unbounded operator (root `always`, `eventually`, or a nested future
  operator without an upper bound) keeps no history buffer: it keeps one
  `activations` element, of constant size, per live obligation instance, which
  holds the instance's status and origin tick. A nested future operator that has
  an upper bound is bounded by the capacity of the outer buffer that holds it;
  an unbounded operator nested under a bounded one still keeps one constant-size
  `activations` element per live instance, and those instances count against
  the one shared `activation_capacity` (overflow is
  `Incomplete(LimitReached{limit: activation, value: activation_capacity, setting: monitor.max_activations})`).
- **results and outcomes** — the `results` element is a `composite_type`/`record`
  with the fields `instance` (a record of `subject` (`string`), `subformula`
  (`integer`) and `origin` (`integer`, the unwrapped tick at which the
  obligation instance began)) and `outcome`, a `composite_type`/`union` whose
  cases are exactly `Failed`, `False`, `Incomplete`, `Indeterminate`,
  `Pending` and `True` (the order of FR-322 union typing): `True`, `False`,
  `Pending` and `Indeterminate` nullary (FR-160); `Incomplete` carrying the
  record `LimitReached{limit, value, setting}` (`limit` `event_buffer`, `value`
  the capacity and `setting` `monitor.max_event_rate`; or `limit` `activation`,
  `value` the `activation_capacity` and `setting` `monitor.max_activations`; or
  `limit` `atom_projection`, `value` the atom's identity, an `integer` (the
  `AtomId` of QSL FR-251), and `setting` `monitor.atom_projection`); `Failed` carrying
  the record `ReadingGapExceeded{gap, max_reading_gap}`. `gap` and `max_reading_gap` are `integer`s. `on_event` and `on_watermark`
  return the first four and `Incomplete`; only `on_gap` returns `Failed`, from
  the checked gap it is given and the plan's `max_reading_gap` premise. The
  executor owns the modular delta, detects the fault and calls `on_gap`.
- **append** — the sequence extension `append(s, x)` is not a new operation:
  it is `quire.op.collection.flatten` of `quire.op.collection.sequence(s,
  quire.op.collection.sequence(x))`, over ordinary sequence types. The closed
  operation catalog is unchanged.

## Semantics

The meaning of the program is declarative and normative (FR-091, FR-092,
FR-160 "Counter-tick intervals and endpoints"): a tick reading `k` covers the
half-open interval `[k, k + 1)`, an obligation instance is `pending` while declared
progress could still resolve its verdict and `indeterminate` when closed
evidence stays conflicting, and an entry expires when its window end lies before
the history boundary. The entries are validated against that meaning by the
reference evaluator and the differential of QSL FR-251-AC-7; the wire fixes no
separate algorithm.

### Atoms without an admitted projection

An atom reads the scalar or aggregate values its capture bindings name,
projected from the model value by existing catalog projections. Where an atom
over a model-typed value has no admitted projection, `on_event` reports, for the
obligations that depend on that atom, `Incomplete` with
`LimitReached{limit: atom_projection, value: <the atom's identity>,
setting: monitor.atom_projection}` and replayable evidence: the reader does not
refuse the plan (it is a run outcome, not a wire defect), the atom is not
dropped, and no new `Outcome` case or payload shape exists. The `limit` kind
names the resource counter in the manner of `event_buffer` and `activation`.
`monitor.atom_projection` is not a runtime tunable and is the one outcome
FR-461 excepts from its raising-setting rule: it names the remedy, the
plan-declared capture binding of the atom (QSL FR-251) that supplies an admitted
projection, and no `--limit` or library setter exists for it; `value` is the
atom's identity, not a count. The closed `limit` kinds of the
plan root are therefore `event_buffer`, `activation` and `atom_projection`.

### Counter-gap path

The first reading is the baseline: its delta is 0, it is not gap-checked and it is passed to `on_event(0)`.
The baseline moves only on readings the executor ingests as events or
watermarks; a clock poll does not move it and calls no entry. For each later
ingested reading the runtime computes the delta modulo `2^counter_width_bits`.
A delta not above `max_reading_gap` is passed to `on_event`. A greater delta is
a reading-gap fault: the runtime calls `on_gap` with the checked gap instead of
`on_event`, the faulting reading becomes the new baseline, and the monitor
continues. After a gap the true elapsed time is unknown (at least `gap`), so
`on_gap` returns a state in which every `buffer_<n>` is empty, `history_start`
and `now` are both the faulting reading's unwrapped tick (the previous `now`
advanced by `gap`). Every past-time operator whose window
reaches before `history_start` (its window start `now - b` is below it)
is `indeterminate` until the window refills, that is until `now - b` is at least
`history_start`. `on_gap` yields for every live obligation instance whose verdict
is still open (its window or deadline spans the gap) a result whose `outcome` is
`Failed(ReadingGapExceeded{gap, max_reading_gap})`, and drops that instance
from the returned state; an instance already decided, and one not yet started,
is unaffected, and a result already emitted is not changed. No temporal verdict
is emitted for a failed instance. These rules are QSL:plan's counter-gap
rulings (QSL-1463) over FR-251 "Counter faults" and RT FR-276, which state only
the modular difference, the `Failed` payload and the absence of a verdict.

### Reporting

Every interval-operator instance is reported once, when it settles, by a result
whose `outcome` is `True`, `False`, `Indeterminate` or `Failed`; later steps do
not repeat it. Every instance that is still pending is reported with `Pending`
at each `on_watermark` step and at no `on_event` step. A `Failed` settlement comes only from `on_gap`. A root verdict is per
activation position, except that an unbounded root `always` or `eventually`
reports one folded outcome per activation: `Pending` at the watermark steps
until it resolves, then one settled outcome, not one outcome per position.

## Reader order

The plan step visits every `temporal`/`monitor_plan` node in ascending node-id
order, after the temporal step of FR-322 "Body steps". For one node the checks
run in this order and the first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the seventeen bindings in order,
   `buffers` is an aggregate of aggregates of exactly `subformula` then
   `capacity`, `counter_width_bits`, `max_reading_gap` and `activation_capacity` are `integer`
   literals, `max_event_rate` and `uncertainty` are `rational` literals and
   `counter` is a `text` literal, else
   `invalid_semantic_graph` at the first binding, in body order, that breaks
   the rule; for an absent binding the locus is the index it would occupy
   and for an extra binding the index after the seventeenth.
2. **References.** Each reference resolves to a node of the form stated above
   (`tick_unit`: a `scalar_type`/`unit` node whose dimension is the base `Time`
   dimension) and each entry is a `function`/`pure_function` node with no
   `declaration`,
   else `invalid_semantic_graph` at the binding.
3. **Signatures.** Each entry's parameters and `semantic_type` equal the
   signature above, else `ill_typed`/`type-mismatch` at the entry's binding; the
   `outcome` union's cases or an `Incomplete` or `Failed` payload that is not
   the stated record refuses the same at the `results` binding.
4. **Premises.** In body order, `counter_width_bits`, `max_reading_gap`,
   `max_event_rate`, each `capacity`, `counter`, `uncertainty` and `activation_capacity` meet their
   range, and each `rational` literal is in the canonical spelling of
   FR-322-AC-114, else `invalid_package`/`invalid-value` at the literal's
   `value`.
5. **Buffers and state.** `buffers` is strictly ascending by subformula node id,
   each subformula is reached by the clause's formula, and the `state` record
   has exactly the fields `now` and `history_start` (`integer`s), then one `buffer_<n>` sequence field per `buffers` entry, then `activations` (a sequence of records with the fields `subformula`, `origin` and `status`), then `event_position` (an `integer`), in that order and no other field, else
   `invalid_semantic_graph` at the `buffers` binding (order, repeat or
   unreached subformula) or the `state` binding (a missing, mistyped, extra or misordered field).
A `dependencies` list other than the nodes the body references and the
`semantic_type` is FR-322's graph-shape refusal, `invalid_semantic_graph` at the
node's `/dependencies`, which precedes this step.

## IR and QSL boundary

QSL emits the nodes above on the checked-package wire; there is no second
serialization and no plan identity beyond the node keys. An IR reader SHALL
validate the checks above and expose the admitted plan, its entries and its
premises by reference as typed data. The runtime executes the entries: it
supplies the gap-checked delta and the unwrapped boundary tick, enforces the
capacity premises, detects the reading-gap fault and calls `on_gap`, which
returns the `Failed` outcome. The reader does not run
a body.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-487-AC-1 | A plan root over a clause, a `State` record (`now`, `history_start`, `buffer_0`, `buffer_1`, `activations`, `event_position`), the four entries with the signatures above, the six-case `Outcome` union and the premises `counter_width_bits` 32, `max_reading_gap` 1000, `max_event_rate` 10, `activation_capacity` 8 and two buffers of capacity 4 and 7 is admitted; a capacity edit is still admitted (`MP-01`, `MP-02`). | Test (TC-978) |
| FR-487-AC-2 | A root with a binding missing (including `uncertainty`, `MP-39`), reordered or extra, or with `counter_width_bits` or `max_reading_gap` that is not an `integer` literal, `max_event_rate` or `uncertainty` that is not a `rational` literal (`MP-35`, `MP-37`) or a `counter` that is not a `text` literal (`MP-42`), refuses `invalid_semantic_graph` at the first breaking binding; a `clause` that is not a `temporal_clause`, a `state` that is not a record and an entry that is not a `pure_function` refuse the same at the binding (`MP-03` to `MP-09`, `MP-28`, `MP-40`, `MP-41`, `MP-48`, `MP-49`, `MP-60` and `MP-61`). | Test (TC-978) |
| FR-487-AC-3 | `init` with a parameter, `on_event` with two parameters or with the delta before the state, `on_watermark` whose boundary is not an `integer`, `on_gap` with the gap before the state or a non-`integer` gap (`MP-50`, `MP-51`), and an entry whose result is not the `(State, Results)` tuple refuse `ill_typed`/`type-mismatch` at the entry's binding; an `Outcome` union missing `Pending`, with an extra case or with a `Failed` payload that is not `ReadingGapExceeded` refuses the same at `results` (`MP-10` to `MP-17`, `MP-29` and `MP-52`). | Test (TC-978) |
| FR-487-AC-4 | `counter_width_bits` 0 and 65, `max_reading_gap` `2^width`, `max_event_rate` 0 or negative, a negative `uncertainty`, an empty `counter` and a `capacity` 0 refuse `invalid_package`/`invalid-value` at the literal; `buffers` out of ascending order, with a repeated subformula or naming an unreached subformula, and a `state` lacking `buffer_1` or holding a non-sequence `buffer_1`, refuse `invalid_semantic_graph` at `buffers` or `state` (`MP-18` to `MP-27`, `MP-30`, `MP-31`, `MP-34`, `MP-36`, `MP-38`, `MP-46` and `MP-47`). | Test (TC-978) |
| FR-487-AC-5 | Buffer extension is `flatten(sequence(s, sequence(x)))` over the existing operations `quire.op.collection.flatten` and `quire.op.collection.sequence`: both are in the closed operation catalog, the extension of a `Sequence<T>` by a `T` is a `Sequence<T>`, and the catalog and both V2 schemas are unchanged (`MP-32`). | Test (TC-978) |
| FR-487-AC-6 | The identity-bearing premises are bindings of the root and part of its key: the `counter` name, the `tick_unit` node, `counter_width_bits`, `max_reading_gap`, `max_event_rate`, `uncertainty`, the `on_gap` entry, the `activation_capacity` and the buffer capacities. A different `uncertainty` (`0`), `counter`, `max_event_rate` (`1/10`, a rational), `tick_unit` node or capacity yields a different root key (`MP-02`, `MP-33`, `MP-43` to `MP-45`, the `on_gap` node `MP-53` and the capacity `MP-58`); a `tick_unit` that is not a `scalar_type`/`unit` node, or is a unit of the `Length` dimension, refuses `invalid_semantic_graph` at the `tick_unit` binding (`MP-40`, `MP-41`). The tick intervals are derived from the clause and the unit scales and are not bound. | Test (TC-978) |
| FR-487-AC-7 | On a reading-gap fault the runtime calls `on_gap(State, gap)` with the checked gap (an `integer` count of ticks above `max_reading_gap`) in place of `on_event`: every open instance yields `Failed(ReadingGapExceeded{gap, max_reading_gap})` and leaves the instance state, a decided instance and the results already emitted are unaffected, and with no open instance the results are empty (the buffers and time markers still change, AC-9) (`MP-54` to `MP-56`). The first reading is the baseline with no gap check, and the faulting reading becomes the new baseline: readings 65530, 4, 65500, 65510 under width 16 and `max_reading_gap` 100 call the baseline `on_event(0)`, `on_event(10)`, `on_gap(65496)`, `on_event(10)` (`MP-57`). | Test (TC-978) |
| FR-487-AC-8 | The root carries `activation_capacity` (member 16, an `integer` literal at least 0) and its state is exactly `now`, `history_start`, one `buffer_<n>` per buffer, then `activations`, a sequence of records with `subformula`, `origin` and `status`, then `event_position`; unbounded operators keep no history buffer, only one constant-size `activations` element per live instance, and an instance beyond the capacity yields `Incomplete(LimitReached{limit: activation, value, setting: monitor.max_activations})`, never a silent drop (`MP-73`). A capacity of 0 is admitted and is part of the key (`MP-58`); a negative capacity refuses `invalid_package`/`invalid-value` at the literal (`MP-59`); a missing or non-`integer` capacity, a state lacking `history_start` or `activations`, an `activations` element lacking `origin` and a non-`integer` `history_start` refuse `invalid_semantic_graph` (`MP-60` to `MP-65`). The type of an `activations` element's `status` is not constrained by the reader: a scalar and a record holding an `integer` `position` are both admitted (`MP-81`, `MP-82`). | Test (TC-978) |
| FR-487-AC-9 | After a gap fault `on_gap` returns every `buffer_<n>` empty and sets `history_start` and `now` to the faulting reading's tick, open instances fail and decided ones stay (`MP-54` to `MP-56`); a past-time operator whose window start `now - b` is below `history_start` is indeterminate until `now - b` reaches it (`MP-66` to `MP-68`); a clock poll does not move the baseline: readings 100 (event), 150 (poll), 180 (event) call the baseline `on_event(0)` and `on_event(80)` (`MP-69`). | Test (TC-978) |
| FR-487-AC-10 | Each interval-operator instance is reported once, on settlement, and each pending instance at every `on_watermark` step and no `on_event` step; an unbounded root `always` or `eventually` reports one folded outcome per activation, `Pending` until resolved (`MP-70` to `MP-72`). | Test (TC-978) |
| FR-487-AC-11 | The state ends with `event_position`, an `integer` that `init` sets to 0, `on_event` increases by 1 per admitted event (the baseline reading included), and `on_watermark`, `on_gap` and a clock poll leave unchanged (`MP-77`, `MP-57`, `MP-69`); a state lacking `event_position`, with it typed `text` or placed before `activations` refuses `invalid_semantic_graph` at the `state` binding (`MP-74` to `MP-76`). | Test (TC-978) |
| FR-487-AC-12 | An atom with no admitted projection yields `Incomplete(LimitReached{limit: atom_projection, value: <atom identity>, setting: monitor.atom_projection})` with replayable evidence, not a refusal and not a dropped atom, and an atom with an admitted projection does not; the `limit` kinds of `LimitReached` are exactly `event_buffer`, `activation` and `atom_projection`, each with its own `setting`, and the `Outcome` union and its payload record shapes are unchanged (`MP-78` to `MP-80`). | Test (TC-978) |
| FR-487-AC-13 | The plan root's key covers the call closure as a producer rule: two plans that differ only in the body of a declared helper function called by an entry, an atom or the activation guard (the helper keeps its declaration path and so its own `node_id`, and the entries keep theirs) have different root keys, a helper called only through another helper's body is covered the same way, equal helper bodies give an equal root key, and a helper is never inlined into an entry to be keyed. The reader recomputes no identifier and refuses nothing for it, so this repository has no reader-visible vector; the verifier is the producer's test, quire-spec-language FR-251-AC-38 (TC-1081), which this criterion cites as its owner. | Inspection |
| FR-487-AC-14 | The `buffer_<n>` of an unbounded past operator additionally holds one summary cell that is not counted in the capacity, and `State` keeps exactly its fields; the reader imposes no rule on the element type of a buffer (quire-spec-language FR-251, "Unbounded past operators"). | Inspection |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `monitor_plan` among the
`temporal` node's `semantic_form` values.
`proposals/checked-package-v2/monitor-plan-vectors.json` publishes the cases
`MP-01` to `MP-82`.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the node form, node keys,
  `dependencies`, function nodes and calls, union typing and the reader's
  refusal order.
- [FR-370](FR-370-checked-package-temporal-clause-body.md) owns the clause the
  plan monitors; FR-160 and FR-094 the tick cover, progress and the outcomes
  `pending` and `indeterminate`.
- [FR-145](../../functional/expressions/FR-145-convert-and-query-collections.md)
  owns `flatten`, `fold` and the sequence constructor.

## References

- STD-647 (this requirement); quire-spec-language FR-251 (the closed monitor
  program), QSL-1463 (producer) and IR-721 (reader).
