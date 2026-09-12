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
---
# FR-233: Compensation registration

## Description

When an authored forward effect is established, the conformance evaluator SHALL register exactly the compensation obligations paired with that effect and bind each registration to its role, eligibility conditions, retry policy, commit boundary and recovery relation (FR-056).

FR-056 states that the evaluator "does not invoke the compensating operation".

## Properties

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
- **Key** — the source compensation definition identity together with the
  established forward effect identity. Equality is componentwise over that pair
  and nothing else. Display name, source path, span, message text, locale,
  rendering, transport status, trace identifier and repetition of the observation
  never participate (FR-009, FR-047, FR-051). FR-056-AC-3 states that "Replaying
  the same effect identity does not create a second registration", and
  FR-051-AC-4 refuses "A transport trace ID substituted for an authored workflow,
  attempt or effect identity".
- **Key order** — none. No requirement fixes a total order over compensation
  definition identities or effect identities, so this artifact establishes none.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). Registration is
  causally after the forward effect that created it and before a forbidding
  commit: FR-056-AC-1 states that one successful payment effect "registers one
  matching refund obligation after the effect", and FR-057 requires the evaluator
  to "reject or violate registration after a forbidding commit". That order is
  authored and never derived from key order, timestamp or source position.
- **Absence** — an absent key component compares only to absence. An attempt,
  send, receive or delivery with no established effect leaves the forward effect
  identity absent, and FR-056 forbids registering from it; absence is never
  compared to a default or empty effect identity, and FR-051-AC-3 states that "A
  payment attempt with no established effect remains an attempt and cannot
  activate effect-dependent compensation."
- **Digest domain** — no key component of this object is a digest. Where a
  definition or effect identity is carried as a digest, it retains the domain
  selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-233-AC-1 | Two registrations compare equal only when both the source compensation definition and the forward effect identity compare equal. | Analysis |
| FR-233-AC-2 | A repeated observation of one effect identity yields the same registration rather than a second one. | Analysis |
| FR-233-AC-3 | A registration key with an absent forward effect identity compares only to absence and is never defaulted. | Analysis |
| FR-233-AC-4 | The declared key contains no transport status, trace identifier, delivery count or payload equality. | Inspection |

## Dependencies

- [FR-056](../../functional/protocol/FR-056-register-compensation.md) establishes
  registration, its retained components and the duplicate-registration rule.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  establishes that attempts, deliveries and effects are distinct subjects.
- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  forbidding-commit ordering rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-056 requires each registration to retain "the source definition and effect
that created it", and FR-056-AC-1 pairs "One successful payment effect" with "one
matching refund obligation". No requirement states whether one effect may pair
with more than one compensation definition, so it is not established whether the
definition identity is a required key component or whether the effect identity
alone is the key. Both readings are recorded; neither is selected here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
