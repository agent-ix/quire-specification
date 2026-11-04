---
id: FR-230
title: "Protocol obligation instance"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-479
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-055
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-232
    type: contains
---
# FR-230: Protocol obligation instance

## Description

When a protocol trigger is established, the conformance evaluator SHALL create each dependent obligation with its exact workflow, role, effect, predicate, capture and evaluation-anchor identities (FR-055).

An obligation instance is the thing an activation creates. It is not a business
action: FR-055 states that "no business action is executed" when an obligation is
created.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| workflow_identity | String | 1..1 |  |
| role_identity | String | 1..1 |  |
| effect_identity | String | 1..1 |  |
| predicate_identity | String | 1..1 |  |
| capture_set | ImmutableActivationCapture | 1..1 |  |
| evaluation_anchor_identity | String | 1..1 |  |
| clause_subject | String | 1..1 |  |
| semantic_trigger_event_identity | String | 0..1 |  |
| activation_state | ObligationActivationState | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| workflow_identity | type not stated in source; see Field sources |
| role_identity | type not stated in source; see Field sources |
| effect_identity | type not stated in source; see Field sources |
| predicate_identity | type not stated in source; see Field sources |
| evaluation_anchor_identity | type not stated in source; see Field sources |
| clause_subject | type not stated in source; required key component (Identity and comparison) |
| semantic_trigger_event_identity | type not stated in source; present only for `trigger_event` origin (Identity and comparison) |
| activation_state | one of triggered, inactive or activation-unknown (FR-231) |


| Property | Source | Note |
| --- | --- | --- |
| Workflow identity | FR-055 Description | Exact, part of the obligation's dependency set. |
| Role identity | FR-055 Description | Exact. |
| Effect identity | FR-055 Description | The effect the obligation depends on. |
| Predicate identity | FR-055 Description | The authored predicate bound to the obligation. |
| Capture set | FR-055 Description, FR-093 | Immutable captured amounts, identifiers and anchors ([FR-232](FR-232-activation-capture.md)). |
| Evaluation-anchor identity | FR-055 Description, FR-055-AC-4 | A different evaluation anchor refuses the binding. |
| Clause subject | FR-093 Behavior | Identity component for an event-triggered instance. |
| Semantic trigger-event identity | FR-093 Behavior | Identity component for an event-triggered instance. |
| Activation state | FR-055 Outputs, FR-093 | One of triggered, inactive or activation-unknown ([FR-231](FR-231-obligation-activation-state.md)). |

## Identity and comparison

- **Identity kind** — *derived*. The instance identity is computed from its
  exact package, definition, workflow, control occurrence, clause subject and
  origin; it is not an independently authored label. An instance is
  constructed only from one complete
  [FR-479](FR-479-obligation-activation-binding.md) activation binding, whose
  activation relation row supplies the control occurrence.
- **Key** — (`package`, `protocol definition`, `workflow instance`, `control
  occurrence`, `clause subject`, `origin kind`, `origin identity`). `origin
  kind` is `whole_execution` or `trigger_event`; the former carries the workflow
  execution identity and the latter the semantic trigger-event identity.
  Equality is componentwise over that tuple and nothing else. Timestamp,
  display name, equal payload, source path, span, message text, locale and
  rendering never participate (FR-093, FR-009, FR-047). FR-093 makes both
  directions explicit: "Repeated delivery of the same semantic trigger shall not
  create a second instance when its admitted payload agrees" and "A distinct
  semantic trigger shall create a distinct instance even if its captured values
  compare equal" (FR-093-AC-1, FR-093-AC-2).
- **Key order** — lexicographic canonical byte order over the complete key, for
  collection comparison only. It is total and has no causal meaning.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). An obligation's
  position relative to its triggering effect is an authored causal edge; it is
  never derived from key order, timestamp or source position, and FR-052-AC-2
  states that "Reordering equal-time events does not add a causal edge absent
  from the protocol."
- **Absence** — an absent required key component compares only to absence.
  Whole-execution origin carries its workflow execution identity rather than an
  absent trigger placeholder, and it never compares equal to trigger-event
  origin. Absence is never defaulted to an empty identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-230-AC-1 | Two instances compare equal only when every package, definition, workflow, control-occurrence, clause-subject, origin-kind and origin-identity component compares equal. | Analysis |
| FR-230-AC-2 | Two instances with equal captured values and distinct semantic trigger identities compare unequal. | Analysis |
| FR-230-AC-3 | Timestamp, display name and payload equality are absent from the declared key. | Inspection |
| FR-230-AC-4 | A whole-execution origin uses the workflow execution identity; a trigger-event origin uses the semantic trigger identity; an absent required origin identity refuses rather than receiving a default. | Analysis |

## Dependencies

- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md) supplies the
  obligation's dependency identities and the rule that activation is independent
  from truth.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  supplies the instance identity rule and the duplicate/distinct-trigger rules.
- [FR-479](FR-479-obligation-activation-binding.md) supplies the binding from
  which every instance is constructed.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md) supplies
  the distinctness of message, send, receive, delivery, attempt and effect.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 obligation identity contract.
