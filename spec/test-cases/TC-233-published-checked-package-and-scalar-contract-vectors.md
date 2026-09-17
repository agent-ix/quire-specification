---
id: TC-233
title: "Published checked-package and scalar contract vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
---

# TC-233: Published checked-package and scalar contract vectors

## Description

Qualify the published QSpec contract artifacts themselves: the V2
checked-package schema and fixtures, the nominal node-identity preimage
schema and vectors, and the compound-unit value identity schema and vectors,
and the complete-value definition lock, rule manifest and package-selection
vectors. This case proves producer-side schema closure and digest
recomputation only. It does not exercise a production reader, linker, lowerer
or evaluator; those criteria remain with TC-217 and TC-187.

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
package-selection vector with an independent selection oracle.

## Expected Results

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Every mutation
is refused by exactly its named check. Every locked artifact matches its
raw-byte digest, and every selection vector is admitted or refused with exactly
its recorded selection code.
