---
id: TC-210
title: "Finite simulation reproducibility"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-210: Finite simulation reproducibility

## Description

Enumerate and seed-sample declared schedules, replay them, and exhaust the exploration bound.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-181. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-181; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.

## Ecosystem lock

The exact version-locked subject is a real `quire.ecosystem-lock/v1`
(FR-335) listing only the QSL revision; its `definitions` member pins the
package definitions and the `quire.simulation.sampler/v1` sampler used for
seeded sampling.

## Refusal vectors

The explicit refusal/non-conclusive mutations include:

- an application whose operation identity is outside the closed operation
  catalog, which refuses `invalid_package`/`unknown-operation` at its
  `source_map` locus;
- a write outside the operation's declared frame, which refuses
  `frame_violation`/`unauthorized-change`;
- a model-described external effect, which the simulator performs none of
  and instead records only as typed trace data.
