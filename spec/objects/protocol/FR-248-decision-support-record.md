---
id: FR-248
title: "Decision-support record"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-282
    type: references
---
# FR-248: Decision-support record

## Description

When a truth result settles, the result producer SHALL retain for every decision-support record its exact oracle and observation identities plus exactly one dependence relation, whose missing, cyclic or self-referential predecessor a strict reader refuses.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| oracle_identity | String | 1..1 |  |
| observation_identity | String | 1..1 |  |
| dependence_relation | DependenceRelation | 1..1 |  |
| predecessor | String | 0..1 |  |
| dependence_group | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| oracle_identity | type not stated in source; see Field sources |
| observation_identity | type not stated in source; see Field sources |
| predecessor | type not stated in source; see Field sources |
| dependence_group | type not stated in source; see Field sources |


| Property | Description |
| --- | --- |
| oracle identity | The exact oracle the record identifies (FR-061). FR-062 derives every E02 oracle contribution for a `satisfied` or `violated` projection from these exact identities. |
| observation identity | The exact observation the record identifies (FR-061). |
| dependence relation | Exactly one of `independent`, `shared`, `derived` or `unknown`, as defined by [FR-282](FR-282-dependence-relation.md) (FR-061). See **Dependence relation** below. |
| predecessor | Required by a `derived` relation: "A derived relation SHALL identify its exact predecessor" (FR-061). A missing, cyclic or self-referential predecessor is refused (FR-061, FR-061-AC-7). |
| dependence group | Required by a `shared` relation: "a shared relation SHALL identify its dependence group" (FR-061). |

## Dependence relation

The vocabulary is **defined by
[FR-282](FR-282-dependence-relation.md)**; the table below is repeated here only
as a reading aid, and where the two differ FR-282 governs.

FR-061 states the closed set verbatim: "Every retained decision-support record
SHALL identify its exact oracle and observation plus one dependence relation from
`independent`, `shared`, `derived` or `unknown`." FR-062 requires the projection
to "preserve the source `independent`, `shared`, `derived` or `unknown` relation
and lineage" and states that "The adapter SHALL NOT invent an independent oracle
relation."

| Relation | Description |
| --- | --- |
| `independent` | The record does not depend on another. It is never inferred: FR-061 forbids inferring independence "from distinct display names or implementations", and FR-062 forbids inventing it. |
| `shared` | The record identifies its dependence group (FR-061). |
| `derived` | The record identifies its exact predecessor (FR-061). |
| `unknown` | The dependence relation is not known (FR-061). |


## Identity and comparison

- **Identity kind** — *value*. A decision-support record has no identity of its
  own; it is compared componentwise over its key.
- **Key** — the oracle identity, the observation identity, the dependence
  relation, and the predecessor or dependence group that relation requires.
  Equality is componentwise over the key and nothing else. Display name, source
  path, span, message text, locale and rendering never participate. FR-061
  supports this directly: a strict reader "SHALL NOT infer independence from
  distinct display names or implementations", so two records with distinct
  display names and one oracle identity are one record, not two independent ones.
- **Key order** — **none is established.** FR-061 states no total order over
  decision-support records, and this artifact supplies none. FR-062 is explicit
  that a presentation order is not an authority for a related collection:
  "`bindingRefs` uses the selected E02 semantic order and SHALL NOT be resorted by
  presentation text."
- **Causal order** — the `derived` predecessor edge. It is an authored lineage
  between records and is never derived from key order, timestamp or source
  position. A cyclic or self-referential edge is refused (FR-061-AC-7).
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. An absent predecessor under a `derived` relation is a
  refusal, not an empty lineage (FR-061). A record absent from the decision-support
  set makes the associated truth `unavailable`; a fact missing outside a completed
  support set remains a completeness gap and does not falsify or delay a settled
  truth (FR-061, FR-094).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-248-AC-1 | Every retained record carries exact oracle and observation identities and exactly one dependence relation. | Inspection |
| FR-248-AC-2 | A `derived` record without an exact predecessor, or with a cyclic or self-referential one, is refused. | Analysis |
| FR-248-AC-3 | A `shared` record without its dependence group is refused. | Analysis |
| FR-248-AC-4 | `independent` is never inferred from distinct display names or implementations. | Analysis |
| FR-248-AC-5 | A missing record inside the exact deciding-fact set makes truth and settlement basis unavailable; a fact missing outside a completed deciding-fact set leaves an already settled truth and its basis byte-identical. | Analysis |
| FR-248-AC-6 | No ordering over decision-support records is asserted by this artifact or inferred from presentation text. | Inspection |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  record, its relation vocabulary and its refusal rules.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the oracle-contribution derivation and the order-preservation rule.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes the inside/outside decision-support distinction.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-282](FR-282-dependence-relation.md) defines the dependence relation
  vocabulary carried on this record.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
