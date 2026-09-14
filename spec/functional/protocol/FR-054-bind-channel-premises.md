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
receiver, message type, ordering policy and closed delivery-cardinality interval
as explicit conformance assumptions. Whether more than one delivery is permitted
is derived only from that interval and is not a sixth independent premise.

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

Delivery cardinality is the authored inclusive interval `[minimum, maximum]` of
nonnegative delivery counts per send, with `minimum <= maximum`. A maximum no
greater than one prohibits duplicate deliveries; a maximum greater than one
permits them. Neither case establishes a second message, attempt or effect.

The linker SHALL NOT infer exactly-once delivery, FIFO, reliable delivery or
synchronized clocks from another premise. A queue or endpoint SHALL remain a
channel/component reference rather than becoming a participant role. A
conformance result SHALL identify the exact premises under which it was derived.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-054-AC-1 | Delivery `[1,2]` permits a second delivery of one send while retaining one authored message and effect obligation; `[1,1]` prohibits the second delivery without a separate duplication premise. | Test (TC-054) |
| FR-054-AC-2 | A FIFO premise orders messages only within the pair of its exact channel key and evaluated exact typed key value; an equal value on another channel is unordered relative to it. | Test (TC-054) |
| FR-054-AC-3 | Missing or conflicting sender, receiver, message type, ordering policy or delivery interval; an unknown ordering member; an invalid interval; or an invalid FIFO binder/expression refuses admission. | Test (TC-054) |
| FR-054-AC-4 | Changing any of the five independent channel premises, including the admitted FIFO expression definition or an interval endpoint, changes the conformance request identity and its result provenance. | Test (TC-054) |
| FR-054-AC-5 | `unordered` carries no FIFO key scope, and neither delivery cardinality nor a synchronized-clock fact creates one. | Test (TC-054) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md).
- [FR-051](FR-051-preserve-communication-identities.md).
