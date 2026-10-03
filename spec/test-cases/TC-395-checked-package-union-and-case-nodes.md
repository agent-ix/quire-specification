---
id: TC-395
title: "Checked-package union, union value and case nodes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-440
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-395: Checked-package union, union value and case nodes

## Description

Show that a checked package encodes a union declaration, its constructions and
a `case` over it as the `composite_type`/`union`, `value`/`union_value` and
`expression`/`case` nodes FR-440 specifies, that a `case` node's arms lower in
member declaration order, and that the V2 reader refuses each broken join with
its named code and cause.

## Preconditions

- The published V2 package
  `proposals/checked-package-v2/fixtures/positive-union-nodes.json`
  (`union-nodes`) carries `union Shape { Circle(Integer), Rect(Integer, Integer), Empty }`,
  the body of `area(s: Shape): Integer`,
  `case s { Empty: 0; Rect(w, h): w * h; Circle(r): r }`, with its parameter
  and binder nodes, and the constructions `Shape::Rect(2, 3)` and
  `Shape::Empty`. Its `package_id` and its application node keys are
  recomputed from their preimages.
- Both V2 schemas enumerate the `union`, `union_value` and `case` forms, the
  `case` operator class and `quire.op.control.case`. The reader vectors
  (N01 and N05 to N09) additionally need the operation catalog to carry the `union`
  family, the `arm_body` result form, the `union_arms` constraint kind and
  the `quire.op.control.case` entry FR-440 requires of it.

## Test Procedure

Read `union-nodes` through the strict V2 reader, then read each mutation
below, each made to an otherwise admitted copy with its node keys recomputed so
that no `stale-node-key` refusal masks the join under test.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| N01 | `union-nodes` as published | admitted; the `Shape` node is `composite_type`/`union` with `declaration` `["Shape"]` and bindings `Circle` (one `Integer` reference), `Rect` (two) and `Empty` (empty aggregate), in that order |
| N02 | the `Shape::Rect(2, 3)` and `Shape::Empty` nodes | `value`/`union_value` nodes typed `Shape`, with the single bindings `Rect` over two payload terms and `Empty` over the empty aggregate |
| N03 | the `area` body | an `expression`/`case` node whose `quire.op.control.case` application holds the scrutinee then the arms `Circle`, `Rect`, `Empty`; the binders `r`, `w`, `h` are `value`/`parameter` nodes typed `Integer` at levels `1`, `1` and `2`, because the parameter `s` takes level `0` (FR-440 "Binder levels" rule 1) and each arm starts one above it (rule 3) |
| N04 | the same source with the arms written `Circle`, `Rect`, `Empty` | the `case` node id and the `package_id` equal N01's |
| N05 | a second `Circle` binding in the `Shape` node; a `Circle` payload reference to a `value` node | `invalid_package`/`duplicate-member` at the `Shape` node; `ill_typed`/`operator-ineligible` at the `Shape` node |
| N06 | a `union_value` binding `Triangle`; a `Rect` binding with one payload term; a `Circle` binding whose payload term is `Text` | each `ill_typed`/`type-mismatch` at the `union_value` node |
| N07 | the `case` arms out of member order; the `Empty` arm removed; a second `Rect` arm; a `Rect` binder aggregate of one reference; the `r` binder typed `Text`; an arm body typed `Text` under `result_type` `Integer` | each `ill_typed`/`operator-ineligible` at the `case` node |
| N08 | FR-440's "Binder levels" worked vector, then the same function with `let t = k in` removed and `t` replaced by `k` | `t`, `w`, `h`, `r`, `x` at levels `3`, `4`, `5`, `4`, `5`; then `w`, `h`, `r`, `x` at `3`, `4`, `3`, `4` |
| N09 | `quire.op.structural.eq` over two `Shape` values; over a `Shape` value and a value of a second union | admitted; `ill_typed`/`operator-ineligible` by `same_type` |
| N10 | the `area` body's `quire.op.control.case` application placed as the body root of the `area` `function` node; the same `case` as an `expression`/`case` node the function body references; the same `case` application nested in another application's arguments | `ill_typed`/`operator-ineligible` at the `function` node; admitted; `ill_typed`/`operator-ineligible` at the nested application's `operator`, not `malformed_wire` (FR-440-AC-7) |
| N11 | `quire.op.structural.eq` over two values of `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }` | admitted with exactly the leaves `["member:Named", "position:0"]` then `["member:Tagged", "position:1"]`, each with the `nfc` text law and mode, and none for `Empty`; both V2 schemas admit the `member:<Ident>` segment (FR-322-AC-45, FR-440-AC-8) |
| N12 | `quire.op.structural.eq` over two values of `union List { Cons(Integer, List), Nil }` whose cycle names one `recursion_group`; over two values of a record whose field's type cycles back to it through a union | each admitted with empty `leaves`, neither `operator-ineligible` (FR-322-AC-46, FR-440-AC-8) |
| N13 | `quire.op.structural.eq` over two values of each of FR-322's `Node`, `Cell`, `Chain` and `Option<Chain>` | each admitted with exactly FR-322's listed leaves in order, ending in `recursion:0`, `recursion:0`, `recursion:0` and `recursion:1` with `laws` `[]` and `mode` `null`; both V2 schemas admit the `recursion:<d>` segment (FR-322-AC-46, FR-440-AC-8) |

## Expected Results

The published package is admitted with exactly the node encodings FR-440
states, reordering source arms changes no identity, and each mutation returns
exactly the named refusal at the named node and no admitted package.
