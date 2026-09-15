---
id: TC-181
title: "Recursive function termination profiles"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-181: Recursive function termination profiles

## Description

Check terminating and non-terminating recursive call graphs under historical and complete-function profiles.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-146. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-146; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
