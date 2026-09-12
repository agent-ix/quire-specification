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

| Property | Source | Note |
| --- | --- | --- |
| Event time | FR-113 Description, FR-113-AC-2 | The instant the observed event carries. Retained as a separate fact from ingestion time, and the axis the half-open window bounds of [FR-266](FR-266-event-time-window.md) are compared against (FR-113 Behavior). |
| Ingestion time | FR-113 Description, FR-113-AC-2 | The instant the record was taken in. Retained as a separate fact from event time; a late event "is retained as late and cannot silently replace the selected settled result" (FR-113-AC-3). |
| Causal relation | FR-113 Behavior, FR-113-AC-2 | The declared relation between subjects. FR-113 Behavior: "Arbitrary timestamp order SHALL NOT establish causality"; the relation is declared under [FR-262](FR-262-declared-instance-relationship.md) and never derived from either time fact. |
| Clock uncertainty | FR-113 Description, FR-113-AC-2 | The interpretation of the declared clock. FR-113 Description retains "declared clock interpretation" among its required inputs, and a mismatched "clock selection" remains an incomplete or refused input rather than settling an assessment (FR-113 Behavior, FR-113-AC-4). |

The four are held apart as facts. They are not four spellings of one timestamp,
and no requirement admits deriving any one of them from another. The Property
column above repeats FR-113's own English wording; it is not a declaration of a
field name, a label spelling or a wire form. See **Open gap**.

## Identity and comparison

- **Identity kind** — *value*. The time facts of an observation have no identity
  of their own; they are compared componentwise as the four facts above,
  alongside the observation they belong to.
- **Key** — **none is established.** No requirement states a key over these
  facts. FR-113 requires the four to remain distinguishable and requires the
  clock selection, assertion definition, scope and source set to be retained
  exactly, but it states neither a component subset that identifies an
  observation's time facts nor a comparison rule over them. This artifact
  supplies none. Where the four facts are compared, they are compared
  componentwise and nothing else participates: display name, source path, span,
  message text, locale and rendering never participate.
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
- **Digest domain** — not applicable; none of these four facts is a digest. The
  progress/assertion definition digest that accompanies them is typed by
  [FR-268](FR-268-progress-and-late-data-authority.md) and carries a domain
  selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-267-AC-1 | Event time, ingestion time, causal relation and clock uncertainty are readable independently of one another for the same observation. | Analysis |
| FR-267-AC-2 | Ordering two observations by either time fact does not establish a causal relation between them. | Analysis |
| FR-267-AC-3 | An absent clock interpretation compares only to absence and does not resolve to a default clock. | Analysis |
| FR-267-AC-4 | The four facts are exactly those named by FR-113-AC-2 and no fact has been added. | Inspection |
| FR-267-AC-5 | No key, key order or label spelling over these facts is asserted by this artifact or inferred by a reader. | Inspection |

## Dependencies

- [FR-113](../../functional/FR-113-preserve-time-progress-and-late-data.md)
  establishes the four facts, their distinctness and the causality prohibition.
- [FR-262](FR-262-declared-instance-relationship.md) types the declared causal
  relation.
- [FR-266](FR-266-event-time-window.md) supplies the window bounds compared
  against event time.
- [FR-268](FR-268-progress-and-late-data-authority.md) types the progress
  authority these facts are interpreted against.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement fixes a label spelling or a wire form for any of these four
facts. FR-113 names them in English prose — "event time", "ingestion time",
causality and "declared clock interpretation" — and nowhere establishes a
hyphenated, underscored or otherwise serialized spelling, a field name or a
type for any of them. The Property column above repeats the prose wording and
declares no label; because no spelling is established, no exact-string
comparison over such a label is established either, and none is asserted here.

Whether `Clock uncertainty` in FR-113-AC-2 and "declared clock interpretation"
in FR-113 Description name the same fact or two related facts is not stated
anywhere, and is not resolved here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
