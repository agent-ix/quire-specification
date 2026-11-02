---
id: TC-199
title: "Uncertain revisioned temporal evaluation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-160
    type: verifies
---

# TC-199: Uncertain revisioned temporal evaluation

## Description

Evaluate partial valuations and clock intervals, then repair an out-of-order observation revision.

## Test Procedure

Use the subject and inputs named by FR-160. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Counter-tick cover vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| TCV-A | bound 3000 ticks, uncertainty 0, request at tick 0, acknowledgements at ticks 2998, 2999, 3000, 3001 and 3002 (`TCV-01` to `TCV-05`) | true, true, indeterminate, false, false: the cover is half-open `[k, k + 1)` | FR-160-AC-6, FR-160-AC-7 |
| TCV-B | uncertainty 1, acknowledgements at 2997, 2998 and 3003 (`TCV-06` to `TCV-08`) | true, indeterminate, false | FR-160-AC-7 |
| TCV-C | ticks 5 and 6, 5 and 5, 5 and 7, and 5 and 6 under uncertainty 1 (`TCV-09` to `TCV-12`) | definitely before, not, definitely before, not | FR-160-AC-6 |

## Expected Results

Every positive and boundary result matches FR-160; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
