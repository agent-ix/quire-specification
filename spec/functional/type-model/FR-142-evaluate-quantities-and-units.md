---
id: FR-142
title: "Evaluate dimensions, quantities and exact unit conversions"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-142: Evaluate dimensions, quantities and exact unit conversions

## Description

When checking a quantity expression, the semantic kernel SHALL derive its
dimension and unit from declared identities and exact conversion functions.

## Inputs

Dimension vectors, unit identities, exact scale/offset conversions, quantities
and operations.

## Outputs

A typed quantity, explicit conversion result/loss or typed refusal.

## Behavior

Addition/comparison require compatible dimensions and an explicit common-unit
conversion. Multiplication/division combine dimension exponents. Affine units
use their selected offset rules and cannot enter multiplicative operations
where that would change meaning. Conversion never occurs implicitly.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-142-AC-1 | Exact compatible conversion followed by arithmetic produces the declared result unit and value. | Test (TC-187) |
| FR-142-AC-2 | Incompatible dimensions or prohibited affine-unit arithmetic refuses at the operator. | Test (TC-187) |
| FR-142-AC-3 | A lossy conversion returns its exact loss classification or refuses when the target permits no loss. | Test (TC-187) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
