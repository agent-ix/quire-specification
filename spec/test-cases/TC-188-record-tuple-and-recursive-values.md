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

Use the exact version-locked subject and inputs named by FR-143. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions at revision `1-draft.1` by
their exact DefinitionRefs in
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| R01 | `record P { a: Integer; b: Integer?; }` values `P { a: 1 }` and `P { a: 1 }`; `P { a: 1, b: 2 }` and `P { b: 2, a: 1 }` | true; true, because source field order is not identity |
| R02 | `record A { x: Integer; }` and `record B { x: Integer; }` in one package, compare `A { x: 1 } == B { x: 1 }`; then `type C = A;` and compare a `C` value with an equal `A` value | ill-typed; true, because an alias creates no declaration identity |
| R03 | Two checked packages each declare `record R { x: Integer; }`, and one expression mixes their values; then package P imports library L's export `R` through two dependency paths that resolve one FR-307 export | ill-typed; the two paths yield one declaration key, and equality of equal values is true |
| R04 | A declaration spelled `variant V { A, B }` | `refused { code: invalid_syntax }` |
| R05 | Recursion rule on `record List { head: Integer; tail: List?; }`, `record N { kids: Sequence<N>[0,2]; }`, `record A { b: B; }` with `record B { a: A?; }`, and `record O { r: Reference<O>; }` | each is admitted |
| R06 | Recursion rule on `record Loop { next: Loop; }`, `record M { kids: Sequence<M>[1,2]; }` and `tuple Pair(Integer, Option<Pair>);` | ill-typed, naming the cycles `Loop -> Loop`, `M -> M` (does not escape) and `Pair -> Pair` (unnamed edge) |
| R07 | For `record P { a: Integer; b: Integer?; }`: `P { a: 1 }`, `P { a: 1, b: null }`, `P { b: 2 }`, `P { a: 1, c: 2 }`, `P { a: 1, a: 2 }` and `P { a: null }`; for `tuple T(Integer, Integer);`: `T(1)`; and `null` supplied for an `Option<Integer>` parameter | `b` absent; `b` explicit null, unequal to the first; then ill-typed at `P { b: 2 }` (missing `a`), `c` (undeclared), the second `a` (duplicate), `null` (required field), `T(1)` (arity) and `null` (not an option value) |
| R08 | `record Two { a: Set<Integer>[0,1]; b: Integer; }`, `function f(): Integer pure { 1 }`, `function g(): Integer pure { f() }` and parameter `q = sequence[1, 2]` of `Sequence<Integer>[0,2]`; evaluate `Two { b: g(), a: convert<Set<Integer>[0,1]>(q) }` under `ScalarLimitsV1` with every counter unlimited except `work_units: 5` | `refused { code: cardinality_out_of_bound }` after exactly two `collection.visit`, one `collection.member-test` (`p = 1`) and `collection.bound`: four work units, with no `function.call`, because field `a` is evaluated first and stops construction |
| R09 | A snapshot with objects `o1` and `o2` of `record O { r: Reference<O>; }` whose references point at each other; then `convert<Reference<O>>(1)` | the reference cycle is admitted and each `r` compares by identity triple only; the conversion is ill-typed, because no source form creates an object identity |

## Expected Results

Every positive and boundary result matches FR-143; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
