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
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
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
- **complete typed semantic content** — every control and edge; every guard and
  constraint value graph and dependency; every event kind and binding; every
  channel, ordering and FIFO rule; every await anchor, interval and branch;
  every compensation reference; and every finish binder, constraint and closure
  requirement retained by the linked definition.

The graph is finite and acyclic. FR-052 refuses "missing joins, cyclic causal
dependencies, unbounded repetition and termination conditions that cannot be
reached within the declared finite graph", and FR-052-AC-3 refuses "A missing
join input, causal cycle, zero-progress loop or absent finite iteration
maximum".

## Identity and comparison

- **Identity kind** — *derived*. The graph's identity is exactly the
  [FR-288](FR-288-qualified-protocol-definition-identity.md) of its linked
  protocol definition; it has no second flattened graph key.
- **Key** — the complete FR-288 value and nothing else. The complete typed
  semantic content above is consistency content: presenting different content
  under the same FR-288 key is an identity contradiction and refuses. Every
  semantic-content mutation requires a different FR-288. External filesystem
  path, locus, build provenance and rendering metadata do not independently
  extend the key. Authored source text, including a display-name edit, still
  changes FR-288 through its required raw-source digest; this artifact does not
  erase or override that already-selected identity component.
- **Key order** — FR-288's lexicographic key order, used only for canonical
  collection comparison. It establishes no graph-member order, precedence or
  causal edge. The member order above is presentational.
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
- **Digest domain** — inherited exactly from FR-288, including the raw-source
  and admitted semantic-selection digests in their declared FR-201 domains.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-220-AC-1 | Every edge in a constructed graph traces to an authored sequence, branch, join, progress or termination declaration in the protocol body. | Analysis |
| FR-220-AC-2 | Reordering two equal-time observations changes no edge of the graph. | Analysis |
| FR-220-AC-3 | No key order declared by this artifact or by any object it contains is cited as evidence of a causal edge. | Inspection |
| FR-220-AC-4 | A missing join input, a causal cycle and a zero-progress loop each refuse admission independently. Where the bounded control profile is selected, an absent finite iteration maximum also refuses admission independently. | Analysis |
| FR-220-AC-8 | Where a complete-V1 choreography facet is selected, the graph records a loop's invariant and well-founded variant as its termination premise in place of a finite iteration maximum, as FR-171 specifies. | Analysis |
| FR-220-AC-5 | The graph key equals its complete FR-288 qualified protocol-definition identity; there is no independent flattened or position-only graph fingerprint. | Analysis |
| FR-220-AC-6 | Every typed semantic-content mutation requires another FR-288, and different complete graph content offered under one FR-288 refuses as an identity contradiction. | Test (TC-052) |
| FR-220-AC-7 | External path, locus, build and rendering changes do not independently extend the graph key, while a changed authored source byte changes the raw-source digest already inside FR-288. | Analysis |

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
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the exact
  graph key and selected source/semantic digest domains.

## Status

Proposed closure for specification issue #31. Spec Language publishes the
complete typed graph content required for consistency checking. Protocol's
current reduced `ControlShape` and position/numeric fingerprints omit semantic
content and are partial rather than qualification evidence; executable exact
identity evidence remains planned under TC-052.
