---
id: TC-264
title: "Case exhaustiveness obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
---

# TC-264: Case exhaustiveness obligations

## Description

Check a `case` expression's arm set against its scrutinee's declared union in the fixed obligation order, and report the first violated obligation as `undefined_expression` with cause `unproved-exhaustiveness`.

## Test Procedure

Use the exact subject and inputs named by FR-146-AC-10 and FR-272-AC-8. Execute the positive case and each declared boundary through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

This test case exercises `case` exhaustiveness checking under `quire.native.diagnostics/v1` revision `1-draft.4`; it is not locked by `complete-value-lock.json`, whose DefinitionRefs remain scoped to the record/tuple surface TC-188/TC-191 verify at revision `1-draft.1`. Unless a row states limits, run it under limits large enough that no charge is denied.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| E01 | `union Shape { Circle(Integer), Square(Integer), Empty }`; `case s { Circle(r): r; Square(w): w; Empty: 0; }` for parameter `s: Shape` | admitted, all three members covered |
| E02 | The same `case`, with `Circle(r)` listed twice and `Square` omitted | `refused { code: undefined_expression, cause: unproved-exhaustiveness }` naming union `Shape`, the arm set and failed obligation `duplicate-arm`, checked before arity or coverage |
| E03 | The same `case`, with an arm `Triangle(t): t;` in place of `Square` | `refused { code: undefined_expression, cause: unproved-exhaustiveness }` naming obligation `unknown-member`, because `Shape` declares no `Triangle` member |
| E04 | The same `case`, with `Circle(r, extra): r;` | `refused { code: undefined_expression, cause: unproved-exhaustiveness }` naming obligation `arm-arity`, because `Circle` declares one payload position |
| E05 | The same `case`, with `Empty` omitted and no other malformed arm | `refused { code: undefined_expression, cause: unproved-exhaustiveness }` naming obligation `missing-arm` |
| E06 | The same `case`, with a duplicate `Circle` arm and also an unknown `Triangle` arm present together | `refused { code: undefined_expression, cause: unproved-exhaustiveness }` naming only `duplicate-arm`, because obligations are checked in order and the first violation is reported |

## Expected Results

Every positive and boundary result matches FR-146-AC-10 and FR-272-AC-8: the reported cause is always `undefined_expression` with `unproved-exhaustiveness`, no other code family gains a case-exhaustiveness variant, and each refusal names the failed obligation, the union and the arm set.
