---
id: TC-186
title: "Text and enum identity semantics"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-141
    type: verifies
---

# TC-186: Text and enum identity semantics

## Description

Compare normalized text and declaration-qualified enum members at valid and invalid boundaries.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-141. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-141; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
