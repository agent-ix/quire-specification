---
id: TC-233
title: "Published checked-package and scalar contract vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
  - target: ix://agent-ix/quire-specification/interface_004
    type: verifies
---

# TC-233: Published checked-package and scalar contract vectors

## Description

Qualify the published QSpec contract artifacts themselves: the V2
checked-package schema and fixtures, the nominal node-identity preimage
schema and vectors, the operation catalog and its clause and control
operation identities, and the compound-unit value identity schema and
vectors, and the complete-value definition lock, rule manifest and
package-selection vectors. This case proves producer-side schema closure,
digest recomputation and operation-identity and diagnostic-cause closure
only. It does not exercise a production reader, linker, lowerer or evaluator;
those criteria remain with TC-217 and TC-187.

## Test Procedure

Validate each positive fixture against its published schema and apply every
authored adverse mutation. Rebuild each V2 identity preimage from the lock
selections and occurrence-free graph, recompute the JCS SHA-256 package id,
then edit each excluded member and each named preimage member, requiring each
excluded member injected into the identity preimage to be refused. Recompute
every nominal node and compound-unit vector identity and apply each authored
mutation, requiring the exact refusing check it names (`schema` or `semantic`),
including `-0` spellings, and require the semantic oracle itself to refuse
`-0`. Hash every qualification-catalog artifact and rule against the definition
lock and rule manifest, require each catalog role to occupy exactly one always,
conditional or exactly-one slot, and evaluate every accepted and refused
package-selection vector with an independent selection oracle. Check the
operation catalog's closure against both V2 schemas and the value lock;
recompute every operation vector and application node key; classify every
operation mutation, ordering case and declaration mutation with an
independent operation and declaration oracle; check the published fixture's
`state`/`frame` node body against an independent frame oracle, requiring each
`modifies`/`creates`/`deletes` entry to join a declared dependency of the
frame's own required meaning, each member to be duplicate-free and sorted
ascending by entry digest, and a frame body carrying two simultaneous
defects to report the one diagnostic FR-340's fixed precedence names
regardless of which member carries which defect; splice every published
`frame_mutations` vector into the semantic graph and read it with the full
normative reader (`read_package_semantics`, not the standalone frame oracle
alone), evaluated at the reader's frame step, where identity projection,
`package_id` and source-map checks are outside what any vector's
expectation covers; requiring each vector's exact code, locus and (where
the vector states one) cause, including a `model`/`operation_declaration`,
`model`/`record_value_type` or `model`/`event_type` entry misdirected into
`modifies`, `creates` or `deletes`, two vectors that each carry two
simultaneous meaning-join defects (one across member groups, one within one
member group at two digests), and the one vector whose `second_frame`
member is a complete second `state`/`frame` node inserted into the graph
verbatim alongside the mutated published frame node, requiring the
lower-node-id-digest frame's own defect to be reported over the
higher-digest frame's own defect; check that the identical defect set
sorted from two different collection orders yields the identical
diagnostic; and parse the native-diagnostics rows to require each cause in
exactly its code's row and the Diagnostic schema pairing.

## Expected Results

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Every mutation
is refused by exactly its named check. Every locked artifact matches its
raw-byte digest, and every selection vector is admitted or refused with exactly
its recorded selection code. The published fixture's frame node is admitted
with no defect; every frame-entry meaning-join and canonical-order mutation
is refused with exactly its named cause, and a frame body carrying two
simultaneous defects reports the same one diagnostic regardless of which
member carries which defect. Every published `frame_mutations` vector
refuses through the full normative reader with exactly its recorded code,
locus and cause, including every wrong-meaning frame entry, both
two-simultaneous-meaning-join-defect precedence vectors, and the
two-frame-node vector, whose lower-node-id-digest frame's own defect is
reported ahead of the higher-digest frame's own defect; and the identical
defect set sorted from two different collection orders yields the identical
diagnostic.
