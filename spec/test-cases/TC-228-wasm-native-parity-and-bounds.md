---
id: TC-228
title: "WASM native parity and bounds"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-308
    type: verifies
---

# TC-228: WASM native parity and bounds

## Description

Compare supported native/WASM operations and inject unknown wire, oversized input and ambient access.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-308. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-308; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
