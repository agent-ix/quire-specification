---
id: FR-364
title: "Carry and replay a temporal counterexample"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-364: Carry and replay a temporal counterexample

## Description

When an analysis refutes a temporal item, over a model subject or over a
supplied trace, the replayer SHALL reproduce the provider's temporal
counterexample, carried in the one wire form below, under the replay rules
below before the item settles as refuted. This requirement owns every arm,
kind and member of that wire form.

## Inputs

A temporal counterexample; the checked package; the item's clause and resolved
fairness set; for the model arm, the model subject's initial states and
universes, re-admitted from the request's byte provision; for the observed
arm, each observed document the counterexample names, held in the request's
byte provision by its zero-based position in `request.provision`.

## Outputs

A replay settlement: `reproduced-with-evaluated-witness` with the item
`refuted`; a located replay refusal; or `inconclusive` with cause
`replay-parity`.

## Behavior

### Shapes

| Refutation | Shape |
| --- | --- |
| Reachable-state invariant, safety, bounded | A finite prefix. Under a bounded profile it covers the formula's horizon from every activation it reports, or ends at a terminal state. Under infinite-trace it is a bad prefix: every infinite extension of it violates the formula. |
| Liveness | A lasso: a prefix, possibly empty, and a non-empty loop that re-enters at its first position. A model behaviour that ends at a terminal state is the lasso whose loop is the one terminal stutter step. |
| Deadlock ([FR-366](FR-366-check-deadlock-freedom.md)) | A finite prefix from an initial state to the deadlocked state. Model arm only. |
| Undefined evaluation ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)) | A finite prefix to the position at which an expression evaluates undefined, for every property form; `loop` is empty. |
| Task-automaton deadline miss ([FR-419](../analysis/FR-419-analyze-task-set-schedulability.md)) | A prefix to a concrete job-specific missed-deadline state, with `trace_position` at that state, followed by exactly one checked continuation: either a finite timed loop with positive total delay, or a `queue_growth` induction witness. Queue-cap overflow is internal and is never a counterexample state. A continuation witness must establish a legal time-divergent extension; a prefix without one does not refute the timed claim. |

### Wire form

A `TemporalCounterexample` is one JSON object with exactly the members of this
table. The table is the one list of its members, arms and kinds: a requirement
that adds a member, an arm or a kind adds it here, and a member, arm or kind
this table does not list refuses before replay.

| Member | Value |
| --- | --- |
| `trace` | the arm: `"model"` for a behaviour of a model subject; `"observed"` for a supplied trace of observed documents |
| `kind` | `"formula"` for a refutation of an authored clause on which the evaluator returns false; `"deadlock"` for a refutation of the deadlock-freedom item (model arm only); `"time-lock"` for a refutation of the time-lock-freedom item ([FR-417](FR-417-settle-time-lock-freedom.md), model arm only); or the undefined-evaluation envelope below |
| `initial_state` | model arm: the zero-based index of the behaviour's initial state in the subject's ordered list of initial states; observed arm: `null` |
| `prefix` | the ordered steps of the prefix |
| `loop` | the ordered steps of the loop; empty for a finite prefix |
| `queue_growth` | present only for a task-automaton deadline-miss counterexample whose divergent continuation has an unbounded live queue; the inductive witness below; absent for every other counterexample and mutually exclusive with a nonempty `loop` |
| `over_binding` | model arm: the reference of the object the clause's `over` parameter is bound to, `null` for `kind: deadlock`; observed arm: `null` |
| `fairness` | the item's resolved fairness set, each member `{kind, operation, granularity}` with `kind` `weak` or `strong` and `granularity` `whole` or `each`, in fairness-set order; empty for `kind: deadlock` |
| `final_delay` | for `kind: time-lock` only (and then required): the exact rational delay that reaches the time-locked state ([FR-417](FR-417-settle-time-lock-freedom.md)); absent for every other kind |
| `refinement_failure` | for a refinement item only: the refinement failure ([FR-379](../analysis/FR-379-report-and-replay-refinement-counterexamples.md)); absent otherwise |
| `trace_position` | for `kind: formula`, the position at which the evaluator returns false; for the undefined-evaluation envelope, the position of its `where`, which is the prefix's last position; `null` for `kind: deadlock` |

