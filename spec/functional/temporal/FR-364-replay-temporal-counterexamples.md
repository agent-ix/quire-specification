---
id: FR-364
title: "Carry and replay a temporal counterexample over a model subject"
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
---

# FR-364: Carry and replay a temporal counterexample over a model subject

## Description

When an analysis refutes a temporal item over a model subject, the replayer SHALL
reproduce the provider's temporal counterexample, carried in the wire form
below, under the replay rules below before the item settles as refuted.

## Inputs

A temporal counterexample; the checked package; the model subject's initial
states and universes, re-admitted from the request's byte provision; the
item's clause and resolved fairness set.

## Outputs

A replay settlement: `reproduced-with-evaluated-witness` with the item
`refuted`; a located replay refusal; or `inconclusive` with cause
`replay-parity`.

## Behavior

### Shapes

| Refutation | Shape |
| --- | --- |
| Reachable-state invariant, safety, bounded | A finite prefix. Under a bounded profile it covers the formula's horizon from every activation it reports, or ends at a terminal state. Under infinite-trace it is a bad prefix: every infinite extension of it violates the formula. |
| Liveness | A lasso: a prefix, possibly empty, and a non-empty loop that re-enters at its first position. A behaviour that ends at a terminal state is the lasso whose loop is the one terminal stutter step. |
| Deadlock ([FR-366](FR-366-check-deadlock-freedom.md)) | A finite prefix from an initial state to the deadlocked state. |
| Undefined evaluation ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)) | A finite prefix from an initial state to the state at which the claim evaluates undefined, for every property form; `loop` is empty. |

### Wire form

A `TemporalCounterexample` is one JSON object with exactly these members:

| Member | Value |
| --- | --- |
| `kind` | `"formula"` for a refutation of an authored clause on which the evaluator returns false; `"deadlock"` for a refutation of the deadlock-freedom item; or, for a refutation of an authored clause that evaluates undefined, the object `{"undefined_evaluation": {"where": {position, state, locus}, "cause": <reason>}}`, whose `where` is the position, the `quire.simulation.state-key/v1` digest of the state there and the source locus of the expression with no value, and whose `cause` is the undefined value's catalogued reason ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)) |
| `initial_state` | the zero-based index of the behaviour's initial state in the subject's ordered list of initial states |
| `prefix` | the ordered steps of the prefix |
| `loop` | the ordered steps of the loop; empty for a finite prefix |
| `over_binding` | the reference of the object the clause's `over` parameter is bound to; `null` for `kind: deadlock` |
| `fairness` | the item's resolved fairness set, each member `{kind, operation, granularity}` with `kind` `weak` or `strong` and `granularity` `whole` or `each`, in fairness-set order; empty for `kind: deadlock` |
| `trace_position` | for `kind: formula`, the position at which the evaluator returns false; for an `undefined_evaluation` kind, the position at which the claim evaluates undefined, which is the prefix's last position; `null` for `kind: deadlock` |

