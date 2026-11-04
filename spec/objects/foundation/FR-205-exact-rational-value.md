---
id: FR-205
title: "Exact rational value"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-039
    type: specifies
---
# FR-205: Exact rational value

## Description

When an exact rational literal is admitted, the checker SHALL compare its normalized signed numerator and denominator, retaining the original source spelling as a separate authored identity.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| numerator | NormalizeExactRationalLiteralsBeforeDomainAdmission | 1..1 |  |
| denominator | String | 1..1 |  |

## Field sources

- **numerator** (`signed integer`, normalized) — carries the sign. Normalization
  moves the sign to the numerator and divides both components by their greatest
  common divisor
  ([FR-039](../../functional/foundation/FR-039-normalize-exact-rational-literals.md)).
- **denominator** (`signed integer`, normalized) — never zero. A zero
  denominator is rejected before normalization (FR-039).
- Canonical form: a zero numerator normalizes to `0/1` (FR-039).

Only normalized components are checked against the authored numerator and
denominator bounds, and all inclusive boundary values admit (FR-039-AC-3). No
floating point and no implicit integer-to-rational conversion occurs (FR-039).

## Identity and comparison

- **Identity kind** — *value*. A rational has no identity of its own and is
  compared componentwise after normalization.
- **Key** — (`numerator`, `denominator`) **after normalization**. `rational(2,4)`
  and `rational(-2,-4)` both normalize to `1/2` and therefore compare equal;
  `rational(0,-7)` normalizes to `0/1` (FR-039-AC-1). The original spelling is
  not part of the key.
- **Key order** — lexicographic over (`numerator`, `denominator`) in normalized
  form. This is the order that decides collection comparison.
- **Value order** — the exact arithmetic order over the normalized value. It is
  a *different* order from the key order and the two disagree: `1/3` precedes
  `1/2` arithmetically, while `(1,2)` precedes `(1,3)` lexicographically. A
  consumer comparing magnitudes uses the value order; a consumer diffing a
  collection uses the key order. Neither carries causal meaning.
- **Causal order** — not applicable. A rational value participates in no causal
  structure.
- **Absence** — an absent numerator or denominator compares only to absence. A
  missing expected rational type is refused rather than defaulted: no expected
  rational type, ambiguous expected types, a runtime-expression argument or
  implicit integer conversion refuses rather than choosing a domain
  (FR-039-AC-4).

## Value equality is not literal identity

FR-039 keeps two things apart that a single "rational" noun would merge:
"Equivalent values preserve distinct original source spellings/identities."

- Two literals whose normalized components are equal denote **one value** and
  compare equal under this artifact's key.
- Those same two literals remain **distinct authored source identities**, each
  retaining its own spelling and its own source region
  ([FR-203](FR-203-source-region.md)).

A consumer comparing values must not conclude the literals are the same
authored text, and a consumer comparing source identities must not conclude the
values differ.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-205-AC-1 | `rational(2,4)` and `rational(-2,-4)` compare equal as values while retaining distinct original source identities. | Analysis |
| FR-205-AC-2 | `rational(0,-7)` compares equal to `0/1`. | Analysis |
| FR-205-AC-3 | A zero denominator is refused before any comparison is attempted. | Analysis |
| FR-205-AC-4 | An unnormalized pair is never compared against authored bounds. | Analysis |

## Dependencies

- [FR-039](../../functional/foundation/FR-039-normalize-exact-rational-literals.md)
  establishes normalization, bounds checking and the refusal set.
- [FR-206](FR-206-numeric-domain.md) supplies the named type and unit a rational
  is admitted into.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
