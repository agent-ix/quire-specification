---
id: FR-206
title: "Numeric domain"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-044
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-007
    type: specifies
---
# FR-206: Numeric domain

## Description

When a numeric operand or result is checked, the checker SHALL require its named numeric type and its exact unit to match rather than its underlying representation.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| named_type | String | 1..1 |  |
| unit | String | 1..1 |  |
| bounds | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| named_type | type not stated in source; see Field sources |
| unit | type not stated in source; see Field sources |
| bounds | type not stated in source; see Field sources |


- **named_type** (identity component) — the declared numeric type. Ordinary
  operands and results share their named numeric type
  ([FR-044](../../functional/foundation/FR-044-check-composed-numeric-definedness.md)),
  except the two operation-scoped exceptions FR-044 names: rational `/`
  promotion and the FR-149 expected-type rule, under which a binary `+`, `-` or
  `*` operand of `=` or `!=` takes the other operand's `Int[..]` type.
- **unit** (identity component) — the exact unit. Operands and results share it
  (FR-044). Rational operation results are checked in the same named
  dimensionless rational type (FR-044).
- **bounds** (where authored) — inclusive. A sound guard narrowing every
  potentially evaluated result within inclusive bounds permits checking
  (FR-044-AC-1). Where `quire.state.core/v1` is selected without
  `quire.value.complete/v1`, a minimum-only integer remains unadmitted without
  inventing a maximum (FR-044-AC-4); where `quire.value.complete/v1` is
  selected, a minimum-only or wholly unbounded integer declaration is admitted
  as a mathematical integer (FR-044-AC-8).

## Identity and comparison

- **Identity kind** — *value*. A numeric domain has no identity of its own and
  is compared componentwise.
- **Key** — (`named_type`, `unit`). Equality is componentwise over exactly
  these. A display name, a spelling or a rendering of the type never
  participates: wrong named types and units refuse, including in unreachable
  syntax (FR-044-AC-3).
- **Key order** — lexicographic over (`named_type`, `unit`), for collection
  comparison only. It expresses no widening relation and no causal meaning;
  implicit integer-to-rational conversion is prohibited (FR-044-AC-3), so
  adjacency in this order never licenses a conversion.
- **Causal order** — not applicable. A numeric domain participates in no causal
  structure.
- **Absence** — an absent named type or unit compares only to absence and is
  never defaulted. Failure to establish totality is reported as unproved
  definedness, which FR-044 keeps distinct from a proved type or domain error
  and from an observed logical violation; exhausted checking resources are
  reported as incomplete. Neither is a completed Boolean violation
  (FR-044-AC-5).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-206-AC-1 | Two values with equal magnitude and different units compare unequal domains. | Analysis |
| FR-206-AC-2 | An absent unit compares only to absence and receives no default. | Analysis |
| FR-206-AC-3 | Unproved definedness, a proved type or domain error, and an observed logical violation remain three distinct outcomes. | Analysis |
| FR-206-AC-4 | Key-order adjacency between two domains does not permit an implicit conversion between them. | Analysis |

## Dependencies

- [FR-044](../../functional/foundation/FR-044-check-composed-numeric-definedness.md)
  establishes the named type, unit and definedness outcomes.
- [FR-007](../../functional/foundation/FR-007-prove-integer-definedness.md) establishes
  bounded integer definedness.
- [FR-205](FR-205-exact-rational-value.md) is admitted into a numeric domain.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
