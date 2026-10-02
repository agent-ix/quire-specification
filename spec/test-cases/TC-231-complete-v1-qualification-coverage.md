---
id: TC-231
title: "Complete V1 qualification coverage"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-311
    type: verifies
---

# TC-231: Complete V1 qualification coverage

## Description

Validate one primary row per inventory capability, execute the vectors, then remove and mutate a required vector.

## Test Procedure

Use the subject and inputs named by FR-311. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-311; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
