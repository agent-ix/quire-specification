---
id: FR-261
title: "Observation binding"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: specifies
---
# FR-261: Observation binding

## Description

When the selected binding-contract definition identity changes, the observation binder SHALL require a newly linked subject rather than retaining the existing static subject.

FR-110-AC-4 states the same rule in both directions: a changed definition
identity "requires a newly linked subject, while a changed
concrete mapping or admitted record set under the same selection remains an
identified assessment input".

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| selected_binding_contract_definition_identity | String | 1..1 |  |
| declared_binding_identity | String | 1..1 |  |
| field_units | String | 1..1 |  |
| concrete_mapping | String | 1..1 |  |
| admitted_record_set | String | 1..1 |  |

## Field sources

| Property | Source | Note |
| --- | --- | --- |
| Selected binding-contract definition identity | FR-110 Description | Retained exactly. |
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
  it is compared by its selected definition identity.
- **Key** — `definition identity`. FR-110-AC-4 establishes it as the
  subject-changing component: a change to it requires a newly linked subject,
  and a change outside it does not. Equality is over the key and nothing else. The
  concrete mapping and the admitted record set are explicitly outside the key
  (FR-110-AC-4), and display name, source path, span, message text, locale and
  rendering never participate — a role's "local name is diagnostic text and
  never a global lookup key"
  ([observation binding contract](../../../proposals/quire-v1/observation-contract.md)).
- **Key order** — bytewise over `definition identity`, for collection
  comparison only. It carries no precedence, no recency and no causal meaning,
  and FR-113 states that "Arbitrary timestamp order SHALL NOT establish
  causality."
- **Causal order** — none. A binding selection participates in no causal
  structure.
- **Absence** — an absent key compares only to absence. FR-110
  requires that the exact identities be retained "or refuse the observation with
  a typed cause"; an absent selection is never defaulted to a current, latest or
  compatible-looking definition (FR-110 Behavior, FR-110-AC-1).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-261-AC-1 | Two bindings differing in definition identity compare unequal and link different subjects. | Test (TC-140) |
| FR-261-AC-2 | Two bindings differing only in concrete mapping or admitted record set compare equal on the key and retain the same static subject. | Test (TC-140) |
| FR-261-AC-3 | Equal local role names do not merge two roles, and a local name does not participate in the key. | Test (TC-140) |

## Dependencies

- [FR-110](../../functional/observation/FR-110-bind-typed-observations.md) establishes the
  retained selection, the subject-change rule and the substitution prohibitions.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [Observation binding contract](../../../proposals/quire-v1/observation-contract.md)
  supplies the role-identity and local-name rules.

## Assessment inputs

Concrete mappings and admitted record sets remain assessment inputs outside the
static binding key.

## Status

Proposed binding identity closure for `quire-specification#40`.
