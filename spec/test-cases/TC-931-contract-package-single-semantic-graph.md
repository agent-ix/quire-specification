---
id: TC-931
title: "ContractPackage has one identity-keyed semantic graph"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---

# TC-931: ContractPackage has one identity-keyed semantic graph

## Description

Check that the ContractPackage representation uses the I04 node-tag, key and
reference rules and does not serialize separate graph collections or a second
protocol-reference form.

## Test Procedure

Lower an admitted package containing a type, a typed expression, a protocol
clause, and a selected dependency. Inspect the ContractPackage and compare its
node collection, identities, references, and source correspondence with FR-322
and FR-330. Include an explicit recursion group and verify its keyed references
remain finite serialized values. Repeat after changing construction order,
then mutate a protocol reference to an absent key and to the key of an existing
non-protocol node; also mutate a dependency selection and verify its FR-322
refusal.

## Expected Results

The package contains one `semantic_graph.nodes` collection in FR-322 typed-key
order, with each semantic node represented once; its intra-package semantic
references resolve locally, dependency references resolve through the selected
dependency package, and recursion-group cycles remain keyed references in a
finite value. It has no separate
`types`, `expressions`, or `clauses` graph collections. The protocol reference
is the stable node key of the protocol-tagged node in that collection, and the
package identity and source correspondence remain equivalent under canonical
ordering. The absent and wrong-tag targets are refused before any consumer
artifact is emitted.
