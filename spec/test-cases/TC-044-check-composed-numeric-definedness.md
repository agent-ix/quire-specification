---
id: TC-044
title: "Establish exact numeric definedness in the selected domain"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-044
    type: verifies
---
## Description

Planned controls for [FR-044](../functional/FR-044-check-composed-numeric-definedness.md).
No executable test or observed coverage is claimed.

## Test Procedure

Supply explicit signed-64-boundary and small bounded integer/rational domains. Compare guarded/unguarded sums and normalized fractions; mutate units/representation and offer prohibited arithmetic or a minimum-only declaration. Exercise a well-typed expression whose totality is beyond the selected checking method and a run with insufficient proof resources.

Use the explicit dimensionless rational domain Q with normalized numerator
[-1,1] and denominator [1,2]. Independently specify the arithmetic expectations:
`rational(1,2) / rational(1,2)` yields 1/1 after raw 2/2 reduction;
`rational(1,1) / rational(1,2)` lies outside Q; a zero divisor is undefined.
Negative nonzero divisors normalize their sign before checking the same bounds.
For d:Q, compare `d != rational(0,1) implies rational(0,1) / d = rational(0,1)`
with its unguarded quotient and a guard on another independent value. Only the
first supplies the required exact-subject nonzero fact. Mutate the operands to
integers and to equal named rational types carrying a non-dimensionless unit;
both refuse the proposed division admission rather than choosing new result types.

## Expected Results

Only established total operations receive checked status. Diagnostics distinguish unsupported/type/domain errors, unproved definedness and resource incompleteness. Rational normalization precedes bounds and no historical division example qualifies the new domain.
