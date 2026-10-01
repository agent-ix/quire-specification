---
id: TC-182
title: "Complete package capabilities and identity domains"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-132
    type: verifies
---

# TC-182: Complete package capabilities and identity domains

## Description

Substitute a digest across every typed identity domain and add an unknown required capability.

## Test Procedure

Use the subject and inputs named by FR-132. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-132; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
