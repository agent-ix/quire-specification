---
id: TC-226
title: "AOT JIT native and cache parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-306
    type: verifies
---

# TC-226: AOT JIT native and cache parity

## Description

Compare compiled/native results and mutate every cache key component plus unsupported nodes.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-306. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-306; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
