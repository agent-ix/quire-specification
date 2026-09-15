---
id: TC-210
title: "Finite simulation reproducibility"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-210: Finite simulation reproducibility

## Description

Enumerate and seed-sample declared schedules, replay them, and exhaust the exploration bound.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-181. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-181; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
