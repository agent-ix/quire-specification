---
id: TC-246
title: "Unbounded collection type identity and admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-144
    type: verifies
---

# TC-246: Unbounded collection type identity and admission

## Description

Construct and traverse an unbounded collection type, and compare it against
its bounded sibling of the same kind and element type.

## Test Procedure

Use the subject and inputs named by FR-144. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-144-AC-12 and FR-144-AC-13; an
unbounded collection type never returns `cardinality_out_of_bound`; equality
between a bounded and an unbounded sibling type is `ill_typed`; each invalid or
non-conclusive mutation returns its exact typed disposition with
source/provenance and preserves independent sibling results.
