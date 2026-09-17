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
unexplored frontier and cannot claim exhaustive success.

## Exploration contract

The simulator state key is the canonical digest of typed semantic state,
control positions, queues, role lifetimes, observation progress and remaining
bounds. Exhaustive mode performs deterministic breadth-first expansion of
authored choices and enabled schedules, coalescing only equal complete state
keys. Sampling draws from the same successor relation with a pinned
counter-based generator. A trace records every transition identity and remains
replayable without the simulator. The unexplored frontier contains canonical
state keys plus the resource limit that stopped expansion.

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
