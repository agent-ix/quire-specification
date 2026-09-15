---
id: TC-194
title: "Complete equality matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-149
    type: verifies
---

# TC-194: Complete equality matrix

## Description

Execute positive, negative and cross-type comparisons for every complete-V1 value kind.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-149. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-149; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
