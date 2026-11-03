---
id: FR-170
title: "Evaluate explicit channel and queue semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-054
    type: references
  - target: ix://agent-ix/quire-specification/FR-050
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
  - target: ix://agent-ix/quire-specification/FR-465
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
is reject, block or declared loss (with its head, tail or new selector) and remains visible to conformance.

## Queue transition system

Queue state is an ordered sequence of message occurrences, each with a unique
occurrence identity and FIFO key. `send` appends when capacity permits;
`deliver` removes the earliest deliverable occurrence for its key; `duplicate`
creates a new occurrence linked to its source only when duplication is allowed;
and `lose` removes an occurrence only when loss is allowed. At full capacity,
`reject` leaves state unchanged with a rejected send, `block` leaves a pending
send, and `loss` admits the send and loses one message chosen by the declared loss selector: `head` drops the oldest queued message, `tail` drops the newest queued message and `new` drops the arriving message. Under `head` and `tail` the arriving message is then appended; under `new` it is not. The queue is otherwise unchanged.
Every `send` binds its binder to the sent payload and gives its step a **send record** `{channel, payload, outcome, displaced}` as its result, carried as the step's `send_record` ([FR-433](FR-433-replay-protocol-counterexamples.md)): `channel` is the channel's qualified name ([FR-429](FR-429-report-protocol-deadlocks.md) defines its rendering over the [FR-050](FR-050-bind-protocol-instances.md) identity); `outcome` is `accepted` when the message was appended, `rejected` when `reject` refused it at capacity, and `lost` when a `new` selector dropped the arriving message; `displaced` is the queue entry (send node, sending thread path and payload) that a `head` or `tail` selector removed to admit the arriving message, and is none otherwise. A `rejected` or `lost` send leaves no occurrence; a send that displaces one also appends its own.

**Delivery count interval.** On a channel whose delivery is an interval
`[lo,hi]` ([FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md)),
a send chooses a count `d` with `lo <= d <= hi`, one alternative per `d`, and
produces `d` occurrences of the one payload, processed one after another. Each
occurrence is a message as above: it takes one place of the capacity and is
subject to the overflow policy on its own, so under `reject` an occurrence
that finds the queue full is rejected and the later ones are still
processed, and under `loss` the selector applies to each occurrence that finds
the queue full. Under `block` the occurrences are appended while the queue has
room; when it is full with `r` occurrences still unprocessed, the send is
pending with `r` occurrences, and the sender appends them, again while the
queue has room, once a removal frees a place. `d = 0` appends nothing, so it is the interval's only transport
loss, and `d > 1` its only duplication: an interval channel has no separate
`duplicate` or `lose` transition. Its send record has `count` and `entries`
and `pending` in place of `outcome` and `displaced`: `count` is the number of
occurrences the send processes, `entries` holds one `{outcome, displaced}`
per processed occurrence in processing order (empty for `d = 0`), and
`pending` is the number left for a later append under `block`, 0 otherwise.
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
| FR-170-AC-6 | Each send binds its binder to the payload and gives its step a `send_record` `{channel, payload, outcome, displaced}` with `channel` in the qualified name: on an empty queue `accepted` with `displaced` none; at capacity `reject` gives `rejected` and `loss new` gives `lost`, each leaving the queue unchanged; `loss head` gives `accepted` with `displaced` the oldest queue entry and `loss tail` `accepted` with `displaced` the newest queue entry, each appending the arriving message. | Test (TC-378) |
| FR-170-AC-7 | On a channel with delivery `[lo,hi]`, a send has one alternative per `d` from `lo` to `hi` and processes `d` occurrences, each taking one place and subject to the overflow policy: on a capacity 2 channel, `[2,2]` queues two occurrences, `[0,1]` has a `d = 0` alternative that queues none with `entries` empty, under `reject` `[2,2]` with one place free gives `entries` `accepted` then `rejected`, and under `block` `[3,3]` on an empty queue appends two occurrences and leaves the send pending with 1. | Test (TC-378) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
