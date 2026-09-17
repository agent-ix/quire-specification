---
id: FR-326
title: "Checked protocol artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_008
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
---

# FR-326: Checked protocol artifact

## Description

When a protocol declaration completes static checking, the producer SHALL use the immutable CheckedProtocolArtifact contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.checked-protocol/v1`.
- `definition` — package-qualified protocol declaration identity.
- `roles` — definitions and runtime-binding requirements.
- `messages_channels` — typed messages, channel/queue policies and cardinality premises.
- `control_graph` — typed nodes and authored acyclic causal edges.
- `guards_obligations` — source-bound expressions, captures, anchors and origin keys.
- `compensations` — registration, retry, commit and recovery definitions.
- `bounds` — participant, queue, loop, attempt, population and analysis bounds.

## Identity and validation

The key is (`contract_version`, `definition`, canonical checked content). Occurrence identities and runtime observations are excluded.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-326-AC-1 | Every reference resolves to one package-qualified declaration and correct subject kind. | Analysis |
| FR-326-AC-2 | Dynamic topology carries an explicit finite population and bound. | Analysis |
| FR-326-AC-3 | Static checking executes no business action and fabricates no occurrence. | Analysis |

## Dependencies

- [interface_008](./interface_008-checked-protocol.md) owns the operation boundary.
- FR-171 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
