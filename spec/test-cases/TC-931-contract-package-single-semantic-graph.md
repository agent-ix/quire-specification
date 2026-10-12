---
id: TC-931
title: "ContractPackage has one identity-keyed semantic graph"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-4
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-22
    type: references
  - target: ix://agent-ix/quire-specification/FR-322-AC-66
    type: verifies
---

# TC-931: ContractPackage has one identity-keyed semantic graph

## Description

Check that the ContractPackage representation uses the I04 node-tag, key and
reference rules and does not serialize separate graph collections or a separate
`protocol_references` collection.

## Test Procedure

Lower an admitted package containing a type, a typed expression, a protocol
clause, a selected dependency, a local function, and a dependency-exported
function with package-independent signature types. Include a function-call
callee `dependency_reference` to the exported function and an unrelated
lowered sibling. Inspect the ContractPackage and compare its node collection,
identities, references, and source correspondence with FR-322 and FR-330.
Include an explicit recursion group and verify its keyed references remain
finite serialized values. Repeat after changing construction order. In the
fixture, use the existing `protocol_clause` control application from
`proposals/checked-package-v2/fixtures/positive-all-families.json`; verify its
`quire.op.protocol.control` application carries the selected
`protocol_profile` law and catalogued `profile_operator` member, as FR-322-AC-22
requires. This is fixture setup, not a claim that every protocol node has that
body.
Add FR-322's `protocol`/`protocol_reference` carrier to that clause, with a
single `reference` body target equal to the clause's exact local `node_id`,
the same `semantic_type`, one `expression` occurrence and the target as its
sole dependency. Recompute the carrier's structural node key, graph identity
projection and package id, and keep source correspondence total. Admit that
worked package. In separate controls, change both the body target and its
sole `dependencies` entry to the same absent local key, then to the same
present non-protocol node key (the fixture's Boolean node has the clause's
semantic type). Recompute the carrier node key, identity projection and
package id for each control. For the absent-key control, also select and
supply an admitted dependency package containing a protocol node `D` whose
typed key is absent from the local graph; set both carrier fields to `D`'s
key, keep its semantic type locally available, and recompute the lock and
identities. Do not add `D` to the local graph. Separately try a
`dependency_reference` body, an extra body member, an unequal semantic type
with the original target and dependency intact, and an extra dependency.
In a separate admission attempt, supply a dependency package
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
case, the unrelated lowered sibling retains its disposition. The package has no
separate `types`, `expressions`, `clauses`, or `protocol_references` graph
collections. The protocol clause is present once as a protocol-tagged node and
preserves the fixture's catalogued control application, selected
`protocol_profile` law and `profile_operator`. No protocol-reference
collection exists. The carrier is present once in the same node array and its
exact body target resolves to the protocol clause. Both absent-local target
controls refuse `missing_declaration`/`missing-name` at the carrier's
`/body/target`, including when the selected dependency contains that key;
the non-protocol target and unequal
semantic type refuse `ill_typed`/`operator-ineligible`; the
`dependency_reference` body and extra body member fail the closed V2 schema;
the extra dependency refuses `invalid_semantic_graph`. The package identity
and source correspondence remain equivalent under canonical ordering.
