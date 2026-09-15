---
id: TC-184
title: "Original source locus preservation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-134
    type: verifies
---

# TC-184: Original source locus preservation

## Description

Parse and extract CRLF/LF/final-newline variants and inject invalid deletion mappings.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-134. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-134; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
