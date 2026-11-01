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
  - target: ix://agent-ix/quire-specification/TC-372
    type: verifies
---

# FR-419: Analyze task-set schedulability

## Description

When a package declares a task set and a schedulability or response claim
over it, the analyzer SHALL decide the claim by the exact closed-form analysis
of the named policy in exact rationals, conditional on the task set's stated
premises, and SHALL settle it only after its evidence is recomputed.

## Inputs

A `taskset` declaration of the shared grammar's timed forms (including the
optional `bcet <time>` task member); a `schedulable` or `response` claim; the
FR-290 kind `schedulability`.

## Outputs

`proved` with basis `closed-form{analysis}` and its evidence; `refuted` with
its evidence; or an inconclusive or unsupported result with its typed cause.

## Behavior

### Task sets

A task set is for one preemptive uniprocessor. Each task declares its
arrival (periodic with period `T`, or sporadic with minimum inter-arrival time
`T`), its relative deadline `D`, its best-case execution time `BCET` and
worst-case execution time `C`, its optional `priority` (a natural number; a smaller value is a higher priority),
and its criticality `LO` or `HI`
with budgets `C(LO) <= C(HI)` for a `HI` task. `BCET` is an optional
non-negative lower bound on actual job demand, written as `bcet <time>` after
`deadline` and before `wcet`; when omitted it is zero. A
`LO` task selects one actual demand `d` in `[BCET,C]`; a `HI` task selects
one actual demand `d` in `[BCET,C(HI)]` at activation, independent of mode.
For a `HI` job, `C(LO)` is only the service allowance before a source-authorized
mode change; it does not change selected demand. Release jitter `J` and blocking term `B` are optional members; a
task that declares neither has `J = 0` and `B = 0`. Every value is an exact rational time
quantity (FR-142). A `LO` task declares one budget `C`. A task set with a
non-positive `T`, `D` or `C`, a negative `BCET`, `J` or `B`, `BCET > C` for a
`LO` task or `BCET > C(HI)` for a `HI` task, a `HI` task with
`C(LO) > C(HI)`, or a `LO` task with dual budgets refuses with
`invalid_timed_model`/`invalid-task-set`, naming the task, and a task set
that declares no task refuses the same way, naming the task set. The shared
grammar admits each of these forms, so they refuse at check time, never as
`invalid_syntax`. A `fixed_priority`, `amc` or `response` claim over a task set
in which two tasks have equal priority refuses with
`invalid_timed_model`/`invalid-task-set`, naming both tasks, and one over a task
set in which a task declares no `priority` refuses with the same code, naming
that task; an `edf` claim reads no priority, so equal or missing priorities do
not refuse it. An `edf` claim over a task with a `J` or `B` other than zero,
and an `amc` claim over a task with `D > T`, refuse with
`invalid_timed_model`/`invalid-task-set`, naming the task, because the
analysis does not read the term or does not cover that deadline. A task set
with several of these defects reports each: the task set's own defects in
source order, then each claim's claim-dependent defects, claims in source
order.

In a checked package each `schedulable` or `response` claim is one
`claim`/`schedulability` node that holds the whole task set
([FR-482](../../objects/interfaces/FR-482-checked-package-schedulability-claim-body.md)),
and its obligation identity binds every premise.

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
stops at the fixpoint or as soon as `R + J_i` exceeds the claim's bound (`D_i`, or `d` for a `response` claim). A deadline
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
`release S.τ on M::o;` declaration, is a task automaton. The frontend lowers
the source model, task set and selected scheduling policy to one finite-product
timed subject. Its discrete state is the source model state paired with the
policy's finite scheduler control and bounded task-instance slots. Its clock
state contains the source clocks and the auxiliary clocks specified by FR-415.
The schedulability item is the timed invariant `always holds(not
deadline_missed)` over that product and uses FR-416's time-divergent behavior
scope. It is settled only after the exact product semantics and any witness
have been replayed under FR-181 and FR-364.

