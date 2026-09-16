---
id: TC-190
title: "Collection query and conversion algebra"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-145
    type: verifies
---

# TC-190: Collection query and conversion algebra

## Description

Run map/filter/flatten/fold and explicit lossy conversions across every collection kind.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-145. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions at revision `1-draft.1` by
their exact DefinitionRefs in
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| Q01 | `map(x in c: 0)` for `c` = `sequence[1, 2]` of `Sequence<Integer>[0,2]`, `set[1, 2]` of `Set<Integer>[1,2]`, `bag[1, 2]` of `Bag<Integer>[0,2]` and `orderedSet[2, 1]` of `OrderedSet<Integer>[0,2]` | `Sequence<Integer>[0,2]` `[0, 0]`; `Set<Integer>[1,2]` `{0}`; `Bag<Integer>[0,2]` with `0` of multiplicity 2; `OrderedSet<Integer>[0,2]` `[0]` |
| Q02 | `filter(x in bag[1, 1, 2]: x == 1)` of `Bag<Integer>[1,3]`; `filter(x in orderedSet[2, 1, 3]: x != 1)` of `OrderedSet<Integer>[1,3]` | `Bag<Integer>[0,3]` with `1` of multiplicity 2; `OrderedSet<Integer>[0,3]` `[2, 3]` |
| Q03 | `flatten` of `sequence[sequence[1, 2], sequence[2]]`, `set[set[1], set[1, 2]]`, `bag[bag[1], bag[1, 2]]` and `orderedSet[orderedSet[2, 1], orderedSet[1, 3]]`; a `Set<Set<Integer>[1,2]>[1,3]`; then `Sequence<Set<Integer>[0,2]>[0,2]`, `OrderedSet<Bag<Integer>[0,2]>[0,2]` and `Set<Sequence<Integer>[0,2]>[0,2]` | `[1, 2, 2]`; `{1, 2}`; `1` with multiplicity 2 and `2` with multiplicity 1; `[2, 1, 3]`; result type `Set<Integer>[1,6]`; ill-typed; ill-typed; admitted with result kind set |
| Q04 | `fold<Integer>(acc, x in e: acc + x, identity: 0)` and `reduce<Integer>(acc, x in e: acc + x)` for empty `e` of each of `Sequence<Integer>[0,2]`, `Set<Integer>[0,2]`, `Bag<Integer>[0,2]` and `OrderedSet<Integer>[0,2]`; then `fold` without `identity:` and `reduce` with `identity: 0` | `0` for each fold; located undefined for each of the four reductions, with no charge; ill-typed; ill-typed |
| Q05 | Over `s` of `Set<Integer>[0,3]` holding `{1, 2, 3}`: steps `acc + x`, `x * acc`, `acc - x`, `acc + acc`, `add(acc, x)` for `function add(a: Integer, b: Integer): Integer pure { a + b }`; over `Set<Int[0,10]>[0,3]` step `acc + x`; over `Set<Boolean>[0,2]` step `acc and x`; over `Sequence<Integer>[0,3]` step `acc - x` | admitted with fold value 6; admitted; ill-typed; ill-typed (`acc` occurs in `e`); ill-typed, even though `add` is commutative and associative; ill-typed (bounded accumulator); admitted; admitted |
| Q06 | `convert<Set<Integer>[0,3]>(sequence[1, 2, 3])`; `convert<Sequence<Integer>[0,3]>(bag[2, 1, 1])`; `convert<Bag<Integer>[0,2]>(orderedSet[2, 1])`; `convert<Set<Integer>[0,1]>(sequence[1, 1])`; `convert<Set<Integer>[0,1]>(sequence[1, 2])`; `convert<Set<Int[0,5]>[0,2]>(x)` for `x` of `Set<Integer>[0,2]` | `{1, 2, 3}` with `CollectionLoss { discarded: [order, multiplicity] }` although no duplicate occurred; `[1, 1, 2]` with `discarded: []`; `discarded: [order, uniqueness]`; `{1}`, checked after coalescing; `refused { code: cardinality_out_of_bound }`; ill-typed element conversion |
| Q07 | `map(x in s: 0)` for parameter `s = set[1, 2, 3]` of `Set<Integer>[0,3]`; then with `ScalarLimitsV1` with every counter unlimited except `work_units: 6` | `{0}` after three `collection.visit`, two `collection.member-test` with `p = 1`, `collection.bound` (`value_occurrences = 1`) and a retain of `occ = 2`: seven work units and two result units; then `incomplete { limit_kind: work_units, limit: 6, consumed: 6, next_charge: 1, charge_point: collection.result-retain }` |
| Q08 | `exists(x in q: x == 2)` for parameter `q = sequence[1, 2, 3]` of `Sequence<Integer>[0,3]`; then `filter(x in q: x == 1)` under `ScalarLimitsV1` with every counter unlimited except `work_units: 5` | true after visit, one-pair equality (three work units), visit, one-pair equality, then a scalar retain, with no visit of `3`: nine work units and three result units; `incomplete { limit_kind: work_units, limit: 5, consumed: 5, next_charge: 3, charge_point: equality.plan }` at the second predicate, with no partial collection |
| Q09 | `convert<Set<Integer>[0,3]>(q)` for parameter `q = sequence[1, 2, 1]` | `{1, 2}` with `discarded: [order, multiplicity]` after three visits, member tests with `p = 1` and `p = 2`, `collection.bound` and a retain of `occ = 3`: eight work units and three result units |
| Q10 | With `record Holder { r: Reference<Obj>; }` and parameter `hs = {h1, h2}` of `Set<Holder>[0,2]`: `contains(hs, h2)`; then under `ScalarLimitsV1` with every counter unlimited except `work_units: 3`; then `size(hs)` | true after one `collection.member-test` with `p = 4` and a scalar retain: five work units and one result unit; `incomplete { limit_kind: work_units, limit: 3, consumed: 0, next_charge: 4, charge_point: collection.member-test }`; 2 after one scalar retain only |

## Expected Results

Every positive and boundary result matches FR-145; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
