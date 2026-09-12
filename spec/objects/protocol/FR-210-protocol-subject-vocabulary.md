---
id: FR-210
title: "Protocol subject vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-035
    type: specifies
---
# FR-210: Protocol subject vocabulary

## Description

When a protocol requirement names a subject, the specification reader SHALL resolve that subject as a distinct typed kind rather than as a synonym recoverable from a transport record, a trace identifier or an equal label.

## Values

Each member below is named as a distinct subject by at least one requirement.
The citation records which requirement names it. No requirement names this whole
set; see **Open gap**.

| Value | Description |
| --- | --- |
| `message` | The authored typed communication obligation. Named distinct by FR-051 ("preserve message, send, receive, delivery, attempt and effect as distinct typed subjects") and carried as "the single authored message/effect obligation" by FR-054-AC-1. |
| `send` | A protocol event. Named distinct by FR-051 and by FR-049-AC-2. A send alone does not establish a business effect (FR-051 behavior). |
| `receive` | A protocol event held distinct from `send`. Named distinct by FR-051 and by FR-049-AC-2. A receive alone does not establish a business effect (FR-051 behavior). |
| `delivery` | A transport observation. Named distinct by FR-051, FR-049-AC-2, FR-111 and FR-035 (as "message deliveries"). Repeated delivery does not establish a second effect (FR-051-AC-2, FR-111-AC-3). |
| `receipt` | Named distinct by FR-111 ("Workflow instances, deliveries, attempts, effects, and receipts remain distinct"). FR-049-AC-2 and FR-054-AC-1 both use "duplicate receipt" for the admitted repetition that FR-051-AC-1 calls "delivered twice". No requirement states whether `receipt` and `delivery` are one subject or two. |
| `attempt` | One invocation try. Named distinct by FR-051, FR-111 and FR-035; FR-049-AC-2 names it "operation attempt". An attempt with no established effect remains an attempt (FR-051-AC-3). |
| `effect` | A declared successful business-state transition. Named distinct by FR-051, FR-049-AC-2, FR-111 and FR-035. It is never inferred from a send, receive, successful transport status, repeated delivery, trace identifier or equal payload alone (FR-051 behavior). |
| `compensation` | Named distinct by FR-049-AC-2 ("Send, receive, delivery, operation attempt, effect and compensation references remain distinct"). FR-051, FR-111 and FR-035 do not name it in their subject lists. |
| `workflow-instance` | One assessed occurrence. Named distinct by FR-111 and FR-035; FR-050 binds it to its exact typed declaration and configuration identity. FR-051 does not name it in its subject list. |
| `role-instance` | Named distinct by FR-035 ("Workflow instances, role instances, message deliveries, attempts and effects remain distinct subjects") and kept distinct from its static definition by FR-050. FR-051, FR-049-AC-2 and FR-111 do not name it in their subject lists. |

## Identity and comparison

- **Identity kind** — *value*. A subject kind label has no identity of its own
  and is compared componentwise as an exact kind. The identity of an occurrence
  of a subject belongs to that subject's own object artifact, not to this
  vocabulary.
- **Key** — the subject kind itself. Two references of different kinds are
  unequal. FR-050-AC-2 refuses substituting an endpoint or channel identity for
  a role and a component identity for a role instance; FR-051-AC-4 refuses a
  transport trace ID substituted for an authored workflow, attempt or effect
  identity. Display name, source path, span, message text, locale and rendering
  never participate: FR-035 states that "a trace ID, timestamp, endpoint or
  matching textual name alone does not establish a business relationship", and
  FR-049-AC-7 requires the exact declared relationship and endpoint direction
  "even when they share a provider, trace ID or equal scalar labels".
- **Key order** — none is established. No requirement fixes a spelling, an
  ordinal or a precedence for these subject kinds, so this artifact declares no
  key order and supplies none. Members are listed above in the order FR-051
  states them, extended by the members other requirements add; that listing
  order is presentational and carries no meaning.
- **Causal order** — not applicable to the vocabulary. Causal order over
  protocol nodes is the authored partial acyclic order of FR-052, which "SHALL
  construct a finite causal graph ... without deriving causal edges from source
  order or timestamps" (FR-052) and is never derived from key order, timestamp
  or source position (FR-052-AC-2).
- **Absence** — a reference whose subject kind is absent compares only to
  absence. It is never defaulted to `delivery`, to `effect` or to any other
  member; FR-051 forbids inferring an effect from a transport fact alone.
- **Digest domain** — no member of this key is a digest. Where a protocol
  identity is carried beside a digest in provenance, that digest's domain is
  selected under `ix://agent-ix/quire-specification/FR-201`, and an identity
  crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-210-AC-1 | Every member of the table is named as a distinct subject by the requirement cited in its description. | Inspection |
| FR-210-AC-2 | Two references of different subject kinds compare unequal, and no transport record, trace identifier or equal label makes them equal. | Analysis |
| FR-210-AC-3 | A reference with an absent subject kind compares only to absence and receives no default member. | Analysis |
| FR-210-AC-4 | No member of this table is assigned an ordinal, precedence or wire spelling by this artifact. | Inspection |

## Dependencies

- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) names send,
  receive, delivery, operation attempt, effect and compensation.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds roles, role
  instances, workflow instances, messages and channels to exact declarations.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md) names
  message, send, receive, delivery, attempt and effect.
- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md) names workflow
  instances, role instances, message deliveries, attempts and effects.
- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  names workflow instances, deliveries, attempts, effects and receipts.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

Four requirements each state a closed list of "distinct" protocol subjects, and
no requirement cross-references the others. This artifact records all four
verbatim and does not reconcile them.

- FR-051: "the protocol binder SHALL preserve message, send, receive, delivery,
  attempt and effect as distinct typed subjects."
- FR-049-AC-2: "Send, receive, delivery, operation attempt, effect and
  compensation references remain distinct; a duplicate receipt cannot mint
  another effect node."
- FR-111: "Workflow instances, deliveries, attempts, effects, and receipts
  remain distinct."
- FR-035: "Workflow instances, role instances, message deliveries, attempts and
  effects remain distinct subjects."

Unresolved by the prose, in addition to the differing membership:

1. Whether `receipt` (FR-111, FR-049-AC-2, FR-054-AC-1) and `delivery` (FR-051,
   FR-051-AC-1) are one subject or two.
2. Whether "attempt" (FR-051) and "operation attempt" (FR-049-AC-2) are one
   subject, and whether a compensation attempt is an `attempt` or a member of
   `compensation`.
3. Whether "message deliveries" (FR-035) is the `delivery` subject or a
   compound of `message` and `delivery`.
4. Whether any spelling of these subject kinds is normative. No requirement
   fixes a label string, so no key order over the members can be established.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
