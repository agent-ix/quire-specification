---
id: FR-228
title: "Bounded control construct vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-227
    type: references
---
# FR-228: Bounded control construct vocabulary

## Description

When a protocol body is linked, the protocol linker SHALL admit only the authored control constructs named by FR-052 — sequence, labeled choice, parallel branches, join, bounded iteration and termination — each contributing the causal rule stated for it and no other ([FR-052](../../functional/FR-052-represent-bounded-control.md)).

## Values

The members are the constructs FR-052 names in its Description. The citation
records the sentence that states each construct's rule.

| Value | Description |
| --- | --- |
| `sequence` | Authored order between children. "Sequence SHALL add only its authored order" (FR-052). It contributes no edge beyond the authored one, and no edge is derived from source order or timestamps (FR-052). |
| `parallel-branch` | An independently ordered branch. "Parallel branches SHALL admit every interleaving that preserves each branch's causal edges" (FR-052). FR-052-AC-1: "Split shipments S1 and S2 may interleave while each branch preserves its own order and both satisfy the authored join before completion." |
| `join` | The awaited completion of named branches. "A join SHALL name the branches and completion rule it awaits" (FR-052). A missing join input refuses admission (FR-052-AC-3), and FR-049-AC-3 refuses "missing/duplicate join labels". FR-049-AC-3 names the admitted rule "all-branch join". |
| `choice` | A labeled decision with one owning role, typed by [FR-227](FR-227-owned-choice.md). FR-052 names "labeled choice" among the constructs; FR-053 supplies ownership, labeling, guard disjointness and visibility. |
| `bounded-iteration` | Repetition with an authored finite maximum. "Iteration SHALL carry an authored finite maximum and progress edge" (FR-052). A zero-progress loop, an absent finite maximum and unbounded repetition each refuse (FR-052, FR-052-AC-3, FR-049-AC-3). The maximum is identity-bearing (FR-052-AC-4). |
| `termination` | An authored termination declaration. FR-052 takes "termination declarations" as an input and refuses "termination conditions that cannot be reached within the declared finite graph" (FR-052). |

## Identity and comparison

- **Identity kind** — *value*. A construct kind has no identity of its own and
  is compared componentwise as an exact kind. The identity of an occurrence of a
  construct belongs to [FR-221](FR-221-protocol-node-occurrence.md), and the
  graph it is placed in is [FR-220](FR-220-causal-control-graph.md).
- **Key** — the construct kind itself. Two constructs of different kinds are
  unequal, and no construct substitutes for another: FR-052-AC-3 refuses a
  missing join input rather than treating a branch end as a join. Display name,
  source path, span, message text, locale and rendering never participate.
- **Key order** — none is established. No requirement fixes a spelling, an
  ordinal or a precedence over these constructs, so this artifact declares no
  key order and supplies none. The table order follows the order FR-052 states
  and is presentational.
- **Causal order** — each construct contributes the authored edges quoted in its
  row and nothing further. FR-052 constructs the graph "without deriving causal
  edges from source order or timestamps", and FR-052-AC-2 states that
  "Reordering equal-time events does not add a causal edge absent from the
  protocol". No listing order, table order or member order in this artifact is
  readable as a causal edge.
- **Absence** — an absent required component of a construct refuses admission
  rather than receiving a default: "A missing join input, causal cycle,
  zero-progress loop or absent finite iteration maximum refuses admission"
  (FR-052-AC-3). An absent optional component compares only to absence.
- **Digest domain** — no member of this key is a digest. Where a construct is
  carried beside a digest in provenance, that digest carries the domain it was
  derived under, `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-228-AC-1 | Every member of the table is named as a control construct by FR-052 and carries the rule cited in its description. | Inspection |
| FR-228-AC-2 | A missing join input, a causal cycle, a zero-progress loop and an absent finite iteration maximum each refuse admission independently. | Analysis |
| FR-228-AC-3 | No member of this table is assigned an ordinal, a precedence or a wire spelling by this artifact. | Inspection |
| FR-228-AC-4 | Changing a branch, a join rule or an iteration maximum changes protocol identity while the original source span is preserved. | Analysis |

## Dependencies

- [FR-052](../../functional/FR-052-represent-bounded-control.md) names the
  constructs and states each rule and refusal.
- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) names the
  admitted all-branch join and the bounded repeat, and retains every lowered
  node's iteration identity.
- [FR-053](../../functional/FR-053-enforce-choice-visibility.md) supplies the
  choice member's ownership and visibility rules.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

1. **Join policy.** FR-052 states that "A join SHALL name the branches and
   completion rule it awaits", which admits more than one completion rule, while
   FR-049-AC-3 names only "all-branch join" and the
   [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
   states that "All-branch join is the join rule spelled by this candidate.
   Another join policy requires its own admitted rule, including what happens to
   outstanding branches". No requirement states which completion rules are
   admitted, so this artifact enumerates no completion-rule vocabulary.
2. **Termination.** Two requirements use a completion noun for different things
   and neither cross-references the other. FR-052 refuses "termination
   conditions that cannot be reached within the declared finite graph", a
   control-graph property. FR-049-AC-12 states that "A finish record without
   required closure, or with unsettled required obligations, cannot establish
   complete global success", an observation record. No requirement states
   whether a termination declaration and a finish record are the same subject,
   so no key for termination is recorded here beyond its node occurrence.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
