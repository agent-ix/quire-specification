---
id: TC-189
title: "Collection kind algebra"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-144
    type: verifies
---

# TC-189: Collection kind algebra

## Description

Construct, permute, compare and overflow sequence, set, bag and ordered-set values.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-144. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select the `quire.value.complete/v1` definitions at revision `1-draft.1` by
their exact DefinitionRefs in
[`complete-value-lock.json`](../../proposals/quire-v1/definitions/complete-value-lock.json).
Unless a row states limits, run it under limits large enough that no charge is
denied. Integer literals, parameters, `let` names and field projections make no
charge. Every `ill-typed` expectation is a type-checking
`refused { code: ill_typed }` made before any charge.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| C01 | `set[1, 2] == set[2, 1]`, `bag[1, 1, 2] == bag[2, 1, 1]`, `sequence[1, 2] == sequence[2, 1]` and `orderedSet[1, 2] == orderedSet[2, 1]`, each with identical declared types | true; true; false; false |
| C02 | `sequence[1, 1, 2]`, `set[1, 1, 2]`, `bag[1, 1, 2]` and `orderedSet[2, 1, 2]` | occurrences `[1, 1, 2]`; members `{1, 2}`; `1` with multiplicity 2 and `2` with multiplicity 1; members `[2, 1]` |
| C03 | `Set<Integer>[0,2]` from `set[1, 1, 2]`; `Set<Integer>[0,2]` from `set[1, 2, 3]`; `Bag<Integer>[0,2]` from `bag[1, 1, 2]`; `Sequence<Integer>[1,3]` from `sequence[]` | admitted with two members; `refused { code: cardinality_out_of_bound }` after `collection.bound` with `value_occurrences = 3`, with no retain charge and no collection; the same refusal for three bag occurrences; the same refusal for zero occurrences |
| C04 | Canonical order of `Set<Integer>[0,3]` built from `set[3, 1, 2]` and from `set[2, 3, 1]`; of `Set<Option<Integer>>[0,2]` from `set[present(1), none]`; of `Set<Text[0,4;binary-utf8]>[0,2]` from `set["b", "a"]` | `[1, 2, 3]` for both; `[none, present(1)]`; `["a", "b"]`. The canonical representation is this order, and no byte encoding is compared |
| C05 | `enum Color { red, blue, green }` and `ordered enum Level { high, low }`; evaluate `exists(x in c: x == Color::blue)` for `c = set[Color::red, Color::blue, Color::green]` of `Set<Color>[0,3]`, `exists(x in c: x == Color::red)`, and `exists(x in l: x == Level::high)` for `l = set[Level::low, Level::high]` of `Set<Level>[0,2]`; then reorder `Color`'s cases to `green, red, blue` and repeat; then `Color::red < Color::blue` | true after one `collection.visit`, because the order is `blue, green, red` by identifier bytes; true after three visits; true after one visit, because the order is `high, low` by declaration position; identical visit counts after reordering; ill-typed, because a key is not an ordering operator |
| C06 | With `record Holder { r: Reference<Obj>; }`: `map(x in hs: x)` and `convert<Sequence<Holder>[0,2]>(hs)` for `hs` of `Set<Holder>[0,2]`; then `size(hs)`, `contains(hs, h1)` and `hs == hs`; then declare `Set<Float64>[0,2]` and `Bag<F>[0,2]` for `record F { x: Float32; }` | ill-typed; ill-typed; admitted with values 2, true and true; ill-typed; ill-typed |
| C07 | Construct `Set<Integer>[0,3]` from `set[1, 2, 1]` | members `{1, 2}` after exactly `collection.element` three times, `collection.member-test` with `p = 1` (occurrence 2 against `[1]`), `collection.member-test` with `p = 2` (occurrence 3 against `[1, 2]`), `collection.bound` with `value_occurrences = 2` and `collection.result-retain` with `occ = 3`: eight work units, three result units and a `value_occurrences` high-water of 3 |
| C08 | C07 under `ScalarLimitsV1` with every counter unlimited except `work_units: 4`; under `ScalarLimitsV1` with every counter unlimited except `work_units: 7`; under `ScalarLimitsV1` with every counter unlimited except `value_occurrences: 2` | `incomplete { limit_kind: work_units, limit: 4, consumed: 4, next_charge: 2, charge_point: collection.member-test }`; `incomplete { limit_kind: work_units, limit: 7, consumed: 7, next_charge: 1, charge_point: collection.result-retain }`; `incomplete { limit_kind: value_occurrences, limit: 2, consumed: 2, next_charge: 3, charge_point: collection.result-retain }`. No collection is exposed by any of them |
| C09 | `Sequence<Integer>[0,3]` from `sequence[1, 1]`; `Set<Integer>[0,1]` from `set[1, 2]` | two `collection.element`, `collection.bound` and a retain of `occ = 3`: four work units and three result units; two `collection.element`, one `collection.member-test` (`p = 1`) and `collection.bound`, then `refused { code: cardinality_out_of_bound }` after four work units with no result unit |
| C10 | With `record Holder { r: Reference<Obj>; }` and parameters `h1`, `h2` of one universe and `hx` of another: `Set<Holder>[0,3]` from `set[h1, h2, h1]`; then from `set[h1, hx]` | members `{h1, h2}` after three elements, member tests with `p = 2` and `p = 4`, `collection.bound` and a retain of `occ = 5`: eleven work units and five result units; `refused { code: foreign_reference }` after the two `collection.element` charges, with no `collection.member-test` charge |
| C11 | `x` of `Set<Integer>[0,2]` and `s` of `Set<Integer>[0,3]`, both holding `{1, 2}`: `x == s`; then `let y = convert<Set<Integer>[0,3]>(x) in y == s` | ill-typed, because the bound is part of the type; true, with `CollectionLoss { discarded: [] }` |

## Expected Results

Every positive and boundary result matches FR-144; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
