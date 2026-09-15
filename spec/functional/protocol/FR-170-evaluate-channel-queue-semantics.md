---
id: FR-170
title: "Evaluate explicit channel and queue semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-054
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
---

# FR-170: Evaluate explicit channel and queue semantics

## Description

When checking a protocol channel, the checker SHALL require its ordering,
duplication, queue-capacity, overflow and delivery premises explicitly.

## Inputs

Channel identity, FIFO key definition, queue state/bounds, delivery premises and
message occurrences.

## Outputs

Checked channel/queue transitions, incomplete premise result or located refusal.

## Behavior

FIFO ordering applies only within the declared role/workflow/channel/message key.
Duplication, loss and delivery guarantees default to unknown, never reliable.
Capacity may be concrete or symbolic with a discharged bound. Overflow behavior
is reject, block or declared loss and remains visible to conformance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-170-AC-1 | Messages are ordered only inside the exact declared FIFO key and independent keys may interleave. | Test (TC-201) |
| FR-170-AC-2 | Missing duplication/delivery premises yield incomplete/refused analysis rather than an exactly-once assumption. | Test (TC-201) |
| FR-170-AC-3 | Concrete/symbolic capacity and each overflow policy produce the declared queue transition or requires-bound outcome. | Test (TC-201) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
