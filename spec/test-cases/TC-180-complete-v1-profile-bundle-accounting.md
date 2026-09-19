---
id: TC-180
title: "Complete V1 profile bundle accounting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-131
    type: verifies
---

# TC-180: Complete V1 profile bundle accounting

## Description

Link the full 191-capability bundle, remove one required facet, and repeat with changed backend support.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-131. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-131; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
