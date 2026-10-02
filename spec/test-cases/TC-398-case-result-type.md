---
id: TC-398
title: "Case result type"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-398: Case result type

## Description

Check the `case` result type rule and refusal order of FR-146: the expected
type when the position supplies one, otherwise the arms' common inferred type,
with the integer join; the scrutinee before arm bodies before exhaustiveness;
and no body check for an arm whose member is unknown or whose arity is wrong.

## Test Procedure

Declare `union Shape { Circle(Int[0..9]), Rect(Int[0..99], Int[0..99]), Empty }`
and type check each subject.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| R01 | `function f using V (s: Shape, w: Int[0..9]): Int[0..100] pure { case s { Circle(r): 0; Rect(a, b): w; Empty: 0 } }` | admitted; the `case` has type `Int[0..100]` |
| R02 | `let k = case s { Circle(r): r; Rect(a, b): a; Empty: 0 } in k` with no annotation | admitted; the `case` has type `Integer` |
| R03 | an unannotated `let` over `case s { Circle(r): t; Rect(a, b): t; Empty: t }` for a parameter `t: Text[0..8]` | admitted; the `case` has type `Text[0..8]` |
| R04 | an unannotated `let` over `case s { Circle(r): 0; Rect(a, b): "none"; Empty: 0 }` | `refused { code: ill_typed, cause: type-mismatch }` at the `Rect` arm body |
| R05 | an unannotated `let` over `case s { Circle(r): "none"; Rect(a, b): 0; Empty: 0 }` | `refused { code: ill_typed, cause: type-mismatch }` at the `Rect` arm body |
| R06 | an unannotated `let` over `case s { Circle(r): sequence[]; Rect(a, b): sequence[]; Empty: sequence[] }` | `refused { code: ill_typed, cause: ambiguous-literal }` at the `Circle` arm body |
| R07 | R01 with its arms written in the order `Empty`, `Rect`, `Circle` and `Rect`'s body changed to `"none"` | `refused { code: ill_typed, cause: type-mismatch }` at the `Rect` arm body, the first failing arm in source order |
| R08 | `case n { Circle(r): 0; Square(x): 1 }` with `n: Integer` | `refused { code: ill_typed, cause: type-mismatch }` at the scrutinee only |
| R09 | an unannotated `let` over `case s { Circle(r): "none"; Rect(a, b): 0 }` (missing `Empty`) under R04's arm types | `refused { code: ill_typed, cause: type-mismatch }` at the `Rect` arm body only, before `missing-arm` |
| R10 | `case s { Circle(r): 0; Rect(a, b): 0; Empty: 0; Square(q): q + "x" }` | `refused { code: undefined_expression, cause: unproved-exhaustiveness }`, obligation `unknown-member`; the `Square` body is not checked |
| R11 | `case s { Circle(r): 0; Rect(a): b; Empty: 0 }` | `refused { code: undefined_expression, cause: unproved-exhaustiveness }`, obligation `arm-arity`; the `Rect` body, whose `b` is unbound, is not checked |

## Expected Results

Each subject's type or refusal matches the vector, and the refusal locus is
the first failing arm body in source arm order.
