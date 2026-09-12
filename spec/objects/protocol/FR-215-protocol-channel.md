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
---
# FR-215: Protocol channel

## Description

When the protocol linker admits a protocol channel, the linker SHALL bind its sender, receiver, message types, ordering policy, delivery cardinality and duplication premises as explicit conformance assumptions rather than inferring exactly-once delivery, FIFO order across channels, reliable delivery or synchronized clocks.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Sender | FR-054 | A bound premise; a missing or conflicting sender premise refuses admission (FR-054-AC-3). |
| Receiver | FR-054 | A bound premise; a missing or conflicting receiver premise refuses admission (FR-054-AC-3). |
| Message types | FR-054 | A bound premise; see [FR-214](FR-214-protocol-message.md). |
| Ordering policy | FR-054, FR-054-AC-2 | "A FIFO premise orders messages only within its declared channel and key scope." |
| Delivery cardinality | FR-054, FR-054-AC-1 | "At-least-once delivery admits a duplicate receipt while retaining the single authored message/effect obligation." |
| Duplication premise | FR-054 | Whether duplication can occur is authored, never inferred. |
| Typed declaration | FR-050 | Every channel reference is bound "to its exact typed declaration and configuration identity before conformance evaluation." |
| Channel contract | FR-054 | The output is "A channel contract bound into protocol identity, or a located typed refusal." |

The six premise components above are the value object [FR-216](FR-216-channel-premise.md).

A channel is not a role: "The linker SHALL keep roles distinct from components,
endpoints and channels" (FR-050), and "A queue or endpoint SHALL remain a
channel/component reference rather than becoming a participant role" (FR-054).

## Identity and comparison

- **Identity kind** — *authored*. A channel is admitted from a native protocol
  declaration and bound to its exact typed declaration (FR-050); FR-049-AC-1
  binds channel positions to their required declaration kinds. It is not
  computed from a transport record.
- **Key** — **the prose does not fix the key components.** FR-054 states that
  the channel contract is "bound into protocol identity" and that "Changing any
  channel premise changes the conformance request identity and its result
  provenance" (FR-054-AC-4). That establishes that a premise change changes the
  *request* identity; it does not state that the premises are the channel's own
  key. Established exclusions: an endpoint or channel identity substituted for a
  role refuses (FR-050-AC-2), and a name-only substitution refuses (FR-050).
  Equality is componentwise over whatever the key is and nothing else; display
  name, source path, span, message text, locale and rendering never participate
  (FR-035, FR-049-AC-7).
- **Key order** — none. Because the key components are not fixed, no total
  order over the key is stated and none is supplied here.
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
- **Digest domain** — no component named by the prose is a digest. Where a
  channel identity or its premises are carried beside a digest in result
  provenance (FR-054-AC-4), that digest's domain is selected under
  `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-215-AC-1 | A channel admitted without a sender, receiver, message-type, order or delivery premise does not resolve, and no premise is defaulted in its place. | Analysis |
| FR-215-AC-2 | A channel identity supplied in a role position does not resolve, and a queue or endpoint reference does not become a role. | Analysis |
| FR-215-AC-3 | Changing a channel premise yields a different conformance request identity and result provenance from the unchanged one. | Analysis |
| FR-215-AC-4 | A FIFO ordering premise constrains only the messages within its own declared channel and key scope. | Analysis |
| FR-215-AC-5 | This artifact declares no key components and no key order for a channel, and cites the prose exclusions instead. | Inspection |

## Dependencies

- [FR-054](../../functional/FR-054-bind-channel-premises.md) binds the channel
  premises as explicit conformance assumptions.
- [FR-050](../../functional/FR-050-bind-protocol-instances.md) binds channel
  references and separates channels from roles.
- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) binds
  channel positions to their required declaration kinds.
- [FR-216](FR-216-channel-premise.md) is the premise value object this channel
  contains.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

1. No requirement states what constitutes a channel's identity. FR-054-AC-4
   states that changing a premise changes the *conformance request* identity; no
   requirement states whether the premises are components of the channel's own
   key, so this artifact does not treat them as such.
2. FR-054-AC-2 scopes a FIFO premise to a "key scope". No requirement states
   what a channel key scope is, what its components are, or how it relates to
   the identity keys of this bounded context. The two uses of "key" are not
   established to be the same notion.
3. FR-054-AC-3 lists "sender, receiver, message-type, order or delivery"
   premises as refusing when missing or conflicting; the FR-054 description
   lists six premises, adding "duplication premises". Whether a missing
   duplication premise refuses admission is unstated.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
