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

Use the subject and inputs named by FR-181. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-181; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.

## Sampler

Seeded sampling selects the `quire.simulation.sampler/v1` sampler by its
identity.

## Refusal vectors

The explicit refusal/non-conclusive mutations include:

- an application whose operation identity is outside the closed operation
  catalog, which refuses `invalid_package`/`unknown-operation` at its
  `source_map` locus;
- a write outside the operation's declared frame, which refuses
  `frame_violation`/`unauthorized-change`;
- a model-described external effect, which returns only as typed trace data.

## Non-coalescing and cancellation vectors

- Two states whose typed semantic state differs only in an IEEE value's NaN
  payload or a signed zero's sign have distinct state keys and remain
  distinct states.
- A successor that violates a declared invariant is recorded as a successor
  rather than pruned from exploration.
- A caller cancellation mid-exploration returns `incomplete` with cause
  `cancelled`/`caller-cancelled` and carries the unexplored frontier.

## Sampler index-sequence vector

For seed `424242`, trace `0` and enabled-successor-set size `n = 5`, a real
implementation of the counter-mode generator draws, at `draw = 0` for every
step: index `0` at step `0`, index `0` at step `1`, index `4` at step `2`,
index `4` at step `3` and index `4` at step `4`. The step-0 preimage is
`{"draw":"0","seed":"424242","step":"0","trace":"0"}`; its SHA-256 is
`cb7d4b3b8b8491d8310ccc1e07c3ee236d3fe86cf713d1851533a85ef6682622`.
