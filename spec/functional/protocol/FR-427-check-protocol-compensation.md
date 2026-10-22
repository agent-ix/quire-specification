---
id: FR-427
title: "Check compensation templates inside a protocol subject"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-056
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-057
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-058
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-173
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
---

# FR-427: Check compensation templates inside a protocol subject

## Description

When a protocol subject's clause declares compensation templates, the
analyzer SHALL register, activate, attempt, close and end each compensation
as stated below, as steps and folded effects of the protocol system.

## Inputs

A protocol subject whose clause has one or more `compensate` templates, and a
protocol state ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md)).

## Outputs

The registrations, compensation threads and `cattempt` and `cend` steps of
the state, and each registration's status.

## Behavior

### Registration

When a step of a template's forward `effect` node occurs, the same step
registers one compensation for each template naming that node. The
registration is keyed by (template, the effect's thread path, ordinal), its
`as` binder is bound to the effect record, and its registration captures are
evaluated then. Its status is `registered`. No registration is made after a
step of the template's commit node has occurred in the instance
([FR-057](FR-057-enforce-commit-recovery.md)).

### Activation

A `registered` registration activates in the first later step that binds a
record of the template's trigger type for which the activation guard holds
over that record and the registration captures. Activation is folded into
that step: the activation captures are evaluated, the status becomes
`activated`, and the registration's compensation thread is created at its
compensation attempt with attempt count 0. A later eligible record neither
reactivates it nor registers another. Two eligible triggers in sibling
branches activate it in whichever order the behaviour takes them, and both
orders are behaviours.

### Compensation steps

| Kind | Owner | Enabled when | Effect |
| --- | --- | --- | --- |
| `cattempt` | the compensation thread of the registration | the thread rests at its compensation attempt or retry point; its attempt count is below the template's authored `attempts n`; no step of the template's commit node has occurred in its instance; for a retry, the template's retry relation holds over (previous attempt record, this record); and the FR-426 `attempt` conditions hold for an application of the template's operation by its role, with the template's attempt type as the record | as FR-426 `attempt`; the attempt count advances; the thread rests at its retry point |
| `cend` | the compensation thread at its retry point | always | the thread ends and is removed; the recovery predicate is evaluated over the registration and activation captures; a predicate that reads its recovery binder settles the item `unsupported` with cause `recovery-view`, naming the predicate, because no rule derives a recovery view from a model state; the status becomes `recovered` when it is true and `unrecovered` when it is false; an undefined value settles by the undefined-protocol-expression rule ([FR-426](FR-426-apply-protocol-steps.md)) |

The retry relation constrains a retry and never forces one
([FR-058](FR-058-preserve-retry-and-partial-recovery.md)): `cend` stays
enabled at every retry point. Each `cattempt` is one application of the
template's operation with the same atomicity as an attempt.

### Commit and ending

After a step of the template's commit node occurs in the instance, its
registrations still `registered` close (status `closed`), and its
compensation threads take no further `cattempt`; each can still take `cend`.
`commit never` closes nothing. A registration that never activates closes
when its instance finishes and does not hold up finishing. An instance is
finished only when every compensation thread of it has ended.

### Status values

A registration's status in the state key is one of `registered`,
`activated`, `closed`, `recovered` and `unrecovered`, and its captures are
the registration captures followed by the activation captures once
activated.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-427-AC-1 | The `Pay` protocol (TC-380) reaches exactly nine states and eight transitions; `event(Paid)` registers the compensation and `event(Fail)` with `failed` true activates it in the same step, creating the compensation thread. | Test (TC-380) |
| FR-427-AC-2 | `finish` is not enabled while a compensation thread is running; at a retry point whose count has reached `attempts n` or whose retry relation is false, `cend` is the only step of that thread, and the status records the recovery predicate's value. | Test (TC-380) |
| FR-427-AC-3 | A registration whose trigger never occurs stays `registered` and closes when its instance finishes; a registration made before the commit node's step and still `registered` closes at that step, and no registration is made after it. | Test (TC-380) |
| FR-427-AC-4 | Two eligible triggers in sibling branches give two behaviours, each activating the registration once at the first trigger taken; a later trigger reactivates nothing. | Test (TC-380) |
| FR-427-AC-5 | A recovery predicate that evaluates undefined at `cend` settles every item over the subject `refuted` with cause `undefined-evaluation`, never status `unrecovered`; an activation guard that evaluates undefined over a trigger record does the same, and never leaves the registration `registered`. | Test (TC-380) |
| FR-427-AC-6 | A recovery predicate that reads its recovery binder, as `r.bal = 0` does for `recover (r: M::Recovery)`, settles the item `unsupported` with cause `recovery-view` naming the predicate, before any state is explored; the same protocol with the predicate `triggered`, over an activation capture, is admitted. | Test (TC-380) |

## Dependencies

- [FR-056](FR-056-register-compensation.md),
  [FR-057](FR-057-enforce-commit-recovery.md) and
  [FR-058](FR-058-preserve-retry-and-partial-recovery.md) own the
  compensation, commit and retry meaning these steps execute.
- [FR-426](FR-426-apply-protocol-steps.md) owns the attempt rule and
  transition identity.

## References

- Design: quire-spec-language ADR-027 CP-1 to CP-5, ST-11, ST-12, §7.1 and
  QS-10.
- Tickets: Linear STD-140; paired QSL-396.
