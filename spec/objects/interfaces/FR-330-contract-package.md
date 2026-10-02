---
id: FR-330
title: "Target-neutral contract package"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_012
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-016
    type: references
---

# FR-330: Target-neutral contract package

## Description

When checked native meaning is lowered for backend consumption, the producer SHALL use the immutable ContractPackage contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.contract-package/v1`.
- `package` — originating checked-package identity and lock.
- `types` — cycle-free complete type descriptors.
- `expressions` — source-correspondent typed operation graph.
- `clauses` — state, temporal, protocol and analysis obligations.
- `source_map` — total node/item to native semantic/source correspondence.
- `feature_accounting` — one exact-represented or refusal disposition per requested item.
- `identity` — canonical semantic digest excluding presentation.

## Identity and validation

The package identity covers all types, expressions, clauses, profiles and correspondence. It never becomes native source or semantic authority.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-330-AC-1 | Every represented node has identical native meaning under its selected profile. | Analysis |
| FR-330-AC-2 | An unrepresentable item has no substitute node and retains a located refusal. | Analysis |
| FR-330-AC-3 | Package ordering and identity are deterministic across equivalent construction order. | Analysis |

## Dependencies

- [interface_012](./interface_012-contract-package.md) owns the operation boundary.
- FR-195 owns the behavior that produces or consumes this value.
