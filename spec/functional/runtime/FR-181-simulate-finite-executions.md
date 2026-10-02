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
requires-bound/incomplete/refused outcomes.

## Behavior

The simulator distinguishes authored nondeterminism from unspecified input.
Enumeration is deterministic; sampling records the seed and generator version.
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
choices and enabled schedules, coalescing only equal complete state keys.
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
the same successor relation with a pinned counter-based generator; the
recorded generator version is the
`quire.simulation.sampler/v1` definition's DefinitionRef. An unbounded domain
or population returns the I13 `requires-bound` per-item negotiation
disposition before exploration begins; `requires-bound` is never an evaluator
outcome. A trace records every transition identity and remains replayable
without the simulator. The unexplored frontier contains state-key digests
plus the resource limit that stopped expansion.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-181-AC-1 | Enumeration covers each finite declared choice/schedule exactly under canonical ordering. | Test (TC-210) |
| FR-181-AC-2 | Seeded sampling reproduces identical traces under the same ecosystem lock. | Test (TC-210) |
| FR-181-AC-3 | Missing bounds or exhausted work returns requires-bound/incomplete with the unexplored scope, and caller cancellation returns incomplete with cause cancelled/caller-cancelled and the unexplored frontier. | Test (TC-210) |
| FR-181-AC-4 | Exhaustive exploration coalesces only equal complete state keys and visits successors in canonical breadth-first order; states differing only in NaN payload or zero sign have distinct state keys and remain distinct states, and a successor that violates an invariant is recorded rather than pruned. | Test (TC-210) |
| FR-181-AC-5 | Every generated trace replays through the same native transition relation with the same terminal disposition. | Test (TC-210) |
| FR-181-AC-6 | Exploring a model that describes external effects performs none of them: the effects appear only as typed trace data, and a request for evaluator I/O, mutation or an ambient store refuses with its source location. | Test (TC-210) |
| FR-181-AC-7 | The same operation applied with the same arguments to two receivers yields two transition identities that differ only in `receiver`, ordered by their JCS bytes; a state with no enabled transition identity is recorded with no successor. | Test (TC-290) |

## Dependencies

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
