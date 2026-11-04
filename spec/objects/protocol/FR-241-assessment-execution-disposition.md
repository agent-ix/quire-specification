---
id: FR-241
title: "Assessment execution disposition vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-241: Assessment execution disposition vocabulary

## Description

When a result records whether the requested evaluator ran, the strict reader SHALL match the assessment-execution state as an exact string from this closed vocabulary, under which no state other than `completed` carries a successful truth value.

## Values

| Value | Description |
| --- | --- |
| `completed` | The requested evaluator completed. Required for truth `satisfied`, `violated` or `pending` (FR-061 result-combination rules). Projects to E02 `execution.status` `completed` (FR-062). |
| `refused` | The evaluator refused the request. Truth and settlement basis are unavailable (FR-061). Projects to E02 `execution.status` `invalid-input` (FR-062). |
| `unsupported` | The requested capability is unsupported. Truth and settlement basis are unavailable (FR-061). One unsupported request does not remove an independent supported result (FR-061-AC-5). Projects to E02 `unsupported` (FR-062). |
| resource_incomplete | Wire spelling `resource-incomplete`. A resource ceiling stopped the assessment. Truth and settlement basis are unavailable (FR-061); an independently settled sibling result is unchanged (FR-094-AC-9). Projects to E02 `exhausted` (FR-062). |
| `failed` | The selected analysis backend failed for this exact claim, without invoking a fallback or changing independent results (FR-060). Truth and settlement basis are unavailable (FR-061). Projects to E02 `failed` (FR-062). |

## Identity and comparison

- **Identity kind** — *value*. A disposition label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact; spellings are not normalized
  aliases. Display name, source path, span, message text, locale and rendering
  never participate in identity. FR-061 states this directly for the sibling
  decision-support axis: a strict reader "SHALL NOT infer independence from
  distinct display names or implementations".
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no severity ranking, precedence or causal meaning; `failed` is not
  ordered after `completed` by this order.
- **Causal order** — none. An execution disposition is not a causal node and is
  never derived from key order, timestamp or source position.
- **Absence** — an absent assessment-execution state compares only to absence. It
  is never defaulted to `completed` or to any other member. FR-061 refuses "an
  omitted required axis".

## Substitution prohibitions

FR-061: "The result producer SHALL NOT attach a successful truth value to a
refused, unsupported, failed or resource-incomplete execution." FR-061 further
forbids using assessment execution "as a substitute for any closure axis": a
`completed` execution is not decision-scope closure, surrounding-execution
closure or complete-global-conformance closure.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-241-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-241-AC-2 | A result carrying `refused`, `unsupported`, `resource-incomplete` or `failed` with truth `satisfied` or `violated` is refused. | Analysis |
| FR-241-AC-3 | An absent assessment-execution state compares only to absence and receives no default member. | Analysis |
| FR-241-AC-4 | `completed` is not read as evidence of any progress or closure axis. | Analysis |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md) establishes the
  failed backend disposition.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  five states and their combination rules.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the total projection of each state into E02.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
