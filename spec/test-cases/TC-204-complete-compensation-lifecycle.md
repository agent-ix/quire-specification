---
id: TC-204
title: "Complete compensation lifecycle"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-173
    type: verifies
---

# TC-204: Complete compensation lifecycle

## Description

Traverse every permitted state transition and reject undeclared, identity-collapsed or stale transitions.

## Test Procedure

Use the subject and inputs named by FR-173. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-173; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
