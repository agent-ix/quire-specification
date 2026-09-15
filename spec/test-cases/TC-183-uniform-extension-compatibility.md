---
id: TC-183
title: "Uniform extension compatibility"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-133
    type: verifies
---

# TC-183: Uniform extension compatibility

## Description

Select a valid extension, then inject dependency cycles, identifier collisions, weakening and an older reader.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-133. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-133; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
