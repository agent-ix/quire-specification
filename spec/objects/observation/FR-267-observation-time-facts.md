---
id: FR-267
title: "Distinguishable observation time facts"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
---
# FR-267: Distinguishable observation time facts

## Description

When a timed obligation is assessed, the observation service SHALL keep event time, ingestion time, causal relation, and clock uncertainty distinguishable from one another.

FR-113-AC-2 names exactly these four facts and requires that they "remain
distinguishable".

## Values

| Value | Description |
| --- | --- |
| Event time | The instant the observed event carries. Retained by FR-113 Description as a separate fact from ingestion time, and the axis the half-open window bounds of [FR-266](FR-266-event-time-window.md) are compared against (FR-113 Behavior). |
| Ingestion time | The instant the record was taken in. Retained by FR-113 Description as a separate fact from event time; a late event "is retained as late and cannot silently replace the selected settled result" (FR-113-AC-3). |
| Causal relation | The declared relation between subjects. FR-113 Behavior: "Arbitrary timestamp order SHALL NOT establish causality"; the relation is declared under [FR-262](FR-262-declared-instance-relationship.md) and never derived from either time fact. |
| Clock uncertainty | The interpretation of the declared clock. FR-113 Description retains "declared clock interpretation" among its required inputs, and a mismatched "clock selection" remains an incomplete or refused input rather than settling an assessment (FR-113 Behavior, FR-113-AC-4). |

These four are held apart as facts. They are not four spellings of one
timestamp, and no requirement admits deriving any one of them from another.

## Identity and comparison

- **Identity kind** — *value*. A fact label has no identity of its own; it is
  compared componentwise as an exact label.
- **Key** — the label itself. Equality is componentwise over the label and
  nothing else; display name, source path, span, message text, locale and
  rendering never participate.
- **Key order** — the order of the rows above, for collection comparison only.
  It carries no precedence, no preference and **no causal meaning**: FR-113
  Behavior states that "Arbitrary timestamp order SHALL NOT establish
  causality", so neither this row order nor an order over the instants carried
  under `Event time` or `Ingestion time` establishes a causal edge.
- **Causal order** — the `Causal relation` fact is itself the declared
  relationship structure of [FR-262](FR-262-declared-instance-relationship.md).
  It is never derived from key order, from a timestamp, from a trace ID or from
  a shared provider identity (FR-111 Behavior, FR-113 Behavior).
- **Absence** — a fact that was not supplied is explicitly absent and compares
  only to absence. An absent event time is not ingestion time, an absent causal
  relation is not an absent edge inferred from timestamps, and an absent clock
  interpretation is not a default clock: a mismatched or missing clock selection
  "SHALL retain an incomplete or refused input rather than settle or restart from
  it" (FR-113 Behavior).
- **Digest domain** — not applicable; these labels carry no digest. The
  progress/assertion definition digest that accompanies them is typed by
  [FR-268](FR-268-progress-and-late-data-authority.md) and carries a domain
  selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-267-AC-1 | Event time, ingestion time, causal relation and clock uncertainty are readable independently of one another for the same observation. | Analysis |
| FR-267-AC-2 | Ordering two observations by either time fact does not establish a causal relation between them. | Analysis |
| FR-267-AC-3 | An absent clock interpretation compares only to absence and does not resolve to a default clock. | Analysis |
| FR-267-AC-4 | The four values are exactly those named by FR-113-AC-2 and no member has been added. | Inspection |

## Dependencies

- [FR-113](../../functional/FR-113-preserve-time-progress-and-late-data.md)
  establishes the four facts, their distinctness and the causality prohibition.
- [FR-262](FR-262-declared-instance-relationship.md) types the declared causal
  relation.
- [FR-266](FR-266-event-time-window.md) supplies the window bounds compared
  against event time.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-113 names these four facts in English prose and nowhere fixes a label
spelling, a wire form or a type for any of them. The Value column above repeats
the prose wording; it is not a declaration of a serialized label, and no
hyphenated or underscored spelling is established by any requirement. Whether
`Clock uncertainty` in FR-113-AC-2 and "declared clock interpretation" in
FR-113 Description name the same fact or two related facts is not stated.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
