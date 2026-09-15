---
id: TC-229
title: "Analysis codegen accounting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-309
    type: verifies
---

# TC-229: Analysis codegen accounting

## Description

Produce source-bound findings/artifacts and test unsupported nodes and heuristic-proof separation.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-309. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-309; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
