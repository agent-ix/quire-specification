---
id: TC-400
title: "Supplied union values and the value profile"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-143
    type: verifies
---

# TC-400: Supplied union values and the value profile

## Description

Validate supplied union values before evaluation, and show that a union-bearing
package is admitted under `quire.value.complete/v1` with the same selection as
a record-only package.

## Test Procedure

Declare `union Shape { Circle(Integer), Rect(Integer, Integer), Empty }`,
`union Other { Circle(Integer) }` and `function area using V (s: Shape): Integer pure { ... }`,
and call `area` with each supplied argument. Then compare package selections.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| P01 | supplied `Shape::Rect(2, 3)`: `Shape`'s key, `Rect`'s member key, payload `[2, 3]` | admitted and evaluated |
| P02 | `Shape`'s key with `Other`'s `Circle` member key | `refused { code: invalid_runtime_input, cause: wrong-value-kind }` before any evaluation or charge |
| P03 | `Shape`'s key with a member key that is no member's key | the same refusal |
| P04 | `Shape`'s key, `Rect`'s member key, payload `[2]` | the same refusal |
| P05 | `Shape`'s key, `Circle`'s member key, payload `["x"]` | the same refusal |
| P06 | a declaration key that is not a key of the evaluating package | the same refusal |
| P07 | a union package selecting `quire.value.complete/v1` with exactly the always-selected definitions and no trigger; the same package with `Shape` replaced by a record and `area` by a record function | both admitted with equal selections |

## Expected Results

Every ill-formed supplied union value is refused before evaluation or charge,
and declaring a union changes no definition selection.
