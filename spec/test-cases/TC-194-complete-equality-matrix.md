---
id: TC-194
title: "Complete equality matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-149
    type: verifies
---

# TC-194: Complete equality matrix

## Description

Execute positive, negative and cross-type comparisons for every complete-V1 value kind.

## Test Procedure

Select language `ix:native`, edition `1-draft` revision `1-draft.2`, and the
`quire.value.complete/v1`, `quire.value.text.unicode-17.0.0/v1` and
`quire.value.ieee754-2019-default/v1` definitions at revision `1-draft.1`, using
their exact DefinitionRefs from
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Build fixtures with distinct I04 semantic-node keys
for declarations that intentionally share names or shapes. Execute every row
through the real local Rust boundary:

Every `ill-typed` expectation below means a type-checking
`refused { code: ill_typed }`; it is not a peer evaluator outcome.

| Vector | Operands and selected relation | Expected result/disposition |
| --- | --- | --- |
| E01 | Boolean `true,true`; `true,false` | true; false |
| E02 | Integer pairs `(1,1)` and `(1,2)`; then `Int[0,2]` value `1` and `Rational[0,2;1,1]` value `1/1` with `convert<Rational[0,2;1,1]>` on the left operand, and without conversion | true; false; true; ill-typed |
| E03 | Decimal pairs `(10,1)`/`(100,2)` and `(10,1)`/`(11,1)`; then `Decimal[0,100;0,2]` value `(10,1)` and `Rational[0,100;1,100]` value `1/1` with `convert<Rational[0,100;1,100]>` on the left operand | true; false; true with both source types/provenance unchanged |
| E04 | `Rational[0,1;1,3]` value `1/3` under `convert<Decimal[0,100;2,2;nearest-even]>` compared with decimal `(33,2)` | ill-typed, not false, because the source denominator bound exceeds one |
| E05 | `100 cm` and `1 m` after explicit canonical-unit conversion; then `1 m` and `1 s` | true; incompatible dimensions are ill-typed |
| E06 | U+00E9 and `U+0065 U+0301` under `nfc`; `a` and `b` under `nfc`; same payload typed as `nfc` and `binary-utf8` | true; false; different profiles are ill-typed |
| E07 | `enum-A::READY` with itself and with `enum-A::DONE`, then with `enum-B::READY` | true; false; different declaration node is ill-typed without an explicit common type |
| E08 | two `none` values of the same `Option<Integer>`; `present(1)` pairs; `none` versus `present(1)` | true; true; false |
| E09 | same declared nullable field containing `null`; then its absent state versus explicit `null` | true; false, with no absence/null coercion |
| E10 | records of declaration `record-A` with equal fields, one unequal nested field, then equal-shaped `record-B` | true; false; different declaration is ill-typed |
| E11 | tuples of declaration `tuple-A` with equal positions, one unequal position, then equal-shaped `tuple-B` | true; false; different declaration is ill-typed |
| E12 | sequences `[1,2]`, `[1,2]`, `[2,1]` | first pair true; permutation false |
| E13 | sets inserted as `[1,2]` and `[2,1]`, then `[1,3]` | permutation true; changed membership false |
| E14 | bags `[1,1,2]`, `[2,1,1]`, `[1,2,2]` | permutation true; changed multiplicity false |
| E15 | ordered sets `[1,2,1]`, `[1,2]`, `[2,1]` | first-occurrence normalization makes first pair true; changed order false |
| E16 | two `Reference<object-A>` values with the same universe and object identity but different loaded state and observation; then different object identities with equal fields; then equal object labels from different universes | true; false; `refused { code: foreign_reference }` before `equality.plan`, with no charge |
| E17 | binary32 `+0` and `-0` under numeric equality, total-order equivalence and bit identity | true; false; false |
| E18 | qNaN `0x7fc00001` with itself under numeric equality, total-order equivalence and bit identity | false; true; true |
| E19 | binary32 `1.0` and binary64 `1.0` without conversion, then after explicit exact conversion to binary64 | ill-typed; numeric equality true while both source widths remain unchanged |
| E20 | Evaluate two outer-record constructor expressions where a required nested field expression returns undefined, refused or incomplete; then, for `record Two { a: Integer; b: Integer; }`, evaluate the left operand `Two { b: eB, a: eA }` where `eA` is refused and `eB` would charge and be incomplete | the same disposition propagates before either outer value exists, and no Boolean is produced; then `eA`'s refusal propagates because field `a` is first in declaration order, `eB` and the right operand are never evaluated and no charge is made |
| E21 | Two equal values of `record List { head: Integer; tail: List?; }` containing integers 1–8 with the last `tail` absent, once as duplicated trees and once with shared immutable DAG tails, under `ScalarLimitsV1 { integer_bits: 4, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 17, work_units: 19, result_units: 1 }`; then deny the 17th `equality.pair` and separately deny `equality.result-retain` | both admitted representations return true and charge exactly 17 pairs; either denied named charge is incomplete with no Boolean |
| E22 | Under unlimited limits, with `record Holder { r: Reference<Obj>; }` (no canonical key) and `h1`, `h2`, `h3` holding distinct objects of one universe: compare set `[h1,h2]` with `[h2,h1]`, `[h1,h2]` and `[h1,h3]`; compare bag `[h1,h1,h2]` with `[h2,h1,h1]`, `[h1,h2,h1]`, `[h1,h2,h2]` and `[h2,h2,h2]`; then set `[h1,h2]` with `[h1]` | true, true and false, each with `1 + 4 × 2 = 9` pairs and 11 work units in both operand orders; true, true, false and false, each with `1 + 9 × 2 = 19` pairs and 21 work units in both operand orders; false with one pair and three work units, because member counts differ. Every count is identical across insertion and hash orders, and serialization refusal remains separate |
| E23 | Run E21's duplicated-tree comparison with E21's tuple except `work_units: 18` | the `equality.plan` reservation of `17 + 2 = 19` work units is unavailable before any counter changes: exactly `incomplete { limit_kind: work_units, limit: 18, consumed: 0, next_charge: 19, charge_point: equality.plan }`, with no pair event, Boolean or other member |
| E24 | Compare records `{ d: 1 cm }` and `{ d: 1 cm }` of one declared record type under `ScalarLimitsV1 { integer_bits: 0, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 2, work_units: 4, result_units: 1 }`; then set `work_units: 3` | true after exactly `equality.plan` (two planned pairs), `equality.pair` for `$` and for `$.d`, and `equality.result-retain`: four work units and one result unit, with no `unit.*` read, edge or event; then `incomplete { limit_kind: work_units, limit: 3, consumed: 0, next_charge: 4, charge_point: equality.plan }` |
| E25 | Under `ScalarLimitsV1 { integer_bits: 0, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 1, work_units: 3, result_units: 1 }`, compare Boolean `true == true`, integer `7 != 7`, decimal `(10,1) == (100,2)` and E16's equal references; then repeat the Boolean comparison with `work_units: 2` | true, false, true and true, each after exactly `equality.plan`, one `equality.pair` and `equality.result-retain`, with no other family charge; then `incomplete { limit_kind: work_units, limit: 2, consumed: 0, next_charge: 3, charge_point: equality.plan }` |
| E26 | With left operand `convert<T>(e)`: `Int[0,2]` 1 to `Decimal[0,200;2,2]` versus `(100,2)`; `Decimal[0,9;0,0]` 3 to `Int[0,9]` versus 3; `Rational[0,5;1,1]` 4 to `Int[0,5]` versus 4; `Decimal[0,9;0,1]` 1 to `Int[0,9]`; `Int[0,300]` 1 to `Decimal[0,200;2,2]`; `Integer` 1 to `Rational[0,2;1,1]`; `Decimal[0,100;1,2]` `(10,1)` to `Decimal[0,100;0,2]` versus `(1,0)`; `Decimal[0,100;0,2]` to `Decimal[0,100;1,2]`; then `let x = convert<Decimal[0,100;2,2;nearest-even]>(r) in x == d` for E04's `r` and `d` | true; true; true; ill-typed; ill-typed; ill-typed; true; ill-typed; true, with the conversion's `DecimalLoss` retained. Each ill-typed result is a static refusal with no charge even though the value fits |
| E27 | Pair counts under unlimited limits: `Option<Integer>` `none`/`none`, `present(1)`/`present(2)`, `none`/`present(1)`; `record R { a: Integer; b: Integer?; }` `{a:1}`/`{a:1}`, `{a:1,b:null}`/`{a:1}`, `{a:1,b:2}`/`{a:2,b:2}`; sequences `[1,2]`/`[1,2,3]` and `[1,2]`/`[1,3]`; keyed sets `[1,2]`/`[2,1]` and `[1,2]`/`[1,2,3]`; keyed bags `[1,1,2]`/`[1,2,2]` | `(true,1)`, `(false,2)`, `(false,1)`; `(true,3)`, `(false,3)`, `(false,3)`, with no early exit; `(false,1)`, `(false,3)`; `(true,3)`, `(false,1)`; `(false,4)` from rank pairs `(1,1)`, `(1,2)` and `(2,2)`. Work units are the pair count plus two |
| E28 | `==` on binary32 `0x3f800000` with itself, and on two values of `record F { x: Float32; }`; then declare `Set<Float64>[0,2]` | each is `refused { code: ill_typed }` before any charge |

