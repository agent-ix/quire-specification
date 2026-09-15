---
id: TC-196
title: "Conformance redefinition and dispatch"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-151
    type: verifies
---

# TC-196: Conformance redefinition and dispatch

## Description

Resolve valid specialization/dispatch and reject cycles, weakening, ambiguity and incomplete closure.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-151. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-151; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
