---
id: FR-181
title: "Simulate finite composed executions reproducibly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/TC-372
    type: verifies
  - target: ix://agent-ix/quire-specification/AD-008
    type: references
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-specification/FR-394
    type: references
  - target: ix://agent-ix/quire-specification/FR-406
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-417
    type: references
  - target: ix://agent-ix/quire-specification/FR-420
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
---

# FR-181: Simulate finite composed executions reproducibly

## Description

When simulation is requested, the simulator SHALL enumerate or sample only the
declared nondeterministic choices and schedules within explicit state, time and
resource bounds.

## Inputs

Checked package, initial state/population, environment choices, scheduler
profile, seed/enumeration mode, bounds and `max_state_key_bytes`, a positive
limit on the length in octets of one canonical state key.

## Outputs

Typed execution traces/results with coverage and replay provenance, or
requires-bound/incomplete/refused outcomes, or an `unsupported` result with its
typed cause.

## Behavior

The simulator distinguishes authored nondeterminism from unspecified input.
Enumeration is deterministic; sampling records its seed.
Every trace is executable by the same domain runtimes. The simulator reads its inputs and emits traces and frontier records only; every transition it explores is a pure function of the simulator state key and the selected successor, and any external effect a model describes is represented as typed data in the trace. Exhaustion reports the
unexplored frontier and cannot claim exhaustive success. Cancellation returns
`incomplete` with cause `cancelled`/`caller-cancelled` and carries the
unexplored frontier the same way.

## Exploration contract

The simulator state key is the typed canonical form of the simulator's typed
semantic state, control positions, queues, role lifetimes, observation
progress and remaining bounds, serialized with RFC 8785 JCS. The state key is
those canonical JCS bytes themselves; no hash of them is computed. Coalescing
compares the full state-key bytes, and traces and frontier records carry the
state key as that value. When the canonical state key of a reached state
exceeds `max_state_key_bytes`, the simulator SHALL stop expanding that state, retain it in the
unexplored frontier by position without its key, and report `incomplete` with
cause `state-key-limit-exceeded`; it SHALL NOT truncate or hash the key.

### Typed canonical form

A decimal string is `0`, or an optional `-` followed by `[1-9][0-9]*`. A
domain-object reference is its FR-204 (universe, type, object_identity)
triple, realized under `quire.model.complete/v1`
([model-complete.md](../../../proposals/quire-v1/definitions/model-complete.md))
as `{"type":"reference","universe":<node id>,"object_type":<node id>,"identity":"<lowercase hex of the identity bytes>"}`, where each `<node id>` is a structural checked-package node id `{package, path, ordinal}` (FR-322): `universe` that of the declaration node of the universe's least root type and `object_type` that of the declaration node of the object's most-specific type (model-complete.md "Object universe").
The null literal is `{"type":"null"}`. Every value encodes as a tagged JCS
object, `{"type": <tag>, ...}`:

- integer — `{"type":"integer","value":"<decimal>"}`, the exact value as a
  decimal string.
- boolean — `{"type":"boolean","value":true|false}`.
- float32/float64 — `{"type":"float32"|"float64","bits":"<hex>"}`, the
  value's exact IEEE bits, as `bitIdentical` reads them (FR-148), as 8 (for
  `float32`) or 16 (for `float64`) lowercase hex digits, big-endian; distinct
  NaN payloads and signed zeros therefore stay distinct.
- rational — `{"type":"rational","numerator":"<decimal>","denominator":"<decimal>"}`,
  in its FR-205 normalized numerator/denominator form: the greatest common
  divisor is 1, the denominator is positive, and zero is `0`/`1`. A reader
  normalizes any other spelling with a nonzero denominator; a zero
  denominator is outside the form.
- decimal — `{"type":"decimal","coefficient":"<decimal>","scale":"<decimal>"}`,
  its retained (pre-normalization) coefficient and scale (FR-140), each an
  exact decimal string.
- text — `{"type":"text","value":"<string>"}`.
- string — `{"type":"string","value":"<string>"}`, the value of the native
  `String` type (QSL FR-056): its exact Unicode scalar sequence as a JSON
  string, with no normalization, case folding or length limit. A composed
  and a decomposed spelling of one character are two different strings, and
  the empty string is a string. `string` is not `text`: a `text` form never
  reads as a `String` and a `string` form never reads as a `Text`.
