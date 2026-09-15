---
id: TC-215
title: "Continuous hybrid analysis"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: verifies
---

# TC-215: Continuous hybrid analysis

## Description

Compute a sound enclosure, replay a hybrid violation and force unsupported/convergence outcomes.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-193. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-193; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
