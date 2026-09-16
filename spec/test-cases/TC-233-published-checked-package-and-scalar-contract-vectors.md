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

Qualify the published QSpec contract artifacts themselves: the frozen V1 and
V2 checked-package schemas and fixtures, the nominal node-identity preimage
schema and vectors, the migration outcome schema and receipts, and the
compound-unit value identity schema and vectors, and the complete-value
definition lock, rule manifest and package-selection vectors. This case proves producer-side
schema closure, digest recomputation and receipt classification only. It does
not exercise a production reader, linker, lowerer or evaluator; those criteria
remain with TC-217 and TC-187.

## Test Procedure

Validate each positive fixture against its published schema and apply every
authored adverse mutation; pin the frozen V1 publication bytes. Rebuild each V2
identity preimage from the lock selections and occurrence-free graph, recompute
the JCS SHA-256 package id, then edit each excluded member and each named
preimage member. Compare equal digest bytes under the V1 and V2 package domains
as typed keys and against each schema. Recompute every nominal node and
compound-unit vector identity and apply each authored mutation, requiring the
exact refusing check it names (`schema` or `semantic`), including `-0`
spellings. Classify every published migration receipt with an independent
oracle in the order byte-only, missing, ambiguous, stale, target. Hash every
qualification-catalog artifact and rule against the definition lock and rule
manifest, require each catalog role to occupy exactly one always, conditional
or exactly-one slot, and evaluate every accepted and refused package-selection
vector with an independent selection oracle.

## Expected Results

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Distinct package
domains never compare equal. Every mutation is refused by exactly its named
check, and every refusal receipt yields exactly its recorded code and subject.
Every locked artifact matches its raw-byte digest, and every selection vector
is admitted or refused with exactly its recorded selection code.
