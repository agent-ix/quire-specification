---
id: FR-263
title: "Declared finite population"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-264
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-263: Declared finite population

BLOCKED-BY QSpec #513: this requirement's `sha256-jcs` / `quire.verification.jcs` digest identity (rows removed from FR-201).

## Description

When a population is assessed, the assessment request SHALL retain the selected domain package identity and digest, the population declaration key, the declared finite membership rule and digest, the population identity, the bounds, the observation sources, the configuration and closure identities and digests, and the completeness and progress dependencies.

FR-112 states that list as the retained set for "a population or event-time
window", and FR-112-AC-3 requires that "A complete bounded population retains
membership and closure provenance."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Population identity | FR-112 Description | Retained with the snapshot/window identity. |
| Declared finite membership rule and digest | FR-112 Description | Typed by [FR-264](FR-264-membership-rule.md). |
| Bounds | FR-112 Description | For a window, the half-open bounds of [FR-266](FR-266-event-time-window.md). |
| Domain package identity, digest and population declaration key | FR-112 Description, FR-112-AC-4 | A mismatch refuses "without ambient defaulting". The closed extent comes only from this spec-declared population declaration. |
| Observation sources | FR-112 Description | Retained exactly. |
| Configuration identity and digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses. |
| Closure identity and digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses; E owns closure semantics (FR-112 Dependencies). |
| Completeness and progress dependencies | FR-112 Description | Retained exactly. |

The v1 population identity preimage is the RFC 8785 canonical form of
`{"identityVersion":"quire.observation.population-identity/v1-draft.1","declaration":...,"membershipRuleIdentity":...,"selection":...,"observationSources":[...],"configuration":...,"closure":...,"completenessDependencies":[...],"progressDependencies":[...]}`.
Arrays that denote sets SHALL be sorted, distinct and bounded before identity
derivation. `selection` SHALL contain exactly one FR-265 snapshot or FR-266
event-time window. `declaration` holds the domain package identity and the
population declaration key under `sha256-jcs`. Configuration and closure definition
digests use FR-201 `raw-artifact-digest`; the derived population identity uses
FR-201 `quire.verification.jcs`.

FR-112 Behavior states that "Unknown required membership, missing external-provider
observations, or an absent declared closure input are incomplete/refused
assessment causes and cannot establish aggregate success", and that a "declaration key
outside the checked package, missing selected identity, digest mismatch, or
ambient default" is refused "rather than supplying a population, membership,
configuration, or closure input".

## Identity and comparison

- **Identity kind** — *derived*. The identity is `sha256-jcs:<hex>` over the
  exact versioned preimage above.
- **Key** — the complete derived population identity. Every retained semantic
  selection listed in the preimage participates; presentation never does.
- **Key order** — none. Because the key is not fixed, no total order over it is
  stated and none is supplied here. Members are "exactly listed/digested"
  (FR-112 Behavior), so a population is compared as a set under its declared
  membership rule rather than by a member order; any order later established
  would serve collection comparison only and would carry no causal meaning,
  since FR-113 states that "Arbitrary timestamp order SHALL NOT establish
  causality."
- **Causal order** — none. Membership is declared, not caused. A relationship
  between two members exists only as the declared relationship of
  [FR-262](FR-262-declared-instance-relationship.md).
- **Absence** — an absent key component compares only to absence. FR-112
  Behavior makes unknown membership its own state: "Unknown required membership
  ... cannot establish aggregate success", and FR-112-AC-2 requires that
  "Unknown membership and a missing provider record remain visible incomplete
  reasons." A population with unknown membership is therefore neither an empty
  population nor a complete one, and an absent component is never supplied by an
  ambient default (FR-112 Behavior, FR-112-AC-4).
- **Digest domain** — the population identity and membership-rule identity use
  `quire.verification.jcs`. The domain package and population declaration key
  use `sha256-jcs`. Configuration and closure definition-byte digests use
  `raw-artifact-digest`. A crossing domain is
  unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-263-AC-1 | Changing the declared snapshot, window or membership rule yields a different assessment subject. | Test (TC-142) |
| FR-263-AC-2 | A population with unknown required membership compares unequal to both an empty population and a complete one. | Test (TC-142) |
| FR-263-AC-3 | A mismatched declaration key, identity, membership digest, configuration digest or closure digest does not resolve and receives no ambient default. | Test (TC-142) |
| FR-263-AC-4 | Independent derivation over the exact versioned preimage yields the population identity; mutation of any semantic key component changes it and presentation permutation does not. | Test (TC-142) |
| FR-263-AC-5 | A population with neither or both snapshot/window selections, duplicate or unbounded set members, or a foreign/missing digest domain is refused before identity derivation. | Test (TC-142) |

## Dependencies

- [FR-112](../../functional/observation/FR-112-declare-population-and-completeness.md)
  establishes the retained set, the incomplete causes and the refusal cases.
- [FR-264](FR-264-membership-rule.md) types the declared membership rule.
- [FR-265](FR-265-selected-snapshot.md) and
  [FR-266](FR-266-event-time-window.md) type the two selected bounds subjects.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) supplies the digest
  domain rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Completeness boundary

FR-295 completeness is a separate derived fact about the exact declared
population. It does not participate in the population identity preimage and
cannot be inferred from progress or closure alone.

## Status

Proposed population identity closure for `quire-specification#40`.
