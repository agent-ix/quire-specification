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
  - target: ix://agent-ix/quire-specification/FR-154-AC-45
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154-AC-46
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154-AC-47
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-149-AC-27
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
quantity"), with the well-formedness and membership of `Rational[..]` (FR-149
"Bounded rationals"). RV-09 and RV-12 are published as machine vectors in
`proposals/checked-package-v2/quantity-carrier-vectors.json` (`CQ-W16` to
`CQ-W19`, `CQ-E21` to `CQ-E26`), and RV-07's payload and position case as
`model-tuple-union-vectors.json` `TK-36`.

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
| RV-01 | Admit R1 under S1; then R1 plus alias `RA` with `target` `Ratio` and field `A/q` typed `RA`; then R1 with `Ratio`'s `constraints` empty; then R1 with `minNumerator` `-9223372036854775808` | admitted with one original declaration for `Ratio`; admitted; admitted; admitted (FR-154-AC-42) |
| RV-02 | R1 with, each alone: a fifth constraint `minLength` 0; `maxNumerator`'s `appliesTo` set to `ix://test/orders/A`; then to RV-01's alias `RA`; a second `maxNumerator` 7; `minDenominator` removed; `maxNumerator` operand `1.5`, `1e2`, `"3/4"` and `9223372036854775808` in turn; `maxDenominator` operand `0.5`; `maxNumerator` `operands` `{}`; `maxNumerator` `operands` `{value: 7, unit: 1}`; `minNumerator` 8; `minDenominator` 0; `minDenominator` 13; `maxDenominator` 0; `maxDenominator` -1 | one refusal each at the `Ratio` node: `unsupported_construct`/`declaration-form` at `Constraint.keyword` naming `minLength`; `invalid_model_binding`/`malformed-declaration` at `Constraint.appliesTo`, twice; the same at the second `Constraint.keyword` naming `maxNumerator`; the same at `TypeDefinition.constraints` naming `minDenominator`; the same at that `Constraint.operands`, seven times, with no bound rounded or widened; `unsupported_construct`/`declaration-form` at `TypeDefinition.constraints` naming the bounds, five times (FR-154-AC-43) |
| RV-03 | R1 with, each pair alone: `minLength` 0 and a second `maxNumerator`; `maxNumerator`'s `appliesTo` `A` and a second `maxNumerator`; `minLength` 0 and `maxNumerator`'s `appliesTo` `A`; a second `maxNumerator` and `minDenominator` removed; `minDenominator` removed and `maxNumerator` `1.5`; `maxNumerator` `1.5` and `minNumerator` 8; `maxDenominator` `0.5` and `maxNumerator` `1.5` | one refusal each: `minLength`; the `appliesTo`; `minLength`; the repeated `maxNumerator`; the missing `minDenominator`; the `maxNumerator` operand; the `maxNumerator` operand (keyword order, though `maxDenominator` is listed first) (FR-154-AC-44) |
| RV-04 | R1 with `A/r` given its own `maxDenominator` 6 constraint; R1 with `A/op`'s parameter `p` given its own `maxNumerator` 6 constraint; R1 plus alias `RA` of `Ratio` carrying a `maxNumerator` constraint; R1 with value type `Code` (`scalar` `string`) carrying a `maxNumerator` 6 constraint whose `appliesTo` is `Ratio` | `invalid_model_binding`/`malformed-declaration` at `A/r` and `Field.constraints`; the same at `A/op` and `Operation.params[0].constraints`; the same at `RA` and `TypeDefinition.constraints`; the same at `Code` and `Constraint.appliesTo`, with `Ratio` itself admitted (FR-154-AC-45) |
| RV-05 | R1 with `Ratio` given `unit` in turn `ix://quire/unit/m`, `m`, the number `1` and `ix://quire/native/Rational`; R1 with value type `Code` (`scalar` `string`) given `unit` `ix://quire/unit/m` | admitted by intake; `invalid_model_binding`/`malformed-declaration` at `Ratio` and `TypeDefinition.unit`, three times; the same at `Code` and `TypeDefinition.unit` (FR-154-AC-46) |
| RV-06 | Bind R1 and supply, in turn, `A/r` values `7/12`, `-3/1`, `0/1`, `8/1`, `1/13`, `2/4`, `-4/1` and the float `0.5`; then bind R2 and supply `1/4` and `1/13` | the first three admitted; each other `invalid_runtime_input`/`invalid-value` naming the member path `A/r`, no value reduced or rounded; `1/4 m` admitted and `1/13` refused the same (FR-154-AC-47) |
| RV-07 | Build the checked package for R1 with RV-01's alias; then R1 plus value type `Ratio2` of equal bounds and field `A/r2` typed `Ratio2`; then `Ratio2` with `maxDenominator` 13; then R1 plus tuple type `P` with position `p0` typed `Ratio` and variant type `V` with case `c` whose `payloadType` is `Ratio`; then that package with `Ratio`'s `constraints` empty | `A/r`, `p`, the result and `A/q` each typed `Rational[-3, 7; 1, 12]` naming one `bounded_domain`/`rational_range` node over `scalar_type`/`rational` with bindings `numerator_min` `-3`, `numerator_max` `7`, `denominator_min` `1`, `denominator_max` `12`; `A/r2` names the same node; a second node; admitted (`TK-36`); `P`'s position refused `invalid_model_binding`/`malformed-declaration` and `V`'s case refused `unsupported_construct`/`declaration-form`, and `A/r` has no type (FR-322-AC-380) |
| RV-08 | Build the checked package for R2; then R2 plus value type `Ratio2` of equal bounds and `unit` `ix://quire/unit/m`, and `Ratio3` of equal bounds and `unit` `ix://quire/unit/cm`; then R2 plus `Ratio4` of equal bounds and no unit; then R2 with `Ratio`'s `unit` `ix://quire/unit/nosuch` | `A/r` typed `Quantity<m, Rational[-3, 7; 1, 12]>` by a `scalar_type`/`carried_quantity` node with `unit` `m`'s node, `carrier` the `rational_range` node of RV-07 and `rounding` `exact`; `Ratio2` names that node and `Ratio3` another; `Ratio4` names the `rational_range` node, a third node; `missing_declaration`/`missing-name` at `Ratio` and `TypeDefinition.unit` (FR-322-AC-381) |
| RV-09 | A checked package with RV-08's `carried_quantity` node; then with its `rounding` set to `nearest-even`; then with its `carrier` set to the `scalar_type`/`rational` node; then a `quire.op.quantity.convert` from a `cm` operand into the node with `mode` `{kind: rounding, value: exact}`, with `{kind: rounding, value: nearest-even}` and with `null` | admitted; `invalid_package`/`invalid-derived-shape` at the node, twice; admitted; `invalid_package`/`operation-mode-type-mismatch` at the application, twice (FR-322-AC-382; `CQ-W06`, `CQ-W16` to `CQ-W19`) |
| RV-10 | Type check `Rational[-3, 7; 1, 12]`, `Rational[8, 7; 1, 12]`, `Rational[0, 1; 0, 12]`, `Rational[0, 1; -1, 12]` and `Rational[0, 1; 13, 12]`, each alone and as the carrier of `Quantity<m, C>`; decide membership in `Rational[-3, 7; 1, 12]` of `1/4`, `-3`, `0`, `7/12`, `8`, `-4`, `1/13` and `127/5000` | the first well formed; each other `ill_typed` at the type, alone and as the carrier; the first four members, the last four not (FR-149-AC-27) |
| RV-11 | Over U1, type check `Quantity<m, Rational[-3, 7; 1, 12]>` where `Quantity<m>`, `Quantity<m, Rational[-3, 7; 1, 13]>` and `Quantity<cm, Rational[-3, 7; 1, 12]>` are required, and as an operand of `+`, `=` and `<`; then `convert<Quantity<m>>(a) = convert<Quantity<m>>(b)` and `convert<Quantity<m>>(a) < convert<Quantity<m>>(b)` for two values `a`, `b` of it; write `Quantity<m, Rational>` | each use refuses `ill_typed`/`type-mismatch` at the expression before any charge; the converted comparisons check; the bare carrier refuses `ill_typed`/`type-mismatch` at the carrier (FR-142-AC-31) |
| RV-12 | Over U1, evaluate `convert<Quantity<m, Rational[-3, 7; 1, 12]>>` of `25 cm`, `-3 m`, `1 in`, `8 m` and `-4 m`; `8 m` under `L(5,0,0,1,1,0)`; then `convert<Quantity<cm>>` of the first result | `1/4 m`; `-3 m`; `rational_out_of_domain`/`outside-domain` after `unit.target-domain` and before `unit.result-retain` with the carrier as `expected` and no value, three times, never `inexact_conversion`; `incomplete` at `unit.target-domain`; `25 cm` (FR-142-AC-31; `CQ-E21` to `CQ-E26`) |
## Expected Results

Each vector gives the disposition its row states, with the refusal code,
cause, node and member path named, and no other refusal.
