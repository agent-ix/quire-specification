---
id: FR-419
title: "Analyze task-set schedulability"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
---

# FR-419: Analyze task-set schedulability

## Description

When a package declares a task set and a schedulability or response claim
over it, the analyzer SHALL decide the claim by the exact closed-form analysis
of the named policy in exact rationals, conditional on the task set's stated
premises, and SHALL settle it only after its evidence is recomputed.

## Inputs

A `taskset` declaration of the shared grammar's timed forms; a `schedulable`
or `response` claim; the FR-290 kind `schedulability`.

## Outputs

`proved` with basis `closed-form{analysis}` and its evidence; `refuted` with
its evidence; or an inconclusive or unsupported result with its typed cause.

## Behavior

### Task sets

A task set is for one preemptive uniprocessor. Each task declares its
arrival (periodic with period `T`, or sporadic with minimum inter-arrival time
`T`), its relative deadline `D`, its worst-case execution time `C`, its
optional `priority` (a natural number; a smaller value is a higher priority),
and its criticality `LO` or `HI`
with budgets `C(LO) <= C(HI)` for a `HI` task. Release jitter `J` and a
blocking term `B` are optional members; a task that declares neither has
`J = 0` and `B = 0`. Every value is an exact rational time
quantity (FR-142). A `LO` task declares one budget `C`. A task set with a
non-positive `T`, `D` or `C`, a `HI` task with `C(LO) > C(HI)`, or a `LO` task
with dual budgets refuses with `invalid_timed_model`/`invalid-task-set`,
naming the task. A `fixed_priority`, `amc` or `response` claim over a task set
in which two tasks have equal priority refuses with
`invalid_timed_model`/`invalid-task-set`, naming both tasks, and one over a task
set in which a task declares no `priority` refuses with the same code, naming
that task; an `edf` claim reads no priority, so equal or missing priorities do
not refuse it.

### Premises

`C`, `C(LO)`, `C(HI)`, `B` and `J` are stated premises. They are part of the
task set, so part of the subject and its obligation identity, and a verdict
holds for exactly those values.

### Claims

`schedulable N on S under fixed_priority`, `under edf` or `under amc`, and
`response N on S.τ <= d under fixed_priority`, `N` the claim's name and `S`
the task set, each requiring the FR-290 kind `schedulability`. The proof basis
`closed-form{analysis}` names the analysis as `fixed-priority`, `edf` or
`amc-rtb` (FR-331).

### Fixed priority

Each task's worst-case response time is the least fixpoint of
`R = C_i + B_i + Σ_{j ∈ hp(i)} ⌈(R + J_j) / T_j⌉ · C_j`, iterated from
`C_i + B_i` in exact rationals; the response time is `R + J_i`. The iteration
stops at the fixpoint or as soon as `R + J_i` exceeds `D_i`. A deadline
beyond the period uses the busy-period extension over the jobs of the level-`i`
busy period: job `q` of that busy period (0 when `D_i <= T_i`, whose busy
period ends with the first job unless that job misses) iterates
`R = (q + 1) · C_i + B_i + Σ_{j ∈ hp(i)} ⌈(R + J_j) / T_j⌉ · C_j` from
`(q + 1) · C_i + B_i`, and its response time is `R + J_i − q · T_i`. A miss
is the task and the job `q` whose iterates reach a response time above the
bound, which is `D_i` for a `schedulable` claim and `d` for a
`response N on S.τ <= d` claim. For `q > 0`, every earlier job `p < q` keeps
the busy period open: its completion `w_p`, the fixpoint of the same
recurrence for job `p`, satisfies `w_p > (p + 1) · T_i − J_i` (strictly);
otherwise job `q` lies past the busy period and is not a miss. The iterates
rise monotonically to the fixpoint, so the first iterate above the bound
proves the miss, with jitter, blocking and `D_i > T_i` included.

### EDF

Utilization above 1 fails. Otherwise the demand bound
`dbf(t) = Σ_i max(0, ⌊(t − D_i) / T_i⌋ + 1) · C_i` must not exceed `t` at any
absolute deadline up to the synchronous busy-period length, checked by Quick
Processor-demand Analysis.

### Mixed criticality (AMC-rtb)

Low-mode response times with `C(LO)`; high-mode response times with `C(HI)`
over higher-priority `HI` tasks; and the mode-change bound that charges
higher-priority `LO` tasks only up to the low-mode response time. A low-mode
miss is a miss and refutes. AMC-rtb is sufficient: a high-mode criterion that
fails settles `inconclusive`, `sufficient-test-failed`.