The arms are coupled. The replayer SHALL refuse before replay, as
`malformed_wire`, an observed-arm counterexample that has a non-null
`initial_state` or `over_binding`, a model-arm step in its `prefix` or `loop`,
a `kind` of `deadlock` or `time-lock` (which have no observed form), or an
undefined-evaluation envelope whose `where` carries `state_key` or
`state_keys`. The replayer SHALL refuse the same way a model-arm counterexample
that has a null `initial_state`, an observed-arm step (a `document`) in its
`prefix` or `loop`, or an undefined-evaluation envelope whose `where` carries
`document`. A counterexample that fails any of these conditions
is a mix of the arms.

A model-arm step is `{transition, post_state}`: `transition` is the step's
[FR-181](../runtime/FR-181-simulate-finite-executions.md) transition identity,
or `{"type":"terminal-stutter"}` for the terminal stutter step; `post_state` is
the state key of the step's post-state. An
observed-arm step is `{document}`: the zero-based position in `request.provision` of the observed
document at that position. The loop's entry is the position after the
prefix's last step.

For a task-automaton deadline-miss counterexample, the prefix ends at the
concrete missed-deadline product state of a named live job: its absolute
deadline has been reached and its exact remaining demand is positive. An
internal queue-cap overflow is not an endpoint and does not itself refute. If
overflow occurs, the producer must derive an actual missed-deadline state from
the legal task trace and provide that trace and state here. Exactly one of a
finite `loop` or `queue_growth` is then present. The finite loop is a concrete
positive-delay product path whose final state has the same source state and
canonical scheduler control as its entry and whose clock valuations belong to
the same FR-418 refined region. Its exact delays sum to a strictly positive
rational. When a completed slot is reused on the loop, the replayer
alpha-renames the next iteration's job occurrence while preserving task
declaration, slot, generation relation, demand choice and transition
provenance; immutable live-job identities may not be merged.

`queue_growth` is a second, inductive continuation form for a queue that grows
without bound, where a finite full-state loop cannot close. It contains a
finite `stem` from the concrete miss state to the inductive entry, symbolic
queue words `before` and `after`, an exact finite cycle script, the source
control and FR-418 refined clock region at both cycle boundaries, the positive
elapsed time, and the occurrence/generation renaming. A symbolic
queue word is `prefix · growth_block^(k+b) · suffix` for every integer
`k >= 0`, evaluated at cycle `k+c`; `b` is `block_offset` and `c` is
`cycle_offset`. The nonempty `growth_block` is the repeated cohort. Each job
template records its task declaration, occurrence, generation, virtual queue
position and nominal activation/deadline offsets using two-index affine forms
`base + cycle*k + cohort*i`, where outer cycle `k` and repeated-block copy
index `i` are independent natural numbers. Prefix templates use `i=0`, each
repeated-block copy uses its zero-based copy index, and suffix templates use
`i=k+b`. It also records selected fixed
work, remaining work, jitter, optional declared priority, current service allowance and
scheduler status. The script names every release,
dispatch, preemption, resumption, service amount, completion, budget change
and source feedback transition in the cycle; it states the exact symbolic
policy order among all ready template jobs and identifies the concrete missed
job only in the prefix. Each fresh cycle activation carries both its exact
source release-transition identity and its derived task-transition identity.

The witness states a positive integer `iteration_shift = s`. The `before`
word has `cycle_offset=0` and a finite `block_offset=b`; `after` has
`cycle_offset=s` and `block_offset=b+s`. It satisfies
`after(k) = before(k+s)` for every integer `k >= 0`, after applying the
declared identity map and exact two-index affine clock/identity updates. This
is checked as an equality of regular queue words and every per-job field, not
just a cardinality comparison. The checker derives `backlog_growth` as
`s * length(growth_block)`; a supplied value that differs refuses. Every
retained old occurrence maps injectively to one output occurrence with the
same task declaration, exact demand, slot-generation lineage and source
provenance. Each map entry identifies its source and destination template and
affine cycle/cohort mapping with an explicit source cohort index range and
destination function `(k,i) -> (k+s,i+δ)` (or an explicit prefix/suffix
mapping); distinct live inputs cannot map to one output. The union of map
domains covers every retained source occurrence exactly once. A consumed head
has no map only when the cycle contains its exact completion and feedback.
Every newly appended occurrence is outside all map ranges and has one matching
fresh release identity. For the representative overload, `before(k)` is
`head · G^(k+1)`, where `head` and each `G` job have two units of demand. The
finite stem completes the concrete missed job 0 over `[1,2]` and admits job 2,
so induction starts at `t=2` with head job 1 and `G` job 2. Each two-unit
cycle serves and completes its head, admits the next periodic job at `t=3`
and the following one at `t=4`, and makes the old first `G` job the next head.
Thus `after(k)` is `head' · G^(k+2)`, where `head'` is the old first `G` job,
and `after(k) = before(k+1)`. The two-index occurrence/deadline formulas
advance by two source periods per cycle and one period per cohort copy. The
live queue grows by one.

