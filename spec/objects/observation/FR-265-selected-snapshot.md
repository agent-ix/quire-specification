---
id: FR-265
title: "Selected snapshot"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
---
# FR-265: Selected snapshot

## Description

When a snapshot is selected for an assessment, the assessment request SHALL retain it as a subject distinct from a selected event-time window.

FR-112 Behavior states it directly: "The request SHALL retain the distinction
between a selected snapshot and a selected event-time window."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Snapshot identity | FR-112 Description | Retained among "population and snapshot/window identities". |
| Distinctness from a window | FR-112 Behavior | A snapshot is not a window; the two are separately selected subjects. |
| Subject participation | FR-112-AC-1 | "Changing a declared snapshot/window or membership rule changes the assessment subject." |
| Declaration and membership selection | FR-112 Description | Exact population declaration key and FR-264 rule used for this capture. |
| Capture assertion | FR-112 Behavior | Exact immutable capture identity, not an inferred timestamp. |
| Admitted observations | FR-112 Behavior | Sorted distinct bounded FR-296 observation keys. |

## Identity and comparison

- **Identity kind** — *value* under FR-112, with no derived content hash.
- **Key** — declaration selection, membership rule, capture assertion and
  sorted distinct admitted observation identities. Equality is over the
  key and nothing else; display name, source path, span, message text, locale
  and rendering never participate.
- **Key order** — none. Unlike a window, a snapshot has no time bounds.
  Ordering snapshots by a capture timestamp would supply an order the prose does
  not state, and FR-113 states that "Arbitrary timestamp order SHALL NOT
  establish causality."
- **Causal order** — none.
- **Absence** — an absent key component compares only to absence. A missing
  selected identity is refused rather than supplied by an ambient default
  (FR-112 Behavior, FR-112-AC-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-265-AC-1 | A selected snapshot and a selected event-time window compare unequal and are not substitutable for one another. | Test (TC-142) |
| FR-265-AC-2 | Changing the selected snapshot yields a different assessment subject. | Test (TC-142) |
| FR-265-AC-3 | A missing selected snapshot identity compares only to absence and receives no ambient default. | Test (TC-142) |
| FR-265-AC-4 | Two snapshots compare equal exactly when every key component is equal; every key mutation makes them unequal and presentation permutation does not. | Test (TC-142) |
| FR-265-AC-5 | Timestamp or ingestion order cannot replace the capture assertion or supply an order over snapshots. | Test (TC-142) |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the snapshot/window distinction and the refusal cases.
- [FR-266](FR-266-event-time-window.md) types the other selected bounds subject.
- [FR-263](FR-263-declared-population.md) is the population the snapshot bounds.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Snapshot boundary

A snapshot is an immutable capture selection, not a time window. FR-266's
half-open bounds never apply to it, and its capture assertion does not create a
timestamp-derived causal or total order.

## Status

Proposed snapshot identity closure for `quire-specification#40`.
