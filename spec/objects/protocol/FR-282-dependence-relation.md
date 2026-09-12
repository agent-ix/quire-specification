---
id: FR-282
title: "Decision-support dependence relation vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-282: Decision-support dependence relation vocabulary

## Description

When a decision-support record is retained, the result producer SHALL carry exactly one dependence relation from this closed vocabulary alongside that record's exact oracle and observation identities.

FR-061 states it directly: "Every retained decision-support record SHALL
identify its exact oracle and observation plus one dependence relation from
`independent`, `shared`, `derived` or `unknown`."

## Values

| Value | Description |
| --- | --- |
| `independent` | The record does not depend on another retained record. FR-061 forbids inferring this: a strict reader "SHALL NOT infer independence from distinct display names or implementations". |
| `shared` | The record belongs to a dependence group, and that group SHALL be identified (FR-061). |
| `derived` | The record is derived from a predecessor, and that exact predecessor SHALL be identified (FR-061). |
| `unknown` | The dependence relation was not established. It is not independence (FR-061). |

A strict reader SHALL refuse a missing, cyclic or self-referential predecessor
(FR-061, FR-061-AC-7).

## Identity and comparison

- **Identity kind** — *value*. A relation label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. The identified predecessor or
  dependence group is a separate component of the retained record, not part of
  this label. Display name, source path, span, message text, locale, rendering
  and implementation never participate — FR-061 makes that exclusion explicit
  for this very axis.
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no causal meaning and no confidence ranking; in particular
  `unknown` is not a weaker `independent`.
- **Causal order** — the `derived` relation induces a predecessor relation over
  *records*, which must be acyclic: a cyclic or self-referential predecessor is
  refused (FR-061-AC-7). That relation holds between records, never between
  these labels, and is never derived from key order.
- **Absence** — a decision-support record retained without a dependence relation
  has an absent relation, which compares only to absence. It is never defaulted
  to `independent`; FR-061 forbids inferring independence.
- **Digest domain** — not applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-282-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-282-AC-2 | An absent relation compares only to absence and is never read as `independent`. | Analysis |
| FR-282-AC-3 | A `derived` record without an identified predecessor, or with a cyclic or self-referential one, is refused. | Analysis |
| FR-282-AC-4 | Two records with distinct display names or implementations are not thereby `independent`. | Analysis |

## Dependencies

- [FR-061](../../functional/FR-061-report-orthogonal-results.md) establishes the
  vocabulary, the identification obligations and the refusals.
- [FR-062](../../functional/FR-062-project-results-to-portable-verification.md)
  carries it to the portable verification boundary.
- [FR-248](FR-248-decision-support-record.md) is the record this relation is
  carried on.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