The checker verifies the cycle against the product transition rules and proves
the one-cycle lifting rule for every `k`: source control and refined region
return; each old live occurrence is either preserved by the injective
renaming or removed by its replayed completion; each new occurrence has a
unique task/release identity and generation; each service/completion and
source-feedback edge is enabled in the stated scheduler order; the output word
is exactly the declared `after` transform; and `after` has strictly more live
jobs than `before`. The renaming is injective over every retained live job and
cannot merge two identities. Clock-region and per-template clock updates are
recomputed, including saturation, diagonal guards, bounded subtraction and
nominal deadlines. The exact cycle delays sum to a positive rational. These
checks establish by induction an infinite legal product run with unbounded
queue growth and unbounded elapsed time. A source-only time-progress premise,
a queue-cap overflow, or a finite trace that does not prove this lifting rule
is not a continuation witness. A reader refuses a malformed or non-liftable
certificate before settling `refuted`.

The selected demands, activation jitter and every mode-change allowance delta
in the represented prefix and cycle are carried by their task-transition
identities and checked as exact rationals. The source activation is checked
against the bound task's periodic or sporadic inter-arrival rule; actual
release is exactly the selected jitter after nominal activation, and the
absolute deadline remains nominal activation plus `D`. Each `LO` job's
selected `d` is in `[BCET,C]`; each `HI` job's selected `d` is in
`[BCET,C(HI)]`. A `HI` job has LO-mode allowance `C(LO)` until an admitted
source transition enters HI mode. Reaching service `C(LO)` alone does not
change mode. A completed job at the threshold has no update; an incomplete
job with no enabled LO-to-HI source transition remains at that allowance. A
source-authorized `budget-change` carries exactly
`budget_delta = C(HI)-C(LO)`, leaves actual `d` and remaining work unchanged,
and preserves job identity, optional declared priority and deadline.

For a task-derived timed subject, a model-arm `transition` may be the typed
`task-transition` identity defined by FR-181 and FR-331. Its `action` names an
activation, release, dispatch, preemption, resumption, completion, budget change
or deadline-miss step. A finite queue-cap overflow has no task-transition identity and
may appear only as a `queue` frontier in bounded-search evidence; it is
refused as a counterexample step. Each task identity carries the task-set/task
declarations, job sequence, slot and generation, the nominal source activation,
the job's exact selected rational demand and jitter and absolute deadline,
plus `priority` exactly when its checked task declaration has one; absent
priority is omitted, never synthesized. A supplied priority must equal the
checked task declaration, and a missing or extra priority is a replay refusal.
Replay derives the scheduling policy from the bound checked claim. Under
fixed priority it recomputes ready order by ascending numeric priority. Under
EDF it recomputes ready order by absolute deadline, with ties by ascending
canonical task name in Unicode scalar value order (FR-482), then canonical
source activation transition identity and zero-based occurrence. Source
declaration order and any declared task priority do not affect EDF ties.
Declared priority is provenance only and is ignored for EDF ordering. The triggering source or scheduler
transition is also checked. A
`budget-change` identity also carries the exact non-negative `budget_delta`;
it does not alter `demand`.
All steps for one live job repeat equal immutable identity fields; replay
refuses a change to any of them. A source activation and its bound task's
activation are checked as one derived transition; subsequent actual release
is checked at the selected jitter delay. A completion-feedback step checks
the derived completion and the source transition it enables.

