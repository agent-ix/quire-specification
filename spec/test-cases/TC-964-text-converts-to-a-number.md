---
id: TC-964
title: "Text converts to a number in one locale-free grammar per target"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-478
    type: verifies
---

# TC-964: Text converts to a number in one locale-free grammar per target

## Description

Verify `convert<T>` of a `Text[..]` or `String` operand against FR-478. Each
target reads its own grammar exactly. The test checks:

- the exact integer, retained decimal, reduced rational and unit quantity,
  and the IEEE bits and flags under the target's rounding mode;
- decimal rounding only under a non-`exact` decimal mode;
- undefined for every text outside the grammar;
- each target's out-of-domain or strict-`exact` refusal;
- the identity, laws and mode each operand and target select;
- linking in a pure function with no obligation;
- FR-478's charge order.

Scope: FR-478-AC-1 to FR-478-AC-11.

## Test Procedure

Select language `ix:native`, edition `1-draft`, and the
`quire.value.complete/v1` and `quire.value.text.unicode-17.0.0/v1`
definitions through
[`value-complete.md`](../../proposals/quire-v1/definitions/value-complete.md).
Execute these vectors through the real local Rust boundary. Text operands are
`Text[0, 64; unicode-scalars]` unless the row names another profile or a
`String`. Quantity rows declare `dimension Time; unit s : Time = 1; unit ms :
Time = 1/1000 * s;`. IEEE results are written as bits and a flag set.

