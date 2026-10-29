---
id: FR-285
title: "Participation fact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-212
    type: references
---
# FR-285: Participation fact

## Description

When a canonical protocol result reports participation, the producer SHALL emit
one closed discriminated participation fact that preserves exact role-instance
references and distinguishes a known required set from one whose membership is
unknown. Participation is not a source status enumeration and an adapter SHALL
derive, not copy, a target status.

## Properties

| Variant | Required properties | Meaning |
| --- | --- | --- |
| `known` | `requiredRoleInstanceRefs`, `participatingRoleInstanceRefs` | The complete required-role-instance set is known. Both collections are finite, duplicate-free and in the canonical order supplied by the applicable FR-212 role-instance identity rule. Every participating reference is a member of the required set. |
| `unknown` | `participatingRoleInstanceRefs`, `missingDependencyRefs`, `reasons` | The complete required-role-instance set is not known. Every known participant and every dependency whose absence prevents determining the required set is retained exactly. `reasons` is a non-empty collection of typed causes; no complete required set is invented. |

Exactly one variant SHALL be present. The properties of the other variant are
forbidden. An empty collection is supplied information and is distinct from an
absent property. Display names, role definitions, component identities,
endpoints and presentation order SHALL NOT substitute for role-instance
identity or its canonical order. Until the selected FR-212 identity rule
establishes a complete key and order, a producer that must construct or reorder
one of these collections SHALL refuse rather than sort by display text or
invent a key.

## Projection correspondence

FR-062 derives the E02 participation status as follows. A `known` fact with an
empty required set becomes `not-required`. A non-empty known set with all, some
or none of its members participating becomes `complete`, `partial` or
`not-started`, respectively. An `unknown` fact becomes `unknown`. Target labels
are not members of this source object and SHALL NOT be copied back into it.

## Identity and comparison

- **Identity kind** — *value*. The fact has no standalone identity.
- **Key** — its discriminator plus every required collection, compared
  componentwise using exact FR-212 role-instance references and exact missing-
  dependency and reason values.
- **Key order** — collection order is the canonical order supplied by the
  selected member-identity rule. No order is inferred from display names,
  source positions or timestamps.
- **Causal order** — none. Participation does not establish event order.
- **Absence** — absence is distinct from an empty supplied collection and from
  the `unknown` variant. Required-property absence is refused.
- **Digest domain** — not applicable; the fact carries no digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-285-AC-1 | A strict reader accepts exactly the `known` and `unknown` shapes and refuses mixed, incomplete or extra variant properties. | Analysis |
| FR-285-AC-2 | Known participation retains exact duplicate-free required and participating role-instance references, and every participant belongs to the required set. | Analysis |
| FR-285-AC-3 | Unknown participation retains every known participant, at least one missing dependency and at least one typed reason without inventing a complete required set. | Analysis |
| FR-285-AC-4 | Empty, absent and unknown participation remain distinguishable, and display text never supplies identity or order. | Analysis |
| FR-285-AC-5 | E02 `complete`, `partial`, `not-started`, `not-required` and `unknown` are derived exactly from the source fact and are never accepted as source variants. | Test (TC-064) |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) owns the
  canonical result participation axis.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  owns the target projection.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  supplies observed participation facts.
- [FR-212](FR-212-role-instance.md) owns role-instance identity and canonical
  member ordering; construction refuses while the selected rule is incomplete.

## Status

Accepted FS02 vocabulary ruling. This object is the complete source
participation shape; no source participation enum exists.
