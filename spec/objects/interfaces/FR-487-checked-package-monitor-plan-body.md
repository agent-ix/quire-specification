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
  the `tick_unit` node the canonical text of `uncertainty` and the index segment of the `on_gap` node (FR-322 `path`).
  Every premise of the plan, and so the subject, clock binding, uncertainty,
  gap, rate and capacities of QSL FR-251, is part of the key: changing one
  yields a different root key and so a different plan. Changing an entry body changes that entry's own derived `node_id`
  and so the root's: the plan is identified by the content keys of the root and
  its entries, and no other plan identity exists.
- **body** — `{term: "aggregate", members: [binding("clause", …),
  binding("state", …), binding("inputs", …), binding("scope", …),
  binding("results", …), binding("init", …), binding("on_event", …),
  binding("on_watermark", …), binding("counter_width_bits", …),
  binding("max_reading_gap", …), binding("max_event_rate", …),
  binding("buffers", …), binding("counter", …), binding("tick_unit", …),
  binding("uncertainty", …),
  binding("on_gap", …)]}`, in this order:
  - `clause` — a `reference` to the `temporal`/`temporal_clause` node (FR-370)
    the program monitors. Its `temporal_bindings` row (FR-322 "Temporal clock
    bindings") is the plan's clock binding: the plan repeats no clock name,
    kind or profile;
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
    rounded up, plus one). Capacities are premises of the plan; sequence types
    stay ordinary `composite_type`/`sequence` nodes;
  - `counter` — a `text` literal, non-empty: the counter's name, the first
    component of the plan's tick clock binding key (QSL FR-251
    `ClockBindingKey`; the clause's `temporal_bindings` row keeps the clause's
    own clock name and kind, and the plan's counter is a separate premise);
  - `tick_unit` — a `reference` to the `scalar_type`/`unit` node whose `dimension_node_id` equals, by identity, the
    exact base `Time` dimension node (the test of FR-322-AC-127; this reader's
    refusal of a node that fails it is the reference-kind refusal below, not the
    timed root's `invalid-time-quantity`) that names one tick's unit; the unit's
    exact `root_scale` is the tick scale, so no scale is repeated and the width
    is `counter_width_bits`;
  - `uncertainty` — a `rational` literal in the canonical spelling of FR-322-AC-114, at least 0, in ticks: the `u` of the
    widened cover `[k - u, k + 1 + u)` (FR-160 "Counter-tick intervals and
    endpoints").

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
- **state** — the `state` record has a field `now`, an `integer`, the unwrapped
  tick of the latest admitted reading, and for each `buffers` entry a field
  named `buffer_<n>` (`<n>` the entry's position in `buffers`, from 0) typed by
  a `composite_type`/`sequence` node. A body keeps each such sequence at no
  more than its capacity; an event that would exceed it returns the
  `Incomplete` outcome below instead of appending.
- **results and outcomes** — the `results` element is a `composite_type`/`record`
  with the fields `instance` (a record of `subject` (`string`), `subformula`
  (`integer`) and `origin` (`integer`, the unwrapped tick at which the
  obligation instance began)) and `outcome`, a `composite_type`/`union` whose
  cases are exactly `Failed`, `False`, `Incomplete`, `Indeterminate`,
  `Pending` and `True` (the order of FR-322 union typing): `True`, `False`,
  `Pending` and `Indeterminate` nullary (FR-160); `Incomplete` carrying the
  record `LimitReached{limit, value, setting}` (`limit` `event_buffer`,
  `value` the capacity, `setting` `monitor.max_event_rate`); `Failed` carrying
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

### Counter-gap path

The first reading is the baseline: its delta is 0 and it is not gap-checked.
For each later reading the runtime computes the delta modulo
`2^counter_width_bits`. A delta not above `max_reading_gap` is passed to
`on_event`. A greater delta is a reading-gap fault: the runtime calls `on_gap`
with the checked gap instead of `on_event`, the faulting reading becomes the
new baseline, and the monitor continues. `on_gap` yields for every live
obligation instance whose verdict is still open (its window or deadline spans
the gap) a result whose `outcome` is
`Failed(ReadingGapExceeded{gap, max_reading_gap})`, and drops that instance
from the returned state; an instance already decided, and one not yet started,
is unaffected, and a result already emitted is not changed. No temporal verdict
is emitted for a failed instance. The state carries no typed projection of its
live instances beyond what the body keeps in its own fields.

## Reader order

The plan step visits every `temporal`/`monitor_plan` node in ascending node-id
order, after the temporal step of FR-322 "Body steps". For one node the checks
run in this order and the first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the sixteen bindings in order,
   `buffers` is an aggregate of aggregates of exactly `subformula` then
   `capacity`, `counter_width_bits` and `max_reading_gap` are `integer`
   literals, `max_event_rate` and `uncertainty` are `rational` literals and
   `counter` is a `text` literal, else
   `invalid_semantic_graph` at the first binding, in body order, that breaks
   the rule; for an absent binding the locus is the index it would occupy
   and for an extra binding the index after the sixteenth.
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
   `max_event_rate`, each `capacity`, `counter` and `uncertainty` meet their
   range, and each `rational` literal is in the canonical spelling of
   FR-322-AC-114, else `invalid_package`/`invalid-value` at the literal's
   `value`.
5. **Buffers and state.** `buffers` is strictly ascending by subformula node id,
   each subformula is reached by the clause's formula, and the `state` record
   has exactly the field `now` (an `integer`) and then one `buffer_<n>` sequence field per `buffers` entry, in that order, and no other field, else
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
capacity premises and constructs the `Failed` outcome. The reader does not run
a body.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-487-AC-1 | A plan root over a clause, a `State` record (`now`, `buffer_0`, `buffer_1`), the three entries with the signatures above, the six-case `Outcome` union and the premises `counter_width_bits` 32, `max_reading_gap` 1000, `max_event_rate` 10 and two buffers of capacity 4 and 7 is admitted; a capacity edit is still admitted (`MP-01`, `MP-02`). | Test (TC-978) |
| FR-487-AC-2 | A root with a binding missing (including `uncertainty`, `MP-39`), reordered or extra, or with `counter_width_bits` or `max_reading_gap` that is not an `integer` literal, `max_event_rate` or `uncertainty` that is not a `rational` literal (`MP-35`, `MP-37`) or a `counter` that is not a `text` literal (`MP-42`), refuses `invalid_semantic_graph` at the first breaking binding; a `clause` that is not a `temporal_clause`, a `state` that is not a record and an entry that is not a `pure_function` refuse the same at the binding (`MP-03` to `MP-09`, `MP-28`, `MP-40`, `MP-41`, `MP-48` and `MP-49`). | Test (TC-978) |
| FR-487-AC-3 | `init` with a parameter, `on_event` with two parameters or with the delta before the state, `on_watermark` whose boundary is not an `integer`, `on_gap` with the gap before the state or a non-`integer` gap (`MP-50`, `MP-51`), and an entry whose result is not the `(State, Results)` tuple refuse `ill_typed`/`type-mismatch` at the entry's binding; an `Outcome` union missing `Pending`, with an extra case or with a `Failed` payload that is not `ReadingGapExceeded` refuses the same at `results` (`MP-10` to `MP-17`, `MP-29` and `MP-52`). | Test (TC-978) |
| FR-487-AC-4 | `counter_width_bits` 0 and 65, `max_reading_gap` `2^width`, `max_event_rate` 0 or negative, a negative `uncertainty`, an empty `counter` and a `capacity` 0 refuse `invalid_package`/`invalid-value` at the literal; `buffers` out of ascending order, with a repeated subformula or naming an unreached subformula, and a `state` lacking `buffer_1` or holding a non-sequence `buffer_1`, refuse `invalid_semantic_graph` at `buffers` or `state` (`MP-18` to `MP-27`, `MP-30`, `MP-31`, `MP-34`, `MP-36`, `MP-38`, `MP-46` and `MP-47`). | Test (TC-978) |
| FR-487-AC-5 | Buffer extension is `flatten(sequence(s, sequence(x)))` over the existing operations `quire.op.collection.flatten` and `quire.op.collection.sequence`: both are in the closed operation catalog, the extension of a `Sequence<T>` by a `T` is a `Sequence<T>`, and the catalog and both V2 schemas are unchanged (`MP-32`). | Test (TC-978) |
| FR-487-AC-6 | The identity-bearing premises are bindings of the root and part of its key: the `counter` name, the `tick_unit` node, `counter_width_bits`, `max_reading_gap`, `max_event_rate`, `uncertainty` and the buffer capacities. A different `uncertainty` (`0`), `counter`, `max_event_rate` (`1/10`, a rational), `tick_unit` node or capacity yields a different root key (`MP-02`, `MP-33`, `MP-43` to `MP-45`, and the `on_gap` node, `MP-53`); a `tick_unit` that is not a `scalar_type`/`unit` node, or is a unit of the `Length` dimension, refuses `invalid_semantic_graph` at the `tick_unit` binding (`MP-40`, `MP-41`). The tick intervals are derived from the clause and the unit scales and are not bound. | Test (TC-978) |
| FR-487-AC-7 | On a reading-gap fault the runtime calls `on_gap(State, gap)` with the checked gap (an `integer` count of ticks above `max_reading_gap`) in place of `on_event`: every open instance yields `Failed(ReadingGapExceeded{gap, max_reading_gap})` and leaves the state, a decided instance and the results already emitted are unaffected, and with no open instance the results are empty and the state is unchanged (`MP-54` to `MP-56`). The first reading is the baseline with no gap check, and the faulting reading becomes the new baseline: readings 65530, 4, 65500, 65510 under width 16 and `max_reading_gap` 100 call baseline, `on_event(10)`, `on_gap(65496)`, `on_event(10)` (`MP-57`). | Test (TC-978) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `monitor_plan` among the
`temporal` node's `semantic_form` values.
`proposals/checked-package-v2/monitor-plan-vectors.json` publishes the cases
`MP-01` to `MP-57`.

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
