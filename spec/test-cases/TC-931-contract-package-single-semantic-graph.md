---
id: TC-931
title: "ContractPackage has one identity-keyed semantic graph"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-4
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
finite serialized values. Repeat after changing construction order. Inspect the
protocol node and confirm its body uses the existing catalogued
`quire.op.protocol.control` application and selected `protocol_profile` law.
The current FR-322 contract defines no protocol-reference carrier or slot, so
this case does not synthesize a protocol target edge or mutate one. In a
separate admission attempt, supply a dependency package
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
target-validation relation is not asserted: the package has no separate
`protocol_references` collection, and TC-931 verifies the existing
protocol-tagged node and its catalogued body. The package identity and source
correspondence remain equivalent under canonical ordering. A future FR-322
contract must define a carrier and slot before a protocol-target mutation case
can be specified.
