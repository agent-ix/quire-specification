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

- **named_type** (identity component) — the declared numeric type. Ordinary
  operands and results share their named numeric type
  ([FR-044](../../functional/foundation/FR-044-check-composed-numeric-definedness.md)).
- **unit** (identity component) — the exact unit. Operands and results share it
  (FR-044). Rational operation results are checked in the same named
  dimensionless rational type (FR-044).
- **bounds** (where authored) — inclusive. A sound guard narrowing every
  potentially evaluated result within inclusive bounds permits checking
  (FR-044-AC-1). A minimum-only integer remains unadmitted without inventing a
  maximum (FR-044-AC-4).

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
- **Digest domain** — not applicable.

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
