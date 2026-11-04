---
id: FR-246
title: "Protocol adequacy state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-246: Protocol adequacy state vocabulary

## Description

When a result states whether the exact declared obligation and case set was exercised, the result producer SHALL carry exactly one protocol-adequacy state from this closed vocabulary, which is never represented as E02 observation adequacy or as an evidence-sufficiency decision.

## Values

| Value | Description |
| --- | --- |
| `demonstrated` | Every assessment execution and required input for the declared case set is complete and the exact required case set is present. Subject closure is required only when the declared adequacy claim depends on it. An obligation-path demonstration additionally requires `triggered` activation, and observation adequacy remains independently `adequate` (FR-061). |
| not_demonstrated | Wire spelling `not-demonstrated`. The exact declared obligation and case set was not demonstrated (FR-061). An untriggered compensator, absent external observation, unsupported obligation or missing trigger telemetry cannot become demonstrated success (FR-115). |
| `inapplicable` | The declared obligation and case set does not apply (FR-061). |
| `unknown` | Whether the declared obligation and case set was exercised is not known (FR-061). |

## Identity and comparison

- **Identity kind** — *value*. A protocol-adequacy label has no identity of its
  own and is compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states a strict
  reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no sufficiency ranking and no causal meaning.
- **Causal order** — none. Protocol adequacy is not a causal node; the exercised
  obligations and cases it summarizes carry their own identities.
- **Absence** — an absent protocol adequacy compares only to absence. It is
  distinct from the supplied values `unknown` and `inapplicable` and is never
  defaulted to either.

## Retention boundary

FR-062 places this axis outside the projection: "Protocol adequacy, completeness,
decision-scope progress/closure, surrounding-execution progress/closure,
complete-global-conformance closure, settlement basis and claim | Retained in the
canonical protocol-result artifact; never substituted for an E02 finding axis."
FR-062 further states that the adapter "SHALL describe protocol adequacy only in
the canonical protocol result through the exact exercised protocol obligations
and cases. It SHALL NOT represent protocol adequacy as E02 observation adequacy
or as an evidence-sufficiency decision." The disjoint observation-adequacy
vocabulary is [FR-245](FR-245-observation-adequacy-state.md); the two share only
the member `unknown` spelling and are never interchangeable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-246-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-246-AC-2 | A protocol-adequacy member is never emitted as an E02 observation-adequacy value or as a sufficiency decision. | Analysis |
| FR-246-AC-3 | An absent protocol adequacy compares only to absence and is never defaulted. | Analysis |
| FR-246-AC-4 | `demonstrated` on an obligation path is refused without `triggered` activation and independently `adequate` observation adequacy. | Analysis |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  four members and the demonstration rule.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the retention boundary.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
