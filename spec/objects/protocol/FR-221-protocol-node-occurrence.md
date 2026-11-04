---
id: FR-221
title: "Protocol node occurrence"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-052
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-203
    type: references
  - target: ix://agent-ix/quire-specification/FR-210
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-221: Protocol node occurrence

## Description

When a source node is lowered into the causal control graph, the frontend SHALL retain for every lowered node, including synthesized control edges and bounded occurrences, its declaration span and its exact derivation and iteration identity ([FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) AC-11).

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| subject_kind | ProtocolSubjectVocabulary | 1..1 |  |
| static_declaration_path | String | 1..1 |  |
| workflow_instance_identity | String | 1..1 |  |
| iteration_identity | String | 1..1 |  |
| role_ownership | String | 1..1 |  |
| declaration_span | SourceRegion | 1..1 |  |
| bound_typed_record | String | 1..1 |  |

## Field sources

- **subject kind** ([FR-210](FR-210-protocol-subject-vocabulary.md))
  — which distinct typed subject this node observes. FR-049-AC-2 states that
  "Send, receive, delivery, operation attempt, effect and compensation
  references remain distinct".
- **static declaration path** — the control name and declaration path that
  identifies the static node. A relative reference and its fully qualified path
  resolve to the same static node
  ([choreography surface](../../../proposals/quire-v1/choreography-surface.md)).
- **workflow instance identity** — the assessed workflow instance the occurrence
  belongs to. FR-050 keeps workflow instances distinct from their static
  definitions, and FR-050-AC-3 keeps "Two concurrent workflow instances sharing
  one provider" distinct as assessment subjects.
- **iteration identity** — the ordinal of every enclosing bounded iteration.
  FR-049-AC-11 requires each lowered node to retain its "exact
  derivation/iteration identity".
- **role ownership** — the exact declaration-local role slot for a role-owned
  node, or an explicit structural-owner marker.
- **declaration span** ([FR-203](../foundation/FR-203-source-region.md)) —
  retained by FR-049-AC-11 and preserved across identity change by FR-052-AC-4.
- **bound typed record** — the typed record bound to the occurrence. The record
  payload alone cannot prove the correspondence to a workflow, node occurrence,
  role, operation or observation identity (choreography surface); FR-051 states
  that the binder "SHALL NOT infer a business effect from a send, receive,
  successful transport status, repeated delivery, trace identifier or equal
  payload alone".

## Identity and comparison

- **Identity kind** — *derived*. An occurrence is not written in source with an
  identity of its own; it is computed from the static node it instantiates and
  the enclosing instance and iteration structure.
- **Key** — (`qualified protocol-definition identity`, `workflow-instance key`,
  `declaration-local control handle`, `role slot or structural-owner marker`,
  `outer-to-inner bounded-repeat ordinal sequence`). Equality is componentwise
  over exactly these. Display
  name, source path, span, message text, locale and rendering never participate:
  FR-049-AC-11 retains the declaration span alongside the identity rather than
  inside it, and FR-049-AC-7 requires the exact declared relationship and
  endpoint direction "even when they share a provider, trace ID or equal scalar
  labels". A transport trace identifier substituted for an authored identity
  refuses before conformance evaluation (FR-051-AC-4).
- **Key order** — lexicographic over the five components in the declared
  sequence, including repeat ordinals from outermost to innermost. It exists
  only for stable comparison and carries no causal meaning.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md), held in
  [FR-220](FR-220-causal-control-graph.md). It is never derived from a key
  order, from a timestamp or from source position: FR-052-AC-2 states that
  "Reordering equal-time events does not add a causal edge absent from the
  protocol", and FR-113 states that "Arbitrary timestamp order SHALL NOT
  establish causality".
- **Absence** — every key component is required. An occurrence outside every
  bounded iteration has a present zero-length ordinal sequence. Omitting or
  adding a repeat dimension, or supplying an out-of-domain ordinal, refuses;
  missing input never becomes ordinal zero.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-221-AC-1 | Every lowered node, including a synthesized control edge and a bounded occurrence, retains its declaration span and its derivation and iteration identity. | Inspection |
| FR-221-AC-2 | Two occurrences of one static node in two workflow instances compare unequal. | Analysis |
| FR-221-AC-3 | An equal payload, an equal scalar label or a shared trace identifier does not make two occurrences equal. | Analysis |
| FR-221-AC-4 | An occurrence outside every repeat carries a present empty ordinal sequence; a missing sequence or missing, surplus or out-of-domain ordinal refuses without a default. | Analysis |
| FR-221-AC-5 | A concrete occurrence binds exactly one workflow key, admitted control handle, exact role slot or structural marker and one in-domain ordinal per outer-to-inner repeat dimension. | Analysis |

## Dependencies

- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) retains
  the typed node structure, its declaration spans and its iteration identity.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) supplies the
  causal order the occurrences carry.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) supplies the
  workflow-instance identity.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked-definition component.

## Status

Draft. Specification issue #28 promoted the implemented occurrence-schema
dimensions into FR-049 and fixed the key/order rule. Cross-repository runtime
binding evidence remains planned.
