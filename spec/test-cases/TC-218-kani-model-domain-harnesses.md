---
id: TC-218
title: "Kani model-domain harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-196
    type: verifies
---

# TC-218: Kani model-domain harnesses

## Description

Generate boundary harnesses from exact domains and inspect missing-bound, unsupported and pin accounting.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-196. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-196; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
