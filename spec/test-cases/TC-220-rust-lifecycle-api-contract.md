---
id: TC-220
title: "Rust lifecycle API contract"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
    type: verifies
---

# TC-220: Rust lifecycle API contract

## Description

Call every lifecycle stage with valid and wrong-stage/version/capability requests.

## Test Procedure

Use the subject and inputs named by FR-300. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-300; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
