---
id: IT-074
title: "Verification plans dispatch exact method providers"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-190
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-194
    type: verifies
---

# IT-074: Verification plans dispatch exact method providers

## Objective

Verify that the real `quire-verification` Rust planner links and dispatches
model-testing, simulation, relational, hybrid and synthesis providers with exact
domains, dependencies and non-conclusive results.

## Target Integration

Pinned planner and method-provider Rust APIs exchange I14/I15 envelopes in local
processes or isolated Rust executables according to their manifests.

## Preconditions

Every provider manifest and tool/options pin is present in the ecosystem lock.

## Inputs

A dependency-bearing plan plus missing-provider, cyclic, invalid-domain,
exhaustion and synthesis-candidate failure variants.

## Test Procedure

1. Link and execute the valid plan.
   - IT-074-SC-01: dispatch order and evidence accounting match dependencies.
2. Replay generated/shrunk/candidate artifacts natively.
   - IT-074-SC-02: typed verdict and lineage are preserved.
3. Execute each invalid/incomplete variant.
   - IT-074-SC-03: exact non-conclusive/refusal results preserve siblings.

## Expected Results

No provider changes admission, a candidate is not a proof, and all method
limitations remain typed and reproducible.