Each release creates one job with a stable identity: the task-set and task
declarations, the source release transition identity, and that release's
zero-based occurrence among transitions with that identity in the replayed
prefix. A job keeps this identity, its task priority, release time and absolute
deadline until completion or removal by an explicitly specified task-set
policy. A scheduler slot is storage for that job, not its identity; moving a
job in the queue or reusing a slot after completion does not rename a live job.
Each source declaration `release S.τ on M::o;` resolves `S` and `τ` to exactly
one task-set declaration in `S`; a missing or ambiguous task binding is an
invalid task automaton. The source operation is that task's **activation**.
All release declarations bound to one task share one activation clock and one
occurrence counter, so adding another source operation cannot bypass the
task's arrival rule. The first activation establishes the task's phase. A
`periodic T` task may activate again only exactly `T` after its previous
activation; a `sporadic T` task may activate again only after at least `T`.
For a periodic task, the product's arrival-clock invariant `x <= T` prevents
time from passing the next required activation; if no bound source operation
can activate it at `x = T`, that path is time-locked and FR-417 reports it.
For a sporadic task, the guard `x >= T` prevents early activation. Both rules
apply to every activation, including zero-time source-operation cycles.

An activation is the job's nominal arrival. Its exact release jitter `j` is
chosen once from `[0,J]`, where absent `jitter` means `J=0`; the instance
becomes ready exactly `j` time units after activation. The deadline clock
starts at nominal activation and its absolute deadline is activation time plus
`D`, matching FR-419's closed-form response bound `R + J`. Jitter delays
availability to the scheduler; it does not shift the deadline. The pending
instance retains its source activation identity and selected jitter, and it
cannot be released twice. This construction is part of the derived subject
and is replayed with its exact delay. Under a schedulable path, a task's live
instances are bounded by the number of nominal activations in the `D` window:
at most `ceil(D/T)+1` for both periodic and sporadic tasks. This bound follows
from the enforced minimum nominal activation spacing `T`; the product does
not rely on WCET as a minimum service amount to bound the queue. It applies to
all live jobs on a no-miss path, including pending jittered releases, because
the absolute deadline remains measured from nominal activation.
If the selected jitter places actual release after the nominal absolute
deadline, the pending job misses at that deadline with its full positive
remaining demand; it receives no service before actual release.
If no finite bound is established for an admitted release mapping,
the analysis stops with replayable bounded-search evidence rather than
truncating or refusing the source behavior.

Release, dispatch, preemption, resumption, task completion, budget change and
deadline miss are derived transitions, each carrying the source/task
provenance and job identity that enables it. Exceeding a finite analysis slot
cap is a search/frontier signal, not a task transition or a counterexample
step. These derived transitions do not claim to be source-model operations.

The product has one processor. At a delay of `d`, the running job receives
exactly `d` service and every waiting or preempted job receives zero service;
all source and auxiliary clocks still advance by `d`. A preemption preserves
the suspended job's accumulated service, priority, identity and absolute
deadline. Completion removes that job and makes the next policy-selected job
eligible to run. A release or scheduler action takes zero time unless its
transition explicitly carries a positive delay. A source completion-feedback
transition is synchronized with the corresponding derived completion step and
retains both transition identities in replay evidence.

