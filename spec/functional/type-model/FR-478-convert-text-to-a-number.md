---
id: FR-478
title: "Convert text to a number"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-141
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-140
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-148
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-044
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-478: Convert text to a number

## Description

When evaluating `convert<T>(e)` whose operand `e` has a `Text[..]` or `String`
type and whose target `T` is an integer, decimal, rational, `Float32`,
`Float64` or quantity type, the semantic kernel SHALL read the operand as a
number in the fixed, locale-free grammar of `T`'s kind and return that number
as a value of `T`. An exact target receives the exact number. A decimal target
rounds only under a rounding mode its type declares. An IEEE target rounds once
under its type's rounding mode. Text that is not a number in the grammar is
undefined. A number outside `T` is refused.

## Use case

A model holds a number that arrived as text, such as an amount typed into a
form field, a measurement read from a document (`250 ms`), a ratio (`3/4`) or a
sensor reading in scientific notation (`6.02e23`), and a claim or a pure
function needs its numeric value. The conversion gives that value with no
locale and no guessing. It rounds only where the target type names a rounding
mode. Every text that is not a number in the grammar has no value.

## Inputs

- The operand: a `Text[min, max; profile]` value (FR-141) or a `String` value
  (FR-322 `scalar_type`/`string`).
- The target type `T` (FR-140, FR-142, FR-148, FR-044):
  - `Integer` or `Int[lo, hi]`;
  - an unbounded `Decimal` or `Decimal[c1, c2; s1, s2; m]`;
  - an unbounded `Rational` or `Rational[lo, hi; dmin, dmax]`;
  - `Float32[m]` or `Float64[m]`, where an omitted `m` is strict `exact`;
  - `Quantity<u>`.
- The selected `quire.value.accounting/v1` limits.

## Outputs

One of these:

- a completed value of `T`. For an IEEE target it is a bit pattern and its
  flag set. For a rounded decimal it carries a `DecimalLoss`;
- an undefined outcome with no value;
- a refusal with no value (see "Value and outcomes");
- `incomplete` at the first denied charge.

I13 dispositions are not evaluator outputs.

## Behavior

### Source form and operation identities

The source form is the existing `convert<T>(e)`; no word is reserved. When the
static type of `e` is a `Text[..]` type or `String`, `convert<T>(e)` SHALL
type-check exactly when `T` is one of the target types under Inputs. The
result type is `T`. For any other `T` there is no conversion from text: a
`Boolean`, a `Text[..]` or `String` other than the operand's own type, enum,
`UUID`, `Timestamp`,
`Option`, collection, record, tuple, union or reference target refuses
`refused { code: ill_typed, cause: operator-ineligible }`.

The checked package records the conversion as one
`quire.checked-operation-catalog/v1` identity. Every row has operator
`convert`, result `member` and member `type_argument` naming `T`'s type node.
A `Text[..]` operand selects the `quire.op.text.*` row: operands `["text"]`,
and laws `text_profile` plus any law the target needs. A `String` operand
selects the `quire.op.string.*` row: operands `["string"]`, and no
`text_profile` law, because a `String` has no text profile.

| Target | Text identity | String identity | Further law | Mode | Constraint |
| --- | --- | --- | --- | --- | --- |
| `Integer`, `Int[..]` | `quire.op.text.to_integer` | `quire.op.string.to_integer` | none | none | `member_family` `integer` |
| `Decimal`, `Decimal[..]` | `quire.op.text.to_decimal` | `quire.op.string.to_decimal` | none | `rounding`, pinned by `T` | `member_family` `decimal` |
| `Rational`, `Rational[..]` | `quire.op.text.to_rational` | `quire.op.string.to_rational` | none | none | `member_family` `rational` |
| `Float32[..]` | `quire.op.text.to_float32` | `quire.op.string.to_float32` | `ieee_profile` | `rounding`, pinned by `T` | `member_family` `float32` |
| `Float64[..]` | `quire.op.text.to_float64` | `quire.op.string.to_float64` | `ieee_profile` | `rounding`, pinned by `T` | `member_family` `float64` |
| `Quantity<u>` | `quire.op.text.to_quantity` | `quire.op.string.to_quantity` | none | `rounding` = `exact`, pinned by `T` | `member_family` `quantity` |

