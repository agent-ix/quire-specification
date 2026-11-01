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
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-415
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-418
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-416
    type: verifies
---

# TC-372: Task-set schedulability

## Description

Decide schedulability and response bounds by closed-form analysis and recompute the evidence before settlement.

The task-product rows below are normative vectors for the downstream QSL
implementation PR ([FR-419](../functional/analysis/FR-419-analyze-task-set-schedulability.md)
and [FR-364](../functional/temporal/FR-364-replay-temporal-counterexamples.md)).
They are not executable provider trace-tag bindings in this QSpec PR;
behavioral trace tags belong in those implementation tests once they execute
the product and replay semantics.

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
| RT-09 | A task automaton with fixed positive work, finite task types, finite relative deadlines, completion feedback and preemption; one with interval work, completion feedback and preemption | The first lowers to the bounded-subtraction/diagonal product and is decided exactly; the second uses sound bounded exploration, and a finite negative prefix settles `incomplete`, `limit-reached`, never `proved` or `unsupported` |
| RT-10 | Each claim's capability kind | `schedulability` |
| RT-11 | `Ctl` with `priority 2` removed from `t2`, under `CtlFp`, under `T3` and under `CtlEdf` | `invalid_timed_model`/`invalid-task-set` naming `t2`, for `CtlFp` and `T3`; `CtlEdf` admitted and `proved` |
| RT-12 | Tasks `a` (`C = 1`, `T = D = 4`, priority 1) and `b` (`C = 2`, `T = D = 6`, jitter 1, blocking 3, priority 2), fixed priority | `refuted` with task `b`, job 0 and iterates 5, 7 |
| RT-13 | Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2), fixed priority; then evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 | `refuted` with task `b`, job 0 and iterate 2; `inconclusive`, `replay-parity` (job 1's completion 6 does not exceed 6) |
| RT-14 | `Ctl` with a non-positive deadline; a `LO` task with dual budgets; two tasks of equal priority, under `CtlFp`, under `CtlEdf`, under an `amc` claim and under `T3` | `invalid_timed_model`/`invalid-task-set` naming the task; the same; `invalid-task-set` naming both tasks under `CtlFp`, the `amc` claim and `T3`, admitted under `CtlEdf` |
| RT-15 | `taskset Empty { }`; `Ctl` with `t1`'s `bcet -1 ms`; with `bcet 2 ms wcet 1 ms`; with `t3` made `hi bcet 4 ms wcet 2 ms wcet_hi 3 ms`; with `bcet` repeated; with `t1`'s `jitter -1 ms`, with `t2`'s `blocking -1 ms`, with `t3` written `lo wcet_hi 6 ms`; `Ctl` with `t1`'s `blocking 1 ms` and with `t2`'s `jitter 1 ms` under `CtlEdf`; `Ctl` with `t1`'s `deadline 5 ms` under an `amc` claim; `Ctl` with `t1`'s `jitter -1 ms` and `t2`'s `blocking -1 ms` under `CtlEdf` | each malformed BCET/time bound parses then refuses `invalid_timed_model`/`invalid-task-set` naming its task (`t1`, `t1`, then `t3`); repeated `bcet` is `invalid_syntax`/`unexpected-token`; other outcomes remain respectively `invalid_timed_model`/`invalid-task-set` naming `Empty`, `t1`, `t2`, `t3`, `t1`, `t2` and `t1`; for the last, four refusals in the order `t1`, `t2` (task set) then `t1`, `t2` (`edf`) |
| RT-16 | A fixed-work job A released at 0 with `bcet = wcet = 2`/deadline 3/priority 2; job B released at 1 with `bcet = wcet = 1`/deadline 2/priority 1; A executes on [0,1], is preempted on [1,2], resumes on [2,3], and completes | Exact product replay preserves A's service at 1 through preemption, charges B only while scheduled, and completes A exactly at its original absolute deadline; changing A's deadline or slot generation in replay is rejected |
| RT-17 | RT-16 with A's deadline 5/2; the task-product trace reaches A's miss at 5/2 and carries a nonempty positive-delay loop from the miss state that returns to the same source/scheduler control and refined region | FR-364 replays the exact miss prefix and loop, including job provenance, and the checker verifies the loop is repeatable with positive total delay; the item settles `refuted`. A zero-delay loop or a loop returning to a different refined region is rejected and cannot refute |
| RT-18 | A task with exact per-instance work chosen as 3/2 from an admitted interval, completion feedback and preemption; bounded exploration reaches its deadline miss and provides a valid replayable positive-delay loop; then the same item with a search stopped at `max_states=1` | The first exact witness settles `refuted`; the second settles `incomplete`, `limit-reached` and carries `task_search` replaying its explored prefix, complete below-limit successors, first stopped frontier and effective limit. Altering a demand, job identity, edge, frontier, stop reason, configured limit or accounting count is rejected. The bounded evidence never proves absence |
| RT-19 | One HI job has LO demand 1 and a mode-transition increase of exact delta 1/2 while live; its original absolute deadline, priority and job identity are held through the mode change; then replay variants extend its relative deadline, assign another job's slot generation or change the recorded delta | Exact bounded exploration replays the original job with unchanged selected demand and actual remaining work, but an increased allowance and unchanged deadline/priority/identity; each tampered variant is rejected. A bounded negative search is `incomplete`, not `proved` |
| RT-20 | A queue-cap frontier with no derived deadline miss; then a concrete job-specific deadline miss with only a time-locking continuation; then the same missed prefix with a checked positive-delay loop | The frontier is incomplete and not a counterexample; the time-lock-only prefix does not refute and is reported separately by FR-417; the final case may refute only after FR-364 replays the miss and loop and the divergence check verifies positive total delay and exact refined-region closure |
| RT-21 | One task is bound to two source release declarations and shares one activation phase; test a periodic activation exactly at `T`, a sporadic activation at `T-ε`, a periodic source path unable to activate at `T`, and a nonzero jitter `J=1`, selected `j=1`, nominal activation 0, `D=4`, response `R=3` | The exact-period event is admitted, the early sporadic event is disabled, the missing periodic event is a time-lock under FR-417, and jittered readiness at 1 retains absolute deadline 4, matching `R+J=4` |
| RT-22 | A release operation with no task-set match, with two matches, or whose `S/τ` pair names another task; a selected jitter below 0 or above `J`; and two same-task source release operations attempting activations less than `T` apart | Each invalid binding is rejected as `invalid_timed_model`/`invalid-task-set`; invalid jitter is rejected at admission; the too-early sporadic activation is disabled, and all source declarations for the task share the same activation-phase clock |
| RT-23 | A HI job selects `d=C(LO)` and completes at the threshold; one selects `d>C(LO)` and source LO→HI is disabled; one receives an enabled source LO→HI event while active; a job selects `d=0` with positive jitter | The first completes without a budget-change; the second remains at LO allowance and receives no inferred mode change; the third records exact allowance delta `C(HI)-C(LO)` without changing `d`, remaining work, deadline, priority or identity; the zero-demand job completes at nominal activation without service or a miss |
| RT-24 | A `LO` task uses `bcet 2 ms wcet 2 ms`; a `HI` task uses `bcet 2 ms wcet 2 ms wcet_hi 3 ms`; then a `HI` task uses `bcet 2 ms wcet 2 ms wcet_hi 2 ms`; omit `bcet` on a task; repeat `bcet` in one task | The LO task is singleton work 2. The HI task with unequal budgets is a variable-demand/mode-change product and uses bounded exploration; the equal-budget HI task has singleton work 2 and may select the exact fixed-work path; omission normalizes to BCET 0; duplicate `bcet` is `invalid_syntax`/`unexpected-token` |
| RT-25 | One periodic source release for task `t` every `T=1`; `bcet=wcet=2`, `D=1`, fixed priority; job 0 misses at `t=1`; the finite stem completes job 0 over `[1,2]` and admits job 2; `queue_growth` starts at `t=2` with before word `head(job 1, rem=2) · G^(k+1)`; each cycle serves and completes the head over two units, admits exactly jobs `2k+3` at `t=3+2k` and `2k+4` at `t=4+2k`, and returns after two units with `head'` equal to the former first G job and after word `head' · G^(k+2)`; two-index occurrence/deadline formulas make `after(k)=before(k+1)` and all retained identities map injectively | The reader replays the concrete miss and stem, proves by one-cycle word induction that every cycle returns to the same source/clock region, follows the same fixed-priority order, completes one job, admits two, and increases live backlog by exactly one with positive elapsed time; the task claim may settle `refuted` |
| RT-26 | RT-25 with, in turn, zero cycle elapsed, wrong source exit state or clock region, work/service balance changed so the head is not completed or service exceeds one running job, an inversion in `ready_order`, a missing completion-feedback identity, one fresh occurrence reused from a live job, a generation map that merges two live jobs, or `backlog_growth=0` | Each witness is rejected before settlement as `invalid_runtime_input`/`invalid-value`; none establishes a divergent continuation or `refuted` |
| RT-27 | Exact fixed-work product (a) reaches a concrete miss with a finite positive-delay loop; (b) reaches a miss whose continuations are time-locked while another reachable branch remains time-divergent and miss-free; (c) has no miss on any time-divergent behavior; (d) has an internal queue-cap overflow without a replayable miss and continuation witness | (a) `refuted` with the finite loop; (b) FR-417 reports the time-lock and the task item does not refute from it while analysis continues on the divergent branch; (c) `proved` only if the finite product closes; (d) overflow is never a witness and cannot refute; if the finite-product gate does not close, bounded exploration may return replayable `incomplete` |
| RT-28 | A fixed-work task whose finite product closes under the declared live-job cap; then the same task with one periodic activation edge omitted from the supplied certificate; then the same task with a reachable allocation beyond the cap | The complete recomputed product selects the exact fixed-work procedure; the missing successor or cap overflow fails the finite-product gate, leaves the task admitted to bounded exploration, and cannot settle `proved` or `unsupported` |
| RT-29 | Two EDF tasks `a` and `b`, each with fixed work 1, period and relative deadline 4, no declared priority, and simultaneous nominal activation; list `b` before `a` in the source and replay both equal-deadline dispatch orders. Repeat with a task identity that invents a `priority` member. | The positive replay accepts both identities with `priority` omitted and orders equal-deadline jobs by canonical task name (`a` before `b`), then activation transition identity and occurrence, independent of source declaration order. The forged-priority identity is refused as `invalid_runtime_input`/`invalid-value`; no default priority is substituted |
| RT-30 | Two EDF tasks `a` and `b` have equal absolute deadlines, declare priorities 9 and 1 respectively, and `b` is listed first in the source; replay equal-deadline dispatch in both orders. Then make `b`'s absolute deadline earlier and replay with `b` first, followed by a replay omitting a declared priority and one supplying priority to a task with no declaration. | EDF accepts only `a` first for the tie because canonical task name controls the tie, regardless of source order or declared priority; it accepts `b` first when `b`'s deadline is earlier. Replay refuses a missing declared value and an extra value for a task without a declaration as `invalid_runtime_input`/`invalid-value` |

## Expected Results

Every response time, demand, verdict and evidence check is exactly as stated.