The exact fixed-work class applies when the admitted source/product semantics
constrains each job's actual demand to one positive exact value `W_i` (the
singleton case `BCET_i = WCET_i` in the task-automata notation). The task-set
`BCET` supplies a lower bound and `C` (or `C(HI)`) supplies the upper bound;
`BCET = C` for a `LO` task selects its positive singleton directly. The
exact fixed-work route admits a `HI` task only when `BCET = C(LO) = C(HI)`;
mode-change products with unequal budgets remain on bounded exploration. Other existing source/product
constraints may also establish a singleton if the reader proves them for every
job. A WCET `C_i` alone never implies an exact demand. Queue capacity comes
from the task-set activation bound above, not from WCET. An existing sound
reduction may also select an exact finite procedure when its proven policy and
feedback premises cover the admitted model. Otherwise, the analyzer uses
bounded exploration. The exact fixed-work lowering allocates at most
`ceil(D_i / T_i)+1` slots for type `i`, in addition to the scheduler's finite
control. More than that many simultaneously live
instances raises an internal queue-cap frontier. This signal is not itself a
miss or a refutation: the analyzer must derive a concrete job-specific
deadline-miss state from the legal task trace, replay that trace, and check its
time-divergent continuation before returning `refuted`. If it cannot derive
and replay that deadline miss, it returns `incomplete`, never a queue-overflow
counterexample. The fixed-work product is decided by the
bounded-subtraction construction: each task instance has unit-rate service and
deadline clocks; diagonal clock constraints compare service/deadline values;
completion subtracts the completed job's fixed work from suspended-job service
clocks. A subtraction of constant `c` is enabled only in `c <= x <= M_x` and
leaves `x - c >= 0`; FR-415 fixes and validates every clock ceiling `M_x`.
These clocks never have a rate other than one. The resulting finite refined
region graph is the exact decision procedure for this class.

For a task automaton using a task-set member's execution bounds, each activated
`LO` job selects one exact rational total demand `d` in `[BCET, C]`; each
`HI` job selects one exact rational total demand `d` in `[BCET, C(HI)]`. The choice is made once
at nominal activation, is retained across pending jitter, preemption and replay,
and is never resampled. A zero-demand job completes at activation without
processor service and cannot be a deadline miss.
For a `HI` job while the source subject is in `LO` mode, `C(LO)` is its maximum
service allowance; it does not change the job's selected `d`. A `HI` job with
`d > C(LO)` needs the source-authorized mode change before receiving service
beyond that allowance. A `LO` to `HI`
mode change is enabled only by an admitted source transition whose post-state
enters `HI` mode. The task product synchronizes with that transition and then
emits one derived `budget-change` for each active `HI` job, carrying the exact
allowance increase `budget_delta = C(HI) - C(LO)`. It preserves `d`, remaining
actual work, accumulated service, job identity, priority and absolute deadline.
Reaching service `C(LO)` alone never changes the source mode. If `d` equals
the service at that threshold, the job completes and has no budget-change
step. If `d` exceeds the threshold and no `LO` to `HI` source transition is
enabled, the job receives no service beyond `C(LO)` and remains live; the
product continues all other enabled source and scheduler behavior. A later
deadline miss refutes only with the replay evidence below.

An HI task released after the source enters `HI` mode selects from
`[BCET, C(HI)]` and has no budget-change step. Completion occurs only after the
selected demand `d` units of scheduled service; a budget increase never adds
work or resets service. Variable-demand and mode-change products use sound
bounded exploration with the exact rational demands and allowance deltas
selected on each explored path. A finite negative search can settle `proved`
only when its certificate checks an exact abstraction covering every admitted
demand, mode change and scheduler successor; sampled or endpoint-only paths
never establish absence. Otherwise an exhausted configured search bound
settles `incomplete`, `limit-reached`, with the replayable FR-331 task-search
frontier. A live-job or state cap used only to bound exploration is not a
task-queue overflow in the modeled system and cannot refute schedulability.
A concrete miss is a refutation only after FR-364 replays each exact rational
demand, delay, derived scheduler step, source mode transition, allowance update
and missed-deadline state, and FR-418 or fresh exploration verifies a
time-divergent continuation from that state. The fixed-work finite-region
decision procedure is not claimed for interval demand with completion
feedback and preemption; it is undecidable in general (Fersman et al.,
Theorem 5). A budget change has the same rule: unless an exact finite
abstraction covers its demand updates, bounded exploration may refute by an
exact miss witness but cannot prove by a finite negative prefix. The result
names the actual stopped limit and value and carries the replayable frontier.
It never turns a bounded negative search into `proved`
or an undecidable class into a permanent refusal. A time-locking product state
is reported separately by FR-417 and is not a deadline miss unless the claim's
time-divergent scope is witnessed.