The conversion is a `convert`, so its `rounding` mode is pinned by the result
type alone (FR-322 `type_pinned_modes`). A decimal or IEEE target pins its
declared mode, or `exact` when the mode is omitted. A quantity target pins
`exact`. An integer or rational target pins no mode, so its row carries
`mode` `null`. A `Text[..]` operand's profile is read from the operand's type
node. The `text_profile` law names the Unicode definition that profile
applies.

### Reading the operand

The kernel SHALL read a `Text[..]` operand's retained scalar sequence under its
profile (FR-141). That is the normalized sequence under `nfc`, `nfd`, `nfkc`
and `nfkd`, the decoded sequence under `unicode-scalars`, and the payload
under `binary-utf8`. So two equal texts convert to equal values, and under
`nfkc` the fullwidth `５` (U+FF15) reads as `5`. The kernel SHALL read a
`String` operand's Unicode scalar sequence as it is, with no normalization, as
`String` equality compares it (FR-322). The operand is a number for `T`
exactly when that whole sequence matches `T`'s production below.

### Number grammar

In the productions below, `digit` is one of the ASCII digits U+0030 to
U+0039, `-` is U+002D, `.` is U+002E, `/` is U+002F, `e` is U+0065, and the
space is U+0020:

```ebnf
natural       = '0' | nonzero, { digit } ;
nonzero       = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;
positive      = nonzero, { digit } ;
fraction      = '.', digit, { digit } ;

integer-text  = [ '-' ], natural ;
decimal-text  = [ '-' ], natural, [ fraction ] ;
rational-text = decimal-text | [ '-' ], natural, '/', positive ;
float-text    = [ '-' ], natural, [ fraction ], [ exponent ]
              | [ '-' ], 'inf' | 'nan' ;
exponent      = 'e', [ '-' ], natural ;
quantity-text = rational-text, [ ' ', unit-name ] ;
```

