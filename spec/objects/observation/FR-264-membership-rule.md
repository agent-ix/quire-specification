---
id: FR-264
title: "Declared membership rule"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-264: Declared membership rule

## Description

When a membership rule is declared, the assessment request SHALL retain its exact rule identity and digest rather than accepting an ambient default.

FR-112 Description names "declared finite membership rule/digest" among the
retained identities, and FR-112-AC-4 requires that a "membership-digest ...
mismatch is refused without ambient defaulting".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Rule identity | FR-112 Description | "declared finite membership rule". |
| Rule digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses; its domain is selected under [FR-201](../foundation/FR-201-canonical-identity-domain.md). |
| Finiteness | FR-112 Description | The rule is declared *finite*. |

FR-112 Behavior requires that members be "exactly listed/digested", and that
"Unknown required membership ... cannot establish aggregate success."

## Identity and comparison

- **Identity kind** — *value*. A membership rule selection has no identity of
  its own; it is compared componentwise over its identity and digest.
- **Key** — (`rule identity`, `rule digest`). FR-112-AC-1 establishes that
  changing the membership rule changes the assessment subject, and FR-112-AC-4
  that a membership-digest mismatch refuses; both components therefore
  participate. Equality is componentwise over the key and nothing else; display
  name, source path, span, message text, locale and rendering never participate.
  The admitted member set produced by applying the rule is not part of the
  rule's key.
- **Key order** — lexicographic over (`rule identity`, `rule digest`), for
  collection comparison only. It carries no precedence, no recency and no causal
  meaning.
- **Causal order** — none. A membership rule participates in no causal
  structure.
- **Absence** — an absent key component compares only to absence. An absent rule
  or digest is refused "rather than supplying a population, membership,
  configuration, or closure input" (FR-112 Behavior); it is never defaulted to a
  current or compatible-looking rule, and never to an empty population.
- **Digest domain** — the rule digest carries a selected domain under
  `ix://agent-ix/quire-specification/FR-201`. An identity crossing domains is
  unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-264-AC-1 | Two membership rules differing in identity or digest compare unequal and yield different assessment subjects. | Analysis |
| FR-264-AC-2 | An absent membership rule or digest compares only to absence and receives no ambient default. | Analysis |
| FR-264-AC-3 | Two rules with equal digest bytes selected under different identity domains compare unequal. | Analysis |
| FR-264-AC-4 | Changing the admitted member set under the same rule identity and digest leaves the rule's key unchanged. | Analysis |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the declared finite membership rule and its refusal cases.
- [FR-263](FR-263-declared-population.md) is the population the rule bounds.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-112 does not state which identity domain the membership digest is derived
under, and does not state the form of a membership rule beyond that it is
declared and finite. This artifact selects neither.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
