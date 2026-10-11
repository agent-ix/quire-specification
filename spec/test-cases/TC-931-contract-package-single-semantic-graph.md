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
clause, a selected dependency, a local function, and a dependency-exported
function with package-independent signature types. Include a function-call
callee `dependency_reference` to the exported function and an unrelated
lowered sibling. Inspect the ContractPackage and compare its node collection,
identities, references, and source correspondence with FR-322 and FR-330.
Include an explicit recursion group and verify its keyed references remain
finite serialized values. Repeat after changing construction order, then
mutate a protocol reference to an absent key and to the key of an existing
non-protocol node. In a separate admission attempt, supply a dependency package
whose recomputed `package_id` differs from the selected row while keeping the
reference and lock unchanged.

## Expected Results

The package contains one `semantic_graph.nodes` collection in FR-322 typed-key
order, with each semantic node represented once; its intra-package semantic
references resolve locally, the dependency function call resolves through the
selected admitted package, and recursion-group cycles remain keyed references
in a finite value. The dependency-package mutation refuses
`stale_dependency`/`content-mismatch` naming the selection identity and both
package ids during package admission, before lowering begins, so no
ContractPackage is emitted for that attempt. In the valid admitted mixed-request
case, the unrelated lowered sibling retains its disposition. The package has no separate
`types`, `expressions`, or `clauses` graph collections. The protocol reference
is the stable node key of the protocol-tagged node in that collection, and the
package identity and source correspondence remain equivalent under canonical
ordering. The absent and wrong-tag targets are refused before any consumer
artifact is emitted.
