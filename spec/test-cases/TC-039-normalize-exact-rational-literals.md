---
id: TC-039
title: "Normalize exact rational literals before domain admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-039
    type: verifies
---
## Description

Accepted portable controls for [FR-039](../functional/foundation/FR-039-normalize-exact-rational-literals.md).
The matrix remains Planned; these expected cases have not been implemented or
executed as this standard's conformance evidence.

## Test Procedure

Admit the explicit literal pairs and zero cases. Use a synthetic authored rational domain with numerator [-1,1] and denominator [1,2]; cover normalized min/max and one outside each bound, including raw components above a bound that reduce inside it. Remove or make the expected type ambiguous, replace a constant with a name, and exhaust normalization resources. Compare original source identities for equivalent normalized values.

## Expected Results

Normalization and inclusive authored bound checks occur in the specified order. Zero denominators and incompatible types refuse distinctly; malformed literal arguments remain syntax errors. Equivalent values do not erase source spelling identity. Resource exhaustion cannot produce an approximate or false result.
