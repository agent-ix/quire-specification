---
id: TC-205
title: "Sound global projection"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-174
    type: verifies
---

# TC-205: Sound global projection

## Description

Project a valid graph and report hidden-choice, incompatible-merge, unsupported and exhausted cases.

## Test Procedure

Use the subject and inputs named by FR-174. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-174; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
