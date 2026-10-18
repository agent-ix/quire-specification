---
id: FR-054
title: "Bind explicit channel and delivery premises"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-051
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-215
    type: references
  - target: ix://agent-ix/quire-specification/FR-216
    type: references
  - target: ix://agent-ix/quire-specification/FR-224
    type: references
---
## Description

When admitting a protocol channel, the protocol linker SHALL bind its sender,
receiver, message type, ordering policy and delivery policy as explicit
conformance assumptions, and its queue's capacity and overflow policy. Whether
a duplicate delivery or a loss is permitted is derived only from the delivery
policy and is not a further independent premise.

## Inputs

Typed channel/model references and authored communication premises.

## Outputs

A channel contract bound into protocol identity, or a located typed refusal.

## Behavior

The only ordering policies are `unordered` and `fifo`. A FIFO policy SHALL retain
its declaration-local message binder and exact admitted typed key expression;
the expression's result type SHALL have defined stable equality semantics. At runtime the FIFO
partition key is the pair of the exact static channel key and the exact typed
value produced by that expression for the message. Equal expression values on
different channels never create cross-channel order. `unordered` positively
asserts that no FIFO expression or partition scope exists.

The delivery policy is one of `unknown`, `at-most-once`, `at-least-once` and
`exactly-once-premise`. `unknown` permits duplication and loss and supports no
delivery conclusion; `at-most-once` prohibits duplication and permits loss;
`at-least-once` permits duplication, prohibits loss and requires eventual
delivery or an explicit nonprogress result; `exactly-once-premise` is an
authored environment premise that prohibits both. None establishes a second
message, attempt or effect. The queue's capacity is a literal bound or a
symbolic name with a discharged bound, and its overflow policy is `reject`,
`block` or `loss` with a `head`, `tail` or `new` selector ([FR-170](FR-170-evaluate-channel-queue-semantics.md)).

The linker SHALL NOT infer exactly-once delivery, FIFO, reliable delivery or
synchronized clocks from another premise. A queue or endpoint SHALL remain a
channel/component reference rather than becoming a participant role. A
conformance result SHALL identify the exact premises under which it was derived.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-054-AC-1 | Delivery policy `at-least-once` permits a second delivery of one send while retaining one authored message and effect obligation; `exactly-once-premise` prohibits the second delivery without a separate duplication premise. | Test (TC-054) |
| FR-054-AC-2 | A FIFO premise orders messages only within the pair of its exact channel key and evaluated exact typed key value; an equal value on another channel is unordered relative to it. | Test (TC-054) |
| FR-054-AC-3 | Missing or conflicting sender, receiver, message type, ordering policy or delivery policy; an unknown ordering member; an unknown delivery policy; or an invalid FIFO binder/expression refuses admission. | Test (TC-054) |
| FR-054-AC-4 | Changing any of the five independent channel premises, including the admitted FIFO expression definition or the delivery policy, changes the conformance request identity and its result provenance. | Test (TC-054) |
| FR-054-AC-5 | `unordered` carries no FIFO key scope, and neither the delivery policy nor a synchronized-clock fact creates one. | Test (TC-054) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md).
- [FR-051](FR-051-preserve-communication-identities.md).
