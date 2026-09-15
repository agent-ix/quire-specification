---
id: TC-198
title: "Closed model lookup"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-153
    type: verifies
---

# TC-198: Closed model lookup

## Description

Query lookup/all-instances with complete, unknown, foreign and over-bound populations.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-153. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-153; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
