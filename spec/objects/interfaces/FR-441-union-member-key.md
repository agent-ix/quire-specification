---
id: FR-441
title: "Union member key"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-141
    type: references
  - target: ix://agent-ix/quire-specification/FR-440
    type: references
---

# FR-441: Union member key

## Description

When a checked package declares a union, the producer, every reader and
every evaluator SHALL identify each member of that union by the pair
(`declaration_node_id`, `member`) below, compared componentwise, so that every
implementation distinguishes the same members of the same checked package.

## Properties

- **Member key** — the pair (`declaration_node_id`, `member`), where
  `declaration_node_id` is the `node_id` of the union's
  `composite_type`/`union` node ([FR-440](FR-440-checked-package-union-and-case-nodes.md))
  or, for a union declared by a selected domain package, of its
  `model`/`variant_type` node of family `union` ([FR-322](FR-322-checked-package-artifact.md)
  "Tuple value types and payload-bearing variant types"), and `member` is the
  member identifier, byte for byte as declared: a case name of the variant
  type, nullary or payload-bearing. A `model`/`variant_type` node carries no
  members, so the key names the variant type's declaration node and the case
  name, and the cases and their payloads are read from the selected document.
  The pair is not hashed: two member keys are equal exactly when their
  `declaration_node_id` values are equal and their `member` strings are equal
  byte for byte. An enum member is identified the same way under its enum
  declaration ([FR-141](../../functional/type-model/FR-141-evaluate-text-and-enumerations.md)).
- **Scope** — a checked package never carries a member key: no node of the
  semantic graph has it as its `node_id`, and a reader derives it from the
  union node it reads. It is the member identity
  (declaration node id, member identifier) of
  [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md).
  Two members are the same member exactly when their member keys are equal. A member key is scoped to one checked package, as the
  union declaration key it is computed from is (FR-143).
- **Use** — a supplied union value names its active member by its member
  key (FR-143, "Declarations and identity"). An evaluator selects a `case`
  arm by comparing the scrutinee value's member key with the arm member's
  key, so a member key of another union, or an enum member key, selects no
  arm.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-441-AC-1 | Withdrawn: the member-key hash vectors it tested are removed. | Withdrawn |
| FR-441-AC-2 | Withdrawn: the member-key hash vectors it tested are removed. | Withdrawn |
| FR-441-AC-3 | Withdrawn: the member-key hash vectors it tested are removed. | Withdrawn |
| FR-441-AC-4 | Withdrawn: the member-key hash vectors it tested are removed. | Withdrawn |

## Dependencies

- [FR-440](FR-440-checked-package-union-and-case-nodes.md) owns the union
  type node whose node id the member key names.
- [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md)
  owns union member identity and the admission of supplied union values.
- [FR-322](FR-322-checked-package-artifact.md) owns the `node_id` shape and
  the componentwise comparison of node ids.

## Status

Accepted complete-V1 boundary value contract for the union member key.
