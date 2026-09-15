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
Every trace is executable by the same domain runtimes. Exhaustion reports the
unexplored frontier and cannot claim exhaustive success.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-181-AC-1 | Enumeration covers each finite declared choice/schedule exactly under canonical ordering. | Test (TC-210) |
| FR-181-AC-2 | Seeded sampling reproduces identical traces under the same ecosystem lock. | Test (TC-210) |
| FR-181-AC-3 | Missing bounds or exhausted work returns requires-bound/incomplete with the unexplored scope. | Test (TC-210) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
