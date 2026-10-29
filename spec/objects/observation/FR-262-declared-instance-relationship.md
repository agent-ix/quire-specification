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
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-262: Declared instance relationship

## Description

When an observation is associated with a workflow subject, the correlation service SHALL select only an explicit typed relationship carrying its domain package identity and version, exact relationship declaration key, and ordered authority-qualified endpoints.

FR-111 defines exactly `resolved`, `missing`, `conflicting` and `ambiguous`
correlation outcomes, and states that the three non-success outcomes replace
heuristic matching. It also states that the service refuses "a missing domain package identity, a
relationship declaration or kind outside the checked package, or a declaration
key naming a different package identity or version rather than reconstructing a
relationship from a foreign key, trace ID, provider name, or ambient
configuration".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Relationship type | FR-111 Description | "an explicit typed relationship". |
| Domain package identity and version | FR-111 Description | The domain package identity and version named by the checked package `model_selections`; absence or mismatch refuses (FR-111-AC-4). |
| Relationship declaration key | FR-111 | Exact (domain package identity, IR node identity) key of the spec-declared relationship; a kind label does not replace it. |
| Related instance endpoints | FR-111 Behavior | Ordered source and target, each carrying one complete FR-287 key and satisfying the relationship declaration's endpoint type/direction. |

FR-111 Behavior states that "a telemetry trace ID or shared provider identity
does not establish either relationship", and FR-111-AC-1 that "A refund linked
to O1 is refused for O2 even when both orders share a provider or trace
infrastructure."

## Identity and comparison

- **Identity kind** — *value*. A relationship has no identity of its own; it is
  compared componentwise. It is not *derived*: FR-111 forbids reconstructing it
  "from a foreign key, trace ID, provider name, or ambient configuration".
- **Key** — the relationship type, domain package identity and version,
  exact relationship declaration key, and ordered
  source and target FR-287 endpoint keys.
  FR-111-AC-4 establishes the declaration key as
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
- **Digest domain** — not applicable; the relationship carries no digest.

## Correlation outcomes and distinct subjects

The closed outcome set is `resolved`, `missing`, `conflicting`, `ambiguous`.
`missing` means no admitted candidate exists for a required slot and is
incomplete. `conflicting` means a candidate under the required declaration and
source endpoint asserts an incompatible endpoint or direction and refuses.
`ambiguous` means multiple distinct admitted candidates satisfy one required
slot and refuses. A relationship declaration or kind outside the checked package
or a foreign authority is an admission refusal, not a fifth correlation outcome.

Workflow instances, messages, receive occurrences, deliveries, attempts,
effects and receipt observations remain distinct. A receipt is an observation
record referring to a delivery and never aliases that delivery, the message or
the receive occurrence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-262-AC-1 | A relationship declared between O1 and its refund does not resolve for O2 sharing a provider or trace infrastructure. | Analysis |
| FR-262-AC-2 | A missing, foreign or identity-or-version-mismatched relationship declaration key does not resolve and is not reconstructed from a foreign key, trace ID, provider name or ambient configuration. | Analysis |
| FR-262-AC-3 | `resolved`, `missing`, `conflicting` and `ambiguous` are the complete unequal outcome set; missing is incomplete, conflicting/ambiguous refuse, and a declaration outside the checked package is an admission refusal rather than another relationship outcome. | Test (TC-141) |
| FR-262-AC-4 | Exact replay of one complete relationship value is idempotent and adds no causal edge; two distinct admitted values satisfying one required slot are ambiguous. | Test (TC-141) |
| FR-262-AC-5 | Source and target each retain a complete FR-287 key in authored order; empty, wrong-kind, foreign, reversed or statically incompatible endpoints refuse. | Test (TC-141) |
| FR-262-AC-6 | Message, receive occurrence, delivery and receipt observation remain non-interchangeable even when transport or display data match. | Test (TC-141) |
| FR-262-AC-7 | Withdrawn: it existed only for the declaration digest and its domain, which the relationship no longer carries. | Withdrawn |

## Dependencies

- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  establishes the explicit typed relationship and its refusal cases.
- [FR-213](../protocol/FR-213-workflow-instance.md) supplies the related
  instance subject and records that its key is unstated.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](../protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  supplies the ordered source and target keys.

## Status

Proposed closure for specification issue #31. Relationship declarations,
endpoint types and direction are spec-declared and checked by S3
([AD-006](../../assurance/AD-006-model-graph-state-contracts.md)). Observation
currently uses unqualified endpoint identities, ignores relationship identity
when matching and treats exact replay as ambiguity; that implementation is
partial and incompatible. Runtime adoption remains planned under TC-141 and
the existing Observation/Protocol owner tickets.
