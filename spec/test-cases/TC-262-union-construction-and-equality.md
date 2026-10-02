---
id: TC-262
title: "Union construction and equality"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-143
    type: verifies
---

# TC-262: Union construction and equality

## Description

Construct declared union values, refuse malformed constructions, and compare union values for equality across matching and distinct declared unions.

## Test Procedure

Use the exact subject and inputs named by FR-143-AC-12. Execute the positive case, each declared boundary, and each explicit refusal through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Unless a row states limits, run it under limits large enough that no charge is denied.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| U01 | `union Shape { Circle(Integer), Square(Integer), Empty }`; construct `Shape::Circle(3)`, `Shape::Square(3)` and `Shape::Empty` | each admitted, with `Shape::Circle(3)` and `Shape::Square(3)` distinct values despite the same payload |
| U02 | Two constructions of `Shape::Circle(3)`; `Shape::Circle(3)` and `Shape::Circle(4)`; two constructions of `Shape::Empty` | true; false; true |
| U03 | `union A { M(Integer) }` and `union B { M(Integer) }` in one package; compare a parameter of `A` holding `A::M(1)` with a parameter of `B` holding `B::M(1)` | `refused { code: ill_typed, cause: type-mismatch }`, because a union declaration creates its own identity and equal member shapes from different declared unions never become interchangeable |
| U04 | `Shape::Circle(3, 4)` (arity mismatch); `Shape::Triangle(3)` (undeclared member); `Shape::Circle` (missing required payload) | each `refused { code: ill_typed, cause: type-mismatch }` |
| U05 | `union Empty2 { E }`; construct `Empty2::E` and, separately, attempt `Empty2::E(1)` | `Empty2::E` admitted; `Empty2::E(1)` `refused { code: ill_typed, cause: type-mismatch }`, because `E` declares no payload |

## Expected Results

Every positive and boundary result matches FR-143-AC-12; each invalid mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
