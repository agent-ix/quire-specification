---
id: TC-208
title: "Protocol refinement relation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-177
    type: verifies
---

# TC-208: Protocol refinement relation

## Description

Check a refining model and counterexamples for extra trace, deadlock, divergence and stronger assumptions.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-177. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-177; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
