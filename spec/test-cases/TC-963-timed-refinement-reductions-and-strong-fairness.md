---
id: TC-963
title: "Timed refinement, reductions over dense time and strong fairness over time"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-382
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-416
    type: verifies
---

# TC-963: Timed refinement, reductions over dense time and strong fairness over time

## Description

Check a refinement whose concrete model is timed, the refusals of a timed
abstract model and of a mapping that reads a clock, symmetry that moves clocks
with their objects, partial-order reduction and a state constraint over a
`time dense` subject settling `unsupported` at negotiation, and strong
fairness read at every instant.

## Test Procedure

Check each vector through the real local Rust boundary of the provider named
by the requirement, and compare the complete typed result (verdict, cause,
counterexample and refusal locus) with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| TR-01 | FR-377-AC-14's `TReq` refining `Req` | Safety half `proved`; no step-function call for a delay |
| TR-02 | TR-01 with mapping `sent -> (x >= 1)`; a refinement whose abstract model is `TReq` and concrete model a copy of `TReq` | `unsupported_construct`/`expression-form` at the read of `x`; `unsupported_construct`/`declaration-form` at the abstract model reference |
| TR-03 | FR-377-AC-15's `TReq` with `trap`, `enter` and `undo`; then with `when self.trap { x <= 0 }` | `refuted`, counterexample `send` (delay 1), `enter` (delay 1), `undo` (delay 0) failing at position 3, which replays; then `proved`, and the model's time-lock-freedom item refuted |
| TR-04 | FR-382-AC-7's `Timer` with symmetry `[[t1, t2]]`: deadlock freedom; `always holds(not (t1.armed and t2.armed))` | `proved` with fewer stored symbolic states than unreduced; `refuted` as unreduced, its counterexample replaying |
| TR-05 | The `Timer` symbolic state with `t1` armed, `t2` unarmed and zone `Z`, and its image with the two objects and their clocks swapped | Equal canonicalised states |
| TR-06 | `Timer`'s deadlock-freedom item selecting partial-order reduction; the same selecting a state constraint | Each `unsupported`, `unsupported_projection`/`unsupported-requested-capability` at negotiation naming the reduction, no state expanded |
| TR-07 | FR-416-AC-9's `Blink` with `eventually holds(b.fired)` under `fair { weak Blink::fire }`, then `fair { strong Blink::fire }` | `refuted` with a lasso of `tick` steps of loop delay 1; then `proved` with an accepted zone certificate |

## Expected Results

Every verdict, refusal and counterexample is exactly as stated.
