---
id: TC-200
title: "Infinite-trace provider boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
---

# TC-200: Infinite-trace provider boundary

## Description

Prove/refute supported properties and test finite-prefix, missing-premise and resource-inconclusive cases.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-161. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-161; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
