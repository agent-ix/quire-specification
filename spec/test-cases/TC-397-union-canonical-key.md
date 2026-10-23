---
id: TC-397
title: "Union canonical key and union collection elements"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-144
    type: verifies
---

# TC-397: Union canonical key and union collection elements

## Description

Show that union values are admitted as set, bag and ordered-set elements, keyed
by the FR-144 union row; that set and bag visiting order is independent of
insertion order and of member declaration order; and that an ordered set keeps
first-occurrence order.

## Test Procedure

Declare `union Shape { Rect(Integer, Integer), Circle(Integer), Empty }` and,
separately, the same members declared `Empty, Circle, Rect`. Run each vector
under limits large enough that no charge is denied, and record the visiting
order through a short-circuiting `exists` whose predicate records each
visited member.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| Y01 | `set[Shape::Rect(2, 3), Shape::Empty, Shape::Circle(5), Shape::Circle(1), Shape::Rect(2, 3)]` | four members, visited `Circle(1)`, `Circle(5)`, `Empty`, `Rect(2, 3)` |
| Y02 | the Y01 elements in reverse insertion order; the Y01 set under the reordered declaration | the same four members in the same visiting order |
| Y03 | `bag[...]` of the Y01 elements | `Rect(2, 3)` with multiplicity two, visited consecutively |
| Y04 | `orderedSet[...]` of the Y01 elements | first-occurrence order `Rect(2, 3)`, `Empty`, `Circle(5)`, `Circle(1)` |
| Y05 | `union F { V(Float64), N }` and a `Set<F>` declaration; then `f1 = f2` for two parameters of `F` | admitted; admitted, `f1 = f2` comparing the payload float by `numericEqual` |
| Y06 | `Sequence<F>` | admitted |
| Y07 | `Shape::Rect(2, 3) < Shape::Empty` | `refused { code: ill_typed, cause: operator-ineligible }`, because a key is not an ordering operator |
| Y08 | `set[F::V(f0), F::V(fm0), F::N, F::V(f0)]` for `f0` holding `float64(bits: 0x0000000000000000)` and `fm0` holding `float64(bits: 0x8000000000000000)` | three members visited `N`, then `V(-0)`, then `V(+0)`, because a payload float is a member by `bitIdentical` and keyed by `totalOrder` |

## Expected Results

Union elements key by member identifier bytes and then payload keys; set and
bag visiting order never depends on insertion or declaration order, an ordered
set keeps first-occurrence order, and a union with a float payload is a member
of the keyed collection kinds by `bitIdentical` while its `=` stays ineligible.
