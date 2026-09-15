---
id: TC-217
title: "Complete contract IR lowering"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-195
    type: verifies
---

# TC-217: Complete contract IR lowering

## Description

Lower supported complete-V1 nodes and mutate source/type/anchor/bounds plus unsupported nodes.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-195. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-195; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
