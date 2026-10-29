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

> BLOCKED-BY QSpec #513: this file still cites the removed `sha256-jcs` / `quire.verification.jcs` identity domain (FR-201); delete that wording when #513 lands.

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
| Required members | FR-112 Behavior | Under v1, the complete sorted distinct bounded semantic-member identity set. |

FR-112 Behavior requires that members be "exactly listed/digested", and that
"Unknown required membership ... cannot establish aggregate success."

## Identity and comparison

- **Identity kind** — *derived*. The v1 rule identity is `sha256-jcs:<hex>` over
  the RFC 8785 canonical bytes of `{"identityVersion":"quire.observation.explicit-members-identity/v1-draft.1","members":[...]}`.
- **Key** — (contract `quire.observation.explicit-members/v1`, derived rule
  identity, FR-201 `quire.verification.jcs` domain). FR-112-AC-1 establishes that
  changing the membership rule changes the assessment subject, and FR-112-AC-4
  that a membership-digest mismatch refuses; both components therefore
  participate. Equality is componentwise over the key and nothing else; display
  name, source path, span, message text, locale and rendering never participate.
  The required member set is inside the derived identity preimage. The observed
  subset is not.
- **Key order** — lexicographic over (`rule identity`, `rule digest`), for
  collection comparison only. It carries no precedence, no recency and no causal
  meaning.
- **Causal order** — none. A membership rule participates in no causal
  structure.
- **Absence** — an absent key component compares only to absence. An absent rule
  or digest is refused "rather than supplying a population, membership,
  configuration, or closure input" (FR-112 Behavior); it is never defaulted to a
  current or compatible-looking rule, and never to an empty population.
- **Digest domain** — `quire.verification.jcs`; a crossing domain is unequal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-264-AC-1 | Two membership rules differing in identity or digest compare unequal and yield different assessment subjects. | Test (TC-142) |
| FR-264-AC-2 | An absent membership rule or digest compares only to absence and receives no ambient default. | Test (TC-142) |
| FR-264-AC-3 | Two rules with equal digest bytes selected under different identity domains compare unequal. | Test (TC-142) |
| FR-264-AC-4 | Changing the required member set changes the rule identity; changing only the observed subset leaves the rule key unchanged and affects completeness instead. | Test (TC-142) |
| FR-264-AC-5 | Duplicate, unsorted on-wire, unbounded, unknown-contract or same-identity/unequal-member rules refuse without a partial population. | Test (TC-142) |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the declared finite membership rule and its refusal cases.
- [FR-263](FR-263-declared-population.md) is the population the rule bounds.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## V1 rule form

The v1 contract admits exactly an explicit finite required-member set. A future
query or predicate rule requires a successor contract and cannot be interpreted
under this identity.

## Status

Proposed v1 membership closure for `quire-specification#40`.
