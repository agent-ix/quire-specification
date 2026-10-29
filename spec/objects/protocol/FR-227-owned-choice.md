---
id: FR-227
title: "Owned choice"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-053
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-052
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
---
# FR-227: Owned choice

## Description

When a labeled protocol choice is admitted, the protocol linker SHALL require one owning role and evidence that the owner can distinguish every selected branch from facts available at its decision point ([FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md)).

## Properties

- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — a choice is an
  authored control node in the causal control graph
  ([FR-220](FR-220-causal-control-graph.md)).
- **owning role** — exactly one. FR-053 states that the linker "SHALL reject
  ownerless, multiply owned, unlabeled and overlapping choices", and
  FR-053-AC-3 refuses "Ownerless, multiply owned, duplicate-label and
  guard-overlap mutants" independently.
- **labeled branches** — each branch carries its label and its guard. Duplicate
  labels refuse (FR-053, FR-049-AC-3).
- **visibility premises** — the declared role-visible facts and messages
  admitted with the choice. FR-053 Outputs names "An admitted owned choice with
  explicit visibility premises", and FR-053 states that the linker "SHALL reject
  a choice when its distinguishing fact is available only to another role and no
  causal message exposes it to the owner".
- **distinguishing fact** — the fact the owner uses to select a branch, together
  with the causal message that exposes it to the owner (FR-053, FR-053-AC-1).

A refusal is local: "Rejecting an unobservable choice SHALL NOT imply that every
global trace is invalid or that a general projection theorem has been attempted"
(FR-053), and the refusal "identifies the exact choice, owner and unavailable
distinguishing fact without reporting global conformance success"
(FR-053-AC-4).

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is.
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`), as held by FR-221. Equality is componentwise over that
  key and nothing else. Display name, source path, span, message text, locale
  and rendering never participate. The branch labels are authored components of
  the choice and are identity-bearing for the protocol: "Changing a branch, join
  rule or iteration maximum changes protocol identity and preserves the original
  source span" (FR-052-AC-4).
- **Key order** — none is established. No requirement fixes an order over
  branch labels or over guards; FR-053 requires labels to be distinct and
  guards not to overlap, which are uniqueness and disjointness rules, not an
  order. This artifact supplies no key order.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220. A branch edge is authored, never derived from key order, timestamp or
  source position (FR-052, FR-052-AC-2, FR-113). A label order, were one
  authored, would carry no causal meaning and would not select a branch.
- **Absence** — an absent owner is refused rather than defaulted (FR-053); an
  absent distinguishing fact is refused rather than treated as false
  (FR-053-AC-2). An absent optional component compares only to absence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-227-AC-1 | An ownerless, a multiply owned, a duplicate-label and a guard-overlap choice each refuse independently. | Analysis |
| FR-227-AC-2 | A choice whose distinguishing fact is visible only to another role, with no causal message exposing it to the owner, refuses. | Analysis |
| FR-227-AC-3 | A choice refusal names the exact choice, owner and unavailable distinguishing fact and reports no global conformance success. | Inspection |
| FR-227-AC-4 | No order over branch labels is cited as selecting a branch or as establishing a causal edge. | Inspection |

## Dependencies

- [FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md) establishes
  ownership, labeling, guard disjointness and the visibility obligation.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) places the
  choice in the finite acyclic causal graph and makes its branches
  identity-bearing.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Visibility evidence contract

Visibility is discharged by a checked evidence record linking the owner role,
choice occurrence, distinguishing typed fact, producing declaration/message and
an authored causal path ending before the decision point. A locally owned state
fact may omit a message only when the model contract grants that role read
authority at the decision anchor. Equal names, payloads, timestamps or later
messages do not discharge visibility.

## Status

Accepted complete-V1 owned-choice contract.
