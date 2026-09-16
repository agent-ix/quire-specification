---
id: TC-191
title: "Total pure function checking"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-191: Total pure function checking

## Description

Check typed pure calls, proved recursion, failed termination and prohibited effects.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-146. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions at revision `1-draft.1` by
their exact DefinitionRefs in
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| P01 | `record Node { head: Integer; next: Option<Node>; }` and `function last(n: Node): Integer pure decreases(n) { if present(n.next) then last(value(n.next)) else n.head }` | admitted: the recursive argument is a field projection of the parameter under the `present` guard fact |
| P02 | `function loop(n: Int[0,9]): Integer pure decreases(n) { loop(n) }`; the same without `decreases` | `refused { code: undefined_expression }` naming cycle `loop -> loop`, edge `loop(n)` and obligation `n < n`; the same refusal with obligation "missing measure" |
| P03 | `function even(n: Int[0,9]): Boolean pure decreases(n) { if n == 0 then true else odd(n - 1) }` with `odd` symmetric; then `odd` declaring `decreases(M2(n, 0))` for `tuple M2(Integer, Integer);` | admitted; `undefined_expression` naming the component `even -> odd -> even`, because the measure arity differs within it |
| P04 | `function q(a: Int[-9,9], b: Int[-9,9]): Rational[-9,9;1,9] pure { a / b }`; the same body as `if b != 0 then a / b else rational(0, 1)`; `value(o)` for `o: Option<Integer>` without `present(o)`; `reduce<Integer>(acc, x in s: acc + x)` for `s` of `Sequence<Integer>[0,3]` and of `Sequence<Integer>[1,3]` | `undefined_expression` at the division; admitted; `undefined_expression` at `value`; `undefined_expression` for minimum 0; admitted for minimum 1 |
| P05 | `function r(x: Integer): Integer pure { if true then 1 else undeclaredHost(x) }` | `refused { code: missing_declaration }` at `undeclaredHost` although the branch is unreachable, with no call edge |
| P06 | `last` from P01 applied to a three-node chain; then under `ScalarLimitsV1` with every counter unlimited except `work_units: 2` | the third node's head after exactly three `function.call` charges; `incomplete { limit_kind: work_units, limit: 2, consumed: 2, next_charge: 1, charge_point: function.call }` for the third call, and `last` remains admitted |

## Expected Results

Every positive and boundary result matches FR-146; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
