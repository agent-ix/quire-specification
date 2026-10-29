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

Check that the ContractPackage representation uses the I04 node-tag, node_id and
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
fixture, use a `protocol_clause` node with the FR-464 clause body; verify its
`quire.op.protocol.control` application carries the selected
`protocol_profile` law and catalogued `profile_operator` member, as FR-322-AC-22
requires.
Add FR-322's `protocol`/`protocol_reference` carrier to that clause, with a
single `reference` body target equal to the clause's exact local `node_id`,
the same `semantic_type`, one `expression` occurrence and the target as its
sole dependency (the occurrence rules follow under "Occurrence vectors"). Give the carrier its structural node_id and keep source correspondence total. Admit that
worked package. In separate controls, change both the body target and its
sole `dependencies` entry to the same absent local node_id, then to the same
present non-protocol node_id (the fixture's Boolean node has the clause's
semantic type). For the absent-key control, also select and
supply an admitted dependency package containing a protocol node `D` whose
node_id is absent from the local graph; set both carrier fields to `D`'s
node_id, keep its semantic type locally available. Do not add `D` to the local graph. Separately try a
`dependency_reference` body, an extra body member, an unequal semantic type
with the original target and dependency intact, and an extra dependency.
In a separate admission attempt, supply two structurally unequal dependency
packages under the one selected identity.

## Expected Results

The package contains one `semantic_graph.nodes` collection in the producer's
graph order (FR-322), with each semantic node represented once; its intra-package semantic
references resolve locally, the dependency function call resolves through the
selected admitted package, and recursion-group cycles remain keyed references
in a finite value. The dependency-package mutation refuses
`invalid_package`/`conflicting-definition` naming the selection identity
during package admission, before lowering begins, so no
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
the extra dependency refuses `invalid_semantic_graph`. The source correspondence remains equivalent under canonical ordering.

## Occurrence vectors

A `protocol_reference` carrier holds one or more `expression` occurrences and
one node exists per target (FR-322-AC-66): the carrier node_id comes from tag,
form, semantic type, declaration, recursion and body, never from occurrences.
Each vector starts from the worked package above (carrier `R` to clause `P`) and
changes the graph where it states.

| Vector | Setup | Expected |
|---|---|---|
| V1 positive: two references to one protocol | `R` carries `expression` occurrences with ordinals 0 and 1; `source_map` has two entries (`R`, `expression`, 0) and (`R`, `expression`, 1) naming different source regions. The graph holds no second carrier. | Admitted. One carrier node for `P`; `R.node_id` equals that of the one-occurrence package; the reader resolves `R` to `P` once. |
| V2 positive: occurrences outside identity | V1 with each occurrence's regions changed, or with a third `expression` occurrence and `source_map` entry added. | Admitted; carrier `node_id` unchanged (occurrences and `source_map` are outside identity). |
| V3 negative: no occurrence | `R.occurrences` is empty (its `source_map` entries removed). | Fails the closed V2 schema. |
| V4 negative: wrong role | One of `R`'s occurrences has role `declaration` (or `type`, `anchor`, `claim`, `generated`), with its `source_map` entry changed to match. | Fails the closed V2 schema. |
| V5 negative: duplicate ordinal | `R` carries two `expression` occurrences with ordinal 0, or two `source_map` entries keyed (`R`, `expression`, 0). | Refuses `invalid_source_map`: for the `occurrences` variant at the later occurrence's `/semantic_graph/nodes/{n}/occurrences/{j}` pointer (the `source_map` holds one entry (`R`, `expression`, 0)); for the `source_map` variant at the later entry's `/source_map/{i}` pointer. |
| V6 negative: two nodes, one key | The graph holds `R` twice (each with one occurrence ordinal 0, and the `source_map` holding one entry (`R`, `expression`, 0), so V5 cannot fire first), or two carriers to `P` with equal node_id. | Refuses `invalid_semantic_graph`; node identifiers are unique. |

Assertions compare typed fields, causes and keys, never message text.
