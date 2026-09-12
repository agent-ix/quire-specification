---
id: FR-281
title: "Claim strength vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
---
# FR-281: Claim strength vocabulary

## Description

When a result retains the strength of a claim, the result producer SHALL keep each strength as a distinct source-bound value from this closed vocabulary rather than collapsing them into one success fact.

FR-061 states it directly: "The result producer SHALL retain proven,
bounded-checked, tested and observed claim strengths as distinct source-bound
values."

## Values

| Value | Description |
| --- | --- |
| `proven` | Established by proof (FR-061). |
| `bounded-checked` | Established by checking within a declared finite bound (FR-061). |
| `tested` | Established by test (FR-061). |
| `observed` | Established by observation (FR-061). |

Each value is **source-bound**: it carries the source that established it and
does not transfer to another claim (FR-061).

## Identity and comparison

- **Identity kind** — *value*. A strength label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. Display name, source path, span,
  message text, locale and rendering never participate.
- **Key order** — lexicographic over the label, for collection comparison only.
  **It is not a strength ranking.** FR-061 requires these be retained as
  *distinct* values, not as points on a scale, and nothing in the prose
  establishes that one strength substitutes for or dominates another.
- **Causal order** — none.
- **Absence** — a claim retained without a strength has an absent strength,
  which compares only to absence and is never defaulted to `observed` or to any
  other member.
- **Digest domain** — not applicable; the source binding that establishes a
  strength may carry a digest, whose domain is selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Open gap

FR-061 is the only requirement that names these four values. No requirement
states whether the set is closed, whether a claim may carry more than one
strength, or what relation (if any) holds between them. This artifact records
the four named values and declares no ordering or dominance among them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-281-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-281-AC-2 | Two claims with different strengths remain distinct and neither is promoted to the other. | Analysis |
| FR-281-AC-3 | Key-order adjacency between two strengths does not establish that one dominates the other. | Analysis |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  four strengths and their source binding.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  retains claim strength as a separate fact.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
