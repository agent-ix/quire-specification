---
id: TC-233
title: "Published checked-package and scalar contract vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-342
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-044
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-140
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-270
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-271
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
vectors, and the complete-value and complete-model definition locks and
package-selection vectors. This case proves producer-side schema closure,
digest recomputation and operation-identity and diagnostic-cause closure
only. It does not exercise a production reader, linker, lowerer or evaluator;
those criteria remain with TC-217 and TC-187.

## Test Procedure

Validate each positive fixture against its published schema and apply every
authored adverse mutation. Rebuild each V2 identity preimage from the lock
selections and occurrence-free graph, recompute the JCS SHA-256 package id,
then edit each excluded member and each named preimage member, requiring each
excluded member injected into the identity preimage to be refused. Insert the
published dependency selections of `dependency-selection-vectors.json` into
the all-families package's lock and identity preimage, require the schema to
admit it and its recorded package id to recompute, require each authored
entry mutation, and each `model_entry_mutations` entry placed in
`model_selections`, to fail the schema in either member at the mutated entry, and
apply each order vector's selections to require its recorded order outcome. Validate a `dependency_reference` term against both schemas, and its bare-digest, other-domain, missing-`node` and extra-member mutations; place it as the callee of the `function-call` operation vector and recompute the application-node id after changing only its `package` digest, then only its `node` (FR-322-AC-36). Read
a package whose function calls a dependency `function` returning `Boolean`
through a `dependency_reference` callee, with its dependency package supplied,
and then its FR-322-AC-37 mutations: no dependency package supplied, a
`package` absent from `dependency_selections`, a `node` with no
`declaration`, a `node` naming a declared record or a function returning one,
a function over `Set<R>` and one over a tuple holding `R` for a declared
record `R`, a function over `Reference<M::T>` for a model type `M::T`,
the term as a non-callee argument, the term as a node body root, and the target listed in
`dependencies`. Recompute
every nominal node and compound-unit vector identity and apply each authored
mutation, requiring the exact refusing check it names (`schema` or `semantic`),
including `-0` spellings, and require the semantic oracle itself to refuse
`-0`. Require every qualification-catalog entry of both definition locks to name
an authored artifact under a unique role and identity, require each
complete-value catalog role to occupy exactly one always, conditional or
exactly-one slot and each complete-model role the always slot, and evaluate
every accepted and refused
package-selection vector with an independent selection oracle. Check the
operation catalog's closure against both V2 schemas and the value lock;
recompute every operation vector and application node key; classify every
operation mutation, ordering case and declaration mutation with an
independent operation and declaration oracle; check the published fixture's
`state`/`frame` node body against an independent frame oracle, requiring the
frame's `semantic_type` to be a `model`/`object_type` node, each
`modifies` relationship entry, field entry (declaring node plus field name)
and `creates`/`deletes` entry to join a declared dependency of the entry's
own required meaning, each member to be duplicate-free and sorted
ascending by entry order key (declaring node digest, then field name), and
a frame body carrying two simultaneous
defects to report the one diagnostic FR-340's fixed precedence names
regardless of which member carries which defect; splice every published
`frame_mutations` vector into the semantic graph and read it with the full
normative reader (`read_package_semantics`, not the standalone frame oracle
alone), evaluated at the reader's frame step, where identity projection,
`package_id` and source-map checks are outside what any vector's
expectation covers; requiring each vector's exact code, locus and (where
the vector states one) cause, including a field entry on a process, a
relationship or a systems node, a relationship entry naming an object type,
a `model`/`record_value_type` or `model`/`event_type` entry misdirected into
`creates` or `deletes`, an undeclared field entry, two field entries of one
declaring node in descending name order, two vectors that each carry two
simultaneous meaning-join defects (one across member groups, one within one
member group at two digests), and the one vector whose `second_frame`
member is a complete second `state`/`frame` node inserted into the graph
verbatim alongside the mutated published frame node, requiring the
lower-node-id-digest frame's own defect to be reported over the
higher-digest frame's own defect; check that the identical defect set
sorted from two different collection orders yields the identical
diagnostic; validate the clause-operations package's `state`/`state_clause`,
`state`/`operation_anchor`, `state`/`frame` and `state`/`transition` nodes
against the schema's node-form body rules and occurrence roles, and apply
each authored body, member, argument-shape and role mutation, requiring the
schema to refuse it; read the package's invariant and precondition through
the full reader, requiring the state step to admit them, and read each
authored anchor, parameter, `self`-type, arity, body-root misplaced-clause
and anchor-before-clause mutation, requiring its exact state refusal and
locus; require a `quire.op.state.clause` application nested in another
clause's condition to fail the schema and refuse `malformed_wire` at the
nested application;
and parse the native-diagnostics rows to require each cause in
exactly its code's row and the Diagnostic schema pairing. Parse the
`stage_limit_exceeded` and `duplicate_selection` rows of both native-diagnostics
and FR-272 and require each to list exactly FR-272-AC-10's causes. Parse
the rows of FR-272-AC-11's eleven codes in both documents and require each to
list exactly that criterion's causes and payload keys, and require FR-272's
canonical payload-name list to hold every one of those keys. A payload key is
any backticked name written `key`: after the cause list, canonical or not.
Require FR-270's
Revision cell to quote the catalog header's revision, and FR-271's values
table to hold exactly one row for each FR-271-AC-6 code. Require both
`invalid_source_identity` rows to name the two labels in order and to state
FR-272-AC-12's labels-before-path precedence; require the catalog's payload
value spellings to hold FR-272-AC-13's domain forms and widths, the flag order
FR-148 names and the labels in the row's order; and require FR-044,
`quire.value.accounting/v1` and the catalog to state FR-044-AC-10's
evaluation-time refusals and `sum` carve-out, with FR-145 keeping a non-member
`sum` seed or prefix a located undefined outcome; and require FR-140 and the
decimal, unit-target and integer-target membership sentences of
`quire.value.accounting/v1` to carry the same `sum` carve-out on
`decimal_out_of_domain` and the unit target refusals (FR-140-AC-5).
Validate a Diagnostic entry carrying each of `type-mismatch`,
`missing-selection`, `unsupported-selection` and `wrong-selection-role` with
every code of the schema's code enum (FR-322-AC-47).

