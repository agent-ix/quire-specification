---
id: TC-212
title: "Model-based generation and shrinking"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-190
    type: verifies
---

# TC-212: Model-based generation and shrinking

## Description

Generate model-bound boundaries and shrink a failure while preserving domain and verdict.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-190. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-190; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
