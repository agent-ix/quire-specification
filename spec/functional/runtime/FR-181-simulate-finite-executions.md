---
id: FR-181
title: "Simulate finite composed executions reproducibly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-008
    type: references
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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
`sha256-jcs`. Coalescing compares the full state-key bytes, not the digest;
the digest identifies states only in traces and frontier records.

### Typed canonical form

Every value encodes as a tagged JCS object, `{"type": <tag>, ...}`:

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
  its exact coefficient and scale (FR-140), each an exact decimal string.
- text — `{"type":"text","value":"<string>"}`.
- `Option<T>` — `{"type":"option","value":<encoded-value>|null}`.
- `Set<T>`/`Bag<T>` — `{"type":"set"|"bag","elements":[<encoded-value>, ...]}`,
  sorted by each element's own encoded bytes, ascending
  unsigned-lexicographic; a bag keeps every duplicate.
- `Sequence<T>` — `{"type":"sequence","elements":[<encoded-value>, ...]}`, in
  the sequence's own order.
- `OrderedSet<T>` — `{"type":"ordered-set","elements":[<encoded-value>, ...]}`,
  in the ordered set's own order, admitting no duplicate encoded element.
- record/tuple — `{"type":"record"|"tuple","name":"<qualified-name>", ...}`,
  carrying every declared field or component in its declaration order.
- enum/union — `{"type":"enum"|"union","name":"<qualified-name>","member":"<member>", ...}`,
  carrying the selected member's declared components, if any, in declaration
  order.
- `Reference<T>` — `{"type":"reference","name":"<qualified-name>","id":<encoded-value>}`,
  carrying the referent's own identity.

This covers every value type in the complete-V1 grammar's `type-ref`
([Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md)). A
control position, queue, role lifetime and observation-progress map in the
simulator state is sorted the same way, by its own encoded key bytes.

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
of the typed canonical form of their transition identity. Sampling draws from
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
| FR-181-AC-4 | Exhaustive exploration coalesces only equal complete state keys and visits successors in canonical breadth-first order; states differing only in NaN payload or zero sign are not coalesced, and a successor that violates an invariant is recorded rather than pruned. | Test (TC-210) |
| FR-181-AC-5 | Every generated trace replays through the same native transition relation with the same terminal disposition. | Test (TC-210) |
| FR-181-AC-6 | Exploring a model that describes external effects performs none of them: the effects appear only as typed trace data, and a request for evaluator I/O, mutation or an ambient store refuses with its source location. | Test (TC-210) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
