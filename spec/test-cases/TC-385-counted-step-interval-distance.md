---
id: TC-385
title: "Counted-step interval distance and protocol anchors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-431
    type: verifies
---

# TC-385: Counted-step interval distance and protocol anchors

## Description

Check which protocol steps count for interval distance, the anchors of
protocol positions, and that formulas with no interval operator do not depend
on counting.

## Test Procedure

Evaluate each vector's formula over the named behaviour through the real
local Rust boundary, recording the truth value at each position, each
position's anchor and the counted distance between positions.

## Vectors

| Vector | Behaviour and formula | Expected result |
| --- | --- | --- |
| CS-01 | `Fill` (TC-376, repaired) along s0, s1, `attempt(B)`, `attempt(A)`, `join`, `finish`; `eventually[0,2] holds(k.v = 3)` | holds at position 0: the counted steps are the two attempts, distance 2 to the state with `v = 3` |
| CS-02 | the same behaviour; `eventually[0,1] holds(k.v = 3)` at position 0 | false: distance 2 |
| CS-03 | `Pay` failure path (TC-380); `eventually[0,2] holds(k.bal = 0)` | holds at position 2, fails at position 1 |
| CS-04 | a behaviour of one step of each kind | `attempt`, `cattempt`, `event`, `activate`, `send`, `receive` and the terminal stutter step advance distance by 1; `fork`, `join`, `finish`, `timeout`, `cend`, `duplicate`, `lose`, `spawn`, `retire`, `fence` and `flush` advance it by 0 |
| CS-05 | `Fill` along s0, s1, s3, s4, s5, s6 (positions 0 to 5, `B` first); `once[0,1] holds(k.v = 0)` | false at position 5: the two attempts lie between position 1 and position 5, distance 2; true at position 2, after `attempt(B)`, distance 1 back to position 1 |
| CS-06 | anchors along `Fill` | position 0 `initialization`; `attempt` positions the operation anchor; `fork`, `join` and `finish` positions a `protocol` anchor naming the kind and node |
| CS-07 | `Fill`; `always holds(k.v <= 3)` and `eventually holds(k.v >= 2)` | the same verdicts when every uncounted step is read as counted |

## Expected Results

Every truth value, distance and anchor matches the table.