The **undefined-evaluation envelope** is the one shape of an undefined
evaluation for every engine:
`{"type": "undefined-evaluation", "where": {...}, "cause": <reason>}`. `cause`
is the undefined value's catalogued reason. `where` names the evaluation point
with these key names:

| Key | Value |
| --- | --- |
| `position` | the trace position of the evaluation point |
| `state_key` | model arm: the state key of the state there |
| `document` | observed arm: the `U64String` position in `request.provision` of the observed document there |
| `locus` | the source locus of the expression that has no value |
| `transition` | for an undefined guard only: the FR-181 transition identity whose guard it is |
| `predicate` | for a predicate of a state-graph claim only: `"P"`, `"Q"`, `"X"` or `"Y"`, the predicate that has no value ([FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md)) |
| `variables` | for a hyperproperty only: the trace or execution variables with their aliases, `[{"variable":"<ident>","alias":"<ident>"}, ...]`, in the order of the plural keys ([FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)) |

An engine whose evaluation point is not one position names it with the same
keys in their plural forms (`positions`, `state_keys`), and its own
requirement states which keys its `where` carries.

A counterexample travels as an
[FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`counterexamples` entry whose identity binds the item's subject or supplied
trace. Every engine returns this one type. An engine reads each step from its
own encoding and names the initial state by index, so its counterexample
replays with no engine present.

### Replay

The replayer SHALL, in this order:

1. Re-admit the checked package. Model arm: re-admit the subject's initial
   states and universes, and select initial state `initial_state`. Observed
   arm: admit each step's document from the byte provision; a position the
   provision does not hold refuses `invalid_runtime_input`/`invalid-value`.
2. Model arm: re-execute each step of the prefix, then of the loop, through
   the successor relation of
   [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md). A step's
   `post_state` selects which successor of its transition identity the step
   took, since one identity can have several post-states. A step whose
   transition identity is not enabled at its pre-state refuses
   `invalid_runtime_input`/`invalid-value`. A terminal stutter step at a state
   that is not terminal refuses `invalid_runtime_input`/`invalid-value`.
   For a task-derived subject, use FR-419's product successor relation: replay
   the source transition and its task release/completion feedback, verify the
   selected demand is within the admitted interval, verify policy order and
   job slot generation, apply exact bounded subtraction and source-authorized
   budget allowance updates, and compare the full source-plus-scheduler
   post-state key. Every
   subtraction must satisfy its declared `c <= x <= M_x` side condition and
   leave a non-negative clock. A deadline-miss refutation also requires the
   exact time-divergent continuation checked by FR-418; a queue-cap overflow
   is never a counterexample endpoint, and a missed-deadline state without
   such a continuation does not refute the timed claim. When `queue_growth` is
   present, replay its `stem` immediately after the prefix and require its last
   product state to equal the witness's `source_entry`, `region_entry` and
   `before(0)` queue valuation exactly.
   Observed arm: the trace is the steps' documents in order.
3. For a `queue_growth` witness, require an empty finite `loop` and check its
   source entry and exit state and DBM are equal, its DBM clock order and
   matrix dimensions are canonical, and `elapsed` equals the exact sum of its
   cycle delays and is strictly positive. Instantiate the queue words at
   `k=0`, execute every cycle step using the task-product transition relation,
   and check every ready-order word against the policy derived from the bound
   claim: fixed-priority ordering by checked priority, or EDF ordering by
   absolute deadline and the FR-419 deterministic tie rule. Verify that all releases obey their task's
   inter-arrival and jitter constraints; each service amount is charged only
   to the selected running job; completions and feedback cite the exact
   enabling identities; deadlines, demands, allowances, bounded subtractions
   and refined clock regions agree with the source and FR-415 semantics. Then
   check the symbolic lifting obligations: every retained job has exactly one
   destination under the injective occurrence/generation map; every removed
   job is completed in the script; every fresh job is a legal release with a
   fresh identity; the transformed output word equals `before(k+iteration_shift)`
   after the injective identity map for every `k >= 0`; and the cycle's
   source/clock-region and per-template affine clock updates preserve this
   recurrence. It derives the positive backlog growth from the iteration shift
   and growth-block length. The checker must prove the cycle script's guards, policy choice,
   feedback and queue transform by the stated word induction, not by sampling
   a finite number of repetitions. Any failed obligation refuses
   `invalid_runtime_input`/`invalid-value`; a valid witness denotes an infinite
   time-divergent continuation. It does not by itself replay or replace the
   finite concrete miss prefix.
4. If `queue_growth` is absent, for an ordinary lasso check that the loop's
   last step returns to the loop's entry (model arm: its post-state is the
   entry state; observed arm: its document is the entry document), else refuse
   `invalid_runtime_input`/`invalid-value`. For a task-automaton deadline-miss
   lasso, check instead that the final source-plus-scheduler control equals
   the loop entry modulo canonical live-slot alpha-renaming, the exact clock
   valuations occupy the same FR-418 refined region, and the exact sum of
   loop delays is positive. A zero-delay loop, a non-equivalent region, a
   changed live job's task/demand/deadline/declared priority, or a stale slot generation
   refuses `invalid_runtime_input`/`invalid-value`.
