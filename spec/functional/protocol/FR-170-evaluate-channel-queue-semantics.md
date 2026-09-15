---
id: FR-170
title: "Evaluate explicit channel and queue semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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

## Queue transition system

Queue state is an ordered sequence of message occurrences, each with a unique
occurrence identity and FIFO key. `send` appends when capacity permits;
`deliver` removes the earliest deliverable occurrence for its key; `duplicate`
creates a new occurrence linked to its source only when duplication is allowed;
and `lose` removes an occurrence only when loss is allowed. At full capacity,
`reject` leaves state unchanged with a rejected send, `block` leaves a pending
send, and `loss` applies the declared head/tail/new-message loss selector.
At-most-once prohibits duplication, at-least-once requires eventual delivery or
an explicit nonprogress result, and exactly-once requires both plus stable
message identity. Unknown premises support none of those conclusions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-170-AC-1 | Messages are ordered only inside the exact declared FIFO key and independent keys may interleave. | Test (TC-201) |
| FR-170-AC-2 | Missing duplication/delivery premises yield incomplete/refused analysis rather than an exactly-once assumption. | Test (TC-201) |
| FR-170-AC-3 | Concrete/symbolic capacity and each overflow policy produce the declared queue transition or requires-bound outcome. | Test (TC-201) |
| FR-170-AC-4 | Send, deliver, duplicate and lose preserve occurrence identity and modify only the selected FIFO-key queue. | Test (TC-201) |
| FR-170-AC-5 | Exactly-once is established only from explicit no-loss, no-duplication, progress and stable-identity premises. | Test (TC-201) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
