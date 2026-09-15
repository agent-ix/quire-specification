---
id: TC-193
title: "IEEE exceptional and rounding profiles"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-148
    type: verifies
---

# TC-193: IEEE exceptional and rounding profiles

## Description

Evaluate NaN, infinity, signed zero, width and rounding-mode discriminator vectors.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-148. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-148; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
