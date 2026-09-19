---
id: TC-247
title: "Unbounded population query"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-153
    type: verifies
---

# TC-247: Unbounded population query

## Description

Query `allInstances`/`lookup` against a closed population binding that
declares no maximum, alongside one that does.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-153. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-153-AC-9; a population binding
with no declared maximum returns the unbounded `Set<Reference<T>>` and never
`cardinality_out_of_bound`; each invalid or non-conclusive mutation returns its
exact typed disposition with source/provenance and preserves independent
sibling results.
