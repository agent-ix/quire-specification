---
id: TC-224
title: "Model query and rendering determinism"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-304
    type: verifies
---

# TC-224: Model query and rendering determinism

## Description

Render equal inputs twice and test subject, loss-accounting and resource refusal cases.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-304. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-304; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