- UUID — `{"type":"uuid","value":"<uuid>"}`, the K2 canonical UUID payload
  (QSL FR-056): 32 lowercase hexadecimal digits in groups `8-4-4-4-12`
  joined by four hyphens.
- Timestamp — `{"type":"timestamp","nanoseconds":"<decimal>"}`, the K2
  canonical Timestamp payload (QSL FR-056): the exact signed count of
  nanoseconds since `1970-01-01T00:00:00Z` on the POSIX scale, within the
  signed 128-bit domain, as a decimal string. No zone, calendar text or
  precision selector is part of the form.
- Bytes — `{"type":"bytes","value":"<hex>"}`, the canonical payload of the
  native `Bytes` type (FR-149, FR-322 "Scalar literal values"): the octets in
  sequence order as lowercase hexadecimal digits, two per octet, the empty
  string for the empty sequence. A reader refuses an odd digit count, an
  uppercase digit or any other character as outside the form.
- JsonObject — `{"type":"json_object","value":"<jcs-text>"}`, the canonical
  payload of the native `JsonObject` type (FR-149, FR-322 "Scalar literal
  values"): a JSON string holding the RFC 8785 (JCS) serialization of the
  value, which is its own JCS serialization again (no repeated member name,
  members in UTF-16 code unit order, no insignificant whitespace). The text is
  a string inside the form, so its quotes and backslashes are escaped by the
  enclosing JCS serialization. A reader refuses text that is not one JSON
  value or is not already canonical as outside the form.
- `Option<T>` — `{"type":"option","value":<encoded-value>|null}`.
- `Set<T>`/`Bag<T>` — `{"type":"set"|"bag","elements":[<encoded-value>, ...]}`,
  sorted by each element's JCS UTF-8 bytes, ascending unsigned-lexicographic;
  a bag keeps every duplicate.
- `Sequence<T>` — `{"type":"sequence","elements":[<encoded-value>, ...]}`, in
  the sequence's own order.
- `OrderedSet<T>` — `{"type":"ordered-set","elements":[<encoded-value>, ...]}`,
  in the ordered set's own order, admitting no duplicate encoded element.
- record — `{"type":"record","name":"<qualified-name>","fields":[{"name":"<field>","value":<encoded>|null}, ...]}`,
  one entry per declared field in declaration order, `null` for an absent
  `?` field.
- tuple — `{"type":"tuple","name":"<qualified-name>","components":[<encoded>, ...]}`,
  in declaration order.
- enum — `{"type":"enum","name":"<qualified-name>","member":"<member-ident>"}`.
- union — `{"type":"union","name":"<qualified-name>","member":"<member-ident>","components":[<encoded>, ...]}`,
  an empty array for a nullary member.
- `Reference<T>` — the reference triple defined above.

The `sequence`, `set`, `bag` and `ordered-set` forms are also the wire value
form of a supplied snapshot or input collection value, each element being the
typed canonical form of the element type; its admission (bounds, duplicate
refusal, element order and refusal causes on input) is
[FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md)
"Supplied collection values". A bag is always repeated elements, never
element-count pairs.

### Canonical element encoding

Equal values of a type encode identically only where that type's form below is
canonical. The element of a `Set` or `Bag` is written in its canonical encoding
before the elements are ordered by encoded bytes, recursively through nested
options, collections, records, tuples and unions. Integer, boolean,
rational (FR-205 normalized), string (the exact scalar sequence), Bytes and
JsonObject (their canonical payloads), enum,
union member, record, tuple, option, reference, UUID, Timestamp and
`Float32`/`Float64` bits are canonical as defined above, as is a quantity
(the normalized rational with its unit; see the unit-bearing quantity rule
below). Two kinds are not canonical as defined above and are canonical here:

- decimal: the FR-140 normalized coefficient and scale (trailing decimal zeros
  removed while `scale > 0`; every zero is coefficient `0`, scale `0`), never
  the retained coefficient and scale. This applies only to the element of a set
  or bag, so the retained representation elsewhere is unchanged.
- text under a profile that normalizes (`nfc`, `nfd`, `nfkc`, `nfkd`): the
  FR-141 compared form. Text under `unicode-scalars` or `binary-utf8` is
  canonical as written.

A reader accepts any encoding of a set or bag element and normalizes it.

An element of a `Set`, `Bag` or `OrderedSet` may be of type `String`,
`Rational[..]`, `Float32`, `Float64` or `Quantity<u>`, as of any other type
that has a canonical key
([FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md)).
Two elements are the same member exactly when FR-144's collection membership
relation holds, which compares a float by its bits: the `float32`/`float64`
form is canonical, so two floats are one member exactly when their `bits`
strings are equal, and the set or bag orders its elements by those encoded
bytes. Because `bits` is fixed-width lowercase hexadecimal, that byte order is
the order of the unsigned bit pattern; it is not the FR-148 `totalOrder`
visiting order, which FR-144 defines separately.

An alias encodes as its target type. A domain-object value is held in state
as `Reference<T>`. A population is not a value: a state member that holds
objects of a population is typed `Set<Reference<T>>` and encodes as a `set`
of references, and a type that is a population has no typed canonical form.

A unit-bearing quantity has exactly one canonical encoding in every position
(a state member, a record field, a set or bag element): the FR-205 normalized
rational of its exact magnitude with an added `"unit":"<qualified-name>"`
naming the declared unit identity from its FR-142 dimension/unit conversion
graph, in the `rational` form. A quantity is never encoded in the `decimal`
form, so the quantity `1.50` in `Units::metre` is always
`{"denominator":"2","numerator":"3","type":"rational","unit":"Units::metre"}`
and the decimal retained-representation rule above does not apply to it. A
reader also accepts the `decimal` form with the same unit for a quantity,
reads the exact value `coefficient × 10^-scale`, and the state key carries the
normalized `rational` form, so every encoding of one quantity value yields one
state key.

The state key is
`{"type":"simulation-state","semantic":<encoded>,"control":<map>,"queues":<map>,"roles":<map>,"observations":<map>,"bounds":<map>,"memory":<map>}`.
Each map is `{"type":"map","entries":[{"key":<encoded>,"value":<encoded>}, ...]}`,
entries sorted ascending unsigned-lexicographically by the key's JCS UTF-8
bytes. A `fifo` queue's contents encode as a `sequence` in delivery order; an
`unordered` queue's contents encode as a `bag`.
For a model subject `control`, `queues`, `roles`, `observations`, `bounds`
and `memory` are empty maps. For a protocol subject their entries are those
of [FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md),
and `memory` holds the memory component of
[FR-434](../memory/FR-434-select-memory-models-for-parallel.md), empty under
`sc`.

A successor is an operation applied to an argument vector from the
operation's finite declared parameter domains, enabled when the operation's
effective precondition is true, times every post-state within the
operation's declared frame for which the operation's postcondition holds
([FR-013-AC-1](../foundation/FR-013-validate-operation-frames.md)). A successor that violates an invariant is recorded as a
successor rather than pruned from exploration.

Exhaustive mode performs deterministic breadth-first expansion of authored
choices and enabled schedules, coalescing only equal complete state keys,
except that a model-check item selecting a state-space reduction explores
under [FR-383](../analysis/FR-383-explore-under-state-space-reduction.md),
which coalesces equal canonical state keys and expands ample sets.
Within one level, parent states keep their FIFO discovery order, and each
parent's own successors are visited in ascending unsigned-lexicographic order
of the JCS UTF-8 bytes of the typed canonical form of their transition
identity, `{"type":"transition","operation":"<qualified-name>","receiver":<encoded>|null,"arguments":[<encoded>, ...]}`
(`receiver` the operation's receiver reference, `null` for an operation with
no receiver; arguments in parameter declaration order). Two transitions of one
operation with different receivers have different transition identities. A
protocol subject's steps carry the `protocol-transition` identity of
[FR-426](../protocol/FR-426-apply-protocol-steps.md) and are visited in the
same order.

Two successors of one parent with equal transition identities and different
post-states are visited in ascending unsigned-lexicographic order of their
post-states' state-key bytes. Two successors with equal transition identities
and equal post-state keys are one successor.

The subject's initial states are ordered in ascending unsigned-lexicographic
order of their state-key bytes, and initial states with equal state keys are
coalesced into one. This canonical order is the subject's ordered list of
initial states: level 0 of breadth-first expansion visits the initial states
in it, and every zero-based initial-state index (the FR-364 `initial_state`,
the FR-393 `initial`, the FR-392 walk's initial state, the FR-408 test's
initial state) names a position in it. The order in which a request supplies
the initial states does not change it.

A state at which no transition identity is enabled is terminal; a timed
subject has its own definition under "Timed subjects". Exploration
records a terminal state as a state with no successor. A temporal analysis
reads a terminal state by
[FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md):
closed under a bounded profile, and extended by the terminal stutter step
`{"type":"terminal-stutter"}` under the infinite-trace profile. Sampling draws from
the same successor relation with the counter-based generator that the
`quire.simulation.sampler/v1` definition
([sampler definition](../../../proposals/quire-v1/definitions/simulation-sampler.md))
defines; a sampled trace is named by
its seed and trace index. A sampled step makes one or more draws, each with its own choice index: with no workload, one uniform draw over the enabled successor set; under a workload ([FR-406](../analysis/FR-406-interpret-probabilistic-models.md)), one draw for the operation by exact integer weight, one uniform draw for the scheduled identity, one draw per random parameter in declaration order by its exact integer weights and, over a timed subject, one draw per delay and per tie-break ([FR-420](../analysis/FR-420-sample-stochastic-timed-models.md)). A statistical run assigns trace index `i · m + j` to sample `i` of test `j` among `m` tests. No draw selects a sampled trace's initial state. A sampled trace whose
requirement names its initial state (an FR-392 walk, an FR-408 test) starts
there. Otherwise, with `k` initial states in canonical order, trace `t` starts
at the initial state with index `t mod k`, so a subject with one initial state
starts every trace there.
An unbounded domain
or population returns the I13 `requires-bound` per-item negotiation
disposition before exploration begins; `requires-bound` is never an evaluator
outcome. A trace records every transition identity and remains replayable
without the simulator. The unexplored frontier contains state keys
plus the resource limit that stopped expansion.

## Exploration from a given state and the explored graph

Exhaustive mode may start from a given state instead of the subject's initial
states: the given state is the only initial state, and exploration, ordering,
coalescing and limits are as above. Trap replay of a state-graph claim
([FR-394](../analysis/FR-394-replay-state-graph-evidence.md)) explores the
forward closure of a recorded state this way, unreduced.

The explored graph is the set of explored states with every explored edge
(source, transition identity, post-state) retained. It is the state graph over
which state-graph claims ([FR-390](../analysis/FR-390-analyze-state-graph-claims.md))
are evaluated. A state whose successors exploration has not fully computed is
open, and every other explored state is closed
([FR-391](../analysis/FR-391-settle-state-graph-claims.md)).

## Timed subjects

Over a timed subject ([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md))
a state is a **timed state** `(s, v)`: `s` the discrete state (every non-clock
field, the presence of every guarded clock place included) and `v` the clock
valuation of the present clocks, every present clock 0 initially. A clock that
a step makes present takes the constant of its write, and one that a step makes
absent leaves `v` (FR-415 "Clock places").

A clock collection (FR-415 "Clock collections") has an element place for each
clock of each element, named by object, field path and slot. The discrete
state holds the collection's data parts only: its typed canonical form encodes
each element as its data part, with no clock component, so two states that
differ only in clock values have one state key, the size of a `Set<Clock>` or
`Bag<Clock>` is part of that key, and the live places of a state are those its
collection sizes and presence tests determine. The valuation `v` is defined on
exactly those places, listed in object, field and slot order, and a step defines
`v'` on the live places of its post-state by carry, fresh constant and reset as
FR-415 states, so a clock moves with its element and an inserted or removed
element changes the dimension of the valuation by the places it adds or drops.
The explored symbolic state of a timed subject pairs a discrete state with a
zone over the live places of that discrete state: a place that is not live is
inactive and unconstrained, a transition projects out the places it drops, maps
the places it carries to their new slots, and adds a fresh place as the equality
to its constant, so the dimension of a zone is the number of live places of its
discrete state and is bounded by the model's clock dimension (FR-415 "Cardinality
bound").

### Task-automaton product states and steps

A task-derived timed subject additionally carries the task scheduler's finite
control state: the current mode, policy queue order, each live job's task
declaration, nominal activation identity and occurrence, selected jitter,
slot generation, priority, absolute deadline and execution-demand phase, and
whether it is pending jitter, ready, running, preempted or complete. It also
carries each task's activation-clock phase and the source operation's unique
task binding. This state is part of the canonical discrete state key.
The derived auxiliary clocks are those of FR-415 "Task-derived timed
products"; they are separate from source-model clock places. The number of
live jobs and auxiliary clocks is finite under FR-419's validated slot bound
or the active bounded-search limit. A slot is reclaimed only after completion
or an explicit policy removal; a later job receives a new generation.

Each derived task step has a typed identity distinct from a source operation
identity. It names the action, task-set and task declarations, job occurrence,
slot and generation, immutable activation transition, exact selected demand
and jitter, priority, and original absolute deadline. An activation names the
source transition that created it; the later actual release names that same
activation. A scheduler step names the derived transition
that enabled it, and a source completion-feedback step names both the source
transition and the derived completion that enabled that transition. The same
identity and provenance are carried in a timed counterexample. No derived activation, release, dispatch, preemption, resumption, completion,
budget change or deadline miss is represented as a fabricated source-model
operation. A finite search slot-cap signal is not a task transition.

During a delay of `d`, every source and auxiliary clock advances by `d`.
Service is credited only to the job selected as running by the queue policy.
The product's diagonal constraints and bounded-subtraction transitions
preserve each suspended job's accumulated service across preemption while its
clocks continue to advance at unit rate. A completion transition removes one
job only when its exact selected demand has been met; an HI budget change
records its exact delta on every affected live job and retains each job's
release, priority and deadline identity. A deadline is missed only when the
absolute deadline is reached and the job still has positive remaining
demand.

The finite fixed-work product may be explored and certified by the exact
refined-region relation in FR-418. Variable-demand and mode-change products
may use bounded exploration. A finite counterexample path is not a refutation
until FR-364 replay validates every demand choice and product transition and
the bad state has a checked time-divergent continuation. An exhausted bounded
search with no replayable witness is incomplete; its explored prefix is not a
proof of the unbounded timed claim.

- A **delay** by real `d >= 0` takes `(s, v)` to `(s, v + d)`. It is admissible
  when `d = 0`, or when `s` is not urgent and every time invariant whose
  predicate holds at `s` holds at `v + d` (then at every point between).
- A **discrete step** by transition identity `t` takes `(s, v)` to `(s', v')`
  when `t`'s data precondition holds at `s`, its guard holds at `v`, `s'` is a
  post-state of the successor rule above, `v'` is `v` with `t`'s resets
  applied, and every time invariant whose predicate holds at `s'` holds at
  `v'`.
- A **timed behaviour** alternates one delay, possibly 0, with each discrete
  step from an initial state at time 0. Position 0 is the initial state and
  position `i > 0` the post-state of the `i`-th discrete step; each position
  carries its time stamp, the sum of the delays up to it, and its clock
  valuation. Delays are not positions.
- A timed state is **terminal** when no discrete step is enabled at it or after
  any admissible delay, a delay of 0 included. A terminal timed state in which
  time can pass without bound (a quiescent one,
  [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) "Idle
  tail") is a deadlock candidate, reported by the deadlock-freedom item
  ([FR-366](../temporal/FR-366-check-deadlock-freedom.md)). A terminal timed
  state in which the admissible delay is bounded by a time invariant or by
  urgency is a **bounded-delay terminal state**. It has no time-divergent
  continuation, so it is a time-lock in the sense of
  [FR-417](../temporal/FR-417-settle-time-lock-freedom.md) "Time-locks"
  (local when no positive delay is admissible either, non-local otherwise), and
  the time-lock-freedom item reports it.
- Under `time tick O period p` every delay is 0 and the time stamp of position
  `i` is `p` times the number of `O` steps among steps 1 to `i`.

A timed trace step carries, beside its transition identity and post-state
key, `delay`: the exact non-negative rational delay before the step in the
model's unit. The post-state key is the canonical key of the
discrete state and the exact clock valuation. A time-lock counterexample (`kind`
`time-lock`) ends with its member `final_delay` ([FR-417](../temporal/FR-417-settle-time-lock-freedom.md)).

**Timed replay** starts every clock at 0 and, for each step, checks that the
delay is admissible, applies it, checks that the transition identity is
enabled at the delayed state, selects the successor by the post-state key,
applies the resets and checks the target's time invariants, all in exact
rational arithmetic. A violated delay or guard refuses
`invalid_runtime_input`/`invalid-value`. Every non-empty set of
valuations of an automaton with rational constants contains a rational point,
so every timed refutation has a counterexample with exact rational delays.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-181-AC-1 | Enumeration covers each finite declared choice/schedule exactly under canonical ordering. | Test (TC-210) |
| FR-181-AC-2 | Seeded sampling reproduces identical traces for the same seed and trace index. | Test (TC-210) |
| FR-181-AC-3 | Missing bounds or exhausted work returns requires-bound/incomplete with the unexplored scope, and caller cancellation returns incomplete with cause cancelled/caller-cancelled and the unexplored frontier. | Test (TC-210) |
| FR-181-AC-4 | Exhaustive exploration with no selected reduction coalesces only equal complete state keys and visits successors in canonical breadth-first order, and a model-check item selecting a reduction explores under FR-383; states differing only in NaN payload or zero sign have distinct state keys and remain distinct states, and a successor that violates an invariant is recorded rather than pruned. | Test (TC-210) |
| FR-181-AC-5 | Every generated trace, every state-graph witness path, and every trace concretised from a reduced search replays through the same native transition relation with the same terminal disposition, and no trace carries a canonical state, permutation or reduction. | Test (TC-210, TC-335, TC-327) |
| FR-181-AC-6 | Exploring a model that describes external effects performs none of them: the effects appear only as typed trace data, and a request for evaluator I/O, mutation or an ambient store refuses with its source location. | Test (TC-210) |
| FR-181-AC-7 | The same operation applied with the same arguments to two receivers yields two transition identities that differ only in `receiver`, ordered by their JCS bytes; a state with no enabled transition identity is recorded with no successor. | Test (TC-290) |
| FR-181-AC-13 | Exhaustive exploration from a given state treats it as the only initial state, explores its forward closure in canonical breadth-first order under the request's limits, and retains every explored edge with each state's open or closed status. | Test (TC-335) |
| FR-181-AC-9 | Under a workload each sampled step draws the operation by exact integer weight, the scheduled identity uniformly and each random parameter by its weights, each with its own choice index; with unit weights and one choice per step the selection equals the unweighted uniform selection. | Test (TC-354) |
| FR-181-AC-10 | Sample `i` of test `j` among `m` tests uses trace index `i · m + j`. | Test (TC-354) |
| FR-181-AC-11 | Timed replay checks each delay's admissibility, the guard at the delayed state, the post-state key over the discrete state and exact clock valuation, and the target's time invariants in exact rationals; a violated delay or guard refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-369) |
| FR-181-AC-12 | Under `time tick O period p` a position's time stamp is `p` times the number of `O` steps up to it. | Test (TC-369) |
| FR-181-AC-25 | Over a model with `ts: Sequence<Timer>[0, 3]` the discrete state key of a state ignores the clock values and holds the data parts of the elements, the size of a `Set<Clock>` field is in the key, the valuation of a state lists exactly the live places in object, field and slot order, a step that filters out the element at slot 0 moves the clock of the element at slot 1 to slot 0 with its value, a step that inserts a fresh element at the front gives it the literal's constant and moves the others up one slot, and the post-state key changes with the valuation and the live places. | Test (TC-369) |
| FR-181-AC-14 | A sampled choice whose integer total `n` exceeds `2^256` stops the whole sampling run before any draw and settles `unsupported` with cause `total-above-draw-range{total}`, `total` being `n`; no draw value is computed and the sampler does not loop. | Test (TC-354) |
| FR-181-AC-15 | A sampled choice whose integer total is exactly `2^256` accepts the draw-0 value and selects the least index `i` with `v mod n < w_0 + … + w_i`. | Test (TC-354) |
| FR-181-AC-16 | Two successors of one parent with equal transition identities and different post-states are visited in ascending order of their post-states' state-key bytes, whatever order the successor relation yields them in; two with equal transition identities and equal post-state keys are one successor. | Test (TC-210) |
| FR-181-AC-17 | Initial states are visited and indexed in ascending order of their state-key bytes, with equal keys coalesced into one: supplying the same initial states in reversed order, or one of them twice, gives the same initial-state list, the same exploration and the same initial-state indices. | Test (TC-210) |
| FR-181-AC-18 | With `k` initial states and no requirement naming the start, sampled trace `t` starts at the initial state with index `t mod k` and makes no draw to select it; with one initial state every trace starts there, and the draws of a trace are exactly those of its trace index. | Test (TC-210) |
| FR-181-AC-19 | Each family encodes to these JCS bytes: UUID `{"type":"uuid","value":"00112233-4455-6677-8899-aabbccddeeff"}`; Timestamp `-1` `{"nanoseconds":"-1","type":"timestamp"}`; decimal `1.50` `{"coefficient":"150","scale":"2","type":"decimal"}`; quantity `1.50` in unit `Units::metre` `{"denominator":"2","numerator":"3","type":"rational","unit":"Units::metre"}`; enum `{"member":"red","name":"Shop::Color","type":"enum"}`; record `Shop::Item{sku: 7, note: absent}` `{"fields":[{"name":"sku","value":{"type":"integer","value":"7"}},{"name":"note","value":null}],"name":"Shop::Item","type":"record"}`; tuple `Shop::Pair(true, -3)` `{"components":[{"type":"boolean","value":true},{"type":"integer","value":"-3"}],"name":"Shop::Pair","type":"tuple"}`; nullary union member `{"components":[],"member":"none","name":"Shop::Shape","type":"union"}`. | Test (TC-210) |
| FR-181-AC-20 | As the element of a `Set`, decimal `1.50` encodes as `{"coefficient":"15","scale":"1","type":"decimal"}` while in a record field it keeps coefficient `150` and scale `2`; the quantity `1.50` in `Units::metre` encodes as `{"denominator":"2","numerator":"3","type":"rational","unit":"Units::metre"}` both as a set element and in a record field, and its supplied decimal form `{"coefficient":"150","scale":"2","type":"decimal","unit":"Units::metre"}` is read and written back as that rational form, so a `Set<Quantity<Units::metre>>` of the rational `3/2` and that decimal form is one member; a `Set<Timestamp>` of `2` and `10` encodes as `{"elements":[{"nanoseconds":"10","type":"timestamp"},{"nanoseconds":"2","type":"timestamp"}],"type":"set"}`, ordered by encoded bytes. | Test (TC-210) |
| FR-181-AC-21 | A state member holding the objects of a population encodes as a `set` of reference triples ordered by encoded bytes, and two states whose population member holds the same reference identities, inserted into the set in two different orders, have equal state keys; a UUID payload with uppercase hex, a Timestamp payload `+1` or `01`, and the Timestamp payloads `170141183460469231731687303715884105728` (2^127) and `-170141183460469231731687303715884105729` (-2^127 - 1) are outside the form, while `170141183460469231731687303715884105727` (2^127 - 1) is inside it. | Test (TC-210) |
| FR-181-AC-22 | Each family encodes to these JCS bytes: string `ready` `{"type":"string","value":"ready"}`; the empty string `{"type":"string","value":""}`; rational `-2/4` read and written `{"denominator":"2","numerator":"-1","type":"rational"}`; `float32` `1.0` `{"bits":"3f800000","type":"float32"}`; `float64` negative zero `{"bits":"8000000000000000","type":"float64"}`; the `float64` quiet NaN with payload 1 `{"bits":"7ff8000000000001","type":"float64"}`; the quantity `1/1000` in `Units::metre` `{"denominator":"1000","numerator":"1","type":"rational","unit":"Units::metre"}`. A rational with denominator `0`, a `float32` `bits` string of 7 or 9 hex digits, and a `bits` string with an uppercase digit are outside the form. | Test (TC-210) |
| FR-181-AC-23 | As set elements, `Set<String>` of `b`, `a` and `é` (U+00E9) encodes as `{"elements":[{"type":"string","value":"a"},{"type":"string","value":"b"},{"type":"string","value":"é"}],"type":"set"}` and the composed `é` and the decomposed U+0065 U+0301 are two members; `Set<Rational>` of `2/4` and `1/3` encodes as `{"elements":[{"denominator":"2","numerator":"1","type":"rational"},{"denominator":"3","numerator":"1","type":"rational"}],"type":"set"}`; `Set<Float64>` of the bits `fff0000000000000`, `bff0000000000000`, `8000000000000000`, `0000000000000000`, `3ff0000000000000` and `7ff0000000000000` encodes with `elements` in the order `0000000000000000`, `3ff0000000000000`, `7ff0000000000000`, `8000000000000000`, `bff0000000000000`, `fff0000000000000`; and `Set<Quantity<Units::metre>>` of `1/2` and `2/4` is one member `{"denominator":"2","numerator":"1","type":"rational","unit":"Units::metre"}`. | Test (TC-210) |
| FR-181-AC-24 | Two states whose `Set<Float64>` member holds `+0` and `-0` have distinct state keys; two whose `Set<Float64>` member holds the same bits inserted in two orders have equal state keys; two whose `Set<String>` member holds the composed and the decomposed spelling of `é` have distinct state keys; and two whose `Set<Rational>` member holds `2/4` and `1/2` have equal state keys. | Test (TC-210) |
| FR-181-AC-26 | Each family encodes to these JCS bytes: Bytes `00ff` `{"type":"bytes","value":"00ff"}`; the empty Bytes `{"type":"bytes","value":""}`; JsonObject `{"a":1,"b":[true,null]}` `{"type":"json_object","value":"{\"a\":1,\"b\":[true,null]}"}`; JsonObject `"x"` `{"type":"json_object","value":"\"x\""}`; JsonObject `[]` `{"type":"json_object","value":"[]"}`. A reader refuses `{"type":"bytes","value":"0"}`, `"00FF"`, `"0g"` and `"0x00"`, and `{"type":"json_object","value":"{\"b\":1,\"a\":2}"}`, `"{\"a\": 1}"`, `"{\"a\":1.0}"`, `""` and `"{"` as `invalid_runtime_input`/`invalid-value`; a `bytes` form read as a `JsonObject`, or the reverse, and a `string` form read as `Bytes`, is `wrong-value-kind`. | Test (TC-210) |
| FR-181-AC-27 | As set elements, `Set<Bytes>` of `2000`, `00`, `10` and `0000` encodes as `{"elements":[{"type":"bytes","value":"00"},{"type":"bytes","value":"0000"},{"type":"bytes","value":"10"},{"type":"bytes","value":"2000"}],"type":"set"}`, ordered by encoded bytes, and `Set<JsonObject>` of `{}` and `[]` encodes `[]` before `{}`. Two states whose `Set<Bytes>` member holds the same octets inserted in two orders have equal state keys; two whose `JsonObject` field holds the one value `{"a":1,"b":2}`, one built from the member order `b`, `a` and one from `a`, `b`, have equal state keys, both encoding the text `{"a":1,"b":2}`, and two whose field holds `[1,2]` and `[2,1]` have different state keys. | Test (TC-210) |
| FR-181-AC-28 | A timed state is terminal exactly when no discrete step is enabled at it or after any admissible delay, a delay of 0 included. | Test (TC-370) |
| FR-181-AC-29 | A bounded-delay terminal state is a time-lock reported by the time-lock-freedom item and not by the deadlock-freedom item. | Test (TC-370) |
| FR-181-AC-30 | A state whose canonical state key is longer than `max_state_key_bytes` stops expansion of that state and settles the exploration `incomplete` with cause `state-key-limit-exceeded`, naming the limit and its value; a state whose key is exactly `max_state_key_bytes` long is expanded; no truncated or hashed key appears in a trace or frontier. | Test (TC-210) |
| FR-181-AC-31 | In a task product, a job with fixed work 2 scheduled from 0 to 1, preempted from 1 to 2 by a one-unit job, and resumed from 2 to 3 has service 2 at completion although all auxiliary clocks advance by 3; its deadline clock remains at its nominal activation plus `D`, and every source-plus-scheduler post-state key includes queue order, job identity, selected jitter and slot generation. | Test |
| FR-181-AC-32 | Two activations with the same source transition identity get distinct job occurrences and obey the task's periodic `T` or sporadic minimum `T`; selected jitter delays readiness but not the absolute deadline. Preemption/resumption preserve identity and deadline, a completed slot may be reused only with an incremented generation, and a replayed deadline-miss lasso whose region fails to close is not a time-divergent continuation. | Test |

## Dependencies

- [FR-383](../analysis/FR-383-explore-under-state-space-reduction.md) owns
  exploration under a selected state-space reduction.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.

## References

- Protocol and memory members of the state key: quire-spec-language ADR-027
  PS-8, TS-2, QS-3 and QS-4, and ADR-025 MS-1; Linear STD-140 and STD-138,
  paired QSL-396 and QSL-372.
