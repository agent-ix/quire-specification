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

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| definition | String | 1..1 |  |
| roles | String | 1..1 |  |
| messages_channels | String | 1..1 |  |
| control_graph | String | 1..1 |  |
| guards_obligations | String | 1..1 |  |
| compensations | String | 1..1 |  |
| bounds | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| definition | type not stated in source; multiplicity not stated in source; see Field sources |
| roles | type not stated in source; multiplicity not stated in source; see Field sources |
| messages_channels | type not stated in source; multiplicity not stated in source; see Field sources |
| control_graph | type not stated in source; see Field sources |
| guards_obligations | type not stated in source; multiplicity not stated in source; see Field sources |
| compensations | type not stated in source; multiplicity not stated in source; see Field sources |
| bounds | type not stated in source; see Field sources |


- `definition` — package-qualified protocol declaration identity.
- `roles` — definitions and runtime-binding requirements.
- `messages_channels` — typed messages, channel/queue policies and cardinality premises.
- `control_graph` — typed control nodes; the acyclic causal edges are derived from them, not carried.
- `guards_obligations` — source-bound expressions, captures, anchors and origin keys.
- `compensations` — registration, retry, commit and recovery definitions.
- `bounds` — participant, queue, loop, attempt, population and analysis bounds: a replicated role's `max`, a queue's capacity, a `repeat`'s `maximum`, a template's `attempts.maximum` and the symbolic bounds a queue names. Analysis bounds are request members, not package content.

## Carriage

The value is carried by the `protocol` nodes of a `quire.checked-semantic-graph/v2`
checked package ([FR-464](FR-464-checked-package-protocol-node-bodies.md),
[FR-465](FR-465-checked-package-protocol-declaration-bodies.md),
[FR-466](FR-466-checked-package-protocol-control-bodies.md)) and by nothing
else: no separate `quire.checked-protocol/v1` document is emitted or read. The
`definition` is the `protocol_clause` node; `roles`, `messages_channels`,
`control_graph`, `guards_obligations` and `compensations` are its role,
channel, queue, control, obligation and compensation nodes, and `bounds` the
bound members of those nodes. Occurrence
identities and runtime observations are excluded from node keys.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-326-AC-1 | Every reference resolves to one package-qualified declaration and correct subject kind. | Analysis |
| FR-326-AC-2 | Dynamic topology carries an explicit finite population and bound. | Analysis |
| FR-326-AC-3 | Static checking executes no business action and fabricates no occurrence. | Analysis |

## Dependencies

- [interface_008](./interface_008-checked-protocol.md) owns the operation boundary.
- FR-171 owns the behavior that produces or consumes this value.
