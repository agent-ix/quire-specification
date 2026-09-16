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
| P01 | `record Node { head: Integer; next: Option<Node>; }` and `function last using V(n: Node): Integer pure decreases(n) { if present(n.next) then last(value(n.next)) else n.head }` | admitted: the recursive argument `value(n.next)` is a projection path of the parameter whose `value` step is under the proved `present(n.next)` guard fact |
| P02 | `function loop using V(n: Int[0,9]): Integer pure decreases(n) { loop(n) }`; the same without `decreases` | `refused { code: undefined_expression, cause: unproved-decrease }` naming cycle `loop -> loop`, edge `loop(n)` and obligation `decrease` (`n < n`); the same refusal with obligation `missing-measure` |
| P03 | `function even using V(n: Int[0,9]): Boolean pure decreases(n) { if n = 0 then true else odd(n - 1) }` with `odd` symmetric; then `odd` declaring `decreases(M2(n, 0))` for `tuple M2(Integer, Integer);` | admitted; `refused { code: undefined_expression, cause: unproved-decrease }` naming the component `even -> odd -> even` and obligation `measure-arity` |
| P04 | With `type Total = Integer;`: `function q using V(a: Int[-9,9], b: Int[-9,9]): Rational[-9,9;1,9] pure { a / b }`; the same body as `if b != 0 then a / b else rational(0, 1)`; a body `value(o)` for parameter `o: Option<Integer>` without `present(o)`; a body `reduce<Total>(acc, x in s: acc + x)` for parameter `s` of `Sequence<Integer>[0,3]` and of `Sequence<Integer>[1,3]`; the `[0,3]` body as `if size(s) >= 1 then reduce<Total>(acc, x in s: acc + x) else 0` | linking returns `refused { code: undefined_expression, cause: unproved-nonzero }` at the division; admitted; `refused { code: undefined_expression, cause: unproved-presence }` at `value`; `refused { code: undefined_expression, cause: unproved-range }` at the `reduce` for minimum 0, naming `size(s) >= 1`; admitted for minimum 1; admitted under the guard fact. TC-190 Q04 is the unlinked kernel-evaluation context of the same `reduce` |
| P05 | `function r using V(x: Integer): Integer pure { if true then 1 else undeclaredHost(x) }`; then the same body calling `M::pay(x)`, where `M::pay` is an operation of imported model `M` | `refused { code: missing_declaration, cause: missing-name }` at `undeclaredHost` although the branch is unreachable, with no call edge; `refused { code: ill_typed, cause: operator-ineligible }` at `M::pay`, also although unreachable |
| P06 | `last` from P01 applied to a three-node chain; then under `ScalarLimitsV1` with every counter unlimited except `work_units: 2` | the third node's head after exactly three `function.call` charges; `incomplete { limit_kind: work_units, limit: 2, consumed: 2, next_charge: 1, charge_point: function.call }` for the third call, and `last` remains admitted |
| P07 | `function double using V(n: Int[0,9]): Integer pure { n + n }` and `function quad using V(n: Int[0,9]): Integer pure { double(double(n)) }`, neither declaring `decreases`; then `double` declaring `decreases(n)` | both admitted, because neither is in a recursive component; `double` is still admitted, with its measure type checked and no obligation |

## Expected Results

Every positive and boundary result matches FR-146; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
