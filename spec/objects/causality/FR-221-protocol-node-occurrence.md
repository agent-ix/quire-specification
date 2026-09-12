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
---
# FR-221: Protocol node occurrence

## Description

When a source node is lowered into the causal control graph, the frontend SHALL retain for every lowered node, including synthesized control edges and bounded occurrences, its declaration span and its exact derivation and iteration identity ([FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) AC-11).

## Properties

- **subject kind** ([FR-210](../protocol/FR-210-protocol-subject-vocabulary.md))
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
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`). Equality is componentwise over exactly these. Display
  name, source path, span, message text, locale and rendering never participate:
  FR-049-AC-11 retains the declaration span alongside the identity rather than
  inside it, and FR-049-AC-7 requires the exact declared relationship and
  endpoint direction "even when they share a provider, trace ID or equal scalar
  labels". A transport trace identifier substituted for an authored identity
  refuses before conformance evaluation (FR-051-AC-4). No requirement sentence
  enumerates these three components as the key; see **Open gap**.
- **Key order** — none is established. No requirement fixes a total order over
  declaration paths, workflow identities or iteration ordinals. A bounded
  iteration ordinal is a component of identity under FR-049-AC-11 and is not a
  declared key order, and this artifact supplies no key order over occurrences.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/FR-052-represent-bounded-control.md), held in
  [FR-220](FR-220-causal-control-graph.md). It is never derived from a key
  order, from a timestamp or from source position: FR-052-AC-2 states that
  "Reordering equal-time events does not add a causal edge absent from the
  protocol", and FR-113 states that "Arbitrary timestamp order SHALL NOT
  establish causality".
- **Absence** — an absent key component compares only to absence. An occurrence
  outside every bounded iteration has an absent iteration identity, and that
  absence is never compared to ordinal zero, to an empty ordinal list or to any
  other supplied value.
- **Digest domain** — no component of this key is a digest. Where an occurrence
  is exchanged beside a digest in provenance, that digest carries the domain it
  was derived under, `ix://agent-ix/quire-specification/FR-201`, and an identity
  crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-221-AC-1 | Every lowered node, including a synthesized control edge and a bounded occurrence, retains its declaration span and its derivation and iteration identity. | Inspection |
| FR-221-AC-2 | Two occurrences of one static node in two workflow instances compare unequal. | Analysis |
| FR-221-AC-3 | An equal payload, an equal scalar label or a shared trace identifier does not make two occurrences equal. | Analysis |
| FR-221-AC-4 | An occurrence with an absent iteration identity compares only to absence and receives no default ordinal. | Analysis |

## Dependencies

- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) retains
  the typed node structure, its declaration spans and its iteration identity.
- [FR-052](../../functional/FR-052-represent-bounded-control.md) supplies the
  causal order the occurrences carry.
- [FR-050](../../functional/FR-050-bind-protocol-instances.md) supplies the
  workflow-instance identity.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

The three key components above are stated together only in the
[choreography surface](../../../proposals/quire-v1/choreography-surface.md),
which FR-049 takes as a dependency: "Control names and declaration paths
identify static nodes. The canonical runtime occurrence additionally includes
the workflow identity and every enclosing bounded-iteration ordinal." No
requirement sentence carries that component list. FR-049-AC-11 requires the
iteration identity to be retained but does not say what the identity is composed
of. Closing this requires a ruling that promotes the surface sentence into a
requirement, or an authored key.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