| Target kind | Production | Sign of zero |
| --- | --- | --- |
| integer | `integer-text` (FR-181's decimal string) | no `-` before a zero value: `-0` is not a number |
| decimal | `decimal-text` | no `-` before a zero value: `-0` and `-0.00` are not numbers |
| rational | `rational-text` | no `-` before a zero value: `-0`, `-0.0` and `-0/3` are not numbers |
| IEEE | `float-text` | `-` is allowed on zero and gives negative zero; no `-` on a zero exponent (`1e-0` is not a number) |
| quantity | `quantity-text` | as rational |

A quantity's `unit-name` is the name of `T`'s unit `u` as the unit declares
it, an identifier compared scalar for scalar. For a domain-package unit it is
the unit's `name` (FR-142), without any `M::` qualifier, so `3 ms` reads under
both `Quantity<ms>` and `Quantity<M::ms>`. A quantity text with no unit name
is a magnitude in `u`. A text that names another unit, even one of the same
dimension, is not a number for `T`. Converting between units is FR-142's
explicit `convert<Quantity<..>>` of a quantity.

No `+` sign, upper-case `E`, `+` in an exponent, leading zero, leading or
trailing white space, digit grouping, other decimal or fraction separator,
hexadecimal significand, `Infinity` or `NaN` spelling, signed `nan`,
non-ASCII digit or locale is part of any production. Text outside its
target's production is not a number for that target. So `1.0` is not an
integer, `1e3` is not a decimal and `3/4` is not a decimal.

### Value and outcomes

The kernel SHALL first form the exact spelled value:

- an `integer-text` spells the integer it denotes;
- a `decimal-text` spells the decimal of signed coefficient `c` and scale
  `s`: `c` is its digits read as one integer, and `s` is the number of digits
  after `.`, or zero with no `.`. So `1.50` spells (`150`, `2`) (FR-140
  "Literal spelling determines an exact coefficient/scale before
  normalization");
- a fraction `n/d` spells the rational `n/d`;
- a finite `float-text` spells the exact rational `c × 10^k`, where `c` is its
  signed significand digits read as one integer and `k` is the exponent minus
  the number of fraction digits.

It SHALL then produce the result for `T`:

- **Integer.** An integer target receives the spelled integer. A number
  outside `Int[lo, hi]` (FR-044) SHALL be
  `refused { code: integer_out_of_domain, cause: outside-domain }`.
- **Decimal.** A decimal target receives (`c`, `s`) as spelled when `s` is at
  most the target's maximum scale `s2`, or when the target is the unbounded
  `Decimal`: the result is retained as spelled, like a literal, and is not
  rescaled to `s2` (the FR-140 exception for this conversion). So `1.5` into
  `Decimal[0, 1000; 0, 2; exact]` retains (`15`, `1`), equal by value to
  `1.50`. When `s > s2`, the conversion is FR-140's rounding of the exact
  value into target scale `s2` under `T`'s mode `m`:
  - discarded digits that are all zero are no rounding step, so `1.500` into
    scale 2 gives (`150`, `2`);
  - at a rounding step, strict `exact` SHALL be
    `refused { code: inexact_decimal, cause: nonzero-discarded-digit }`;
  - at a rounding step under any other mode, the result is the FR-140 rounded
    coefficient at scale `s2`, with its `DecimalLoss`.

  The value is then checked for FR-140 membership. A nonmember SHALL be
  `refused { code: decimal_out_of_domain, cause: outside-domain }` and is
  never re-rounded, clamped or widened.
- **Rational.** A rational target receives the spelled value reduced to
  lowest terms with a positive denominator, so `2/4` gives `1/2` and `0.25`
  gives `1/4`. A reduced value whose numerator lies outside `[lo, hi]` or
  whose denominator lies outside `[dmin, dmax]` (FR-044) SHALL be
  `refused { code: rational_out_of_domain, cause: outside-domain }`. A
  rational is never approximated to fit.
- **IEEE.** `inf` and `-inf` give the signed infinity of `T`'s width, and
  `nan` gives that width's positive canonical quiet NaN (`0x7fc00000`,
  `0x7ff8000000000000`), each with no flag. A finite `float-text`, zeros
  included, is FR-148's explicit conversion of an exact value to an IEEE
  width: the spelled value is rounded once to the width under `T`'s mode, with
  FR-148's flags, overflow and tininess-after-rounding underflow. A spelled
  zero keeps its sign: `-0` gives negative zero. Under strict `exact`, a value
  the width does not represent without discarded information, overflow
  included, SHALL be
  `refused { code: ieee_not_exact, cause: rounding-required }` with its
  would-be flags and no rounded substitute (FR-148).
- **Quantity.** A quantity target receives the quantity of `T`'s unit whose
  exact magnitude is the reduced spelled value. `Quantity<u>` bounds no
  magnitude (FR-142), so a quantity is never refused for its size.
- **Not a number.** A text that is not a number for `T`, the empty text
  included, SHALL be undefined and produce no value. This follows FR-147,
  where a zero divisor is undefined, and FR-148, where a NaN operand of an
  IEEE-to-rational conversion is undefined.

### Totality in a pure function

The conversion carries no pure-function body obligation (FR-146). Whether a
text is a number, and whether its number lies in `T`, is decided at evaluation,
as for FR-148's IEEE-to-rational conversion. Its undefined and refused outcomes
are evaluation outcomes that propagate, and none of them refuses linking.

### Accounting

Under `quire.value.accounting/v1` the conversion SHALL charge, in this order:

1. `text.input-bytes` and `text.decode-scalars` for its operand.
2. For a `Text[..]` operand under `nfc`, `nfd`, `nfkc` or `nfkd`,
   `text.normalize-input`, then one `text.normalize-output` before each
   scalar of the normalized sequence. A `String` operand and the
   non-normalizing profiles charge neither.
3. The grammar check, which charges nothing. A text that is not a number is
   undefined here.
4. The target step:
   - integer: the domain check, which charges nothing;
   - rational and quantity: reduction and the rational domain check, which
     charge nothing;
   - decimal: at a rounding step, a strict `exact` refusal is decided before
     any further charge. Under any other mode the rounding step charges
     `decimal.rounding` with `integer_bits=bits(c)` and
     `decimal_digits=digits(c)`. The membership check follows and charges
     nothing;
   - IEEE: for a finite text, `ieee.round` with `integer_bits` = the target
     width, then the strict `exact` decision. `inf`, `-inf` and `nan` make no
     `ieee.round` charge.

The conversion makes no additional charge for materializing the spelled
numeric value or retaining the converted result. A grammar failure follows
the operand and normalization charges, if any. A target refusal follows those
charges and any target-specific rounding charge. A denied named charge returns
`incomplete` with no value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-478-AC-1 | Under `unicode-scalars`, `convert<Integer>` of `0`, `42`, `-7` and `170141183460469231731687303715884105728` gives `0`, `42`, `-7` and that exact integer. `convert<Int[0, 99]>` of `42` gives `42`, and of `100` and of `-1` refuses `integer_out_of_domain`/`outside-domain`. `convert<Decimal>` of `1.50` gives coefficient `150` and scale `2`, of `-0.5` coefficient `-5` and scale `1`, and of `3` coefficient `3` and scale `0`. | Test (TC-964) |
| FR-478-AC-2 | Each of these texts is undefined with no value for `convert<Integer>` under `unicode-scalars`: the empty text, `+5`, `05`, `-0`, ` 5`, `5 `, `1,000`, `1e3`, `1.0`, `2/1`, `٣` (U+0663) and `５` (U+FF15). For `convert<Decimal>` the same holds for `.5`, `5.`, `-0.00`, `1.2.3`, `1,5`, `1e3` and `3/4`. Under `nfkc`, `５` normalizes to `5` and converts to `5`; under `nfc` it is undefined. | Test (TC-964) |
| FR-478-AC-3 | The type checker admits `convert<T>` of a `Text[..]` operand, and of a `String` operand, to each of `Integer`, `Decimal[0, 1000; 0, 2; exact]`, `Rational[-9, 9; 1, 9]`, `Float32[nearest-even]`, `Float64` and `Quantity<ms>` (AC-10's units). Over `t: Text[0, 64; unicode-scalars]`, to `Boolean`, `Text[0, 8; nfc]`, an enum, `UUID` or a record it refuses `ill_typed`/`operator-ineligible`. Over `t: Text[0, 8; nfc]` the checked package records `quire.op.text.to_integer` for `Int[0, 99]` with mode `null`, `quire.op.text.to_decimal` for `Decimal[0, 1000; 0, 2; nearest-even]` with mode `rounding` `nearest-even`, `quire.op.text.to_rational` for `Rational[-9, 9; 1, 9]` with mode `null`, `quire.op.text.to_float64` for `Float64[toward-zero]` with laws `text_profile` and `ieee_profile` and mode `rounding` `toward-zero`, and `quire.op.text.to_quantity` for `Quantity<ms>` with mode `rounding` `exact`. Each has member `type_argument` naming the target, and each `quire.op.text.*` row carries the `text_profile` law. Over `s: String`, the same targets record the `quire.op.string.*` identity of the same name, with no `text_profile` law. | Test (TC-964) |
| FR-478-AC-4 | `function n using v(t: Text[0, 8; unicode-scalars]): Int[0, 99] pure { convert<Int[0, 99]>(t) }` links with no body obligation. Called with `abc` it is undefined, and with `100` it is refused `integer_out_of_domain`, each at evaluation. | Test (TC-964) |
| FR-478-AC-5 | With exact limits, `convert<Int[0, 99]>` of `42` succeeds after `text.input-bytes` and `text.decode-scalars`; denying `text.input-bytes` returns `incomplete` with no value. `abc` is undefined after operand and applicable normalization charges, with no value. `100` into `Int[0, 99]` is refused after those charges and before a result is returned. | Test (TC-964) |
| FR-478-AC-6 | Decimal rounding is the target's. `convert<Decimal[0, 1000; 0, 2; exact]>` of `1.555` refuses `inexact_decimal`/`nonzero-discarded-digit`, of `1.500` gives coefficient `150` and scale `2`, of `1.5` gives coefficient `15` and scale `1` (retained as spelled, not rescaled to scale 2), and of `10.01` refuses `decimal_out_of_domain`/`outside-domain`. `convert<Decimal[0, 1000; 0, 2; nearest-even]>` of `1.555` gives coefficient `156` and scale `2`, with `DecimalLoss` exact value `311/200` and mode `nearest-even`, and of `1.565` gives coefficient `156`, a tie to even. The `nearest-even` conversion of `1.555` charges `decimal.rounding` with `integer_bits` 11 and `decimal_digits` 4 after the operand and applicable normalization charges. The `exact` refusal charges no `decimal.rounding`. | Test (TC-964) |
| FR-478-AC-7 | `convert<Rational[-9, 9; 1, 9]>` of `2/4` gives `1/2`, of `-3` gives `-3/1`, and of `0.25` gives `1/4`. Of `1/10` and of `10` it refuses `rational_out_of_domain`/`outside-domain`. `convert<Rational>` of `6/4` gives `3/2`. Each of `1/0`, `1/-2`, `-0/3`, `+1/2`, `1 / 2`, `01/2`, `1/02`, `1.5/2` and `1e1` is undefined. | Test (TC-964) |
| FR-478-AC-8 | `convert<Float64[nearest-even]>` gives bits `0x3fb999999999999a` with flags `{inexact}` for `0.1`, and `0x3fd0000000000000` with no flag for `0.25` and for `2.5e-1`. It gives `0x8000000000000000` for `-0`, `0x7ff0000000000000` with `{overflow, inexact}` for `1e309`, and `0x0000000000000000` with `{underflow, inexact}` for `1e-400`. It gives `0x7ff0000000000000` for `inf`, `0xfff0000000000000` for `-inf` and `0x7ff8000000000000` for `nan`, each with no flag. `convert<Float64[toward-zero]>` gives `0x3fb9999999999999` for `0.1` and `0x7fefffffffffffff` with `{overflow, inexact}` for `1e309`. `convert<Float64[toward-positive]>` of `1e-400` gives `0x0000000000000001` with `{underflow, inexact}`. `convert<Float32[nearest-even]>` of `0.1` gives `0x3dcccccd` and `convert<Float32[toward-zero]>` gives `0x3dcccccc`, each with `{inexact}`. | Test (TC-964) |
| FR-478-AC-9 | Strict `exact` IEEE: `convert<Float64>` of `0.25` gives `0x3fd0000000000000`. Of `0.1` it refuses `ieee_not_exact`/`rounding-required` with would-be flags `{inexact}`, and of `1e309` with would-be flags `{overflow, inexact}`, each after `ieee.round` and with no bits. `convert<Float64[nearest-even]>` of `0.1` charges `text.input-bytes`, `text.decode-scalars`, then `ieee.round` (`integer_bits` 64). Of `inf` it charges no `ieee.round`. Each of `1E3`, `1e+3`, `1e03`, `1e-0`, `.5`, `5.`, `1e`, `Infinity`, `NaN`, `-nan`, `+inf` and `0x1p3` is undefined. | Test (TC-964) |
| FR-478-AC-10 | With `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;`, `convert<Quantity<ms>>` gives `3 ms` for `3` and for `3 ms`, and `1/2 ms` for `0.5` and for `2/4 ms`. Each of `3 s`, `3ms`, `3  ms` (two spaces), `3 MS` and `ms` is undefined. Under the domain package `acme/rpc` imported as `M` with its own unit `ms`, `convert<Quantity<M::ms>>` of `3 ms` gives `3` of that unit, and of `3 M::ms` it is undefined. | Test (TC-964) |
| FR-478-AC-11 | Over `s: String`, `convert<Integer>(s)` of `42` gives `42`, and of `５` (U+FF15) it is undefined, because a `String` is never normalized. `convert<Float64[nearest-even]>(s)` of `0.1` gives `0x3fb999999999999a`. `convert<Rational[-9, 9; 1, 9]>(s)` of `2/4` gives `1/2`. Each conversion charges `text.input-bytes` and `text.decode-scalars` and no `text.normalize-*` point. | Test (TC-964) |

## Dependencies

- FR-141 (text profiles and the retained scalar sequence), FR-140 (decimal
  coefficient, scale, rounding step and membership), FR-044 (integer and
  rational ranges), FR-142 (units and `Quantity<u>`), FR-148 (IEEE widths,
  rounding, flags and the exact-to-IEEE conversion), FR-147 and FR-148 (the
  undefined outcomes this one follows), FR-146 (pure-function totality),
  FR-149 (conversion classes), FR-322 (the checked operation catalog, the
  `convert` operator, `type_pinned_modes` and the `string` family).
- The twelve catalog rows above follow in `quire-verification-contracts`'s
  `checked-operation-catalog-v1` (a cross-repository dependency). Both checked-package V2 schemas then
  enumerate them, because the schemas list exactly the catalogued identities.