Each step is `{transition, post_state}`: `transition` is the step's
[FR-181](../runtime/FR-181-simulate-finite-executions.md) transition identity,
or `{"type":"terminal-stutter"}` for the terminal stutter step; `post_state` is
the `quire.simulation.state-key/v1` digest of the step's post-state. The loop's
entry is the position after the prefix's last step. A counterexample travels
as an [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`counterexamples` entry whose identity binds the item's subject.

Every engine returns this one type. An engine reads each step's transition
identity from its own encoding and names the initial state by index, so its
counterexample replays with no engine present.

### Replay

The replayer SHALL, in this order:

1. Re-admit the checked package, the subject's initial states and universes,
   and select initial state `initial_state`.
2. Re-execute each step of the prefix, then of the loop, through the successor
   relation of [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md).
   A step's `post_state` selects which successor of its transition identity
   the step took, since one identity can have several post-states. A step whose
   transition identity is not enabled at its pre-state refuses
   `invalid_runtime_input`/`invalid-value`. A step for which no successor of
   its transition identity has the recorded `post_state` refuses
   `stale_dependency`/`revision-mismatch`. A terminal stutter step at a state
   that is not terminal refuses `invalid_runtime_input`/`invalid-value`.
3. For a lasso, check that the loop's last post-state is the loop's entry
   state, else refuse `invalid_runtime_input`/`invalid-value`.
4. For a lasso, recompute the enabled transition identities at every loop
   state from the model, and check every constraint of `fairness` against the
   loop by [FR-362](FR-362-apply-fairness-constraints.md): weak and strong
   each by its own lasso rule. A loop that fails a constraint is an unfair
   lasso and refuses `invalid_runtime_input`/`invalid-value`, naming the
   constraint.
5. For `kind: formula`, evaluate the clause's formula over the behaviour
   with the trace evaluator. A lasso denotes the infinite trace that repeats
   its loop forever: a position past the represented steps reads the state of
   the loop position congruent to it modulo the loop length, and past
   operators read the denoted trace back to position 0. Evaluation false at
   `trace_position` agrees with the engine; any other result disagrees.
6. For an `undefined_evaluation` kind, evaluate the clause's formula over the
   prefix with the trace evaluator. The undefined value at `trace_position`,
   with the expression locus of the counterexample's own `where` and the
   catalogued reason of its own `cause`, agrees with the engine. Replay reads
   both from the counterexample, never from the item. A defined value
   there, or an undefined value at another position, expression or reason,
   disagrees.
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
| FR-364-AC-2 | A counterexample carries exactly the members `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness` and `trace_position`; each step carries its FR-181 transition identity or the terminal-stutter identity and its post-state digest; each fairness member carries its kind, operation and granularity. A missing, extra or mistyped member refuses before replay. | Test (TC-295) |
| FR-364-AC-3 | A valid formula counterexample replays through the model, passes the fairness check and evaluates false at `trace_position`, settling `reproduced-with-evaluated-witness` and the item `refuted`. | Test (TC-293, TC-295) |
| FR-364-AC-4 | A loop whose last post-state is not its entry state, a step whose transition is not enabled, and a terminal stutter step at a non-terminal state refuse `invalid_runtime_input`/`invalid-value`; a step whose recorded post-state digest matches no successor refuses `stale_dependency`/`revision-mismatch`. | Test (TC-295) |
| FR-364-AC-5 | A lasso that fails a weak or a strong constraint of its fairness set, with enabledness recomputed from the model, refuses as an unfair lasso naming the constraint and settles no `refuted`. | Test (TC-295, TC-299) |
| FR-364-AC-6 | A counterexample on which the evaluator returns true at `trace_position` settles `inconclusive`, cause `replay-parity`; a deadlock counterexample whose last state enables a transition or satisfies `P` settles the same. | Test (TC-295, TC-297) |
| FR-364-AC-7 | A lasso whose formula reads a past operator across the loop boundary evaluates by the infinite trace the lasso denotes. | Test (TC-200) |
| FR-364-AC-8 | A counterexample of an undefined evaluation is a finite prefix with an empty `loop`, an `undefined_evaluation` kind carrying `where` and `cause`, and `trace_position` at its last position; its replay reproduces the undefined value with the locus and reason that kind carries, read from the counterexample alone, settling `reproduced-with-evaluated-witness` and the item `refuted` with cause `undefined-evaluation`; a `trace_position` at which the claim's reads are defined settles `inconclusive`, cause `replay-parity`. | Test (TC-305) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  transition identity and the state-key digest.
- [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md) owns the
  successor relation and the terminal stutter step.
- [FR-362](FR-362-apply-fairness-constraints.md) owns the lasso fairness rules.
- [FR-365](FR-365-build-canonical-temporal-counterexample.md) owns the
  canonical counterexample of an exhaustive explicit-state analysis.

## References

- quire-spec-language ADR-018 CX-1 to CX-4, SM-7, SM-8, DL-4, DL-5 and QS-8;
  ADR-019 SR-8, SV-4 and QS-5, QS-6; Linear STD-131 and STD-132.
