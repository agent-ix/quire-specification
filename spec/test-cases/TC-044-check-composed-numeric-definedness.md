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

## Expected Results

Only established total operations receive checked status. Diagnostics distinguish unsupported/type/domain errors, unproved definedness and resource incompleteness. Rational normalization precedes bounds and no historical division example qualifies the new domain.

