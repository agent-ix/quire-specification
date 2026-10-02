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
every evaluator SHALL key each member of that union by the
`quire.union-member-node/v1` preimage below, computed from the union
declaration's node key and the member identifier, so that every
implementation derives the same member key bytes from the same checked
package.

## Properties

- **Preimage** — the closed JSON object
  `{version: "quire.union-member-node/v1", declaration_node_id, member}`,
  where `declaration_node_id` is the `NodeId` of the union's
  `composite_type`/`union` node ([FR-440](FR-440-checked-package-union-and-case-nodes.md))
  and `member` is the member identifier, byte for byte as declared.
- **Member key** — `{domain: "quire.checked-semantic-node/v1", digest}`,
  where `digest` is the unprefixed 64-lowercase-hex SHA-256 of the UTF-8
  RFC 8785 canonical bytes of the preimage. This is the same canonical
  JCS/SHA-256 rule that keys an enum member under
  `quire.enum-member-node/v1` ([FR-141](../../functional/type-model/FR-141-evaluate-text-and-enumerations.md)).
- **Scope** — a checked package never carries a member key: no node of the
  semantic graph has it as its `node_id`, and a reader derives it from the
  union node it reads. It is the member identity
  (declaration key, member identifier) of
  [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md)
  in byte form. Two members are the same member exactly when their member
  keys are equal. A member key is scoped to one checked package, as the
  union declaration key it is computed from is (FR-143).
- **Use** — a supplied union value names its active member by its member
  key (FR-143, "Declarations and identity"). An evaluator selects a `case`
  arm by comparing the scrutinee value's member key with the arm member's
  key, so a member key of another union, or an enum member key, selects no
  arm.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-441-AC-1 | For a union node whose `node_id.digest` is 64 `b` characters, the member `Circle` has the preimage bytes `{"declaration_node_id":{"digest":"bbbb…bbbb","domain":"quire.checked-semantic-node/v1"},"member":"Circle","version":"quire.union-member-node/v1"}` and the member key digest `40a8492e5616a286cae9977ee9a7d5acab89cc50eb75654df0b8ad1096efd692`, and the member `Empty` the digest `83c93448259809db143345be4182a358f92481032ad0ae3b782a482d3897296b`. | Test (TC-396) |
| FR-441-AC-2 | The member `Circle` of a second union whose `node_id.digest` is 64 `c` characters has the digest `611ebd3f928ce6ca8713c9400e4725af94e51d96f0aaa5ef437ebed8c88ae75c`, different from the first union's `Circle`, and an enum member `Circle` keyed by `quire.enum-member-node/v1` has a third digest. | Test (TC-396) |
| FR-441-AC-3 | Changing the `member` of the `U1`/`Circle` preimage to `circle`, or its `declaration_node_id` to the `U2` node id, changes the member key digest, so a member key binds the member identifier byte for byte and its union. | Test (TC-396) |

## Dependencies

- [FR-440](FR-440-checked-package-union-and-case-nodes.md) owns the union
  type node whose key the preimage names.
- [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md)
  owns union member identity and the admission of supplied union values.
- [FR-322](FR-322-checked-package-artifact.md) owns the `NodeId` shape and
  the canonical JCS/SHA-256 rule of node identity preimages.

## Status

Accepted complete-V1 boundary value contract for the union member key.
Implementation and qualification remain tracked separately.
