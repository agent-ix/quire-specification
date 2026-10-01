---
id: TC-201
title: "Channel and queue premises"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-170
    type: verifies
---

# TC-201: Channel and queue premises

## Description

Exercise FIFO keys, missing delivery/duplication premises, capacities and every overflow policy.

## Test Procedure

Use the subject and inputs named by FR-170. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-170; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
