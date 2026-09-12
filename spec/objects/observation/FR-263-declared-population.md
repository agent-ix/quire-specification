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

## Description

When a population is assessed, the assessment request SHALL retain the selected Producer interface identity, version and digest, the declared finite membership rule and digest, the population identity, the bounds, the observation sources, the configuration and closure identities and digests, and the completeness and progress dependencies.

FR-112 states that list as the retained set for "a population or event-time
window", and FR-112-AC-3 requires that "A complete bounded population retains
membership and closure provenance."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Population identity | FR-112 Description | Retained with the snapshot/window identity. |
| Declared finite membership rule and digest | FR-112 Description | Typed by [FR-264](FR-264-membership-rule.md). |
| Bounds | FR-112 Description | For a window, the half-open bounds of [FR-266](FR-266-event-time-window.md). |
| Selected Producer interface identity, version, digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses "without ambient defaulting". |
| Observation sources | FR-112 Description | Retained exactly. |
| Configuration identity and digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses. |
| Closure identity and digest | FR-112 Description, FR-112-AC-4 | A mismatch refuses; E owns closure semantics (FR-112 Dependencies). |
| Completeness and progress dependencies | FR-112 Description | Retained exactly. |

FR-112 Behavior states that "Unknown required membership, missing external-provider
observations, or an absent declared closure input are incomplete/refused
assessment causes and cannot establish aggregate success", and that an "unknown
producer version, missing selected identity, digest mismatch, or ambient
default" is refused "rather than supplying a population, membership,
configuration, or closure input".

## Identity and comparison

- **Identity kind** — **the prose does not place a population in FR-200's
  authored / derived / value trichotomy.** FR-112 calls the membership rule and
  the population "declared" and requires their identities to be retained, but
  does not state whether the population identity is written in native source or
  computed from its members. See **Open gap**.
- **Key** — **the prose does not fix the complete key.** It fixes that certain
  components participate: FR-112-AC-1 states that "Changing a declared
  snapshot/window or membership rule changes the assessment subject", and
  FR-112-AC-4 that a producer version, identity, membership-digest,
  configuration or closure-digest mismatch refuses. The selected snapshot or
  window ([FR-265](FR-265-selected-snapshot.md),
  [FR-266](FR-266-event-time-window.md)) and the membership rule
  ([FR-264](FR-264-membership-rule.md)) therefore participate. Whether the list
  is complete is not stated. Equality is componentwise over the key and nothing
  else; display name, source path, span, message text, locale and rendering
  never participate.
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
- **Digest domain** — the membership, configuration, closure and producer
  digests each carry a selected domain under
  `ix://agent-ix/quire-specification/FR-201`. An identity crossing domains is
  unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-263-AC-1 | Changing the declared snapshot, window or membership rule yields a different assessment subject. | Analysis |
| FR-263-AC-2 | A population with unknown required membership compares unequal to both an empty population and a complete one. | Analysis |
| FR-263-AC-3 | A mismatched producer version, identity, membership digest, configuration digest or closure digest does not resolve and receives no ambient default. | Analysis |
| FR-263-AC-4 | This artifact declares no identity kind, no complete key and no key order for a population, and cites the prose instead. | Inspection |

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

## Open gap

1. No requirement states a population's identity kind or its complete key
   components; it states only that the snapshot/window and membership rule
   participate and that mismatches refuse.
2. No requirement states the identity domain of the membership, configuration or
   closure digests.
3. Completeness itself is a separate retained fact under FR-115 and FR-116 and
   is not typed here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
