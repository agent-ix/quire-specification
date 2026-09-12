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
---
# FR-262: Declared instance relationship

## Description

When an observation is associated with a workflow subject, the correlation service SHALL select only an explicit typed relationship from the selected Producer interface 1.2.0 identity, version and digest.

FR-111 states that "unknown, conflicting, and ambiguous relationships are typed
non-success correlation outcomes rather than heuristic matches", and that the
service refuses "a missing producer identity, unknown producer version, or
producer digest mismatch rather than reconstructing a relationship from a
foreign key, trace ID, provider name, or ambient configuration".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Relationship type | FR-111 Description | "an explicit typed relationship". |
| Producer interface identity | FR-111 Description | Selected Producer interface 1.2.0 identity. |
| Producer version | FR-111 Description | An unknown version refuses (FR-111-AC-4). |
| Producer digest | FR-111 Description | A digest mismatch refuses (FR-111-AC-4); its domain is selected under [FR-201](../foundation/FR-201-canonical-identity-domain.md). |
| Related instance endpoints | FR-111 Behavior | "A refund is admitted only with its compensated effect and order relationship." |

FR-111 Behavior states that "a telemetry trace ID or shared provider identity
does not establish either relationship", and FR-111-AC-1 that "A refund linked
to O1 is refused for O2 even when both orders share a provider or trace
infrastructure."

## Identity and comparison

- **Identity kind** — *value*. A relationship has no identity of its own; it is
  compared componentwise. It is not *derived*: FR-111 forbids reconstructing it
  "from a foreign key, trace ID, provider name, or ambient configuration".
- **Key** — the relationship type together with the selected Producer interface
  identity, version and digest, and the related instance endpoints it declares.
  FR-111-AC-4 establishes the producer identity, version and digest as
  key-participating: a missing, unknown or mismatched one refuses rather than
  resolves. **The prose does not state how the endpoints themselves are keyed**,
  because it does not state a workflow instance's key
  ([FR-213](../protocol/FR-213-workflow-instance.md)). Equality is componentwise
  over the key and nothing else; display name, source path, span, message text,
  locale and rendering never participate, and a trace ID, provider name, foreign
  key or ambient configuration is not a key component (FR-111 Behavior).
- **Key order** — none. Because the endpoint key components are not fixed, no
  total order over the key is stated and none is supplied here. Any order later
  established would serve collection comparison only and would carry no causal
  meaning: FR-113 states that "Arbitrary timestamp order SHALL NOT establish
  causality."
- **Causal order** — the declared relationship structure itself, and nothing
  else. It is never derived from key order, from a timestamp, from a telemetry
  trace ID or from a shared provider identity (FR-111 Behavior, FR-111-AC-1).
  A repeated transport fact adds no causal node: "A duplicate receipt SHALL NOT
  establish a second business effect" (FR-111 Behavior), and repeated delivery
  "cannot alone create a second attempt or effect" (FR-111-AC-3).
- **Absence** — an absent key component compares only to absence. A missing,
  conflicting or ambiguous required relationship is a distinct non-success
  correlation outcome (FR-111-AC-2) and never a default, an empty relationship
  or an implicit option-none.
- **Digest domain** — the producer digest carries a selected domain under
  `ix://agent-ix/quire-specification/FR-201`. An identity crossing domains is
  unequal regardless of byte equality.

## Subjects that remain distinct

FR-111 Behavior states: "Workflow instances, deliveries, attempts, effects, and
receipts remain distinct." This artifact relates such subjects; it does not
define their vocabulary, which is held elsewhere
([FR-210](../protocol/FR-210-protocol-subject-vocabulary.md)). See **Open gap**.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-262-AC-1 | A relationship declared between O1 and its refund does not resolve for O2 sharing a provider or trace infrastructure. | Analysis |
| FR-262-AC-2 | A missing, unknown-version or digest-mismatched producer relationship identity does not resolve and is not reconstructed from a foreign key, trace ID, provider name or ambient configuration. | Analysis |
| FR-262-AC-3 | Missing, conflicting and ambiguous required relationships compare unequal to one another and to a resolved relationship. | Analysis |
| FR-262-AC-4 | A second occurrence of a transport fact carrying the same declared relationship does not add a causal edge. | Analysis |

## Dependencies

- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  establishes the explicit typed relationship and its refusal cases.
- [FR-213](../protocol/FR-213-workflow-instance.md) supplies the related
  instance subject and records that its key is unstated.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

1. **The non-success outcome vocabulary is named twice and not identically.**
   FR-111 Description: "unknown, conflicting, and ambiguous relationships are
   typed non-success correlation outcomes rather than heuristic matches."
   FR-111-AC-2: "Missing, conflicting, and ambiguous required relationships
   yield distinct non-success correlation outcomes." Whether *unknown* and
   *missing* name one outcome or two is not stated. This artifact enumerates
   neither set as a closed vocabulary.
2. **The communication subject set is named three times and not identically.**
   FR-111 Behavior: "Workflow instances, deliveries, attempts, effects, and
   receipts remain distinct."
   FR-051 Description: the binder preserves "message, send, receive, delivery,
   attempt and effect as distinct typed subjects."
   FR-049-AC-2: "Send, receive, delivery, operation attempt, effect and
   compensation references remain distinct; a duplicate receipt cannot mint
   another effect node."
   Whether FR-111's "receipt" is FR-051's "receive", FR-051's "message", or a
   fourth subject is never reconciled. This artifact records all three lists and
   merges none.
3. No requirement states the identity domain of the Producer interface 1.2.0
   digest.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
