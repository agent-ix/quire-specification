---
id: TC-372
title: "Task-set schedulability"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-419
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-372: Task-set schedulability

## Description

Decide schedulability and response bounds by closed-form analysis and recompute the evidence before settlement.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Task set `Ctl`, one preemptive uniprocessor:

```text
taskset Ctl {
  task t1 periodic 4 ms deadline 4 ms wcet 1 ms priority 1 lo;
  task t2 periodic 6 ms deadline 6 ms wcet 2 ms priority 2 lo;
  task t3 periodic 12 ms deadline 12 ms wcet 5 ms priority 3 lo;
}
```

`CtlFp` is `schedulable CtlFp on Ctl under fixed_priority;`, `CtlEdf` is `schedulable CtlEdf on Ctl under edf;` and `T3` is `response T3 on Ctl.t3 <= 12 ms under fixed_priority;`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RT-01 | `CtlFp` | `proved`, `closed-form{fixed-priority}`, response times `1`, `3`, `12` ms; the third task's fixpoint sequence `5, 9, 12, 12` |
| RT-02 | `CtlEdf` | `proved`; utilization exactly 1 |
| RT-03 | `Ctl` with `C = 6` for the third task, fixed priority | `refuted` with scheduling points 4, 6, 8, 12 and demands 9, 10, 12, 13 |
| RT-04 | RT-03 under EDF | `refuted` with utilization `13/12` |
| RT-05 | `T3`; `T3` with `<= 11 ms` | `proved`; `refuted` |
| RT-06 | A dual-criticality set whose AMC high-mode criterion fails; one with a low-mode miss | `inconclusive`, `sufficient-test-failed`; `refuted` |
| RT-07 | RT-01's evidence altered in one fixpoint value before settlement | `inconclusive`, `replay-parity` |
| RT-08 | `Ctl` with one WCET changed; a task with `C(LO) > C(HI)` | A different obligation identity; `invalid_timed_model`/`invalid-task-set` naming the task |
| RT-09 | A task automaton with fixed execution times and preemption; one with interval execution times, completion feedback and preemption | Lowered to a TT-1 claim and decided; `unsupported`, `stopwatch-required` |
| RT-10 | Each claim's capability kind | `schedulability` |
| RT-11 | `Ctl` with `priority 2` removed from `t2`, under `CtlFp` and under `CtlEdf` | `invalid_timed_model`/`invalid-task-set` naming `t2`; `CtlEdf` admitted and `proved` |

## Expected Results

Every response time, demand, verdict and evidence check is exactly as stated.
