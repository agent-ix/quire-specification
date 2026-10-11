---
id: FR-044
title: "Establish exact numeric definedness in the selected domain"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-206
    type: references
  - target: ix://agent-ix/quire-specification/FR-147
    type: references
  - target: ix://agent-ix/quire-specification/FR-148
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
---
## Description

When checking an admitted numeric expression, the checker SHALL establish that each potentially evaluated result lies in its selected exact authored domain before permitting evaluation, except an IEEE-to-`Rational[..]` conversion, whose membership FR-148 decides at evaluation.

## Inputs

Named integer/rational types with explicit bounds and units, expression operations, unique literal contexts, sound guard facts and checking resource controls.

## Outputs

A checked defined expression, a typed type/domain/definedness refusal, or an explicit resource-incomplete checking result.

## Behavior

Ordinary operands/results share their named numeric type and exact unit. The state contract specifies permitted operations, rational normalization-before-bounds and sum's explicit result-domain transfer. Saturation, machine-width inference, general implicit conversion and silent widening are inadmissible under every profile. The sole operation-scoped exception is FR-149's exact `n/1` promotion of integer operands for rational `/` with an expected `Rational[..]` result type, only where `quire.value.complete/v1` is selected; it supplies no ambient integer-to-rational conversion. Where `quire.state.core/v1` is selected without `quire.value.complete/v1`, a floating-point operand or result is inadmissible as well. Where `quire.value.complete/v1` is selected, an explicitly declared IEEE 754-2019 binary32 or binary64 type is admitted and its definedness is decided by [FR-148](../expressions/FR-148-evaluate-ieee-floating-profiles.md) under `quire.value.ieee754-2019-default/v1`, which FR-019-AC-1 names among the complete-V1 facets; an IEEE operand still reaches an exact named numeric type only through an explicit conversion that reports loss. Failure to establish totality is reported as unproved definedness, distinct from a proved type/domain error or observed logical violation. Checking may be conservative but cannot certify unproved operations or label its limitation a counterexample.

Rational division additionally requires a proved nonzero divisor, exact quotient
normalization and result bounds in the same named dimensionless rational type.
The state-only profile requires both operands of that type. Complete value may
promote integer operands exactly as FR-149 specifies when that rational result
type is expected; it still applies the rational nonzero and domain obligations.
Where `quire.state.core/v1` is selected without `quire.value.complete/v1`,
integer division and remainder are refused; where `quire.value.complete/v1` is
selected, the FR-147 operators are admitted only with the exact quotient,
remainder and definedness contract of the retained
`quire.value.integer-division.*` identity. The literal constructor does not implicitly
convert integer operands or introduce quotient rounding.

At evaluation, the evaluator decides a bounded integer or `Rational[..]`
arithmetic result's membership in its result domain after normalization and
before retaining it (`quire.value.accounting/v1`). A result outside that domain,
other than a `sum` seed or prefix, is refused with no value, as
`refused { code: integer_out_of_domain, cause: outside-domain }` or
`refused { code: rational_out_of_domain, cause: outside-domain }`,
respectively. An expression this check admitted has that membership proved,
so only evaluation of an expression that did not pass this check meets
either refusal. A `sum` seed or prefix is not the expression's result: FR-145
defines `sum` as a fold whose every running total lies in `N`'s domain, so a
running total outside it leaves the fold with no value in `N` even when a later
summand would bring the total back inside, and FR-145 makes that a located
undefined outcome, not a refusal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-044-AC-1 | An unguarded possible out-of-range result refuses; a sound guard narrowing every potentially evaluated result within inclusive bounds permits checking. | Test (TC-044) |
| FR-044-AC-2 | Normalized rational operation results are checked after sign/gcd reduction, including raw intermediates above normalized bounds, without floating approximation. | Test (TC-044) |
| FR-044-AC-3 | Wrong named types/units and ambient implicit integer-to-rational conversion refuse. Under `quire.state.core/v1` without complete value, integer operands to rational `/` refuse even with an expected rational type, and `div`/`rem`/`mod` refuse even in unreachable syntax; under `quire.value.complete/v1`, only FR-149's expected-rational `/` promotion and FR-147's admitted integer operators apply. | Test (TC-044, TC-192) |
| FR-044-AC-4 | Where `quire.state.core/v1` is selected without `quire.value.complete/v1`, a minimum-only integer remains unadmitted without inventing a maximum; signed-64 boundary overflow cannot wrap or widen. | Test (TC-044) |
| FR-044-AC-5 | Insufficient proof capability is identified as unproved definedness, and exhausted checking resources as incomplete; neither is a completed Boolean violation. | Test (TC-044) |
| FR-044-AC-6 | For a dimensionless rational type with normalized numerator [-1,1] and denominator [1,2], `rational(1,2) / rational(1,2)` admits as 1/1 after reducing raw 2/2, while `rational(1,1) / rational(1,2)` refuses its result bound and division by zero refuses definedness. | Test (TC-044) |
| FR-044-AC-7 | An exact nonzero guard permits an otherwise total rational quotient only where that operand's guard fact holds; an unguarded possible zero divisor or a guard for another value cannot discharge the obligation. | Test (TC-044) |
| FR-044-AC-8 | Where `quire.value.complete/v1` is selected, a minimum-only or wholly unbounded integer declaration is admitted as a mathematical integer, and a finite consumer lacking a complete derived domain returns the FR-147 `requires-bound` negotiation disposition. | Test (TC-192) |
| FR-044-AC-9 | Where `quire.value.complete/v1` is selected, an explicitly declared binary32 or binary64 operand is admitted and checked under FR-148; where `quire.state.core/v1` is selected without it, the same declaration refuses at its source location. General implicit conversion outside FR-149's complete-value rational `/` promotion, a saturating result and a machine-width inference refuse under both selections. | Test (TC-193, TC-044) |
| FR-044-AC-10 | At evaluation, a bounded integer or `Rational[..]` arithmetic result outside its result domain, other than a `sum` seed or prefix, is `refused { code: integer_out_of_domain, cause: outside-domain }` or `refused { code: rational_out_of_domain, cause: outside-domain }` with no value, as FR-044, `quire.value.accounting/v1` and the native-diagnostics catalog each state; a `sum` seed or running total outside `N`'s domain is FR-145's located undefined outcome (TC-190 Q12), and each of the three documents states that carve-out. | Test (TC-233) |

## Dependencies

- [Detailed draft contract](../../../proposals/quire-v1/state-contract.md).
- [Shared grammar](../../../proposals/quire-v1/shared-grammar.md).
- [Planned acceptance matrix](../../composed-foundation/tests.md).

Accepted by FS03. Historical definitions and their acceptance remain separate;
this artifact does not establish implementation coverage.
