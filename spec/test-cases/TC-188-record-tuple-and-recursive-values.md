---
id: TC-188
title: "Record tuple and recursive values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-143
    type: verifies
---

# TC-188: Record tuple and recursive values

## Description

Compare same/different declared structures and construct finite versus cyclic recursive values.

## Test Procedure

Use the subject and inputs named by FR-143. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions through
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| R01 | `record P { a: Integer; b: Integer?; }` values `P { a: 1 }` and `P { a: 1 }`; `P { a: 1, b: 2 }` and `P { b: 2, a: 1 }` | true; true, because source field order is not identity |
| R02 | `record A { x: Integer; }` and `record B { x: Integer; }` in one package, compare `a = b` for parameters `a` of `A` and `b` of `B`; then `type C = A;` and compare a `C` parameter with an equal `A` parameter | `refused { code: ill_typed, cause: type-mismatch }`; true, because an alias creates no declaration identity |
| R03 | Two checked packages each declare `record R { x: Integer; }`, and one expression of package P compares a parameter of P's `R` with an expression of the other package's `R`; then a runtime input for P's `R` parameter carries the other package's declaration key; then package P imports library L's export `R` through two dependency paths that resolve one FR-307 export | `refused { code: ill_typed, cause: type-mismatch }` at checking, because no well-typed expression can hold both keys; `refused { code: invalid_runtime_input, cause: wrong-value-kind }` during input validation, before any evaluation or charge; the two paths yield one declaration key, and equality of equal values is true |
| R04 | A declaration spelled `variant V { A, B }` | `refused { code: invalid_syntax, cause: unexpected-token }` at `variant`, because `variant` is the loop-measure keyword and the sum declaration is spelled `union` |
| R05 | Recursion rule on `record List { head: Integer; tail: List?; }`, `record N { kids: Sequence<N>[0,2]; }`, `record A { b: B; }` with `record B { a: A?; }`, and `record O { r: Reference<M::Obj>; }` where `M::Obj` is an object type of imported model `M`; then `record P { x: Integer; }` with `record Bad { r: Reference<P>; }` | each of the first four is admitted; `Bad` is `refused { code: ill_typed, cause: type-mismatch }`, because a reference target must be a model object type |
| R06 | Recursion rule on `record Loop { next: Loop; }`, `record M { kids: Sequence<M>[1,2]; }` and `tuple Pair(Integer, Option<Pair>);` | ill-typed, naming the cycles `Loop -> Loop`, `M -> M` (does not escape) and `Pair -> Pair` (unnamed edge) |
| R07 | For `record P { a: Integer; b: Integer?; }`: `P { a: 1 }`, `P { a: 1, b: null }`, `P { b: 2 }`, `P { a: 1, c: 2 }`, `P { a: 1, a: 2 }` and `P { a: null }`; for `tuple T(Integer, Integer);`: `T(1)`; and `null` supplied for an `Option<Integer>` parameter | `b` absent; `b` explicit null, unequal to the first; then ill-typed at `P { b: 2 }` (missing `a`), `c` (undeclared), the second `a` (duplicate), `null` (required field), `T(1)` (arity) and `null` (not an option value) |
| R08 | `record Two { a: Set<Integer>[0,1]; b: Integer; }`, `function f using V(): Integer pure { 1 }`, `function g using V(): Integer pure { f() }` and parameter `q` of `Sequence<Integer>[0,2]` holding `1, 2`; evaluate `Two { b: g(), a: convert<Set<Integer>[0,1]>(q) }` under `ScalarLimitsV1` with every counter unlimited except `work_units: 6` | `refused { code: cardinality_out_of_bound, cause: above-maximum }` after exactly two `collection.visit`, one `collection.member-walk` (`occ(c) + occ(m) = 2`), one `collection.member-test` (`p = 1`) and `collection.bound`: six work units, with no `function.call` and no `composite.result-retain`, because field `a` is evaluated first in declaration order and stops construction. Evaluating `b` first would instead need two more `function.call` units and end incomplete |
| R09 | A snapshot of imported model `M` with objects `o1` and `o2` of object type `M::Node`, whose attribute `peer` of `Reference<M::Node>` in each points at the other; compare `o1.peer = o1.peer` and `o1.peer = o2.peer`; then `convert<Reference<M::Node>>(1)` | the reference cycle is admitted and each comparison is by identity triple only: true, false; the conversion is `refused { code: ill_typed, cause: type-mismatch }`, because no source form creates an object identity |
| R10 | `R { a: 1 }` for `record R { a: Integer; b: Integer?; }` under `ScalarLimitsV1` with every counter unlimited except `result_units: 1`; then unlimited | `incomplete { limit_kind: result_units, limit: 1, consumed: 0, next_charge: 2, charge_point: composite.result-retain }`; the record, after exactly `composite.result-retain` with `occ = 2`: one work unit and two result units |
| R11 | For `record P { a: Integer; b: Integer?; }` and function bodies over parameter `x` of `P`: `if present(x.b) then value(x.b) else 0`; `value(x.b)`; `x.b + 1`; `x.a + 1`; then `present(x.b)` evaluated for `P { a: 1 }`, `P { a: 1, b: null }` and `P { a: 1, b: 2 }` | admitted; `refused { code: undefined_expression, cause: unproved-presence }` when linked; `refused { code: ill_typed, cause: type-mismatch }`, because a `?` slot is not an `Integer` value; admitted; false, false, true |
| R12 | The model record value type `Money { amount_minor: Integer; currency: Text[0,3;binary-utf8] }` with subtype `TaxedMoney` adding `tax_minor: Integer`, and its subtype `GiftTaxedMoney`; parameters `a`, `b` of static type `Money` holding a `TaxedMoney` and a `Money` of equal `amount_minor` and `currency`; `a = b`, `a != b`, `b = a`; `a` holding a `GiftTaxedMoney` against `b` holding a `TaxedMoney`; two `TaxedMoney` of equal fields, then differing in `tax_minor` | `false`, `true`, `false`; `false`; `true`, then `false`; no refusal and no text-leaf comparison for the unequal declarations |
| R13 | Two units select the model of R12 as `model M = ...` and as `model N = ...`; each builds `TaxedMoney` value `t` with equal fields and a `Set<Money>[0,2]` of `t` and a `Money` | the declaration keys of `Money` and `TaxedMoney` equal their model node keys in both units; `t` in one unit equals `t` in the other; both sets list their members in the same order (FR-143-AC-17) |
| R14 | A runtime snapshot supplies a `TaxedMoney` value built in another checked package that selects the same model, and a plain record value of another package's own `R` | the `TaxedMoney` value is admitted (its key is a declaration key of the evaluating package's selected document); the `R` value is `refused { code: invalid_runtime_input, cause: wrong-value-kind }` (FR-143-AC-9) |
| R15 | `record P { a: Integer; v: Option<Integer>; }`, `function f using V(x: Option<Integer>): Option<Integer> pure { absent }`, and in a state clause over `self` with `v: Option<Integer>` the write `self.v = absent`; then `P { a: 1, v: absent }`, `f(absent)`, `if true then absent else x` at an `Option<Integer>` position, `x = absent` and `x = y` for parameters `x`, `y` of `Option<Integer>`, and `(lookup<M::A>(p, r) absent empty) = absent` for a key `p` holds and for one it does not | admitted and typed `Option<Integer>` at each position; the record holds the `none` of `v`; `f(absent)` is `none`; `x = absent` is true exactly when `x` is `none`; the lookup equality is false for the held key and true for the missing key |
| R16 | `absent = absent`; `let y = absent in y`; `present(absent)`; `value(absent)`; then `P { a: absent, v: absent }` for the R15 record, `absent` supplied for an `Integer` parameter, and `absent` for a `Reference<M::A>` position and a `Set<Integer>[0,2]` position | the first four `refused { code: ill_typed, cause: ambiguous-literal }` before any charge, no expected type; the rest `refused { code: ill_typed, cause: type-mismatch }`, no `Option<T>` expected (the `a` field, the `Integer` parameter, the reference and the set) |
| R17 | For R07's `record P { a: Integer; b: Integer?; }`: `P { a: 1, b: absent }`, `P { a: 1 }` and `P { a: 1, b: null }`; `lookup<M::A>(p, r) absent undefined`, `absent empty`, `absent refused` and `lookup<M::A>(p, r) absent absent` | `type-mismatch` (a `?` slot is not an `Option<Integer>` position); `b` absent; `b` explicit null, unequal to the second; the three modes of FR-153 unchanged; `refused { code: invalid_syntax, cause: unexpected-token }` at the second `absent`, which is no mode |

## Expected Results

Every positive and boundary result matches FR-143; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