4. Fairness. Model arm, for a lasso: recompute the enabled transition
   identities at every loop state from the model, and check every constraint
   of `fairness` against the loop by
   [FR-362](FR-362-apply-fairness-constraints.md): weak and strong each by its
   own lasso rule. A loop that fails a constraint is an unfair lasso and
   refuses `invalid_runtime_input`/`invalid-value`, naming the constraint.
   Observed arm: a non-empty `fairness` settles `unsupported`, cause
   `unsupported_projection`/`missing-fairness-premise`, naming its first
   constraint, with no formula evaluated, since an observed trace carries no
   enabledness (FR-362).
5. For `kind: formula`, evaluate the
   clause's formula over the behaviour with the trace evaluator. A lasso
   denotes the infinite trace that repeats its loop forever: a position past
   the represented steps reads the position congruent to it modulo the loop
   length, and past operators read the denoted trace back to position 0.
   Evaluation false at `trace_position` agrees with the engine; any other
   result disagrees.
6. For the undefined-evaluation envelope, evaluate at `where` the expression
   its `locus` names: the clause's formula over the prefix for a claim, the
   guard of `where.transition` at the last state for a guard, the
   `terminal when` predicate at the last state, or the state constraint at
   the last state, the state `where.state_key` names, for a state constraint.
   The undefined value at
   `trace_position`, with the envelope's own `locus` and `cause`, agrees with
   the engine. Replay reads both from the counterexample, never from the item.
   A defined value there, or an undefined value at another position,
   expression or reason, disagrees.
7. For `kind: deadlock`, at the last state enumerate the enabled transition
   identities and, when the model declares `terminal when P`, evaluate `P`.
   No enabled identity and `P` false (or no `terminal` declaration) agrees
   with the engine; an enabled identity or `P` true disagrees.

Agreement settles `reproduced-with-evaluated-witness` and the item `refuted`.
Disagreement settles the item `inconclusive`, cause `replay-parity`. A refusal
settles no `refuted`.

### Validity, not identity

