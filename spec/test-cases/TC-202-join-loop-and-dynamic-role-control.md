---
id: TC-202
title: "Join loop and dynamic-role control"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-171
    type: verifies
---

# TC-202: Join loop and dynamic-role control

## Description

Exercise every join policy, proved/unproved loops and bounded/unbounded dynamic populations.

## Test Procedure

Use the subject and inputs named by FR-171. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-171; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
