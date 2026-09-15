---
id: TC-213
title: "Relational and hyperproperty analysis"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-191
    type: verifies
---

# TC-213: Relational and hyperproperty analysis

## Description

Evaluate paired executions and mixed trace quantifiers with complete and incomplete domains.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-191. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-191; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
