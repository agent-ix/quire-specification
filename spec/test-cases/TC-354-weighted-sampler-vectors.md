---
id: TC-354
title: "Weighted sampler, choice index and trace index"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-354: Weighted sampler, choice index and trace index

## Description

Check the revised `quire.simulation.sampler/v1`: the choice index in the preimage, weighted selection by exact integer weights, and trace-index assignment.

## Test Procedure

Implement the sampler through the real local Rust boundary and compute each vector's selections from the stated preimages; compare the selected indices and the step-0 digest.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SM-01 | Seed `424242`, trace `0`, choice `0`, uniform over `n = 5`, steps 0 to 4 | Indices `1, 3, 3, 4, 4`; step-0 preimage `{"choice":"0","draw":"0","seed":"424242","step":"0","trace":"0"}` with SHA-256 `d5160380d7495443315376de306a5d3613f3e010df74a5db92853829205995f2` (the TC-210 vector) |
| SM-02 | Seed `424242`, trace `0`, choice `1`, weights `(1, 3)` (sum `n = 4`), steps 0 to 4 | `v mod 4` values `1, 0, 0, 2, 3` select items `1, 0, 0, 1, 1` |
| SM-03 | Weights `(1/2, 3/4)` | Scaled by 4 to `(2, 3)`, `n = 5` |
| SM-04 | `m = 2` tests, sample `i = 1` of test `j = 1` | Trace index `3` |
| SM-05 | A workload step with one enabled operation of unit weight and one identity, no random parameter | The operation and identity choices have `n = 1`, draw no digest and take no choice index; the selection equals the unweighted uniform selection |
| SM-06 | A `uniform` delay with `q = 8` over window `[1, 3]` ms and grid value from `v mod 256` | Delay `1 + 2 · (v mod 256)/256` ms, an exact rational |
| SM-07 | A workload step with one enabled operation and one identity, and one random parameter with weights `(1, 3)`, seed `424242`, trace `0`, step `0` | The parameter's choice takes choice index `0` (the two one-item choices take none), so its preimage is `{"choice":"0","draw":"0","seed":"424242","step":"0","trace":"0"}`, the TC-210 step-0 digest |
| SM-08 | A weighted choice with total `n = 2^256` (weights `(2^256 - 1, 1)`), seed `424242`, trace `0`, choice `0`, step `0` | Accepted at `draw = 0` with the digest `d5160380d7495443315376de306a5d3613f3e010df74a5db92853829205995f2` (`v = 0xd516…95f2` is below `2^256 - 1`); selects index `0` |
| SM-09 | The same choice with total `n = 2^256 + 1` (weights `(2^256, 1)`) | The run settles `unsupported` with cause `total-above-draw-range{total: "115792089237316195423570985008687907853269984665640564039457584007913129639937"}` before any draw: no digest is computed, no choice index is taken and the sampler does not loop |

## Expected Results

Every selected index, the digest and every delay are exactly as stated; equal seeds and sampler identity reproduce identical selections.
