---
id: TC-394
title: "Weak-memory counterexample wire and replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-437
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-439
    type: verifies
---

# TC-394: Weak-memory counterexample wire and replay

## Description

Check the memory members of a weak-memory counterexample, their replay and
refusals, routing by advertised memory model, and memory-step positions.

## Test Procedure

Take the `tso` and `ra` counterexamples of TC-390 SB-02 and SB-03, apply each
mutation below, and replay through the real local Rust boundary. Route the
SB-02 request to fixture candidates advertising the memory models the vectors
name.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| MK-01 | SB-02 counterexample, unmodified | each step carries `memory` with both buffers; the counterexample carries `memory_models` `[{parallel: "SB::Both", model: "tso"}]` and `memory_budgets`; it replays to `reproduced-with-evaluated-witness` |
| MK-02 | SB-03 counterexample, unmodified | each step carries every message sequence with message views, both threads' views, `S` and the SC event graph; each load names its message depth in `memory_choice`; it replays |
| MK-03 | Withdrawn: the carried memory component is not recomputed and compared. | Withdrawn |
| MK-04 | MK-02 with the last load naming depth 2 | refuses `invalid_runtime_input`/`invalid-value`: no such message |
| MK-05 | MK-01 replayed under `sc` by changing `memory_models` | refuses at the first step whose recorded post-state differs |
| MK-06 | SB-02 routed to a candidate advertising only `sc` | `unsupported`, `unsupported_projection`/`unsupported-requested-capability`; never checked under `sc` instead |
| MK-07 | SB-02 routed to a candidate advertising `sc` and `tso` | routed and checked under `tso` |
| MK-08 | a `tso` behaviour with a `flush` step | the `flush` position carries the `memory` anchor and does not advance interval distance |
| MK-09 | TC-390 SB-05 under `ra`, the memory member after `Sx` | `sc-graph` holds the one write event `{"thread":<left>,"kind":"write","ordinal":"0","location":<x>,"write":"0"}`, no edge, no `owed` entry, no summary; `left`'s `cur` view has `at` mapping `x` and `y` to `"0"` and `frontier` mapping `left` to `"1"`; `right`'s `frontier` is empty (FR-437-AC-14) |
| MK-10 | the `ra` counterexample of TC-390 SB-05's protocol to the claim `always holds(not (c1.r = 1 and c2.r = 1))`, unmodified | it replays (FR-439-AC-10) |

## Expected Results

Every replay outcome, refusal and routing decision matches the table.
