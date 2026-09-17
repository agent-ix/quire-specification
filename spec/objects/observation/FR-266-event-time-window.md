---
id: FR-266
title: "Selected event-time window"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-265
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
---
# FR-266: Selected event-time window

## Description

When a window is selected, the assessment service SHALL include an exactly listed or digested member at `start` and exclude one at `end`.

FR-112 Behavior states the window as `[start, end)` and adds that the excluded
member "can participate only in a separately selected later window". FR-113
Behavior applies progress and closure "only to admitted members with instants
`start <= instant < end`".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Window identity | FR-112 Description | Retained among "population and snapshot/window identities". |
| `start` | FR-112 Behavior | Inclusive. A member at `start` is included. |
| `end` | FR-112 Behavior | Exclusive. A member at `end` is excluded and participates only in a separately selected later window. |
| Bounds | FR-112 Description | Retained with the window identity. |
| Distinctness from a snapshot | FR-112 Behavior | Typed by [FR-265](FR-265-selected-snapshot.md). |
| Progress and closure applicability | FR-113 Behavior | `start <= instant < end`. |

FR-112-AC-5 requires that "A selected half-open window includes an exact member
at `start`, excludes one at `end`, and cannot double-count either across
windows." FR-113-AC-5 requires that "Progress/closure includes the selected
window start, excludes its end, and cannot settle or contradict a later window
through that boundary."

## Identity and comparison

- **Identity kind** — *value*. A window selection is compared componentwise over
  its identity and bounds; FR-112 gives it no identity separable from the
  selection it names.
- **Key** — (`window identity`, `start`, `end`). FR-112-AC-1 establishes that
  changing the declared window changes the assessment subject, and FR-112
  Behavior and FR-113 Behavior both make the two bounds semantically
  load-bearing. Equality is componentwise over the key and nothing else; display
  name, source path, span, message text, locale and rendering never participate.
  The admitted member set inside the window is not part of the window's key.
- **Key order** — lexicographic over (`start`, `end`), for collection comparison
  only. FR-112 Behavior supports orderability by naming "a separately selected
  later window" for the excluded `end` member. The order carries **no causal
  meaning**: FR-113 Behavior states that "Arbitrary timestamp order SHALL NOT
  establish causality", and FR-113-AC-5 forbids a window settling or
  contradicting a later window through the shared boundary.
- **Causal order** — none. Adjacency of two windows is not a causal edge, and
  the boundary between them transmits neither settlement nor contradiction
  (FR-113-AC-5).
- **Absence** — an absent key component compares only to absence. A missing
  selected window identity or bound is refused rather than supplied by an
  ambient default (FR-112 Behavior, FR-112-AC-4). A window with no admitted
  member is not the same as a window whose membership is unknown: "Unknown
  required membership ... cannot establish aggregate success" (FR-112 Behavior).
- **Digest domain** — the prose names no window digest in FR-112 or FR-113. The
  [observation binding contract](../../../proposals/quire-v1/observation-contract.md)
  names a "selected window identity/digest" retained by a selected
  `quire.observation.range/v1` definition; where such a digest is carried, its
  domain is selected under `ix://agent-ix/quire-specification/FR-201`, and an
  identity crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-266-AC-1 | A member at `start` is inside the selected window and a member at `end` is outside it. | Test |
| FR-266-AC-2 | A member excluded at `end` is admitted only by a separately selected later window and is not counted twice. | Test |
| FR-266-AC-3 | The order over window bounds is used only to compare windows and is never cited as a causal edge between them. | Test |
| FR-266-AC-4 | A selected event-time window and a selected snapshot compare unequal. | Test |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the half-open window and the later-window rule.
- [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md)
  restricts progress and closure to `start <= instant < end` and forbids
  timestamp-derived causality.
- [FR-265](FR-265-selected-snapshot.md) types the other selected bounds subject.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Cross-family boundary rule

This object remains half-open. FR-289 maps it to an inclusive temporal interval
by requiring an explicit same-clock carrier end strictly greater than the
inclusive upper bound and filtering assessment membership to the inclusive
predicate. The mapping neither includes this object's `end` nor changes the
membership of an adjacent observation window.

## Status

Proposed cross-family reconciliation under FR-289 for
`quire-specification#40`.
