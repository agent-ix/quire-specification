---
id: FR-245
title: "Observation adequacy state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
---
# FR-245: Observation adequacy state vocabulary

## Description

When a result states whether the exact observations required to interpret the requested result are present, the result producer SHALL carry exactly one observation-adequacy state from this closed vocabulary, which protocol adequacy never establishes.

## Values

| Value | Description |
| --- | --- |
| `adequate` | The exact observations required to interpret the requested result are present (FR-061). A demonstrated protocol adequacy additionally requires observation adequacy to remain independently `adequate` (FR-061). |
| `inadequate` | The required observations are not present (FR-061). A missing required role, provider observation, population member or negative case cannot change this to `adequate` (FR-061-AC-2). |
| `unknown` | Whether the required observations suffice is not known (FR-061). |
| `not-assessed` | Observation adequacy was not assessed (FR-061). |

Each state projects to the same E02 `adequacy.status`, "with exact
missing-observation identities when known" (FR-062).

## Identity and comparison

- **Identity kind** — *value*. An adequacy label has no identity of its own and is
  compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states a strict
  reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no sufficiency ranking and no causal meaning.
- **Causal order** — none. Observation adequacy is not a causal node.
- **Absence** — an absent observation adequacy compares only to absence. It is
  distinct from the supplied values `unknown` and `not-assessed` and is never
  defaulted to either.
- **Digest domain** — not applicable; this key contains no digest. See
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).

## The two adequacy domains are disjoint

FR-061 defines two adequacy domains and forbids either from proving the other:
"The result producer SHALL NOT use either adequacy domain as evidence for the
other." FR-062-AC-3 restates the boundary: "E02 adequacy is derived only from
observation adequacy; protocol adequacy remains in the canonical result and
neither adequacy domain can establish the other or an evidence-sufficiency
decision." The protocol adequacy vocabulary is
[FR-246](FR-246-protocol-adequacy-state.md); it shares no member with this one.

## Recorded divergence: bare "adequacy"

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md): "Observation
  adequacy states whether the exact observations required to interpret the
  requested result are `adequate`, `inadequate`, `unknown` or `not-assessed`."
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  Description and FR-115-AC-4 then write the axis unqualified: "activation,
  participation, completeness, adequacy, claim strength, provenance", and
  "activation, participation, completeness, adequacy, claim strength, and
  provenance remain independently readable".
- [FR-116](../../functional/observation/FR-116-map-observation-results-to-consumers.md) omits
  an adequacy axis from its retained-fact list entirely.

FR-115 does not say which of FR-061's two disjoint adequacy domains its bare
"adequacy" names, and FR-116 names neither. This artifact records the divergence
and does not decide it. The repo review
[`SR-208`](../../reviews/protocol-verification-projection-object.md) FND-002
already flags the same word collision: "Protocol adequacy and E02 observation
adequacy share the word 'adequacy' but have different authorities."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-245-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-245-AC-2 | No protocol-adequacy member satisfies a comparison expecting an observation-adequacy member. | Analysis |
| FR-245-AC-3 | An absent observation adequacy compares only to absence and is never defaulted to `unknown` or `not-assessed`. | Analysis |
| FR-245-AC-4 | A dropped required observation cannot move observation adequacy toward `adequate`. | Analysis |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  four members and the disjointness rule.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the E02 adequacy projection.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  supplies the bare-adequacy spelling recorded above.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
