---
id: TC-044
title: "Establish exact numeric definedness in the selected domain"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-044
    type: verifies
---
## Description

Accepted portable controls for [FR-044](../functional/foundation/FR-044-check-composed-numeric-definedness.md).
The matrix remains Planned; no executable test or observed coverage is claimed.

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
under state-only selection both refuse the accepted division admission rather
than choosing new result types. Repeat integer-operand `1 / 2` with expected
result type Q under state-only selection and under
`quire.value.complete/v1`. The state-only checker refuses the integer operands;
complete value promotes them exactly to `1/1` and `2/1` under FR-149, then
checks that the rational quotient `1/2` lies in Q, with a nonzero divisor.
Without the
expected rational type, or when the integer result is requested, `/` does not
become integer division. An ambient integer value used as a rational argument
outside this `/` operation still refuses without explicit conversion.

Tag the state-only and complete-value `1 / 2` controls with
`#[trace("FR-044-AC-3", "FR-044-AC-9", "TC-044")]` when implemented.

## Expected Results

Only established total operations receive checked status. Diagnostics distinguish unsupported/type/domain errors, unproved definedness and resource incompleteness. Rational normalization precedes bounds. Integer operands to rational `/` are refused in state-only selection and promoted only in the complete-value, expected-rational case. No historical division example qualifies the new domain.
