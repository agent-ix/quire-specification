---
id: FR-215
title: "Protocol channel"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-054
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-216
    type: contains
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-215: Protocol channel

## Description

When the protocol linker admits a protocol channel, the linker SHALL bind its sender, receiver, message types, ordering policy and delivery policy premises as explicit conformance assumptions rather than inferring exactly-once delivery, FIFO order across channels, reliable delivery or synchronized clocks.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| sender | String | 1..1 |  |
| receiver | String | 1..1 |  |
| message_types | ProtocolMessage | 0..* |  |
| ordering_policy | String | 1..1 |  |
| delivery_policy | String | 1..1 |  |
| typed_declaration | String | 1..1 |  |
| channel_contract | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| sender | type not stated in source; multiplicity 1..1, a missing premise refuses admission (FR-054-AC-3) |
| receiver | type not stated in source; multiplicity 1..1, a missing premise refuses admission (FR-054-AC-3) |
| message_types | a bound premise: message types of the channel |
| ordering_policy | type not stated in source; multiplicity 1..1, a missing premise refuses admission (FR-054-AC-3) |
| delivery_policy | type not stated in source; see Field sources |
| typed_declaration | type not stated in source; see Field sources |
| channel_contract | type not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Sender | FR-054 | A bound premise; a missing or conflicting sender premise refuses admission (FR-054-AC-3). |
| Receiver | FR-054 | A bound premise; a missing or conflicting receiver premise refuses admission (FR-054-AC-3). |
| Message types | FR-054 | A bound premise; see [FR-214](FR-214-protocol-message.md). |
| Ordering policy | FR-054, FR-054-AC-2 | "A FIFO premise orders messages only within its declared channel and key scope." |
| Delivery policy | FR-054, FR-054-AC-1 | "At-least-once delivery admits a duplicate receipt while retaining the single authored message/effect obligation." |
| Typed declaration | FR-050 | Every channel reference is bound "to its exact typed declaration and configuration identity before conformance evaluation." |
| Channel contract | FR-054 | The output is "A channel contract bound into protocol identity, or a located typed refusal." |

The six premise components above are the value object [FR-216](FR-216-channel-premise.md).

A channel is not a role: "The linker SHALL keep roles distinct from components,
endpoints and channels" (FR-050), and "A queue or endpoint SHALL remain a
channel/component reference rather than becoming a participant role" (FR-054).

## Identity and comparison

- **Identity kind** — *derived*. The channel is authored; its identity is
  derived from the linked definition and its admitted declaration-local handle.
- **Key** — (`qualified protocol-definition identity`, `declaration-local
  channel handle`), exactly as FR-050 states. The six channel premises are
  semantic properties that contribute to the containing linked protocol and
  request identity; they are not repeated as channel-key components. The
  current wire represents the handle as a table index qualified by its admitted
  package and declaration; an unqualified index, display name or locus does not
  participate.
- **Key order** — lexicographic over the two components in the declared
  sequence for stable comparison only. It supplies no FIFO, causal or channel
  priority order.
- **Causal order** — a channel carries no causal order. Its ordering policy is
  a conformance premise scoped to "its declared channel and key scope"
  (FR-054-AC-2) and FR-054 forbids inferring "FIFO order across channels". The
  protocol's causal order remains the authored partial acyclic graph of FR-052,
  never derived from key order, timestamp or source position (FR-052-AC-2).
- **Absence** — an absent key component compares only to absence. A missing
  premise is never defaulted: "The linker SHALL NOT infer exactly-once delivery,
  FIFO order across channels, reliable delivery or synchronized clocks"
  (FR-054), and "Missing or conflicting sender, receiver, message-type, order or
  delivery premises refuse admission" (FR-054-AC-3).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-215-AC-1 | A channel admitted without a sender, receiver, message-type, order or delivery premise does not resolve, and no premise is defaulted in its place. | Analysis |
| FR-215-AC-2 | A channel identity supplied in a role position does not resolve, and a queue or endpoint reference does not become a role. | Analysis |
| FR-215-AC-3 | Changing a channel premise yields a different conformance request identity and result provenance from the unchanged one. | Analysis |
| FR-215-AC-4 | A FIFO ordering premise constrains only the messages within its own declared channel and key scope. | Analysis |
| FR-215-AC-5 | Two channels compare equal exactly when their qualified definition and declaration-local channel handle compare equal; display name and source locus do not participate. | Analysis |

## Dependencies

- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) binds the channel
  premises as explicit conformance assumptions.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds channel
  references and separates channels from roles.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) binds
  channel positions to their required declaration kinds.
- [FR-216](FR-216-channel-premise.md) is the premise value object this channel
  contains.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked-definition component.

## Resolved premise contract

The FIFO scope key is the ordered tuple (`workflow instance`, `channel`,
`sender role instance`, `receiver role instance`, `message type`, optional
`conversation partition`). FIFO compares only deliveries with equal complete
keys. Ordering and delivery-policy premises are all required;
a missing premise yields unavailable/incomplete protocol analysis and a
conflicting premise refuses admission. No missing premise defaults to reliable,
ordered or exactly-once delivery.

## Status

Accepted complete-V1 channel contract.
