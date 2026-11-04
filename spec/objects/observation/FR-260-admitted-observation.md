---
id: FR-260
title: "Admitted observation"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-261
    type: references
  - target: ix://agent-ix/quire-specification/FR-296
    type: references
---
# FR-260: Admitted observation

## Description

When a record is admitted as an observation, the observation binder SHALL retain the exact observation identity, source schema identity, declared binding identity, field units, selected subject identity, and whether the record is internal or externally observed, or refuse the record with a typed cause.

FR-110 states the retained set as one list and makes refusal the only alternative
to retaining it.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | AdmittedObservationIdentityPreimage | 1..1 | identity |
| observation_identity | String | 1..1 |  |
| source_schema_identity | String | 1..1 |  |
| declared_binding_identity | String | 1..1 |  |
| field_units | String | 1..1 |  |
| selected_subject_identity | AuthorityQualifiedRuntimeSubjectReference | 1..1 |  |
| visibility | String | 1..1 |  |
| valuation_presence | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | the FR-296 observation key and nothing else (FR-260 Identity) |
| observation_identity | type not stated in source; see Field sources |
| source_schema_identity | type not stated in source; see Field sources |
| declared_binding_identity | type not stated in source; see Field sources |
| field_units | type not stated in source; multiplicity not stated in source; see Field sources |
| selected_subject_identity | the FR-287 key of the linked subject (FR-296 Key) |
| visibility | type not stated in source; see Field sources |
| valuation_presence | type not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Observation identity | FR-110 Description | "the exact observation identity". |
| Source schema identity | FR-110 Description | Retained exactly; a wrong schema identity "is refused with the affected binding identity" (FR-110-AC-1). |
| Declared binding identity | FR-110 Description | Retained exactly; the binding itself is typed by [FR-261](FR-261-observation-binding.md). |
| Field units | FR-110 Description | A "compatible-looking field" is not a unit (FR-110 Behavior). |
| Selected subject identity | FR-110 Description | The linked subject of FR-110-AC-4. |
| Visibility | FR-110 Behavior | "The binder SHALL preserve whether a record is internal or externally observed." |
| Valuation presence | FR-110 Behavior | "A required absent valuation SHALL be incomplete or refused, never false or an implicit option-none." |

FR-110-AC-3 requires that "A valid observation retains source, binding, subject,
and visibility provenance."

## Identity and comparison

- **Identity kind** — *entity* keyed structurally under FR-296.
- **Key** — the FR-296 observation key (source schema identity and version,
  binding identity and version, subject identity, source ordinal). FR-110 also fixes what is
  excluded: the binder "SHALL NOT substitute a signal name, endpoint, trace ID,
  or compatible-looking field for a declared entity, trigger, unit, or
  valuation" (FR-110 Behavior). Equality is componentwise over the key and
  nothing else; display name, source path, span, message text, locale and
  rendering never participate.
- **Key order** — none. Canonical identity-byte sorting serves deterministic
  set encoding only. Ordering admitted observations by timestamp would contradict FR-113, which states that
  "Arbitrary timestamp order SHALL NOT establish causality"; any order that were
  later established would serve collection comparison only and would carry no
  causal meaning.
- **Causal order** — none derived here. An observation relates to another
  instance only through the explicit typed relationship of
  [FR-262](FR-262-declared-instance-relationship.md); FR-111 states that "a
  telemetry trace ID or shared provider identity does not establish either
  relationship".
- **Absence** — an absent key component compares only to absence. FR-110 is
  explicit for valuations: "A required absent valuation SHALL be incomplete or
  refused, never false or an implicit option-none." Absence never becomes a
  default, an empty value or an implicit option-none.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-260-AC-1 | A signal name, endpoint, trace ID or compatible-looking field supplied for a declared entity, trigger, unit or valuation does not resolve. | Test (TC-140) |
| FR-260-AC-2 | A required absent valuation compares only to absence and does not compare equal to false or to an option-none. | Test (TC-140) |
| FR-260-AC-3 | An internally recorded and an externally observed record remain distinguishable after admission. | Test (TC-140) |
| FR-260-AC-4 | The observation key is the structural FR-296 tuple; the same key on unequal observation members and any missing or defaulted key member are refused. | Test (TC-140) |

## Dependencies

- [FR-110](../../functional/observation/FR-110-bind-typed-observations.md) establishes the
  retained set, the substitution prohibitions and the absent-valuation rule.
- [FR-261](FR-261-observation-binding.md) types the selected binding.
- [FR-262](FR-262-declared-instance-relationship.md) types the only admitted
  relationship between instances.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-296](FR-296-admitted-observation-identity-preimage.md) defines the exact
  key and uniqueness rule.

## Status

Proposed identity closure under FR-296 for `quire-specification#40`.