The paper's fixed-work proof assumes that the underlying timed automaton has
no time-stops. This product does not inherit that assumption: FR-416 limits the
claim to time-divergent behaviours, FR-417 reports time-locks, and every
deadline-miss counterexample must include a separately checked time-divergent
continuation. A finite-slot cap is only an internal search signal; it is never
the counterexample endpoint. A time-lock-only path or slot-cap signal without
a replayed deadline miss and checked continuation is not a refutation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-419-AC-1 | Task set `Ctl` with `(C, T = D)` of `(1, 4)`, `(2, 6)`, `(5, 12)` ms under fixed priority settles `proved` with response times `1`, `3`, `12` and the third task's fixpoint sequence `5, 9, 12, 12`; under EDF it settles `proved` at utilization exactly 1. | Test |
| FR-419-AC-2 | With `C = 6` for the third task, fixed priority settles `refuted` with the third task, job 0 and iterates 6, 10, 13 (response time 13 above 12); EDF settles `refuted` with utilization `13/12`. | Test |
| FR-419-AC-3 | An AMC set whose high-mode criterion fails settles `inconclusive`, `sufficient-test-failed`; a low-mode miss settles `refuted`. | Test |
| FR-419-AC-4 | `check_closed_form` recomputes the evidence before settlement, and altered evidence settles `inconclusive`, `replay-parity`. | Test |
| FR-419-AC-5 | Changing one stated premise changes the obligation identity; an invalid task set refuses `invalid_timed_model`/`invalid-task-set` naming the task; a `fixed_priority`, `amc` or `response` claim over a task with no `priority`, and a `fixed_priority`, `amc` or `response` claim over two tasks of equal priority, refuse `invalid_timed_model`/`invalid-task-set` naming that task (both tasks for the clash), and an `edf` claim over the same sets is admitted; a non-positive `D` and a `LO` task with dual budgets refuse the same way. | Test |
| FR-419-AC-6 | A fixed-work task automaton lowers to the exact finite bounded-subtraction product and a TT-1 claim; interval demand with completion feedback and preemption is searched soundly, and a negative bounded search settles `incomplete`, `limit-reached`, never `proved` or `unsupported`. | Test |
| FR-419-AC-7 | Tasks `a` (`C = 1`, `T = D = 4`, priority 1) and `b` (`C = 2`, `T = D = 6`, jitter 1, blocking 3, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterates 5, 7 (response time `7 + 1 = 8` above 6). | Test |
| FR-419-AC-8 | Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterate 2. Evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 settles `inconclusive`, `replay-parity`, because job 1's completion 6 does not exceed `2 · 3`. | Test |
| FR-419-AC-9 | `Ctl`'s `response T3 on Ctl.t3 <= 11 ms under fixed_priority` (`t3` the third task) settles `refuted` with the third task, job 0 and iterates 5, 9, 12 (the bound is the claim's `d`, 11); `<= 12 ms` settles `proved`. | Test |
| FR-419-AC-10 | Each of these parses and refuses at check time with `invalid_timed_model`/`invalid-task-set`, never `invalid_syntax`: `taskset Empty { }` (naming `Empty`); a task with `bcet -1 ms`; a task with `bcet 2 ms` and `wcet 1 ms`; a `hi` task with `bcet 4 ms`, `wcet 2 ms`, `wcet_hi 3 ms`; a task with `jitter -1 ms`; a task with `blocking -1 ms`; a `lo` task with `wcet_hi 2 ms`; a task with `blocking 1 ms` under `edf`; a task with `jitter 1 ms` under `edf`; a task with `deadline 5 ms` and `periodic 4 ms` under `amc` (each naming the task). A task set with `t1`'s `jitter -1 ms` and `t2`'s `blocking -1 ms`, under `edf`, reports `t1`'s and `t2`'s task-set defects in that order, then the claim's two `edf` defects in the same order. | Test |
| FR-419-AC-11 | A task with `BCET = C = 2` and deadline 3 runs for 1, is preempted by a higher-priority task with `BCET = C = 1` released at `t=1` until `t=2`, then resumes and completes at `t=3`; changing its deadline to `5/2` yields a replayable miss only when the product's absolute deadline expires. | Test |
| FR-419-AC-12 | For interval execution with feedback and preemption, an exact rational job-demand choice within `[BCET,C]` for a LO task or `[BCET,C(HI)]` for a HI task and product trace that reaches a deadline miss replays and refutes only if a time-divergent continuation is checked; a search stopped before complete proof or replay settles `incomplete`, `limit-reached` with the effective bound. | Test |
| FR-419-AC-13 | At an enabled source `LO` to `HI` transition, each affected live HI job retains its selected demand, identity, priority and absolute deadline; its allowance increases by exactly `C(HI)-C(LO)`, without changing remaining actual work. Reaching the LO threshold alone does not change mode; an incomplete job with no enabled source mode transition remains at its LO allowance, and a job whose demand equals the threshold completes without a budget-change step. | Test |
| FR-419-AC-14 | Queue-cap overflow is an internal frontier signal and never a task-transition identity or counterexample endpoint. It may lead to `refuted` only after the producer derives and FR-364 replays a concrete job-specific deadline miss and checks a positive-delay, repeatable divergent continuation; overflow, a time-lock-only path, or a miss without that continuation does not refute. | Test |
| FR-419-AC-15 | Every task-automaton source release operation resolves to exactly one task-set task; repeated operations cannot bypass shared per-task activation timing. Periodic activations occur exactly every `T`, sporadic activations no sooner than `T`; selected `j∈[0,J]` delays scheduler readiness while the deadline remains nominal activation plus `D`. The model's source guard/invariant can make a required periodic activation impossible, yielding FR-417 time-lock rather than a fabricated activation. Ambiguous/missing binding, an early sporadic event and an out-of-range jitter are rejected; the nonzero-jitter boundary replays consistently with response `R+J`. | Test |

