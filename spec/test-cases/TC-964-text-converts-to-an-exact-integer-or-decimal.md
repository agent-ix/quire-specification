---
id: TC-964
title: "Text converts to an exact integer or decimal in one locale-free grammar"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-478
    type: verifies
---

# TC-964: Text converts to an exact integer or decimal in one locale-free grammar

## Description

Verify that `convert<T>` of a `Text[..]` operand reads the FR-478 grammar
exactly, gives the exact integer or retained decimal, is undefined for every
text outside the grammar, refuses a number outside `T` with the target's
out-of-domain code, type-checks only for integer and decimal targets, links
in a pure function with no obligation, and charges in FR-478's order.

Scope: FR-478-AC-1 to FR-478-AC-5.

## Test Procedure

Select language `ix:native`, edition `1-draft`, and the
`quire.value.complete/v1` and `quire.value.text.unicode-17.0.0/v1`
definitions through
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Execute these vectors through the real local Rust boundary. Text operands are
`Text[0, 64; unicode-scalars]` unless the row names another profile.

| Vector | Expression and operand | Expected |
| --- | --- | --- |
| TN-01 | `convert<Integer>`: `0`, `42`, `-7`, `170141183460469231731687303715884105728` | `0`, `42`, `-7`, that exact integer |
| TN-02 | `convert<Int[0, 99]>`: `42`; `100`; `-1` | `42`; `integer_out_of_domain`/`outside-domain`; the same |
| TN-03 | `convert<Decimal>`: `1.50`; `-0.5`; `3` | coefficient `150` scale `2`; `-5` scale `1`; `3` scale `0` |
| TN-04 | `convert<Decimal[0, 1000; 0, 2; exact]>`: `1.555` | `decimal_out_of_domain`/`outside-domain`, no rounded value |
| TN-05 | `convert<Integer>`: empty, `+5`, `05`, `-0`, ` 5`, `5 `, `1,000`, `1e3`, `1.0`, U+0663, U+FF15 | undefined, no value |
| TN-06 | `convert<Decimal>`: `.5`, `5.`, `-0.00`, `1.2.3`, `1,5` | undefined, no value |
| TN-07 | `convert<Integer>` of U+FF15 under `nfkc`; under `nfc` | `5`; undefined |
| TN-08 | `convert<T>` of a text to `Rational`, `Float64`, a quantity, `Text[0, 8]`, an enum; of a `String` to `Integer` | `ill_typed`/`operator-ineligible` at type checking |
| TN-09 | checked package of `convert<Int[0, 99]>(t)` and of `convert<Decimal[0, 1000; 0, 2; exact]>(t)` | `quire.op.text.to_integer` with member `type_argument` `Int[0, 99]`; `quire.op.text.to_decimal` |
| TN-10 | pure function `n(t) = convert<Int[0, 99]>(t)` linked, then called with `abc` and with `100` | links with no obligation; undefined; `integer_out_of_domain` |
| TN-11 | `convert<Int[0, 99]>` of `42` with exact limits; with `text-number.value` denied; of `abc`; of `100` | succeeds after the four named charges; `incomplete`, no value; undefined with no `text-number.*` charge; refused after `text-number.value`, before `text-number.result-retain` |

## Expected Results

Each vector gives exactly the result in its row, and no vector returns a
rounded, clamped or locale-read value.
