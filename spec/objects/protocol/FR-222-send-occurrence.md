---
id: FR-222
title: "Send occurrence"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-215
    type: references
---
# FR-222: Send occurrence

## Description

When a communication is bound to a protocol instance, the protocol binder SHALL preserve the send as a subject distinct from the message, the receive, the delivery, the attempt and the effect ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — a send is an
  authored protocol node, so it carries the node occurrence identity. FR-049-AC-2
  keeps send references distinct from receive, delivery, operation attempt,
  effect and compensation references.
- **channel** ([FR-215](FR-215-protocol-channel.md)) — the declared channel the
  send is made on. FR-054 binds the channel's sender, receiver, message types,
  ordering policy, delivery cardinality and duplication premises as explicit
  conformance assumptions, held as the channel's premise set
  ([FR-216](FR-216-channel-premise.md)).
- **message** — the authored typed communication obligation the send carries.
  FR-054-AC-1 calls it "the single authored message/effect obligation".
- **typed payload record** — the bound record. The payload alone establishes
  nothing further: FR-051 states that the binder "SHALL NOT infer a business
  effect from a send, receive, successful transport status, repeated delivery,
  trace identifier or equal payload alone".

A send is the per-send unit that channel delivery cardinality is stated against:
FR-054 binds "delivery cardinality" as a channel premise, and the
[choreography surface](../../../proposals/quire-v1/choreography-surface.md)
states that "Channel `delivery [a,b]` declares inclusive per-send delivery
cardinality".

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is. Its key is
  the occurrence key of [FR-221](FR-221-protocol-node-occurrence.md).
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`), as held by FR-221. Equality is componentwise over that
  key and nothing else. Display name, source path, span, message text, locale
  and rendering never participate; neither does the payload, since equal payload
  alone establishes nothing (FR-051), nor a transport trace identifier, which
  refuses substitution for an authored identity (FR-051-AC-4).
- **Key order** — none is established. No requirement fixes a total order over
  sends, and this artifact supplies none.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220. It is never derived from key order, timestamp or source position
  (FR-052, FR-052-AC-2, FR-113).
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. A send outside every bounded iteration has an absent
  iteration identity.
- **Digest domain** — no component of this key is a digest. A digest carried
  beside a send in provenance carries the domain it was derived under,
  `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-222-AC-1 | A send reference does not satisfy a comparison expecting a receive, a delivery, an attempt or an effect. | Analysis |
| FR-222-AC-2 | One message sent once retains cardinality 1 for the send subject regardless of how many deliveries are observed. | Analysis |
| FR-222-AC-3 | A send with a successful transport status does not establish a business effect. | Analysis |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves send as a distinct typed subject.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) keeps
  send references distinct from the other communication references.
- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) binds the channel
  the send is made on.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
