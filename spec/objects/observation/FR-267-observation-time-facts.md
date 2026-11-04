---
id: FR-267
title: "Distinguishable observation time facts"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
---
# FR-267: Distinguishable observation time facts

## Description

When a timed obligation is assessed, the observation service SHALL keep event time, ingestion time, causal relation, and clock uncertainty distinguishable from one another.

FR-113-AC-2 names exactly these four facts and requires that they "remain
distinguishable".

These four are co-present facts of one observation, not alternative values of
one slot. FR-113 Description retains all four at once and FR-113-AC-2 requires
all four to remain distinguishable at the same time, so this artifact records
them as the components of one value and not as a vocabulary a single field
selects from.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| event_time | SelectedEventTimeWindow | 1..1 |  |
| ingestion_time | String | 1..1 |  |
| causal_relation | DeclaredInstanceRelationship | 1..1 |  |
| clock_uncertainty | String | 1..1 |  |

## Field sources

| Property | Source | Note |
| --- | --- | --- |
| Event time | FR-113 Description, FR-113-AC-2 | The instant the observed event carries. Retained as a separate fact from ingestion time, and the axis the half-open window bounds of [FR-266](FR-266-event-time-window.md) are compared against (FR-113 Behavior). |
| Ingestion time | FR-113 Description, FR-113-AC-2 | The instant the record was taken in. Retained as a separate fact from event time; a late event "is retained as late and cannot silently replace the selected settled result" (FR-113-AC-3). |
| Causal relation | FR-113 Behavior, FR-113-AC-2 | The declared relation between subjects. FR-113 Behavior: "Arbitrary timestamp order SHALL NOT establish causality"; the relation is declared under [FR-262](FR-262-declared-instance-relationship.md) and never derived from either time fact. |
| Clock uncertainty | FR-113 Description, FR-113-AC-2 | The interpretation of the declared clock. FR-113 Description retains "declared clock interpretation" among its required inputs, and a mismatched "clock selection" remains an incomplete or refused input rather than settling an assessment (FR-113 Behavior, FR-113-AC-4). |

The exact v1 member labels are `event-time`, `ingestion-time`,
`causal-relation`, and `clock-uncertainty`. They are held apart as facts, not
four spellings of one timestamp. `clock-uncertainty` is the uncertainty value
interpreted by the separately selected declared clock; it is not a second clock
axis and cannot supply a missing clock selection.

## Identity and comparison

- **Identity kind** — *value*. The time facts of an observation have no identity
  of their own; they are compared componentwise as the four facts above,
  alongside the observation they belong to.
- **Key** — the exact four-member product keyed by the labels above. Values are
  compared under the selected clock and relationship contracts; display name,
  source path, span, message text, locale and rendering never participate.
- **Key order** — **none is established.** No requirement states an order over
  these four facts, and the order in which they are written above is
  presentation, not an order over anything. An order over the instants carried
  under `Event time` or `Ingestion time` is likewise not a causal order: FR-113
  Behavior states that "Arbitrary timestamp order SHALL NOT establish
  causality".
- **Causal order** — the causal-relation fact is itself the declared
  relationship structure of [FR-262](FR-262-declared-instance-relationship.md).
  It is never derived from a field order, from a timestamp, from a trace ID or
  from a shared provider identity (FR-111 Behavior, FR-113 Behavior).
- **Absence** — a fact that was not supplied is explicitly absent and compares
  only to absence. An absent event time is not ingestion time, an absent causal
  relation is not an absent edge inferred from timestamps, and an absent clock
  interpretation is not a default clock: a mismatched or missing clock selection
  "SHALL retain an incomplete or refused input rather than settle or restart from
  it" (FR-113 Behavior). An absent fact is never filled from one of the other
  three.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-267-AC-1 | Event time, ingestion time, causal relation and clock uncertainty are readable independently of one another for the same observation. | Test (TC-143) |
| FR-267-AC-2 | Ordering two observations by either time fact does not establish a causal relation between them. | Test (TC-143) |
| FR-267-AC-3 | An absent clock interpretation compares only to absence and does not resolve to a default clock. | Test (TC-143) |
| FR-267-AC-4 | The four exact labels are `event-time`, `ingestion-time`, `causal-relation`, and `clock-uncertainty`; aliases and unknown labels are refused. | Test (TC-143) |
| FR-267-AC-5 | Clock uncertainty is interpreted only under the exact selected clock and never supplies or replaces that selection. | Test (TC-143) |

## Dependencies

- [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md)
  establishes the four facts, their distinctness and the causality prohibition.
- [FR-262](FR-262-declared-instance-relationship.md) types the declared causal
  relation.
- [FR-266](FR-266-event-time-window.md) supplies the window bounds compared
  against event time.
- [FR-268](FR-268-progress-and-late-data-authority.md) types the progress
  authority these facts are interpreted against.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Clock-selection boundary

The declared clock selection is authority and context; `clock-uncertainty` is
one observation fact interpreted within it. Neither is derivable from the
other.

## Status

Proposed time-fact vocabulary closure for `quire-specification#40`.
