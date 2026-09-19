---
id: TC-263
title: "Union recursion rule"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-143
    type: verifies
---

# TC-263: Union recursion rule

## Description

Apply the recursion rule's named/escaping admission graph to union member payload positions, admitting exactly the recursions that escape.

## Test Procedure

Use the exact subject and inputs named by FR-143-AC-13. Execute the positive case and each declared boundary through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

This test case exercises `union-decl` recursion, admitted under `quire.value.complete/v1` at its current draft revision; it is not locked by `complete-value-lock.json`, whose DefinitionRefs remain scoped to the record/tuple surface TC-188 verifies at revision `1-draft.1`. Unless a row states limits, run it under limits large enough that no charge is denied.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| N01 | `union List { Cons(Integer, List), Nil }` | `refused { code: ill_typed, cause: type-mismatch }`, naming the cycle `List -> List` on member `Cons`'s second payload position, because that position does not escape |
| N02 | `union List { Cons(Integer, Option<List>), Nil }` | admitted, because the payload position escapes through `Option` |
| N03 | `union Tree { Leaf(Integer), Node(Tree, Tree) }` | `refused { code: ill_typed, cause: type-mismatch }`, naming both non-escaping payload positions of `Node` |
| N04 | `union Tree { Leaf(Integer), Node(Option<Tree>, Option<Tree>) }` | admitted |
| N05 | `union A { M(B) }` with `union B { N(Option<A>) }` | admitted, because the mutual cycle `A -> B -> A` escapes at `B::N`'s payload position |
| N06 | `record Wrap { u: Wrap2; }` with `union Wrap2 { W(Wrap) }` | `refused { code: ill_typed, cause: type-mismatch }`, naming the mixed record/union cycle `Wrap -> Wrap2 -> Wrap`, because neither payload position escapes |

## Expected Results

Every positive and boundary result matches FR-143-AC-13, naming every cycle exactly as the recursion rule already names record and tuple cycles, and preserves independent sibling results.
