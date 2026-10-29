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
| `over_binding` | model arm: the reference of the object the clause's `over` parameter is bound to, `null` for `kind: deadlock`; observed arm: `null` |
| `fairness` | the item's resolved fairness set, each member `{kind, operation, granularity}` with `kind` `weak` or `strong` and `granularity` `whole` or `each`, in fairness-set order; empty for `kind: deadlock` |
| `interval` | for `kind: formula`, the interval of the formula's first interval-carrying operator in pre-order, in the form [FR-370](../../objects/interfaces/FR-370-checked-package-temporal-clause-body.md) encodes it under the clause's profile: the integer form `{lower, upper}` for a closed interval `[a,b]` or `{lower, upper: null}` for a lower-bounded interval `[a,*]`, two non-negative decimal integer strings; or, under `quire.temporal.timed/v1`, the timed form `{lower, upper, lower_end, upper_end}`, where `lower` and `upper` are exact non-negative rationals `{numerator, denominator}` in lowest terms and `lower_end` and `upper_end` are each `closed` or `open`; `null` when the formula has no interval-carrying operator, for `kind: deadlock` and for the undefined-evaluation envelope |
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
   `invalid_runtime_input`/`invalid-value`. A step for which no successor of
   its transition identity has the recorded `post_state` refuses
   `stale_dependency`/`content-mismatch`. A terminal stutter step at a state
   that is not terminal refuses `invalid_runtime_input`/`invalid-value`.
   Observed arm: the trace is the steps' documents in order.
3. For a lasso, check that the loop's last step returns to the loop's entry
   (model arm: its post-state is the entry state; observed arm: its document
   is the entry document), else refuse `invalid_runtime_input`/`invalid-value`.
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
5. For `kind: formula`, compute the interval of the re-admitted clause's
   first interval-carrying operator in pre-order, in the same shape
   (`{lower, upper}`, `{lower, upper: null}`,
   `{lower, upper, lower_end, upper_end}` or `null`), and check that
   `interval` equals it member by member, so a counterexample produced for
   another interval of the same clause is refused; else refuse
   `stale_dependency`/`content-mismatch` naming `interval`. Then evaluate the
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
| FR-364-AC-2 | A counterexample carries exactly the members `trace`, `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness`, `interval` and `trace_position`, plus the required `final_delay` of a `time-lock` counterexample and the `refinement_failure` of a refinement item; each model-arm step carries its FR-181 transition identity or the terminal-stutter identity and its post-state key; each observed-arm step carries its document position; each fairness member carries its kind, operation and granularity. A missing, extra or mistyped member, or an arm or kind the wire-form table does not list, refuses before replay. | Test (TC-295) |
| FR-364-AC-3 | A valid formula counterexample replays through the model, passes the fairness check and evaluates false at `trace_position`, settling `reproduced-with-evaluated-witness` and the item `refuted`. | Test (TC-293, TC-295) |
| FR-364-AC-4 | A loop whose last post-state is not its entry state, a step whose transition is not enabled, and a terminal stutter step at a non-terminal state refuse `invalid_runtime_input`/`invalid-value`; a step whose recorded post-state key matches no successor refuses `stale_dependency`/`content-mismatch`. | Test (TC-295) |
| FR-364-AC-5 | A lasso that fails a weak or a strong constraint of its fairness set, with enabledness recomputed from the model, refuses as an unfair lasso naming the constraint and settles no `refuted`. | Test (TC-295, TC-299) |
| FR-364-AC-6 | A counterexample on which the evaluator returns true at `trace_position` settles `inconclusive`, cause `replay-parity`; a deadlock counterexample whose last state enables a transition or satisfies `P` settles the same. | Test (TC-295, TC-297) |
| FR-364-AC-7 | A lasso whose formula reads a past operator across the loop boundary evaluates by the infinite trace the lasso denotes. | Test (TC-200) |
| FR-364-AC-8 | A counterexample of an undefined evaluation is a finite prefix with an empty `loop`, a `kind` that is the envelope `{"type": "undefined-evaluation", "where", "cause"}` and `trace_position` at its last position; its replay re-evaluates the claim, guard, `terminal when` predicate or state constraint the locus names at the reported state and reproduces the undefined value with the locus and reason the envelope carries, read from the counterexample alone, settling `reproduced-with-evaluated-witness` and the item `refuted` with cause `undefined-evaluation`; a `trace_position` at which the expression is defined settles `inconclusive`, cause `replay-parity`. | Test (TC-305) |
| FR-364-AC-9 | An observed-arm formula counterexample whose documents are held by the byte provision, with an empty `fairness`, replays over the documents' trace and settles `reproduced-with-evaluated-witness` when the formula is false at `trace_position`; the same counterexample with a non-empty `fairness` settles `unsupported`, cause `missing-fairness-premise`, with no formula evaluated; a step position the provision does not hold refuses `invalid_runtime_input`/`invalid-value`; an `interval` unequal to the clause's refuses `stale_dependency`/`content-mismatch` naming `interval`, and an infinite-trace formula counterexample for `eventually[2,*] holds(p)` carries `interval` `{lower: "2", upper: null}` and is refused the same way against the clause `eventually[3,*] holds(p)`. | Test (TC-295) |
| FR-364-AC-10 | The wire schema admits an observed-arm formula counterexample (null `initial_state` and `over_binding`, two `document` steps) and an observed-arm undefined-evaluation envelope whose `where` carries `position`, `document` and `locus`; it refuses an observed-arm counterexample that names an initial state, carries an `over_binding`, carries a model-arm step in its `prefix` or in its `loop`, has `kind` `deadlock`, has `kind` `time-lock` with its `final_delay`, or has an envelope whose `where` carries `state_key` or `state_keys`; and it refuses a model-arm counterexample that has a null `initial_state`, carries a `document` step in its `prefix` or in its `loop`, or has an envelope whose `where` carries `document`. | Test (TC-431) |

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
