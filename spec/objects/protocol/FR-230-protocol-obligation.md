---
id: FR-230
title: "Protocol obligation instance"
type: FR
object: entity
relationships:
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

- **Identity kind** — *derived*. FR-093 states that "Each event-triggered
  instance shall be identified by the clause subject and semantic trigger-event
  identity, not by timestamp, display name or equal payload alone." The instance
  identity is therefore computed from those two identified components, not
  written as a separate authored instance label.
- **Key** — the clause subject together with the semantic trigger-event identity
  (FR-093). Equality is componentwise over that pair and nothing else. Timestamp,
  display name, equal payload, source path, span, message text, locale and
  rendering never participate (FR-093, FR-009, FR-047). FR-093 makes both
  directions explicit: "Repeated delivery of the same semantic trigger shall not
  create a second instance when its admitted payload agrees" and "A distinct
  semantic trigger shall create a distinct instance even if its captured values
  compare equal" (FR-093-AC-1, FR-093-AC-2).
- **Key order** — none. No requirement fixes a total order over the clause
  subject or the semantic trigger-event identity, so this artifact establishes
  none. See the open gap below.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). An obligation's
  position relative to its triggering effect is an authored causal edge; it is
  never derived from key order, timestamp or source position, and FR-052-AC-2
  states that "Reordering equal-time events does not add a causal edge absent
  from the protocol."
- **Absence** — an absent key component compares only to absence. A
  whole-execution-origin instance (FR-093 Inputs) with no trigger event does not
  compare equal to an instance whose trigger-event identity is present, and
  absence is never defaulted to an empty or placeholder identity.
- **Digest domain** — no key component of this object is a digest. Where a
  component identity is carried as a digest, it retains the domain selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md), and an identity
  crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-230-AC-1 | Two instances compare equal only when both the clause subject and the semantic trigger-event identity compare equal. | Analysis |
| FR-230-AC-2 | Two instances with equal captured values and distinct semantic trigger identities compare unequal. | Analysis |
| FR-230-AC-3 | Timestamp, display name and payload equality are absent from the declared key. | Inspection |
| FR-230-AC-4 | An instance whose trigger-event identity is absent compares only to absence and receives no default identity. | Analysis |

## Dependencies

- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md) supplies the
  obligation's dependency identities and the rule that activation is independent
  from truth.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  supplies the instance identity rule and the duplicate/distinct-trigger rules.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md) supplies
  the distinctness of message, send, receive, delivery, attempt and effect.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement fixes a total order over the clause subject or the semantic
trigger-event identity, so no key order is declared here. FR-055 and FR-093 also
do not state whether a whole-execution-origin instance (FR-093 Inputs) carries a
trigger-event identity component at all, or whether its key is the clause subject
alone. Both are recorded as unresolved rather than supplied.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
