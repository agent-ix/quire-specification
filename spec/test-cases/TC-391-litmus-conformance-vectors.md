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
and compare its `ra` column; LT-12 and LT-13 are the two tests on which the
checker forbids the outcome and `ra` allows it.

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
| LT-12 | three threads, all locations 0: `u`: `X := 1` seq_cst; `Y := 1` release. `v`: `a := Y` acquire; `b := Z` seq_cst. `w`: `Z := 1` seq_cst; `c := X` seq_cst | a = 1, b = 0, c = 0 | forbidden | forbidden | allowed (RC11 forbids it: the `po|≠loc ; hb ; po|≠loc` edge from `X := 1` to `b := Z`) |
| LT-13 | `u`: `X := 1` seq_cst; `a := Y` seq_cst. `v`: `Y := 1` relaxed; fence seq_cst; `b := X` relaxed | a = 0, b = 0 | forbidden | forbidden | allowed (RC11 forbids it: `a := Y` precedes the fence through the relaxed store, the fence precedes `X := 1` through the relaxed load) |
| LT-14 | `u`: `X := 1` seq_cst; fence seq_cst; `Y := 1` relaxed. `v`: `a := Y` acquire; `P := 1` seq_cst. `w`: `P := 2` seq_cst; `b := X` seq_cst | a = 1, b = 0, final P = 2 | forbidden | forbidden | forbidden (the fence precedes `P := 1` through the flag: `Y`'s message view records the fence) |
| LT-18 | `u`: `X := 1` seq_cst; `Y := 1` release. `v`: `a := Y` acquire; fence seq_cst; `b := Z` seq_cst. `w`: `Z := 1` seq_cst; `c := X` seq_cst | a = 1, b = 0, c = 0 | forbidden | forbidden | forbidden (`X := 1` precedes `v`'s fence through the flag) |

Separately, pass the synthesized SCC of FR-437-AC-7 straight to the visibility
filter (no protocol is run): states `p`, `q`, `r`, edges `p → q`, `q → p` and
`r → q` by thread `a` (loading nothing), `q → r` a stale load of `ℓ` by thread
`b`, and `p` the only accepting state. A protocol form of this graph needs a
stale `b` load that leaves the state unchanged and `scheduling adversarial;`,
since the default scheduler constraint for `b` would reject `{p, q}` where `b`
is enabled.
## Expected Results

Every cell matches the table; no run reaches a memory budget; the `ra`
column contains every outcome the independent RC11 checker allows and differs
from the checker only on LT-12 and LT-13.

The synthesized SCC of FR-437-AC-7 is decomposed to `{r}` then `{p, q}` and `{p, q}` passes; no memory budget applies and no RC11 comparison does.