## Dependencies

- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  exact time quantities.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) carries the
  kind `schedulability`.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) supply the
  task-automaton lowering target and its time-divergent claim scope.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) defines the
  task-product state and transition identity; [FR-364](FR-364-replay-temporal-counterexamples.md)
  replays task-miss witnesses; [FR-418](../../objects/interfaces/FR-418-zone-certificate.md)
  checks fixed-work refined-region certificates; [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries bounded-search evidence.

## References

- QSL ADR-026 RT-1 to RT-8, OV-10.
- Owner ruling recorded on QSL-373: native closed-form schedulability; WCET is
  a stated premise. The later QSL-585 task-automaton contract supersedes its
  unsupported disposition for variable-demand and mode-change products: those
  products use sound bounded exploration and may settle `incomplete`.
- M. Joseph and P. Pandya, 1986; N. Audsley et al., 1993; J. Lehoczky, 1990;
  S. Baruah, L. Rosier and R. Howell, 1990; F. Zhang and A. Burns, 2009;
  S. Baruah, A. Burns and R. Davis, 2011; E. Fersman et al., 2007.
- Ticket: STD-139 (paired with QSL-373).
- E. Fersman, P. Krčál, P. Pettersson and W. Yi, “Task Automata: Schedulability,
  Decidability and Undecidability,” *Information and Computation* 205(8), 2007,
  Theorems 2 and 5, DOI 10.1016/j.ic.2007.01.009. The fixed-work result is
  design evidence for the exact class above; Theorem 5 explains why interval
  demand with completion feedback and preemption may require an incomplete
  analysis and does not justify refusal.
