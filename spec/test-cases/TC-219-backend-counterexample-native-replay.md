---
id: TC-219
title: "Backend counterexample native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-197
    type: verifies
---

# TC-219: Backend counterexample native replay

## Description

Round-trip, minimize and replay backend counterexamples and inject decode/domain/verdict mismatches.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-197. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| MR-01 | Minimize a `Witness`-arm counterexample and offer a shrunk candidate that had no backend re-run | Not accepted as a `Witness`-arm revision; no revision carries the parent's transcript | FR-197-AC-5 |
| MR-02 | Minimize an `Input`-arm counterexample | Every accepted revision is `Input`-arm, linked to its parent, and preserves domain validity and the native verdict | FR-197-AC-19 |

## Expected Results

Every positive and boundary result matches FR-197; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
