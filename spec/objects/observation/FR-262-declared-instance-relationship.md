---
id: FR-262
title: "Declared instance relationship"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-111
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-213
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-262: Declared instance relationship

## Description

When an observation is associated with a workflow subject, the correlation service SHALL select only an explicit typed relationship carrying its selected Producer interface 1.2.0 identity, version, digest and declared digest domain, exact producer relationship identity, and ordered authority-qualified endpoints.

FR-111 defines exactly `resolved`, `missing`, `conflicting` and `ambiguous`
correlation outcomes, and states that the three non-success outcomes replace
heuristic matching. It also states that the service refuses "a missing producer
identity, unknown producer version, unknown relationship declaration/kind, or
producer digest mismatch rather than reconstructing a relationship from a
foreign key, trace ID, provider name, or ambient configuration".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Relationship type | FR-111 Description | "an explicit typed relationship". |
| Producer interface identity | FR-111 Description | Selected Producer interface 1.2.0 identity. |
| Producer version | FR-111 Description | An unknown version refuses (FR-111-AC-4). |
| Producer digest | FR-111 Description | The exact digest and its explicitly declared FR-201 domain; absence or mismatch refuses. |
| Relationship identity | FR-111 | Exact producer-supplied identity selected under that Producer authority; a kind label does not replace it. |
| Related instance endpoints | FR-111 Behavior | Ordered source and target, each carrying one complete FR-287 key and satisfying the producer declaration's endpoint type/direction. |

FR-111 Behavior states that "a telemetry trace ID or shared provider identity
does not establish either relationship", and FR-111-AC-1 that "A refund linked
to O1 is refused for O2 even when both orders share a provider or trace
infrastructure."

## Identity and comparison

- **Identity kind** — *value*. A relationship has no identity of its own; it is
  compared componentwise. It is not *derived*: FR-111 forbids reconstructing it
  "from a foreign key, trace ID, provider name, or ambient configuration".
- **Key** — the relationship type, selected Producer interface identity,
  version, digest and declared digest domain, exact producer relationship
  identity, and ordered source and target FR-287 endpoint keys.
  FR-111-AC-4 establishes the producer identity, version and digest as
  key-participating: a missing, unknown or mismatched one refuses rather than
  resolves. Equality is componentwise
  over the key and nothing else; display name, source path, span, message text,
  locale and rendering never participate, and a trace ID, provider name, foreign
  key or ambient configuration is not a key component (FR-111 Behavior).
- **Key order** — lexicographic over the key in the declared sequence, using
  FR-287 endpoint order without sorting source and target. It serves canonical
  collection comparison only and carries no causal meaning: FR-113 states that
  "Arbitrary timestamp order SHALL NOT establish causality."
- **Causal order** — the declared relationship structure itself, and nothing
  else. It is never derived from key order, from a timestamp, from a telemetry
  trace ID or from a shared provider identity (FR-111 Behavior, FR-111-AC-1).
  A repeated transport fact adds no causal node: "A duplicate receipt SHALL NOT
  establish a second business effect" (FR-111 Behavior), and repeated delivery
  "cannot alone create a second attempt or effect" (FR-111-AC-3).
- **Absence** — every key component of an offered relationship is required;
  missing/empty, wrong-kind, foreign-authority, reversed or incompatible values
  refuse. Absence of any admitted candidate for a required slot is the distinct
  `missing` incomplete outcome, never a default relationship. A conflicting or
  ambiguous candidate refuses.
- **Digest domain** — the producer digest carries a selected domain under
  `ix://agent-ix/quire-specification/FR-201`. An identity crossing domains is
  unequal regardless of byte equality.

## Correlation outcomes and distinct subjects

The closed outcome set is `resolved`, `missing`, `conflicting`, `ambiguous`.
`missing` means no admitted candidate exists for a required slot and is
incomplete. `conflicting` means a candidate under the required declaration and
source endpoint asserts an incompatible endpoint or direction and refuses.
`ambiguous` means multiple distinct admitted candidates satisfy one required
slot and refuses. An unknown producer version, relationship declaration/kind or
foreign authority is an admission refusal, not a fifth correlation outcome.

Workflow instances, messages, receive occurrences, deliveries, attempts,
effects and receipt observations remain distinct. A receipt is an observation
record referring to a delivery and never aliases that delivery, the message or
the receive occurrence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-262-AC-1 | A relationship declared between O1 and its refund does not resolve for O2 sharing a provider or trace infrastructure. | Analysis |
| FR-262-AC-2 | A missing, unknown-version or digest-mismatched producer relationship identity does not resolve and is not reconstructed from a foreign key, trace ID, provider name or ambient configuration. | Analysis |
| FR-262-AC-3 | `resolved`, `missing`, `conflicting` and `ambiguous` are the complete unequal outcome set; missing is incomplete, conflicting/ambiguous refuse, and unknown producer/declaration input is an admission refusal rather than another relationship outcome. | Test (TC-141) |
| FR-262-AC-4 | Exact replay of one complete relationship value is idempotent and adds no causal edge; two distinct admitted values satisfying one required slot are ambiguous. | Test (TC-141) |
| FR-262-AC-5 | Source and target each retain a complete FR-287 key in authored order; empty, wrong-kind, foreign, reversed or statically incompatible endpoints refuse. | Test (TC-141) |
| FR-262-AC-6 | Message, receive occurrence, delivery and receipt observation remain non-interchangeable even when transport or display data match. | Test (TC-141) |
| FR-262-AC-7 | The producer digest carries an explicit declared domain; absence, cross-domain byte equality or digest mismatch refuses without reconstruction or defaulting. | Test (TC-141) |

## Dependencies

- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  establishes the explicit typed relationship and its refusal cases.
- [FR-213](../protocol/FR-213-workflow-instance.md) supplies the related
  instance subject and records that its key is unstated.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](../protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  supplies the ordered source and target keys.

## Status

Proposed closure for specification issue #31. Spec Language already validates
static producer relationship exports, endpoint types and direction. Observation
currently uses unqualified endpoint identities, ignores relationship identity
when matching and treats exact replay as ambiguity; that implementation is
partial and incompatible. Runtime adoption remains planned under TC-141 and
the existing Observation/Protocol owner tickets.
