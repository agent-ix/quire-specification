---
id: FR-280
title: "Assumption state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-280: Assumption state vocabulary

## Description

When a result retains an assumption, the result producer SHALL carry exactly one state from this closed vocabulary alongside that assumption's exact identity.

FR-061 states it directly: "Every assumption SHALL retain its exact assumption
identity and one state from `met`, `not-met`, `unknown` or `not-assessed`."

## Values

| Value | Description |
| --- | --- |
| `met` | The assumption was assessed and holds (FR-061). |
| `not-met` | The assumption was assessed and does not hold (FR-061). |
| `unknown` | The assumption was assessed but its outcome could not be established (FR-061). |
| `not-assessed` | The assumption was not assessed at all. It is distinct from `unknown`, which records an attempted assessment (FR-061). |

## Identity and comparison

- **Identity kind** — *value*. A state label has no identity of its own and is
  compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. The assumption's own identity is
  a separate component of the retained record and is not part of this label.
  Display name, source path, span, message text, locale and rendering never
  participate; FR-061 requires that a strict reader "SHALL NOT infer
  independence from distinct display names or implementations".
- **Key order** — lexicographic over the label, for collection comparison only.
  It ranks nothing: `met` is not "better than" `not-met`, and the order carries
  no causal meaning.
- **Causal order** — none.
- **Absence** — an assumption retained without a state has an absent state,
  which compares only to absence. It is never defaulted to `not-assessed` or to
  `unknown`; those are two distinct assessed outcomes.
- **Digest domain** — not applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-280-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-280-AC-2 | `unknown` and `not-assessed` remain distinct and neither is a default for an absent state. | Analysis |
| FR-280-AC-3 | An assumption state compares equal only when its label compares equal, independently of the assumption's display name. | Analysis |

## Dependencies

- [FR-061](../../functional/FR-061-report-orthogonal-results.md) establishes the
  vocabulary and the retention obligation.
- [FR-062](../../functional/FR-062-project-results-to-portable-verification.md)
  carries it to the portable verification boundary.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
