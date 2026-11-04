---
id: FR-233
title: "Compensation registration"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-056
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-237
    type: references
  - target: ix://agent-ix/quire-specification/FR-238
    type: references
  - target: ix://agent-ix/quire-specification/FR-239
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-233: Compensation registration

## Description

When an authored forward effect is established, the conformance evaluator SHALL register exactly the compensation obligations paired with that effect and bind each registration to its role, eligibility conditions, retry policy, commit boundary and recovery relation (FR-056).

FR-056 states that the evaluator "does not invoke the compensating operation".

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| source_compensation_definition | String | 1..1 |  |
| forward_effect_identity | String | 1..1 |  |
| role | String | 1..1 |  |
| eligibility_conditions | String | 1..1 |  |
| retry_policy | CompensationRetryPolicy | 1..1 |  |
| commit_boundary | CommitBoundary | 1..1 |  |
| recovery_relation | AuthoredRecoveryRelation | 1..1 |  |

## Field sources

| Property | Source | Note |
| --- | --- | --- |
| Source compensation definition | FR-056 Behavior | "Each registration SHALL retain the source definition and effect that created it." |
| Forward effect identity | FR-056 Behavior, FR-056-AC-3 | Exact workflow/effect identity of the successfully established forward effect. |
| Role | FR-056 Description | Bound at registration. |
| Eligibility conditions | FR-056 Description | Bound at registration. |
| Retry policy | FR-056 Description, FR-058 | [FR-239](FR-239-compensation-retry-policy.md). |
| Commit boundary | FR-056 Description, FR-057 | [FR-237](FR-237-commit-boundary.md). |
| Recovery relation | FR-056 Description, FR-057 | [FR-238](FR-238-recovery-relation.md). |

FR-056 states that the evaluator "SHALL NOT register compensation from a send,
receive, delivery or attempt that lacks the required successful effect" and that
"Repeated observation of one effect SHALL NOT create additional registrations."

## Identity and comparison

- **Identity kind** — *derived*. A registration is not written in source; it is
  created by pairing an admitted compensation definition with a successfully
  established forward effect, and FR-056 requires it to retain exactly those two.
- **Key** — (`static compensation-definition key`, `concrete forward-effect
  key`). The static key is the FR-288 qualified protocol-definition identity
  plus the exact declaration-local compensation handle; the concrete effect key
  is one FR-287 value of kind `business-effect`. Equality is componentwise over that pair
  and nothing else. Display name, source path, span, message text, locale,
  rendering, transport status, trace identifier and repetition of the observation
  never participate (FR-009, FR-047, FR-051). FR-056-AC-3 states that "Replaying
  the same effect identity does not create a second registration", and
  FR-051-AC-4 refuses "A transport trace ID substituted for an authored workflow,
  attempt or effect identity".
- **Key order** — lexicographic over the static compensation-definition key and
  then the concrete forward-effect FR-287 key. It is only for canonical
  collection comparison and establishes no registration, effect or commit
  order.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). Registration is
  causally after the forward effect that created it and before a forbidding
  commit: FR-056-AC-1 states that one successful payment effect "registers one
  matching refund obligation after the effect", and FR-057 requires the evaluator
  to "reject or violate registration after a forbidding commit". That order is
  authored and never derived from key order, timestamp or source position.
- **Absence** — both complete components are required. A missing, empty,
  wrong-kind or foreign component refuses rather than creating a registration.
  An attempt,
  send, receive or delivery with no established effect leaves the forward effect
  identity absent, and FR-056 forbids registering from it; absence is never
  compared to a default or empty effect identity, and FR-051-AC-3 states that "A
  payment attempt with no established effect remains an attempt and cannot
  activate effect-dependent compensation."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-233-AC-1 | Two registrations compare equal only when both the source compensation definition and the forward effect identity compare equal. | Analysis |
| FR-233-AC-2 | A repeated observation of one effect identity yields the same registration rather than a second one. | Analysis |
| FR-233-AC-3 | A registration key with an absent forward effect identity compares only to absence and is never defaulted. | Analysis |
| FR-233-AC-4 | The declared key contains no transport status, trace identifier, delivery count or payload equality. | Inspection |
| FR-233-AC-5 | One effect paired with two distinct static compensation-definition keys creates two registrations, while replaying either exact pair is idempotent. | Test (TC-056) |
| FR-233-AC-6 | Role, eligibility, retry, commit and recovery are separate binding facts; reusing one complete registration key with a conflicting fact refuses rather than minting another registration. | Test (TC-056) |

## Dependencies

- [FR-056](../../functional/protocol/FR-056-register-compensation.md) establishes
  registration, its retained components and the duplicate-registration rule.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  establishes that attempts, deliveries and effects are distinct subjects.
- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  forbidding-commit ordering rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) supplies the
  concrete forward-effect key.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) qualifies the
  declaration-local compensation handle.

## Status

Proposed closure for specification issue #31. Spec Language publishes the
static compensation handle and binding requirements. Protocol currently retains
no compensation records, so executable evidence remains planned under TC-056.