For total-order equivalence in E17/E18, evaluate `totalOrder(a,b)` and
`totalOrder(b,a)` and require both; do not alias it to numeric equality. For
E02-E06 and E19, snapshot both source values before comparison and prove that
explicit conversion creates only comparison values. For E10-E16, include an
equal, unequal and wrong-declaration/kind mutation so every recursive and
collection relation is exercised rather than inferred from one generic helper.
For every value-kind row E01-E19, also compare its left operand with a value of
a disjoint kind without conversion and require `ill_typed`, so the cross-type
matrix is measured rather than inferred from numeric examples.

Generate bounded values for every matrix row with same-type equal and unequal
pairs plus disjoint-kind pairs. Numeric generators include normalized and
non-normalized equal representations and explicit lossless/lossy conversion
requests; text includes same-profile equal/unequal values; enums include the
same declaration with equal/different members and different declarations;
collections vary order, uniqueness and multiplicity; references vary identity
and loaded state. The oracle is the selected row relation plus the exact common-
type precondition. Shrink by semantic structure (magnitude, scalar count,
field/element count, recursion depth) while preserving the row and conversion
precondition. For recursive values, generate duplicated-tree and shared-DAG
representations of the same occurrence tree and an illegal containment back-edge;
the first two have identical equality/accounting, while the back-edge refuses
construction under FR-143 before equality.

For E21, the exact depth-first pair paths are `$`, then
`$.head`, `$.tail`, `$.tail.head`, continuing alternately through
`$.tail^7`, `$.tail^7.head`, and the terminal `$.tail^8`; that is eight record
pairs, eight integer-head pairs and one terminal equal pair of two absent `tail`
slots. `equality.plan`
adds one work unit, the 17 pair events add 17 and result retention adds one.

## Expected Results

Every matrix row returns its tabled Boolean or typed
ill-typed/refused/incomplete disposition. No implicit or lossy conversion,
matching local name, equal record shape, equal referenced state, collection
iteration order or absence/null substitution creates equality. Nested failure
and exhaustion propagate without a false Boolean, no comparison mutates either
operand, and independent sibling comparisons retain their results.
