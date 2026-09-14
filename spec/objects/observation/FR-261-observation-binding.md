---
id: FR-261
title: "Observation binding"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-261: Observation binding

## Description

When a selected binding-contract definition, revision, or byte digest changes, the observation binder SHALL require a newly linked subject rather than retaining the existing static subject.

FR-110-AC-4 states the same rule in both directions: a changed definition,
revision or byte digest "requires a newly linked subject, while a changed
concrete mapping or admitted record set under the same selection remains an
identified assessment input".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Selected binding-contract definition identity | FR-110 Description | Retained exactly. |
| Revision | FR-110 Description | Retained exactly. |
| Byte digest | FR-110 Description | Retained exactly; its domain is selected under [FR-201](../foundation/FR-201-canonical-identity-domain.md). |
| Declared binding identity | FR-110 Description | The declared binding named by the observation. |
| Field units | FR-110 Description | A "compatible-looking field" is not a unit (FR-110 Behavior). |
| Concrete mapping | FR-110 Behavior, FR-110-AC-4 | Changing it under the same selection does not change the static subject; it identifies a changed assessment input. |
| Admitted record set | FR-110 Behavior, FR-110-AC-4 | As for the concrete mapping. |

The [observation binding contract](../../../proposals/quire-v1/observation-contract.md)
states that a declared observation role has "the identity of its owning native
declaration, original source region, and role kind; its local name is diagnostic
text and never a global lookup key", and that "equal local names never merge
roles".

## Identity and comparison

- **Identity kind** — *value*. A binding selection has no identity of its own;
  it is compared componentwise over its selection triple.
- **Key** — (`definition identity`, `revision`, `byte digest`). FR-110-AC-4
  establishes exactly this set as the subject-changing set: a change to any one
  of the three requires a newly linked subject, and a change outside the three
  does not. Equality is componentwise over the key and nothing else. The
  concrete mapping and the admitted record set are explicitly outside the key
  (FR-110-AC-4), and display name, source path, span, message text, locale and
  rendering never participate — a role's "local name is diagnostic text and
  never a global lookup key"
  ([observation binding contract](../../../proposals/quire-v1/observation-contract.md)).
- **Key order** — lexicographic over (`definition identity`, `revision`,
  `byte digest`), for collection comparison only. It carries no precedence,
  no recency and no causal meaning; a higher revision is not a causal successor
  of a lower one, and FR-113 states that "Arbitrary timestamp order SHALL NOT
  establish causality."
- **Causal order** — none. A binding selection participates in no causal
  structure.
- **Absence** — an absent key component compares only to absence. FR-110
  requires that the exact identities be retained "or refuse the observation with
  a typed cause"; an absent selection is never defaulted to a current, latest or
  compatible-looking definition (FR-110 Behavior, FR-110-AC-1).
- **Digest domain** — FR-110 selects `raw-artifact-digest` over the exact
  binding-contract definition bytes. An identity crossing domains is unequal
  regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-261-AC-1 | Two bindings differing in definition identity, revision or byte digest compare unequal and link different subjects. | Test |
| FR-261-AC-2 | Two bindings differing only in concrete mapping or admitted record set compare equal on the key and retain the same static subject. | Test |
| FR-261-AC-3 | Two bindings with equal digest bytes selected under different identity domains compare unequal. | Test |
| FR-261-AC-4 | Equal local role names do not merge two roles, and a local name does not participate in the key. | Test |

## Dependencies

- [FR-110](../../functional/observation/FR-110-bind-typed-observations.md) establishes the
  retained selection, the subject-change rule and the substitution prohibitions.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [Observation binding contract](../../../proposals/quire-v1/observation-contract.md)
  supplies the role-identity and local-name rules.

## Definition digest

The selected binding-contract definition uses FR-201 `raw-artifact-digest`.
Concrete mappings and admitted record sets remain assessment inputs outside the
static binding key.

## Status

Proposed digest-domain closure for `quire-specification#40`.
