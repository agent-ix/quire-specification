---
id: TC-203
title: "Choice obligation and capture binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-172
    type: verifies
---

# TC-203: Choice obligation and capture binding

## Description

Bind visibility evidence, whole-execution origins and duplicate ordered captures at activation.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-172. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-172; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
