---
id: FR-430
title: "Apply scheduler and authored fairness over protocol steps"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
---

# FR-430: Apply scheduler and authored fairness over protocol steps

## Description

When an infinite-trace temporal clause is checked over a protocol subject, the
analyzer SHALL add one derived weak `whole` scheduler constraint per thread
target of the protocol, unless the protocol clause declares `scheduling
adversarial`, and SHALL read authored and derived constraints over the
protocol step classes stated below.

## Inputs

A checked infinite-trace temporal clause with its authored fairness
constraints, and a protocol subject
([FR-425](FR-425-define-protocol-subjects-and-state-keys.md)) with its
clause's `scheduling` member, if any.

## Outputs

The clause's resolved fairness set over the protocol subject, each member
carrying its target and origin; or a located refusal for a constraint whose
target does not resolve.

## Behavior

### Targets

A fairness constraint over a protocol subject names one of four targets. Each
has a `whole` and an `each` granularity; an unmarked constraint is `whole`.

| Target | Spelling | `whole` class | `each` classes |
| --- | --- | --- | --- |
| operation | its qualified operation name | every `attempt` and `cattempt` step that applies it | one per (operation, receiver, arguments) |
| attempt node | `<protocol>::<node>` naming an attempt node | every step of that node | one per (thread path, application) |
| thread target | `<protocol>::<parallel>::<branch>` for a branch, or `<protocol>` for the root thread | every step the target's threads own: their event steps and fences, the forks they perform and the joins they wait at | one per thread |
| compensation template | `<protocol>::<template>` | every `cattempt` and `cend` step of its threads | one per thread |

A branch target does not cover the threads of a `parallel` nested inside it;
each nested branch is its own target. A name that resolves to none of these
refuses `missing_declaration`/`missing-name` at the name.

A class is **enabled** at a state when one of its steps is enabled there
([FR-426](FR-426-apply-protocol-steps.md)), and **taken** by a step that
belongs to it. `duplicate`, `lose`, `activate`, `spawn`, `retire` and memory
steps belong to no class; a memory model adds its own constraints
([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)). Weak and
strong fairness read these classes as they read operation classes over a
model subject.

### Scheduler constraints

Every infinite-trace clause over a protocol subject carries one **scheduler
constraint**, weak and `whole`, per thread target: each branch of each
`parallel`, the root, and each compensation template. Scheduler constraints
are derived from the checked protocol clause, not authored. Each enters the
clause's fairness set with origin `scheduler`, beside authored constraints
(origin `authored`) and memory constraints (origin `memory`), and so enters
the obligation identity, every counterexample and replay. Authored
constraints, `each` constraints included, are added to the scheduler
constraints.

A protocol clause with the member `scheduling adversarial;`, by the
[shared grammar](../../../proposals/quire-v1/shared-grammar.md)'s `protocol-scheduling` production, carries no scheduler
constraint: its fairness set is its authored and memory constraints, and a scheduler may
starve any thread. The member is part of the checked package, so the same
protocol with and without it is two obligations. A second `scheduling` member
refuses `ambiguous_declaration`/`ambiguous-name`.

### Fairness set members

Each member of a fairness set over a protocol subject is
`{"kind":"weak"|"strong","target":<target>,"granularity":"whole"|"each","origin":"authored"|"scheduler"|"memory"}`,
with `target` `{"type":"operation","name":"<qualified-name>"}`,
`{"type":"attempt-node","node":"<node>"}`,
`{"type":"thread-target","node":"<node>"|null,"branch":"<label>"|null}` (both
null for the root) or `{"type":"template","name":"<qualified-name>"}`, or the
target a memory model gives (a `tso` flush is `weak` and `each`; the `ra` visibility member is `strong` and `whole`, FR-437). The set is ordered by the JCS bytes of its
members.

### Machine closure

Weak fairness over step classes is machine-closed: every finite prefix
extends to a behaviour fair under every constraint, scheduler constraints
included. A scheduler constraint therefore never makes a proof vacuous and
never changes a safety verdict; the default changes liveness verdicts only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-430-AC-1 | A liveness claim that fails only on a lasso starving one branch thread settles `proved` under the default scheduler constraints and `refuted` under `scheduling adversarial`, with the starving lasso as its counterexample. | Test (TC-384) |
| FR-430-AC-2 | The resolved fairness set of a clause over a two-branch protocol with no authored constraint holds exactly three `scheduler` members, for the two branches and the root, each weak and `whole`; with `scheduling adversarial` it is empty. | Test (TC-384) |
| FR-430-AC-3 | A protocol with and without `scheduling adversarial` carries two different obligation identities; a safety claim receives the same verdict under both. | Test (TC-384) |
| FR-430-AC-4 | An authored constraint on an attempt node and on a compensation template is discharged by a step of that node or template's threads; an `each` branch constraint is violated by a loop that starves one of two live threads of that branch. | Test (TC-384) |
| FR-430-AC-5 | A branch constraint is not discharged by a step of a thread of a `parallel` nested inside that branch. | Test (TC-384) |
| FR-430-AC-6 | A constraint naming no operation, attempt node, branch, protocol or template refuses `missing_declaration`/`missing-name`; a second `scheduling` member refuses `ambiguous_declaration`/`ambiguous-name`. | Test (TC-384) |

## Dependencies

- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) states that
  fairness filters traces before evaluation.
- [FR-426](FR-426-apply-protocol-steps.md) owns protocol step enabledness.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `protocol-scheduling` production and the fairness constraint spelling.

## References

- Design: quire-spec-language ADR-027 PA-1 to PA-4, QS-6 and QS-13; weak and
  strong fairness over operations in ADR-018 FA-1 to FA-6.
- Tickets: Linear STD-140; paired QSL-396.
