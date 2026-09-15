---
id: TC-197
title: "Systems-model typed binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-152
    type: verifies
---

# TC-197: Systems-model typed binding

## Description

Resolve every model element kind and attempt kind, authority and closure substitutions.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-152. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-152; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
