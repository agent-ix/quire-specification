---
id: FR-232
title: "Immutable activation capture"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-055
    type: specifies
---
# FR-232: Immutable activation capture

## Description

When a temporal scope activates, the evaluator SHALL evaluate every declared capture exactly once at the authored activation anchor (FR-093).

FR-093 adds that "Later mutable observations shall not replace the captured
value."

FR-055 states that "Captured amounts, identifiers and anchors SHALL be immutable
for that obligation."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Declared expression | FR-093 Behavior | Retained with the capture. |
| Declared type | FR-093 Behavior | Retained with the capture. |
| Value | FR-093 Behavior | Evaluated exactly once at the authored activation anchor. |
| Source/model binding | FR-093 Behavior | Retained with the capture. |
| Activation anchor | FR-093 Behavior, FR-055-AC-4 | A different evaluation anchor refuses the binding. |

FR-093 states that "Later mutable observations shall not replace the captured
value", that "A capture that cannot be established shall prevent healthy
activation and shall identify the missing or inconsistent input", and that "One
instance's captures shall not satisfy another instance's propositions."

## Identity and comparison

- **Identity kind** — *value*. A capture has no identity of its own. It belongs
  to exactly one obligation instance ([FR-230](FR-230-protocol-obligation.md))
  and is compared componentwise.
- **Key** — the declared expression, declared type, value, source/model binding
  and activation anchor, compared componentwise and nothing else. Display name,
  source path, span, message text, locale and rendering never participate
  (FR-009, FR-047). Componentwise equality of captures never establishes instance
  equality: FR-093 states that "A distinct semantic trigger shall create a
  distinct instance even if its captured values compare equal", and FR-093-AC-1
  requires that "neither refund can satisfy the other's captured payment
  identity".
- **Key order** — none. No requirement fixes an order over capture components or
  over the declared capture set, so this artifact establishes none.
- **Causal order** — none of its own. A capture is anchored at the authored
  activation anchor; the anchor's position is the authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) and is never
  derived from key order, timestamp or source position.
- **Absence** — a capture that could not be established is absent, and absence
  compares only to absence. FR-093 states that such a capture "shall prevent
  healthy activation and shall identify the missing or inconsistent input"; it is
  never compared to a default, an empty value or an implicit option-none, and
  FR-093-AC-4 holds "Missing, nullable, wrong-type, stale or anchor-mismatched
  capture input" distinct from an established capture.
- **Digest domain** — no key component of this object is a digest. Where a
  captured identifier is carried as a digest, it retains the domain selected
  under [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-232-AC-1 | A capture retains its declared expression, type, value, source/model binding and activation anchor. | Inspection |
| FR-232-AC-2 | A mutable source change after activation does not change an already captured value. | Analysis |
| FR-232-AC-3 | Two captures comparing equal componentwise do not make their owning obligation instances equal. | Analysis |
| FR-232-AC-4 | An unestablished capture compares only to absence and is not defaulted to an empty or null value. | Analysis |

## Dependencies

- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  establishes capture retention, immutability and the anchor rule.
- [FR-055](../../functional/protocol/FR-055-activate-protocol-obligations.md) establishes
  capture immutability for the obligation.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Ordered capture contract

An obligation owns an authored ordered capture sequence. A capture occurrence
key is (`obligation`, zero-based declaration index); equal expression identities
may occur more than once and remain distinct occurrences. Evaluation follows
authored order at the inclusive activation boundary and stores one immutable
typed result or typed missing/conflicting cause per occurrence.

## Status

Accepted complete-V1 activation-capture contract.
