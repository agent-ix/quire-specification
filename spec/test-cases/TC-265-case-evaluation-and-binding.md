---
id: TC-265
title: "Case evaluation and binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-265: Case evaluation and binding

## Description

Evaluate an admitted `case` expression, binding each arm's payload positionally, and check every arm body against the `case` expression's result type.

## Test Procedure

Use the exact subject and inputs named by FR-146-AC-11. Execute the positive case and each declared boundary through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Unless a row states limits, run it under limits large enough that no charge is denied.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| C01 | `union Shape { Circle(Integer), Square(Integer), Empty }`; `case s { Circle(r): r; Square(w): w * w; Empty: 0; }` declaring result `Integer`, evaluated for `Shape::Circle(3)`, `Shape::Square(3)` and `Shape::Empty` | `3`; `9`; `0`, each evaluating only the matched arm's body |
| C02 | `union Pair2 { Both(Integer, Integer) }`; `case p { Both(a, b): a + b; }` for `Pair2::Both(2, 5)` | `7`, with `a` and `b` bound positionally to the two payload values |
| C03 | The same `Shape` from C01 with `case s { Circle(r): r; Square(w): "no"; Empty: 0; }` declaring result `Integer` | `refused { code: ill_typed, cause: type-mismatch }` at the `Square` arm body, because its result does not match the declared `Integer` result type |
| C04 | `case s { Circle(r): r; Square(w): w; Empty: 0; }` nested as the scrutinee of an outer `case` over a second union, evaluated for a value selecting the outer arm whose payload is `Shape::Empty` | `0`, evaluating only the outer matched arm and, within it, only the inner matched arm |

## Expected Results

Every positive and boundary result matches FR-146-AC-11: the matched arm alone is evaluated, its payload is bound positionally to the arm's declared identifiers, and every arm body is checked against the `case` expression's result type (FR-146 "Case result type").
