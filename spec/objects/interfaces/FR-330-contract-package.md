---
id: FR-330
title: "Target-neutral contract package"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/interface_012
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-016
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-035
    type: references
---

# FR-330: Target-neutral contract package

## Description

When checked native meaning is lowered for backend consumption, the producer SHALL use the immutable ContractPackage contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.contract-package/v1`.
- `package` — originating checked-package identity and lock.
- `semantic_graph` — one closed, cycle-free, key-ordered tagged-node collection
  containing type, expression, model, relation, clause and analysis meaning.
- `source_map` — total node/item to native semantic/source correspondence.
- `feature_accounting` — one exact-represented or refusal disposition per requested item.
- `identity` — canonical semantic digest excluding presentation.

## Identity and validation

The consumer need is US-011: runtime, testing, simulation, proof and analysis
methods consume the same checked package so their conclusions and
counterexamples remain comparable and replayable. Therefore I12 does not
publish independent `TypeGraph`, `ExpressionGraph`, `ClauseGraph` or
`ProtocolReference` associated types. It uses one key-ordered semantic node
collection, with existing node tags carrying the semantic category and node
keys carrying reference identity. No consumer-specific graph partition or
second protocol-reference representation is introduced.

The prior `types`, `expressions` and `clauses` labels are not separate wire
properties; those meanings occur as tagged nodes in `semantic_graph.nodes`.
The graph uses the closed node-tag vocabulary, key order and reference rules
defined by [FR-322](FR-322-checked-package-artifact.md), while retaining this
package's own `contract_version` and semantic identity domain.
Every semantic-node reference resolves by stable node key to a compatible node
in this package. Dependency references retain FR-322's selected-package
identity and admitted-dependency resolution rules; they are not flattened into
local nodes. A protocol reference is the node key of a `protocol`-tagged node;
it is not a display name, array position, or separately encoded reference
type. Explicit recursion groups permitted by FR-322 remain graph semantics;
the package is cycle-free as a serialized value, with references represented
by keys rather than recursive object embedding. The package identity covers
every semantic graph node, profile and source correspondence. It never becomes
native source or semantic authority.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-330-AC-1 | Every represented node has identical native meaning under its selected profile. | Analysis |
| FR-330-AC-2 | An unrepresentable item has no substitute node and retains a located refusal. | Analysis |
| FR-330-AC-3 | Package ordering and identity are deterministic across equivalent construction order. | Analysis |
| FR-330-AC-4 | A package has exactly one semantic node collection ordered by the FR-322 typed node key; each node occurs once, intra-package semantic references resolve to compatible nodes, dependency references resolve under FR-322's selected-package rules, and every protocol reference targets a `protocol`-tagged node. Explicit FR-322 recursion groups remain permitted; no recursive object embedding or separate type, expression, clause, or protocol-reference graph is serialized. | Test |

## Dependencies

- [interface_012](./interface_012-contract-package.md) owns the operation boundary.
- [FR-322](FR-322-checked-package-artifact.md) owns the closed tagged-node form,
  node identities, and source occurrences reused by this package.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  owns the lowering behavior that produces this value; Contract IR FR-035
  consumes it as one flat key-ordered node set with stable-identity references.
