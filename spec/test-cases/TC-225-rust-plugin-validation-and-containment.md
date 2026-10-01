---
id: TC-225
title: "Rust plugin validation and containment"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-305
    type: verifies
---

# TC-225: Rust plugin validation and containment

## Description

Run valid trait/process plugins and inject version, rights, malformed, crash and timeout failures.

## Test Procedure

Use the subject and inputs named by FR-305. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-305; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