### Verdicts and evidence

| Outcome | FR-331 value | Evidence |
| --- | --- | --- |
| Schedulable, or response bound met | `proved`, basis `closed-scope`, proof basis `closed-form{analysis}` | Each job's fixpoint sequence of each task (fixed priority, AMC; one job when `D_i <= T_i`); the busy-period length and QPA's check sequence (EDF) |
| Miss | `refuted`, basis `decisive-counterexample` | Fixed priority: the task, the job `q` of its level-`i` busy period that misses, and that job's recurrence iterates up to the first whose response time exceeds the bound; EDF: a `t` with `dbf(t) > t`, or the utilization above 1 |
| AMC high-mode criterion fails | `inconclusive` | `sufficient-test-failed` |

`check_closed_form` recomputes the evidence in exact rationals before the
verdict settles; a disagreement settles `inconclusive`, `replay-parity`.

### Task automata

A timed model whose operations release tasks of a task set, each by a
`release S.τ on M::o;` declaration, is a task automaton. In the decidable class (every combination except interval
execution times together with completion feedback and preemption), the
frontend lowers the model and policy to a timed subject and the claim "no task
misses its deadline" to a TT-1 claim decided by [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md);
the queue length the lowering needs is derived from the task set. A model in
the other class settles `unsupported`, `stopwatch-required`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-419-AC-1 | Task set `Ctl` with `(C, T = D)` of `(1, 4)`, `(2, 6)`, `(5, 12)` ms under fixed priority settles `proved` with response times `1`, `3`, `12` and the third task's fixpoint sequence `5, 9, 12, 12`; under EDF it settles `proved` at utilization exactly 1. | Test (TC-372) |
| FR-419-AC-2 | With `C = 6` for the third task, fixed priority settles `refuted` with the third task, job 0 and iterates 6, 10, 13 (response time 13 above 12); EDF settles `refuted` with utilization `13/12`. | Test (TC-372) |
| FR-419-AC-3 | An AMC set whose high-mode criterion fails settles `inconclusive`, `sufficient-test-failed`; a low-mode miss settles `refuted`. | Test (TC-372) |
| FR-419-AC-4 | `check_closed_form` recomputes the evidence before settlement, and altered evidence settles `inconclusive`, `replay-parity`. | Test (TC-372) |
| FR-419-AC-5 | Changing one stated premise changes the obligation identity; an invalid task set refuses `invalid_timed_model`/`invalid-task-set` naming the task; a `fixed_priority` or `amc` claim over a task with no `priority`, and a `fixed_priority` claim over two tasks of equal priority, refuse `invalid_timed_model`/`invalid-task-set` naming that task (both tasks for the clash), and an `edf` claim over the same sets is admitted; a non-positive `D` and a `LO` task with dual budgets refuse the same way. | Test (TC-372) |
| FR-419-AC-6 | A task automaton in the decidable class lowers to a TT-1 claim; one needing stopwatches settles `unsupported`, `stopwatch-required`. | Test (TC-372) |
| FR-419-AC-7 | Tasks `a` (`C = 1`, `T = D = 4`, priority 1) and `b` (`C = 2`, `T = D = 6`, jitter 1, blocking 3, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterates 5, 7 (response time `7 + 1 = 8` above 6). | Test (TC-372) |
| FR-419-AC-8 | Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterate 2. Evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 settles `inconclusive`, `replay-parity`, because job 1's completion 6 does not exceed `2 · 3`. | Test (TC-372) |
| FR-419-AC-9 | `Ctl`'s `response T3 <= 11 ms` settles `refuted` with the third task, job 0 and iterates 5, 9, 12 (the bound is the claim's `d`, 11); `<= 12 ms` settles `proved`. | Test (TC-372) |

## Dependencies

- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  exact time quantities.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) carries the
  kind `schedulability`.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) supply the
  task-automaton lowering target.

## References

- QSL ADR-026 RT-1 to RT-8, OV-10.
- Owner ruling recorded on QSL-373: native closed-form schedulability; WCET is
  a stated premise; undecidable task-automaton cases settle unsupported.
- M. Joseph and P. Pandya, 1986; N. Audsley et al., 1993; J. Lehoczky, 1990;
  S. Baruah, L. Rosier and R. Howell, 1990; F. Zhang and A. Burns, 2009;
  S. Baruah, A. Burns and R. Davis, 2011; E. Fersman et al., 2007.
- Ticket: STD-139 (paired with QSL-373).