## Expected Results

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Every mutation
is refused by exactly its named check. Each dependency-selection entry mutation fails the
schema at the mutated entry; its `outcome` is the reader refusal a strict
reader reports for it. Each dependency-selection order vector is admitted by
the schema and admitted or refused by the order rule with exactly its
recorded outcome and loci. A `dependency_reference` term is admitted by both schemas and each of its four mutations fails both; as a call's callee it keys a valid application-node preimage, and changing its `package` or its `node` each gives a new, distinct id. The dependency call package is admitted, and its
mutations refuse `missing_import`/`missing-selection`,
`missing_declaration`/`missing-selection`, `missing_declaration`/`missing-name`,
`ill_typed`/`operator-ineligible` (seven times) and `invalid_semantic_graph`. Every catalog entry names an authored
artifact, and every selection vector is admitted or refused with exactly
its recorded selection code. The published fixture's frame node is admitted
with no defect; every frame-entry meaning-join and canonical-order mutation
is refused with exactly its named cause, and a frame body carrying two
simultaneous defects reports the same one diagnostic regardless of which
member carries which defect. Every published `frame_mutations` vector
refuses through the full normative reader with exactly its recorded code,
locus and cause, including every wrong-meaning frame entry, both
two-simultaneous-meaning-join-defect precedence vectors, and the
two-frame-node vector, whose lower-node-id-digest frame's own defect is
reported ahead of the higher-digest frame's own defect; the identical
defect set sorted from two different collection orders yields the identical
diagnostic; and the `stage_limit_exceeded` and `duplicate_selection` rows of
native-diagnostics and FR-272 each list exactly FR-272-AC-10's causes, and
the rows of FR-272-AC-11's eleven codes each list exactly that criterion's
causes and payload keys, every one of them a canonical FR-272 payload name;
FR-270's Revision cell quotes the header's revision; FR-271 has one row
for each FR-271-AC-6 code; both `invalid_source_identity` rows state the
labels-before-path precedence; the payload value spellings match FR-272-AC-13;
and FR-044, FR-140, value-accounting, the catalog and FR-145 give a `sum` seed
or prefix one outcome in every family. The Diagnostic schema admits
`type-mismatch` only with `ill_typed`, `missing-selection` only with
`missing_import` and `missing_declaration`, and `unsupported-selection` and
`wrong-selection-role` only with `unknown_language`, `unknown_edition` and
`unknown_profile`.
