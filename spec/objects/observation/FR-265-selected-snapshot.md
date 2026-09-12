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

## Identity and comparison

- **Identity kind** — **the prose does not place a snapshot in FR-200's
  authored / derived / value trichotomy.** FR-112 calls the snapshot
  *selected* and *declared* and requires its identity to be retained, without
  stating whether that identity is written in native source or computed. See
  **Open gap**.
- **Key** — **the prose does not fix the key components.** It fixes only that
  the snapshot identity participates in the assessment subject (FR-112-AC-1) and
  that it does not merge with a window identity (FR-112 Behavior). Equality is
  componentwise over the key and nothing else; display name, source path, span,
  message text, locale and rendering never participate. This artifact supplies
  no key components.
- **Key order** — none. Unlike a window, a snapshot is given no bounds by the
  prose, so no total order over snapshots is stated and none is supplied here.
  Ordering snapshots by a capture timestamp would supply an order the prose does
  not state, and FR-113 states that "Arbitrary timestamp order SHALL NOT
  establish causality."
- **Causal order** — none.
- **Absence** — an absent key component compares only to absence. A missing
  selected identity is refused rather than supplied by an ambient default
  (FR-112 Behavior, FR-112-AC-4).
- **Digest domain** — the prose names no snapshot digest. Where a snapshot
  identity is carried beside a digest in provenance, that digest's domain is
  selected under `ix://agent-ix/quire-specification/FR-201`, and an identity
  crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-265-AC-1 | A selected snapshot and a selected event-time window compare unequal and are not substitutable for one another. | Analysis |
| FR-265-AC-2 | Changing the selected snapshot yields a different assessment subject. | Analysis |
| FR-265-AC-3 | A missing selected snapshot identity compares only to absence and receives no ambient default. | Analysis |
| FR-265-AC-4 | This artifact declares no identity kind, no key components and no key order for a snapshot, and cites the prose instead. | Inspection |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the snapshot/window distinction and the refusal cases.
- [FR-266](FR-266-event-time-window.md) types the other selected bounds subject.
- [FR-263](FR-263-declared-population.md) is the population the snapshot bounds.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement states a snapshot's identity kind, its key components, or any
bounds or order over snapshots. FR-112 establishes only that a snapshot is
selected, that its identity is retained, that changing it changes the assessment
subject, and that it stays distinct from an event-time window. In particular,
the half-open boundary rule that FR-112 and FR-113 state for a window is stated
for the window only and is not stated for a snapshot.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
