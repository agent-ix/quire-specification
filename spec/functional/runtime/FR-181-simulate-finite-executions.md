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

The simulator state key is the canonical digest of typed semantic state,
control positions, queues, role lifetimes, observation progress and remaining
bounds. Its preimage is the typed canonical form of that state: an IEEE
floating-point value is encoded by its exact bits, as `bitIdentical` is
(FR-148), so distinct NaN payloads and signed zeros stay distinct; an integer
is written as an exact decimal string; a rational is written in its FR-205
normalized numerator/denominator form; and a set or map is sorted by its own
encoded elements. The typed canonical form is serialized with RFC 8785 JCS
and digested with SHA-256 under the `quire.simulation.state-key/v1` digest
domain (FR-201), distinct from `sha256-jcs`. Coalescing compares the full key
bytes, not only the digest; the digest identifies states in traces and
frontier records.

A successor is an operation applied to an argument vector from the
operation's finite declared parameter domains, enabled when the operation's
effective precondition is true, times every post-state within the
operation's declared frame for which the operation's postcondition holds
([FR-013-AC-1](../foundation/FR-013-validate-operation-frames.md)). A successor that violates an invariant is recorded as a
successor rather than pruned from exploration.

Exhaustive mode performs deterministic breadth-first expansion of authored
choices and enabled schedules, coalescing only equal complete state keys.
Within one level, parent states keep their FIFO discovery order, and each
parent's own successors are visited in the ascending canonical-byte order of
their transition identity. Sampling draws from the same successor relation
with a pinned counter-based generator; the recorded generator version is the
`quire.simulation.sampler/v1` definition's DefinitionRef. An unbounded domain
or population returns the I13 `requires-bound` per-item negotiation
disposition before exploration begins; `requires-bound` is never an evaluator
outcome. A trace records every transition identity and remains replayable
without the simulator. The unexplored frontier contains canonical state keys
plus the resource limit that stopped expansion.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-181-AC-1 | Enumeration covers each finite declared choice/schedule exactly under canonical ordering. | Test (TC-210) |
| FR-181-AC-2 | Seeded sampling reproduces identical traces under the same ecosystem lock. | Test (TC-210) |
| FR-181-AC-3 | Missing bounds or exhausted work returns requires-bound/incomplete with the unexplored scope. | Test (TC-210) |
| FR-181-AC-4 | Exhaustive exploration coalesces only equal complete state keys and visits successors in canonical breadth-first order. | Test (TC-210) |
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
