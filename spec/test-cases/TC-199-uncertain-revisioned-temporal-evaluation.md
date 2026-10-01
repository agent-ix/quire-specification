---
id: TC-199
title: "Uncertain revisioned temporal evaluation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-160
    type: verifies
---

# TC-199: Uncertain revisioned temporal evaluation

## Description

Evaluate partial valuations and clock intervals, then repair an out-of-order observation revision.

## Test Procedure

Use the subject and inputs named by FR-160. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-160; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
