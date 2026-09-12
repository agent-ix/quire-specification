---
id: FR-220
title: "Causal control graph"
type: FR
object: aggregate_root
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: contains
  - target: ix://agent-ix/quire-specification/FR-227
    type: contains
  - target: ix://agent-ix/quire-specification/FR-228
    type: contains
---
# FR-220: Causal control graph

## Description

When a protocol body is linked, the protocol linker SHALL construct one finite acyclic causal control graph for authored sequence, labeled choice, parallel branches, join, bounded iteration and termination without deriving causal edges from source order or timestamps ([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)).

## Members

The graph is the aggregate; its members have no standing outside it.

- **node occurrences** ([FR-221](FR-221-protocol-node-occurrence.md)) — the
  typed protocol nodes the graph orders. FR-052 takes "typed protocol nodes" as
  its input.
- **authored causal edges** — the edges written in the protocol body. Sequence
  contributes only its authored order: "Sequence SHALL add only its authored
  order" (FR-052). Parallel branches contribute each branch's own edges:
  "Parallel branches SHALL admit every interleaving that preserves each branch's
  causal edges" (FR-052).
- **branch edges and progress edges** — FR-052 Outputs names "An acyclic finite
  control representation with explicit branch and progress edges".
- **owned choice** ([FR-227](FR-227-owned-choice.md)) — the labeled choice node
  and its owning role.
- **control constructs** ([FR-228](FR-228-bounded-control-construct.md)) — the
  admitted vocabulary of sequence, parallel branch, join, bounded iteration and
  termination, with the rule each contributes.
- **termination declarations** — FR-052 takes "termination declarations" as an
  input and refuses "termination conditions that cannot be reached within the
  declared finite graph" (FR-052).

The graph is finite and acyclic. FR-052 refuses "missing joins, cyclic causal
dependencies, unbounded repetition and termination conditions that cannot be
reached within the declared finite graph", and FR-052-AC-3 refuses "A missing
join input, causal cycle, zero-progress loop or absent finite iteration
maximum".

## Identity and comparison

- **Identity kind** — *derived*. The graph carries no identity written beside
  it. Its identity moves with protocol identity: "Changing a branch, join rule
  or iteration maximum changes protocol identity and preserves the original
  source span" (FR-052-AC-4).
- **Key** — the identity of the linked protocol
  ([FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md)) together with
  the authored control structure that FR-052-AC-4 names as identity-bearing:
  its branches, its join rules and its iteration maxima. Equality is
  componentwise over that key and nothing else. Display name, source path, span,
  message text, locale and rendering never participate: FR-052-AC-4 states that
  changing the structure changes identity while the source span is preserved,
  so the span is carried and not compared. No requirement enumerates the
  complete component list; see **Open gap**.
- **Key order** — none is established. No requirement fixes a total order over
  graphs or over their edges, and this artifact supplies none. The member order
  used above is presentational and follows the order FR-052 states.
- **Causal order** — the authored partial acyclic order over node occurrences.
  It is authored, never derived: FR-052 constructs the graph "without deriving
  causal edges from source order or timestamps", FR-052-AC-2 states that
  "Reordering equal-time events does not add a causal edge absent from the
  protocol", and
  [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md)
  states that "Arbitrary timestamp order SHALL NOT establish causality". A key
  order over any object in this context is therefore never readable as, and is
  never convertible into, a causal edge.
- **Absence** — an absent required component is refused, not defaulted: a
  missing join input and an absent finite iteration maximum refuse admission
  (FR-052-AC-3). An absent optional component compares only to absence and is
  never compared to a default or an empty value.
- **Digest domain** — no member of this key is a digest. Where protocol identity
  is exchanged as a digest, that digest carries the domain it was derived under,
  `ix://agent-ix/quire-specification/FR-201`, and an identity crossing domains
  is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-220-AC-1 | Every edge in a constructed graph traces to an authored sequence, branch, join, progress or termination declaration in the protocol body. | Analysis |
| FR-220-AC-2 | Reordering two equal-time observations changes no edge of the graph. | Analysis |
| FR-220-AC-3 | No key order declared by this artifact or by any object it contains is cited as evidence of a causal edge. | Inspection |
| FR-220-AC-4 | A missing join input, a causal cycle, a zero-progress loop and an absent finite iteration maximum each refuse admission independently. | Analysis |

## Dependencies

- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) establishes the
  finite acyclic causal graph, its constructs and its refusals.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) retains
  the typed causal control under the exact selected source and model
  definitions.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) supplies the
  linked protocol the graph belongs to.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-210](FR-210-protocol-subject-vocabulary.md) supplies the
  distinct protocol subject kinds.

## Open gap

FR-052-AC-4 names three components whose change changes protocol identity —
a branch, a join rule and an iteration maximum. No requirement states that these
three are the complete key. Unresolved by the prose:

1. Whether a change to a node's typed payload, guard or channel binding, which
   FR-052-AC-4 does not name, also changes protocol identity.
2. Whether two protocols with identical authored structure at different source
   spans are one graph identity or two. FR-052-AC-4 preserves the span but does
   not say whether it participates.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
