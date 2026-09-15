---
id: TC-192
title: "Integer division profiles"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-147
    type: verifies
---

# TC-192: Integer division profiles

## Description

Evaluate signed vectors, zero divisors and missing finite-backend domains under each division law.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-147. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-147; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
