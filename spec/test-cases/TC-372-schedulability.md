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
| RT-03 | `Ctl` with `C = 6` for the third task, fixed priority | `refuted` with the third task, job 0 and iterates 6, 10, 13 |
| RT-04 | RT-03 under EDF | `refuted` with utilization `13/12` |
| RT-05 | `T3`; `T3` with `<= 11 ms` | `proved`; `refuted` with the third task, job 0 and iterates 5, 9, 12 |
| RT-06 | A dual-criticality set whose AMC high-mode criterion fails; one with a low-mode miss | `inconclusive`, `sufficient-test-failed`; `refuted` |
| RT-07 | RT-01's evidence altered in one fixpoint value before settlement | `inconclusive`, `replay-parity` |
| RT-08 | `Ctl` with one WCET changed; a task with `C(LO) > C(HI)` | A different obligation identity; `invalid_timed_model`/`invalid-task-set` naming the task |
| RT-09 | A task automaton with fixed execution times and preemption; one with interval execution times, completion feedback and preemption | Lowered to a TT-1 claim and decided; `unsupported`, `stopwatch-required` |
| RT-10 | Each claim's capability kind | `schedulability` |
| RT-11 | `Ctl` with `priority 2` removed from `t2`, under `CtlFp`, under `T3` and under `CtlEdf` | `invalid_timed_model`/`invalid-task-set` naming `t2`, for `CtlFp` and `T3`; `CtlEdf` admitted and `proved` |
| RT-12 | Tasks `a` (`C = 1`, `T = D = 4`, priority 1) and `b` (`C = 2`, `T = D = 6`, jitter 1, blocking 3, priority 2), fixed priority | `refuted` with task `b`, job 0 and iterates 5, 7 |
| RT-13 | Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2), fixed priority; then evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 | `refuted` with task `b`, job 0 and iterate 2; `inconclusive`, `replay-parity` (job 1's completion 6 does not exceed 6) |
| RT-14 | `Ctl` with a non-positive deadline; a `LO` task with dual budgets; two tasks of equal priority, under `CtlFp`, under `CtlEdf`, under an `amc` claim and under `T3` | `invalid_timed_model`/`invalid-task-set` naming the task; the same; `invalid-task-set` naming both tasks under `CtlFp`, the `amc` claim and `T3`, admitted under `CtlEdf` |
| RT-15 | `taskset Empty { }`; `Ctl` with `t1`'s `jitter -1 ms`, with `t2`'s `blocking -1 ms`, with `t3` written `lo wcet_hi 6 ms`; `Ctl` with `t1`'s `blocking 1 ms` and with `t2`'s `jitter 1 ms` under `CtlEdf`; `Ctl` with `t1`'s `deadline 5 ms` under an `amc` claim; `Ctl` with `t1`'s `jitter -1 ms` and `t2`'s `blocking -1 ms` under `CtlEdf` | each parses; `invalid_timed_model`/`invalid-task-set` naming `Empty`, `t1`, `t2`, `t3`, `t1`, `t2` and `t1`; for the last, four refusals in the order `t1`, `t2` (task set) then `t1`, `t2` (`edf`) |

## Expected Results

Every response time, demand, verdict and evidence check is exactly as stated.
