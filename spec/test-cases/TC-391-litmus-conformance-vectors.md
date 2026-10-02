---
id: TC-391
title: "Litmus conformance vectors under sc, tso and ra"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-436
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-437
    type: verifies
---

# TC-391: Litmus conformance vectors under sc, tso and ra

## Description

Check the classic litmus tests under each memory model: whether the weak
outcome is reachable, and that the `ra` outcomes agree with an independent
RC11 checker.

## Test Procedure

Write each litmus test as a protocol with one `parallel`, one branch per
thread, locations as `Cell` fields and registers as branch-local `Core`
fields, as TC-390 does. State each test's claim as the invariant `always
holds(not outcome)`: the outcome is allowed exactly when the invariant is
refuted.
Check each test under `sc`, `tso` and `ra` through the real local Rust
boundary with default budgets, and record whether the outcome is reachable.
Run each test under RC11 with an independent execution-graph model checker
and compare its `ra` column.

## Vectors

"Allowed" means the weak outcome is reachable.

| Vector | Test | Weak outcome | `sc` | `tso` | `ra` |
| --- | --- | --- | --- | --- | --- |
| LT-01 | SB, release stores, acquire loads | both loads read 0 | forbidden | allowed | allowed |
| LT-02 | SB with `seq_cst` fences | both loads read 0 | forbidden | forbidden | forbidden |
| LT-03 | SB with `seq_cst` stores and loads | both loads read 0 | forbidden | forbidden | forbidden |
| LT-04 | MP, `release` flag store, `acquire` flag load | flag 1, data 0 | forbidden | forbidden | forbidden |
| LT-05 | MP, `relaxed` flag accesses | flag 1, data 0 | forbidden | forbidden | allowed |
| LT-06 | LB, `relaxed` | each load reads the other thread's later store | forbidden | forbidden | forbidden |
| LT-07 | IRIW, `release` stores, `acquire` loads | the two readers disagree on the order of the writes | forbidden | forbidden | allowed |
| LT-08 | IRIW, all `seq_cst` | as LT-07 | forbidden | forbidden | forbidden |
| LT-09 | 2+2W, `release` stores | each location ends with its first-written value | forbidden | forbidden | allowed |
| LT-10 | CoRR | the second read returns an mo-earlier value | forbidden | forbidden | forbidden |
| LT-11 | visibility fairness, `ra`: a reader spinning on a flag the writer sets, in a `repeat` with no maximum whose guard stays true until the flag is read as 1, liveness claim `eventually holds(r = 1)` | — | `proved` | `proved` | `proved`: a loop that keeps reading the stale flag is rejected by visibility fairness |

## Expected Results

Every cell matches the table; no run reaches a memory budget; the `ra`
column agrees with the independent RC11 checker on every test.
