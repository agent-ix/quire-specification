---
id: TC-429
title: "Value paths root and step through every witness location"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-207
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-351
    type: verifies
---

# TC-429: Value paths root and step through every witness location

## Description

Builds value paths for each location a separating witness can name and checks
their roots, steps, wire spelling, equality and order. Planned.

## Test Procedure

| Row | Location | Expected | AC |
| --- | --- | --- | --- |
| VP-01 | The third element of a population query result over universe `U` | Root subject `population` with `universe` `U`, step `index` 2; the wire object has exactly `tag` and `universe`, and an extra or missing member refuses | FR-207-AC-9 |
| VP-02 | The second element of `[a, b, c]` built inside the claim by the expression at occurrence key `K` | Root subject `built` with `occurrence` `K`, step `index` 1; the wire object has exactly `tag` and `occurrence`, and an extra or missing member refuses | FR-207-AC-9 |
| VP-03 | `self.owner` followed through its reference to object `O` and then to its field `items` | Steps `member owner`, `reference` naming `O`, `member items`; replacing the `reference` step with `field` or `member` gives an unequal path; at one position a `reference` step orders after a `map_key` step | FR-207-AC-10 |
| VP-04 | The element deciding `forall(x in self.items->filter(i: i > 0): p)` at the second retained element, stored at position 4 of `self.items` | Path to `self.items` with index 4, never index 1 | FR-207-AC-11 |
| VP-05 | The element deciding a quantifier over `self.items->map(i: i * 2)` at position 3 | Path to `self.items` with index 3; the deciding element is the mapped value | FR-207-AC-11 |
| VP-06 | `self.home.level`, where `home` is a relationship end whose navigation returns object `t1` of type `Tank` | The selection of `level` is rooted at the declared object identity of `t1` with the single step `member level`; the navigation records no path and no step, and no `reference` step precedes the member step | FR-207-AC-12 |

## Expected Results

Each witness location has exactly one value path, its wire spelling is closed,
and computed domains always name their stored source.
