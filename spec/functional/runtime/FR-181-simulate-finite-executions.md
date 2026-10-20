---
id: FR-181
title: "Simulate finite composed executions reproducibly"
type: FR
relationships:
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
profile, seed/enumeration mode and bounds.

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
progress and remaining bounds, serialized with RFC 8785 JCS. The state-key
digest is SHA-256 over those canonical JCS bytes, under the
`quire.simulation.state-key/v1` digest domain (FR-201), distinct from
`sha256-jcs`. Coalescing compares the full state-key bytes; the digest
identifies states only in traces and frontier records.

### Typed canonical form

A decimal string is `0`, or an optional `-` followed by `[1-9][0-9]*`. A
domain-object reference is its FR-204 (universe, type, object_identity)
triple, realized under `quire.model.complete/v1`
([model-complete.md](../../../proposals/quire-v1/definitions/model-complete.md))
as `{"type":"reference","universe":"<64 lowercase hex>","object_type":"<64 lowercase hex>","identity":"<lowercase hex of the identity bytes>"}`.
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
  in its FR-205 normalized numerator/denominator form.
- decimal — `{"type":"decimal","coefficient":"<decimal>","scale":"<decimal>"}`,
  its retained (pre-normalization) coefficient and scale (FR-140), each an
  exact decimal string.
- text — `{"type":"text","value":"<string>"}`.
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
rational (FR-205 normalized), enum, union member, record, tuple, option,
reference and `Float32`/`Float64` bits are canonical as defined above. Two
kinds are not canonical as defined above and are canonical here:

- decimal, and a quantity over a decimal: the FR-140 normalized coefficient and
  scale (trailing decimal zeros removed while `scale > 0`; every zero is
  coefficient `0`, scale `0`), never the retained coefficient and scale. This
  applies only to the element of a set or bag, so the retained representation
  elsewhere is unchanged.
- text under a profile that normalizes (`nfc`, `nfd`, `nfkc`, `nfkd`): the
  FR-141 compared form. Text under `unicode-scalars` or `binary-utf8` is
  canonical as written.

A reader accepts any encoding of a set or bag element and normalizes it.

An alias encodes as its target type. A domain-object value is held in state
as `Reference<T>`. A unit-bearing quantity encodes as its underlying exact
rational or decimal value with an added `"unit":"<qualified-name>"` naming
the declared unit identity from its FR-142 dimension/unit conversion graph.

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

A state at which no transition identity is enabled is terminal. Exploration
records a terminal state as a state with no successor. A temporal analysis
reads a terminal state by
[FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md):
closed under a bounded profile, and extended by the terminal stutter step
`{"type":"terminal-stutter"}` under the infinite-trace profile. Sampling draws from
the same successor relation with the counter-based generator that the
`quire.simulation.sampler/v1` definition
([sampler definition](../../../proposals/quire-v1/definitions/simulation-sampler.md))
defines; a sampled trace is named by
its seed and trace index. A sampled step makes one or more draws, each with its own choice index: with no workload, one uniform draw over the enabled successor set; under a workload ([FR-406](../analysis/FR-406-interpret-probabilistic-models.md)), one draw for the operation by exact integer weight, one uniform draw for the scheduled identity, one draw per random parameter in declaration order by its exact integer weights and, over a timed subject, one draw per delay and per tie-break ([FR-420](../analysis/FR-420-sample-stochastic-timed-models.md)). A statistical run assigns trace index `i · m + j` to sample `i` of test `j` among `m` tests.
An unbounded domain
or population returns the I13 `requires-bound` per-item negotiation
disposition before exploration begins; `requires-bound` is never an evaluator
outcome. A trace records every transition identity and remains replayable
without the simulator. The unexplored frontier contains state-key digests
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
field) and `v` the clock valuation, every clock 0 initially.

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
- Under `time tick O period p` every delay is 0 and the time stamp of position
  `i` is `p` times the number of `O` steps among steps 1 to `i`.

A timed trace step carries, beside its transition identity and post-state
digest, `delay`: the exact non-negative rational delay before the step in the
model's unit. The post-state digest is the canonical identity digest of the
discrete state and the exact clock valuation. A time-lock counterexample (`kind`
`time-lock`) ends with its member `final_delay` ([FR-417](../temporal/FR-417-settle-time-lock-freedom.md)).

**Timed replay** starts every clock at 0 and, for each step, checks that the
delay is admissible, applies it, checks that the transition identity is
enabled at the delayed state, selects the successor by the post-state digest,
applies the resets and checks the target's time invariants, all in exact
rational arithmetic. A violated delay or guard refuses
`invalid_runtime_input`/`invalid-value`; a digest with no matching successor
refuses `stale_dependency`/`revision-mismatch`. Every non-empty set of
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
| FR-181-AC-11 | Timed replay checks each delay's admissibility, the guard at the delayed state, the post-state digest over the discrete state and exact clock valuation, and the target's time invariants in exact rationals; a violated delay or guard refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-369) |
| FR-181-AC-12 | Under `time tick O period p` a position's time stamp is `p` times the number of `O` steps up to it. | Test (TC-369) |
| FR-181-AC-14 | A sampled choice whose integer total `n` exceeds `2^256` stops the whole sampling run before any draw and settles `unsupported` with cause `total-above-draw-range{total}`, `total` being `n`; no digest is computed and the sampler does not loop. | Test (TC-354) |
| FR-181-AC-15 | A sampled choice whose integer total is exactly `2^256` accepts the draw-0 digest and selects the least index `i` with `v mod n < w_0 + … + w_i`. | Test (TC-354) |

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
