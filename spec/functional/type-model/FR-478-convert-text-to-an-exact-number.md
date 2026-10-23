---
id: FR-478
title: "Convert text to an exact integer or decimal"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-141
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-140
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-478: Convert text to an exact integer or decimal

## Description

When evaluating `convert<T>(e)` whose operand `e` has a `Text[..]` type and
whose target `T` is an integer or decimal type, the semantic kernel SHALL read
the operand's text as a number in one fixed, locale-free grammar and return
that exact number as a value of `T`. Text that is not a number in the grammar
is undefined, and a number outside `T` is refused.

## Use case

A model holds a number that arrived as text, such as a quantity typed into a
form field or a code read from a document, and a claim or a pure function
needs its numeric value. The conversion gives that value exactly, with no
locale, rounding or guessing, and every text that is not a number in the
grammar has no value.

## Inputs

- The operand: a `Text[min, max; profile]` value (FR-141).
- The target type `T`: `Integer`, `Int[lo, hi]`, an unbounded `Decimal`, or
  `Decimal[c1, c2; s1, s2; m]` (FR-140).
- The selected `quire.value.accounting/v1` limits.

## Outputs

A completed value of `T`; or an undefined outcome with no value; or
`refused { code: integer_out_of_domain | decimal_out_of_domain, cause:
outside-domain }` with no value; or `incomplete` at the first denied charge.
I13 dispositions are not evaluator outputs.

## Behavior

### Source form and operation identities

The source form is the existing `convert<T>(e)`; no word is reserved. When
the static type of `e` is a `Text[..]` type, `convert<T>(e)` SHALL type-check
only for an integer or decimal `T`, and for any other `T` (rational, float,
quantity, text, enum or any composite) SHALL refuse
`refused { code: ill_typed, cause: operator-ineligible }`. A `String`
operand has no text profile and has no such conversion
(`ill_typed`/`operator-ineligible`).

The checked package records the conversion as one
`quire.checked-operation-catalog/v1` identity:

| Target | Identity | Catalog row |
| --- | --- | --- |
| `Integer`, `Int[..]` | `quire.op.text.to_integer` | operator `convert`, operands `["text"]`, result `member`, laws `["text_profile"]`, mode `text_profile`, member `type_argument`, constraint `member_family` `integer` |
| `Decimal`, `Decimal[..]` | `quire.op.text.to_decimal` | operator `convert`, operands `["text"]`, result `member`, laws `["text_profile"]`, mode `text_profile`, member `type_argument`, constraint `member_family` `decimal` |

The operand's type pins the `text_profile` mode as for every
`quire.op.text.*` identity (FR-322 `type_pinned_modes`).

### Number grammar

The kernel SHALL read the operand's retained scalar sequence under its
profile (FR-141: the normalized sequence under `nfc`, `nfd`, `nfkc` and
`nfkd`, the decoded sequence under `unicode-scalars`, the payload under
`binary-utf8`), so two equal texts convert to equal values. A text is a
number exactly when that sequence matches, as a whole:

- for an integer target, `0` or an optional `-` followed by `[1-9][0-9]*`,
  QSpec FR-181's decimal string;
- for a decimal target, an optional `-`, then `0` or `[1-9][0-9]*`, then
  optionally `.` and one or more `[0-9]`, with no `-` before a value that is
  zero (`-0` and `-0.00` are not numbers).

The digits are the ASCII digits U+0030 to U+0039, `-` is U+002D and `.` is
U+002E. No `+` sign, exponent, leading zero, leading or trailing white
space, digit grouping, other decimal separator, non-ASCII digit or locale
is part of the grammar.

### Value and outcomes

