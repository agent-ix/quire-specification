---
id: FR-433
title: "Carry and replay protocol counterexamples"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-429
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-430
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-433: Carry and replay protocol counterexamples

## Description

When a counterexample over a protocol subject is produced or replayed, the
analyzer SHALL carry the protocol members stated below and settle its replay
by re-executing it through the protocol system as stated below.

## Inputs

A counterexample over a protocol subject, and the subject's checked package,
initial states, universes, bindings and resolved memory models.

## Outputs

The counterexample wire; and on replay, `reproduced-with-evaluated-witness`,
an `inconclusive` result with cause `replay-parity`, or a typed refusal.

## Behavior

### Wire

A protocol counterexample is the model-check counterexample object with the
members `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness`
and `trace_position`, read as for a model subject, and these protocol
members:

| Member | Value |
| --- | --- |
| `subject` | `{"type":"protocol-subject","protocol":"<qualified-name>","over":<encoded>,"roles":<map from role ident to binding>}` |
| `prefix`, `loop` steps | `{transition, post_state}` with, for a `send` step only, `send_record`; `transition` is the step's protocol transition identity ([FR-426](FR-426-apply-protocol-steps.md)) or the terminal-stutter identity, `post_state` the state key of its post-state, and `send_record` the step's send record in the form FR-426 gives |
| `fairness` | the resolved fairness set over the protocol subject, each member with its target and origin ([FR-430](FR-430-apply-protocol-fairness.md)) |
| `blocked` | for `kind: deadlock`, the last state's blocked-thread list ([FR-429](FR-429-report-protocol-deadlocks.md)); absent otherwise |
| `max_live_instances` | for an `on each` subject, the instance budget the run used |

The rest points, binders, queues and memory component of each position are
data a renderer recomputes; the post-state key already binds them. A
missing, extra or mistyped member refuses before replay.

### Replay

Replay SHALL recompile the package, re-admit the initial state, universes and
bindings, build the protocol system under the counterexample's resolved
memory models and instance budget, and re-execute each step:

1. A step whose transition identity is not enabled at its pre-state, or a
   loop whose last post-state is not its entry state, refuses
   `invalid_runtime_input`/`invalid-value`.
2. A `send` step
   without a `send_record`, or any other step with one, is a wire-shape defect
   and refuses before replay, like any missing or extra member.
3. A lasso is checked against the fairness set over the protocol step
   classes; an unfair loop refuses `invalid_runtime_input`/`invalid-value`.
4. For `kind: formula`, the formula is evaluated over the replayed behaviour,
   with positions and counted distance read by
   [FR-431](FR-431-count-protocol-steps-for-intervals.md). The evaluator
   returning false at `trace_position` settles
   `reproduced-with-evaluated-witness`; true settles `inconclusive`, cause
   `replay-parity`.
5. For `kind: deadlock`, replay enumerates every protocol step at the last
   state, checks that none is enabled, that the state is not intended and
   that it is not instance-limited, and recomputes the blocked-thread list.
   Agreement on all of these settles `reproduced-with-evaluated-witness`; an
   enabled step, an intended state or a different blocked list settles
   `inconclusive`, cause `replay-parity`.

6. For an FR-364 `undefined-evaluation` envelope, replay re-evaluates the expression
   that `where` names at `trace_position`, the prefix's last position: the
   claim's formula for an authored claim, or the protocol expression for an
   undefined protocol expression ([FR-426](FR-426-apply-protocol-steps.md)).
   The undefined value with the carried locus and reason settles
   `reproduced-with-evaluated-witness` and the item `refuted`, cause
   `undefined-evaluation`; a defined value, or an undefined value with another
   locus or reason, settles `inconclusive`, cause `replay-parity`.

Replay reads no engine state. Two engines that return different valid
counterexamples for one item both replay.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-433-AC-1 | The `Fill` deadlock counterexample carries the steps `fork(Both)` and `attempt(A)` as protocol transition identities with post-state keys, `kind: deadlock` and the blocked list, and replays to `reproduced-with-evaluated-witness`. | Test (TC-387) |
| FR-433-AC-2 | A counterexample whose step is not enabled, or whose loop does not close, refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-387) |
| FR-433-AC-3 | A liveness counterexample whose loop starves a branch under the scheduler constraints refuses as unfair; the same loop under a `scheduling adversarial` clause replays. | Test (TC-384, TC-387) |
| FR-433-AC-4 | A deadlock counterexample whose last state enables a step, or whose recorded blocked list differs from the recomputed one, settles `inconclusive`, cause `replay-parity`. | Test (TC-387) |
| FR-433-AC-5 | A counterexample with a missing `subject`, an extra member or a mistyped transition identity refuses before replay. | Test (TC-387) |
| FR-433-AC-6 | Every refutation vector of TC-376 to TC-386 replays to `reproduced-with-evaluated-witness`. | Test (TC-387) |
| FR-433-AC-7 | The `undefined-evaluation` counterexample of an undefined recovery predicate replays to `reproduced-with-evaluated-witness` and the item `refuted`, cause `undefined-evaluation`; the same counterexample with its last step changed so that the predicate is defined at `trace_position` settles `inconclusive`, cause `replay-parity`. | Test (TC-387) |
| FR-433-AC-8 | Each `send` step of a counterexample carries a `send_record`; a `send` step without a `send_record` and a non-`send` step with one refuse before replay as a missing or extra member. | Test (TC-387) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  post-state key and replayable traces.
- [FR-426](FR-426-apply-protocol-steps.md), [FR-429](FR-429-report-protocol-deadlocks.md),
  [FR-430](FR-430-apply-protocol-fairness.md) and
  [FR-431](FR-431-count-protocol-steps-for-intervals.md) own the identities,
  blocked threads, fairness set and positions replay reads.

## References

- Design: quire-spec-language ADR-027 PX-1, PX-2, PD-4 and QS-8; the
  model-subject counterexample in ADR-018 CX-1 to CX-5.
- Tickets: Linear STD-140; paired QSL-396.