A conformance vector checks that a returned counterexample reproduces and has
the expected `kind` and shape. It never checks which of several valid
counterexamples an engine returned.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-364-AC-1 | A liveness refutation is a lasso with a non-empty loop, a safety refutation a finite prefix with an empty `loop`, a terminated behaviour's refutation a lasso whose loop is the one terminal stutter step, and a deadlock refutation a finite prefix ending at the deadlocked state with `kind: deadlock`. | Test (TC-295, TC-297) |
| FR-364-AC-2 | A counterexample carries exactly the members `trace`, `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness` and `trace_position`, plus the required `final_delay` of a `time-lock` counterexample, `refinement_failure` of a refinement item, or `queue_growth` of an unbounded task-miss continuation; each model-arm step carries its FR-181 transition identity or the terminal-stutter identity and its post-state key; each observed-arm step carries its document position; each fairness member carries its kind, operation and granularity. A missing, extra or mistyped member, or an arm or kind the wire-form table does not list, refuses before replay. | Test |
| FR-364-AC-3 | A valid formula counterexample replays through the model, passes the fairness check and evaluates false at `trace_position`, settling `reproduced-with-evaluated-witness` and the item `refuted`. | Test (TC-293, TC-295) |
| FR-364-AC-4 | A loop whose last post-state is not its entry state, a step whose transition is not enabled, and a terminal stutter step at a non-terminal state refuse `invalid_runtime_input`/`invalid-value`. | Test (TC-295) |
| FR-364-AC-5 | A lasso that fails a weak or a strong constraint of its fairness set, with enabledness recomputed from the model, refuses as an unfair lasso naming the constraint and settles no `refuted`. | Test (TC-295, TC-299) |
| FR-364-AC-6 | A counterexample on which the evaluator returns true at `trace_position` settles `inconclusive`, cause `replay-parity`; a deadlock counterexample whose last state enables a transition or satisfies `P` settles the same. | Test (TC-295, TC-297) |
| FR-364-AC-7 | A lasso whose formula reads a past operator across the loop boundary evaluates by the infinite trace the lasso denotes. | Test (TC-200) |
| FR-364-AC-8 | A counterexample of an undefined evaluation is a finite prefix with an empty `loop`, a `kind` that is the envelope `{"type": "undefined-evaluation", "where", "cause"}` and `trace_position` at its last position; its replay re-evaluates the claim, guard, `terminal when` predicate or state constraint the locus names at the reported state and reproduces the undefined value with the locus and reason the envelope carries, read from the counterexample alone, settling `reproduced-with-evaluated-witness` and the item `refuted` with cause `undefined-evaluation`; a `trace_position` at which the expression is defined settles `inconclusive`, cause `replay-parity`. | Test (TC-305) |
| FR-364-AC-9 | An observed-arm formula counterexample whose documents are held by the byte provision, with an empty `fairness`, replays over the documents' trace and settles `reproduced-with-evaluated-witness` when the formula is false at `trace_position`; the same counterexample with a non-empty `fairness` settles `unsupported`, cause `missing-fairness-premise`, with no formula evaluated; a step position the provision does not hold refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-295) |
| FR-364-AC-10 | The wire schema admits an observed-arm formula counterexample (null `initial_state` and `over_binding`, two `document` steps) and an observed-arm undefined-evaluation envelope whose `where` carries `position`, `document` and `locus`; it refuses an observed-arm counterexample that names an initial state, carries an `over_binding`, carries a model-arm step in its `prefix` or in its `loop`, has `kind` `deadlock`, has `kind` `time-lock` with its `final_delay`, or has an envelope whose `where` carries `state_key` or `state_keys`; and it refuses a model-arm counterexample that has a null `initial_state`, carries a `document` step in its `prefix` or in its `loop`, or has an envelope whose `where` carries `document`. | Test (TC-431) |
| FR-364-AC-14 | The backend-provider schema admits the typed RT-25 `queue_growth` wire vector and refuses omission of `iteration_shift`, zero `elapsed`, and a simultaneous nonempty `loop`; the vector test recomputes the two-unit cycle delay, work balance, positive backlog increment, and verifies `after(k)=before(k+1)` by affine queue-word expansion for the authored index range. These are wire/vector checks only; the task replay and queue-growth behavior above remain normative and require behavioral verification in the downstream QSL implementation. | Test |
| FR-364-AC-15 | Task-transition replay admits omitted priority exactly when the bound task declaration has no priority, admits a supplied value only when it equals the declaration, and refuses a missing or forged value otherwise; replay recomputes EDF order from checked absolute deadlines and the deterministic canonical-task-name/source-transition/occurrence tie rule, with source declaration order and declared fixed priority ignored for EDF ties, and recomputes fixed-priority order from the checked declared values. | Test (TC-372) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  transition identity and the state key.
- [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md) owns the
  successor relation and the terminal stutter step.
- [FR-362](FR-362-apply-fairness-constraints.md) owns the lasso fairness rules.
- [FR-365](FR-365-build-canonical-temporal-counterexample.md) owns the
  canonical counterexample of an exhaustive explicit-state analysis.

## References

- quire-spec-language ADR-018 CX-1 to CX-4, SM-7, SM-8, DL-4, DL-5 and QS-8;
  ADR-019 SR-8, SV-4 and QS-5, QS-6; Linear STD-131 and STD-132; STD-135 and
  STD-136 (the `predicate` and `variables` keys).
