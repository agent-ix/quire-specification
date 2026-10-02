---
id: TC-399
title: "Union construction and case accounting"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-143
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
---

# TC-399: Union construction and case accounting

## Description

Record the exact `quire.value.accounting/v1` charges of union construction and
of `case` evaluation, and the work limit at which each first stops.

## Test Procedure

Declare `union Shape { Circle(Integer), Rect(Integer, Integer), Empty }` and
`function area using V (s: Shape): Integer pure { case s { Circle(r): r; Rect(w, h): w * h; Empty: 0 } }`.
Record every charge point, its counter amounts and its order.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| A01 | evaluate `Shape::Rect(2, 3)` | `2` then `3` evaluated with their own charges, then one `composite.result-retain` with `value_occurrences = 3`, `result_units += 3` and one work unit; the construction adds no other charge |
| A02 | evaluate `Shape::Empty` | one `composite.result-retain` with `value_occurrences = 1` and `result_units += 1`; no argument evaluated |
| A03 | `Shape::Rect(e, 3)` where `e` is a first payload argument whose outcome is undefined | `e`'s undefined outcome is the construction's; `3` is neither evaluated nor charged, and no `composite.result-retain` is charged |
| A04 | `area(Shape::Rect(2, 3))` | A01's charges, then `function.call`, then exactly the `w * h` integer-arithmetic charges; no charge for arm selection or payload binding, and none for the `Circle` or `Empty` arm |
| A05 | A04 under a `work_units` limit equal to its total | completed `6` |
| A06 | A04 under a `work_units` limit one below its total | `incomplete { limit_kind: work_units, ... }` at the last charge point A04 lists |
| A07 | `area` with a scrutinee whose evaluation is undefined | the undefined outcome is the `case`'s, and no arm is evaluated or charged |

## Expected Results

Union construction charges exactly `composite.result-retain` after its payload
arguments, `case` charges exactly its scrutinee and its selected arm body, and
each denied charge returns incomplete with no partial value.