- An integer text SHALL convert to the exact integer it spells.
- A decimal text SHALL convert to the exact decimal whose retained
  coefficient is its digits read as an integer, with its sign, and whose
  retained scale is the number of digits after `.` (zero with no `.`):
  `1.50` is coefficient `150`, scale `2`, and `-7` is coefficient `-7`,
  scale `0` (FR-140 "Literal spelling determines an exact coefficient/scale
  before normalization").
- A text that is not a number, the empty text included, SHALL be undefined
  and produce no value, as a zero divisor is (FR-147) and as a NaN
  IEEE-to-rational operand is (FR-148).
- A number that is not a member of `T`, by FR-044's integer range for an
  `Int[lo, hi]` target and FR-140's membership coefficient and scale for a
  `Decimal[..]` target, SHALL be
  `refused { code: integer_out_of_domain, cause: outside-domain }` or
  `refused { code: decimal_out_of_domain, cause: outside-domain }` with no
  value. A number is never rounded, clamped, truncated or rescaled to fit:
  `1.555` into a target of maximum scale 2 is refused, not rounded.
- An unbounded `Integer` or `Decimal` target SHALL accept every number of
  its grammar.

### Totality in a pure function

The conversion carries no pure-function body obligation (FR-146): whether a
text is a number is decided at evaluation, as FR-148's IEEE-to-rational
conversion is. Its undefined and refused outcomes are evaluation outcomes that
propagate, and neither refuses linking.

### Accounting

Under `quire.value.accounting/v1` the conversion SHALL charge, in this
order: `text.input-bytes` and `text.decode-scalars` for its operand; under
`nfc`, `nfd`, `nfkc` and `nfkd`, `text.normalize-input` and one
`text.normalize-output` before each scalar of the normalized sequence; then
the grammar check, which charges nothing; then `text-number.value`; then the
domain check; then `text-number.result-retain`. An undefined text therefore
makes no `text-number.*` charge, a refusal follows `text-number.value` and
precedes `text-number.result-retain`, and a denied charge returns
`incomplete` with no value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-478-AC-1 | Under `unicode-scalars`, `convert<Integer>` of `0`, `42`, `-7` and `170141183460469231731687303715884105728` gives `0`, `42`, `-7` and that exact integer; `convert<Int[0, 99]>` of `42` gives `42` and of `100` and of `-1` refuses `integer_out_of_domain`/`outside-domain`. `convert<Decimal>` of `1.50` gives coefficient `150`, scale `2`, of `-0.5` coefficient `-5`, scale `1`, and of `3` coefficient `3`, scale `0`; `convert<Decimal[0, 1000; 0, 2; exact]>` of `1.555` refuses `decimal_out_of_domain`/`outside-domain` and is never rounded. | Test (TC-964) |
| FR-478-AC-2 | Each of these texts is undefined with no value for `convert<Integer>`: the empty text, `+5`, `05`, `-0`, ` 5`, `5 `, `1,000`, `1e3`, `1.0`, `٣` (U+0663) and `５` (U+FF15) under `unicode-scalars`; and for `convert<Decimal>`: `.5`, `5.`, `-0.00`, `1.2.3` and `1,5`. Under `nfkc`, `５` normalizes to `5` and converts to `5`; under `nfc` it is undefined. | Test (TC-964) |
| FR-478-AC-3 | `convert<T>` of a `Text[..]` operand to `Rational`, `Float64`, a quantity, a `Text[..]` or an enum, and of a `String` operand to `Integer`, refuses at type checking `ill_typed`/`operator-ineligible`; the checked package of `convert<Int[0, 99]>(t)` records `quire.op.text.to_integer` with member `type_argument` naming `Int[0, 99]`, and of `convert<Decimal[0, 1000; 0, 2; exact]>(t)` records `quire.op.text.to_decimal`. | Test (TC-964) |
| FR-478-AC-4 | A pure function `function n using v(t: Text[0, 8]): Int[0, 99] pure { convert<Int[0, 99]>(t) }` links with no body obligation; called with `abc` it is undefined and with `100` refused `integer_out_of_domain`, each at evaluation. | Test (TC-964) |
| FR-478-AC-5 | With exact limits the conversion of `42` succeeds after `text.input-bytes`, `text.decode-scalars`, `text-number.value` and `text-number.result-retain`; denying `text-number.value` returns `incomplete` with no value; `abc` is undefined with no `text-number.*` charge; and `100` into `Int[0, 99]` is refused after `text-number.value` and before `text-number.result-retain`. | Test (TC-964) |

## Dependencies

- FR-141 (text profiles and the retained scalar sequence), FR-140 (decimal
  coefficient, scale and membership), FR-044 (integer ranges), FR-147 and
  FR-148 (the undefined outcomes this one follows), FR-146 (pure-function
  totality), FR-149 (conversion classes), FR-322 (the checked operation
  catalog and the `convert` operator).
- The catalog rows above are added to `quire-verification-contracts`'s
  `checked-operation-catalog-v1` and then enumerated by both checked-package
  V2 schemas, which list exactly the catalogued identities.
