---
id: TC-981
title: "Rational value types and the Rational carrier"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-154-AC-42
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154-AC-43
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154-AC-44
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-380
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-381
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-382
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-142-AC-31
    type: verifies
---

# TC-981: Rational value types and the Rational carrier

## Description

Admit a domain-package Rational value type (FR-154 "Rational value types"),
type its members as `Rational[n1, n2; d1, d2]` or, with a `unit`, as the
carried quantity `Quantity<u, Rational[n1, n2; d1, d2]>` (FR-322 "Model-owned
members" step 4), and convert into a `Rational[..]` carrier (FR-142 "Carried
quantity").

## Test Procedure

Select `quire.model.complete/v1` through
[`model-complete.md`](../../proposals/quire-v1/definitions/model-complete.md).
Node names, declaration keys and the selection `S1` follow TC-234.

Fixtures:

- **R1** (domain package `test/orders`): TC-234 K1 plus the Rational value type
  `Ratio` (`kind` `scalar`, `scalar` `rational`, construct meaning
  `quire.meaning.model.value-type/v1`) whose `constraints` are, in this order,
  `maxDenominator` 12, `minNumerator` -3, `minDenominator` 1 and `maxNumerator`
  7, each with `appliesTo` `ix://test/orders/Ratio`; field `A/r` typed `Ratio`,
  presence `required`, multiplicity `{1, 1, false, true}`; and operation `A/op`
  with one parameter `p` and a result, both typed `Ratio`.
- **R2**: R1 plus base dimension `Length`, root unit `m` and unit `cm`
  (`target` `m`, `scale` `1/100`), with `Ratio` given `unit`
  `ix://quire/unit/m`.
- **U1**: the unit graph of TC-187 (`m`, `cm`, and `in` with scale
  `127/5000` to `m`).

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| RV-01 | Admit R1 under S1; then R1 plus alias `RA` with `target` `Ratio` and field `A/q` typed `RA`; then R1 with `Ratio`'s `constraints` empty | admitted with one original declaration for `Ratio`; admitted; admitted (FR-154-AC-42) |
| RV-02 | R1 with, each alone: a fifth constraint `minLength` 0; a second `maxNumerator` 7; `minDenominator` removed; `maxNumerator` operand `1.5`, `1e2`, `"3/4"` and `9223372036854775808` in turn; `maxDenominator` operand `0.5`; `minNumerator` 8; `minDenominator` 0; `minDenominator` 13 | at the `Ratio` node: `unsupported_construct`/`declaration-form` at `Constraint.keyword` naming `minLength`; `invalid_model_binding`/`malformed-declaration` at the second `Constraint.keyword` naming `maxNumerator`; the same at `TypeDefinition.constraints` naming `minDenominator`; the same at that `Constraint.operands`, five times, with no bound rounded or widened; `unsupported_construct`/`declaration-form` at `TypeDefinition.constraints` naming the bounds, three times (FR-154-AC-43) |
| RV-03 | R1 with `minLength` 0 added and `minDenominator` removed; R1 with a second `maxNumerator` and `minDenominator` removed | one refusal each: the `minLength` refusal; the repeated-keyword refusal (FR-154-AC-43) |
| RV-04 | R1 with `A/r` given its own `maxDenominator` 6 constraint; R1 plus alias `RA` of `Ratio` carrying a `maxNumerator` constraint; R1 with `Ratio` given `unit` `ix://quire/unit/nosuch`; R1 with value type `Code` (`scalar` `string`) given `unit` `ix://quire/unit/m` | `invalid_model_binding`/`malformed-declaration` at `Field.constraints`; the same at `RA` and `TypeDefinition.constraints`; `missing_declaration`/`missing-name` at `Ratio` and `TypeDefinition.unit` when the member is resolved; `invalid_model_binding`/`malformed-declaration` at `Code` and `TypeDefinition.unit` (FR-154-AC-44, FR-322-AC-381) |
| RV-05 | Bind R1 and supply, in turn, `A/r` values `7/12`, `-3/1`, `0/1`, `8/1`, `1/13`, `2/4`, `-4/1` and the float `0.5` | the first three admitted; each other `invalid_runtime_input`/`invalid-value` naming the member path `A/r`, no value reduced or rounded (FR-154-AC-44) |
| RV-06 | Build the checked package for R1 with RV-01's alias; then R1 plus value type `Ratio2` of equal bounds and field `A/r2` typed `Ratio2`; then `Ratio2` with `maxDenominator` 13; then R1 plus tuple type `P` with position `p0` typed `Ratio` and variant type `V` with case `c` whose `payloadType` is `Ratio`; then that package with `Ratio`'s `constraints` empty | `A/r`, `p`, the result and `A/q` each typed `Rational[-3, 7; 1, 12]` naming one `bounded_domain`/`rational_range` node over `scalar_type`/`rational` with bindings `numerator_min` `-3`, `numerator_max` `7`, `denominator_min` `1`, `denominator_max` `12`; `A/r2` names the same node; a second node; admitted; `P`'s position refused `invalid_model_binding`/`malformed-declaration` and `V`'s case refused `unsupported_construct`/`declaration-form`, and `A/r` has no type (FR-322-AC-380) |
| RV-07 | Build the checked package for R2; then R2 plus value type `Ratio2` of equal bounds and `unit` `ix://quire/unit/m`, and `Ratio3` of equal bounds and `unit` `ix://quire/unit/cm`; then R2 plus `Ratio4` of equal bounds and no unit | `A/r` typed `Quantity<m, Rational[-3, 7; 1, 12]>` by a `scalar_type`/`carried_quantity` node with `unit` `m`'s node, `carrier` the `rational_range` node of RV-06 and `rounding` `exact`; `Ratio2` names that node and `Ratio3` another; `Ratio4` names the `rational_range` node, a third node (FR-322-AC-381) |
| RV-08 | A checked package with RV-07's `carried_quantity` node; then with its `rounding` set to `nearest-even`; then with its `carrier` set to the `scalar_type`/`rational` node; then a `quire.op.quantity.convert` from a `cm` operand into the node with `mode` `{kind: rounding, value: exact}`, with `{kind: rounding, value: nearest-even}` and with `null` | admitted; `invalid_package`/`invalid-derived-shape` at the node, twice; admitted; `invalid_package`/`operation-mode-type-mismatch` at the application, twice (FR-322-AC-382) |
| RV-09 | Over U1, type check `Quantity<m, Rational[-3, 7; 1, 12]>` where `Quantity<m>`, `Quantity<m, Rational[-3, 7; 1, 13]>` and `Quantity<cm, Rational[-3, 7; 1, 12]>` are required, and as an operand of `+` and `<`; write `Quantity<m, Rational>` and `Quantity<m, Rational[8, 7; 1, 12]>` | each use refuses `ill_typed`/`type-mismatch` at the expression before any charge; each carrier refuses `ill_typed`/`type-mismatch` at the carrier (FR-142-AC-31) |
| RV-10 | Over U1, evaluate `convert<Quantity<m, Rational[-3, 7; 1, 12]>>` of `25 cm`, `-3 m`, `1 in` and `8 m`; then `convert<Quantity<cm>>` of the first result | `1/4 m`; `-3 m`; `rational_out_of_domain`/`outside-domain` after `unit.target-domain` and before `unit.result-retain` with the carrier as `expected` and no value, twice, never `inexact_conversion`; `25 cm` (FR-142-AC-31) |

## Expected Results

Each vector gives the disposition its row states, with the refusal code,
cause, node and member path named, and no other refusal.
