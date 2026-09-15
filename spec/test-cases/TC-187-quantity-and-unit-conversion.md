---
id: TC-187
title: "Quantity and unit conversion"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
---

# TC-187: Quantity and unit conversion

## Description

Evaluate compatible exact/lossy conversions and incompatible or affine-unit operations.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-142. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-142; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