| Vector | Expression and operand | Expected |
| --- | --- | --- |
| TN-01 | `convert<Integer>`: `0`, `42`, `-7`, `170141183460469231731687303715884105728` | `0`, `42`, `-7`, that exact integer |
| TN-02 | `convert<Int[0, 99]>`: `42`; `100`; `-1` | `42`; `integer_out_of_domain`/`outside-domain`; the same |
| TN-03 | `convert<Decimal>`: `1.50`; `-0.5`; `3` | coefficient `150` scale `2`; `-5` scale `1`; `3` scale `0` |
| TN-04 | `convert<Decimal[0, 1000; 0, 2; exact]>`: `1.555`; `1.500`; `1.5`; `10.01` | `inexact_decimal`/`nonzero-discarded-digit` with no `decimal.rounding` charge; coefficient `150` scale `2`; coefficient `15` scale `1` (retained as spelled, equal by value to `1.50`); `decimal_out_of_domain`/`outside-domain` |
| TN-05 | `convert<Integer>`: empty, `+5`, `05`, `-0`, ` 5`, `5 `, `1,000`, `1e3`, `1.0`, `2/1`, U+0663, U+FF15 | undefined, no value |
| TN-06 | `convert<Decimal>`: `.5`, `5.`, `-0.00`, `1.2.3`, `1,5`, `1e3`, `3/4` | undefined, no value |
| TN-07 | `convert<Integer>` of U+FF15 under `nfkc`; under `nfc` | `5`; undefined |
| TN-08 | type checking `convert<T>` of a text and of a `String` to `Integer`, `Decimal[0, 1000; 0, 2; exact]`, `Rational[-9, 9; 1, 9]`, `Float32[nearest-even]`, `Float64`, `Quantity<ms>`; of a text to `Boolean`, `Text[0, 8; nfc]`, an enum, `UUID`, a record | each of the first six admitted for both operands; `ill_typed`/`operator-ineligible` for each of the last five |
| TN-09 | checked package over `t: Text[0, 8; nfc]` of `convert<Int[0, 99]>(t)`, `convert<Decimal[0, 1000; 0, 2; nearest-even]>(t)`, `convert<Rational[-9, 9; 1, 9]>(t)`, `convert<Float64[toward-zero]>(t)`, `convert<Quantity<ms>>(t)`; then the same over `s: String` | `quire.op.text.to_integer` mode `null`; `quire.op.text.to_decimal` mode `rounding` `nearest-even`; `quire.op.text.to_rational` mode `null`; `quire.op.text.to_float64` laws `text_profile`, `ieee_profile`, mode `rounding` `toward-zero`; `quire.op.text.to_quantity` mode `rounding` `exact`; each with the `text_profile` law and member `type_argument` naming the target. Over `s`: the `quire.op.string.*` identity of the same name, the same mode and member, no `text_profile` law |
| TN-10 | pure function `n(t) = convert<Int[0, 99]>(t)` linked, then called with `abc` and with `100` | links with no obligation; undefined; `integer_out_of_domain` |
| TN-11 | `convert<Int[0, 99]>` of `42` with exact limits; with `text.input-bytes` denied; of `abc`; of `100` | succeeds after `text.input-bytes` and `text.decode-scalars`; `incomplete`, no value; undefined after operand and applicable normalization charges; refused after those charges, before a result is returned |
| TN-12 | `convert<Decimal[0, 1000; 0, 2; nearest-even]>`: `1.555`; `1.565` | coefficient `156` scale `2`, `DecimalLoss` exact `311/200`, mode `nearest-even`, after operand and applicable normalization charges and `decimal.rounding` (`integer_bits` 11, `decimal_digits` 4); coefficient `156` scale `2` (tie to even) |
| TN-13 | `convert<Rational[-9, 9; 1, 9]>`: `2/4`; `-3`; `0.25`; `1/10`; `10` | `1/2`; `-3/1`; `1/4`; `rational_out_of_domain`/`outside-domain`; the same |
| TN-14 | `convert<Rational>` of `6/4`; `convert<Rational[-9, 9; 1, 9]>` of `1/0`, `1/-2`, `-0/3`, `+1/2`, `1 / 2`, `01/2`, `1/02`, `1.5/2`, `1e1` | `3/2`; each undefined |
| TN-15 | `convert<Float64[nearest-even]>`: `0.1`; `0.25`; `2.5e-1`; `-0`; `1e309`; `1e-400`; `inf`; `-inf`; `nan` | `0x3fb999999999999a` `{inexact}`; `0x3fd0000000000000` `{}`; the same; `0x8000000000000000` `{}`; `0x7ff0000000000000` `{overflow, inexact}`; `0x0000000000000000` `{underflow, inexact}`; `0x7ff0000000000000` `{}`; `0xfff0000000000000` `{}`; `0x7ff8000000000000` `{}` |
| TN-16 | `convert<Float64[toward-zero]>`: `0.1`; `1e309`. `convert<Float64[toward-positive]>`: `1e-400`. `convert<Float32[nearest-even]>` and `convert<Float32[toward-zero]>`: `0.1` | `0x3fb9999999999999` `{inexact}`; `0x7fefffffffffffff` `{overflow, inexact}`; `0x0000000000000001` `{underflow, inexact}`; `0x3dcccccd` `{inexact}` and `0x3dcccccc` `{inexact}` |
| TN-17 | `convert<Float64>` (strict `exact`): `0.25`; `0.1`; `1e309` | `0x3fd0000000000000` `{}`; `ieee_not_exact`/`rounding-required`, would-be flags `{inexact}`, no bits; the same with `{overflow, inexact}`; each refusal after `ieee.round` |
| TN-18 | `convert<Float64[nearest-even]>` of `0.1` and of `inf`, recording charges | `0.1`: `text.input-bytes`, `text.decode-scalars`, `ieee.round` (`integer_bits` 64); `inf`: no `ieee.round` |
| TN-19 | `convert<Float64[nearest-even]>`: `1E3`, `1e+3`, `1e03`, `1e-0`, `.5`, `5.`, `1e`, `Infinity`, `NaN`, `-nan`, `+inf`, `0x1p3` | each undefined |
| TN-20 | `convert<Quantity<ms>>`: `3`; `3 ms`; `0.5`; `2/4 ms`; `3 s`; `3ms`; `3  ms`; `3 MS`; `ms` | `3 ms`; `3 ms`; `1/2 ms`; `1/2 ms`; each of the last five undefined |
| TN-21 | domain package `acme/rpc` imported as `M` with its own unit `ms`: `convert<Quantity<M::ms>>` of `3 ms`; of `3 M::ms` | `3` of `ix://acme/rpc/unit/ms`; undefined |
| TN-22 | `s: String`: `convert<Integer>(s)` of `42` and of U+FF15; `convert<Float64[nearest-even]>(s)` of `0.1`; `convert<Rational[-9, 9; 1, 9]>(s)` of `2/4` | `42`; undefined; `0x3fb999999999999a` `{inexact}`; `1/2`; each charges `text.input-bytes` and `text.decode-scalars` and no `text.normalize-*` point |

## Expected Results

Each vector gives exactly the result in its row. No vector returns a
locale-read value, a clamped value or a value of another unit. No exact target
returns a rounded value. A decimal value is rounded only under a non-`exact`
decimal mode, and an IEEE value only once under its target's mode.
