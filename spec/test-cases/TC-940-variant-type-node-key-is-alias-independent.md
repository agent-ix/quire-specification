---
id: TC-940
title: "Model-owned variant, tuple and union declaration nodes, their keys and their reads"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-141
    type: verifies
---
# TC-940: Model-owned variant, tuple and union declaration nodes, their keys and their reads

## Description

Verify the `model`/`variant_type` declaration node, the type a variant-typed
member derives, and the identity of the node and of its cases (steps 1 to 5,
independent of the alias), then the `model`/`tuple_value_type` node, the
payload-bearing variant type read as a union, the application nodes over them
and a cyclic pair of model types (steps 6 to 12). Scope: FR-322-AC-71,
FR-141-AC-7 (steps 1 to 5); FR-322-AC-140 to FR-322-AC-147 (steps 6 to 12).

## Test Procedure

Use the M24 package extended with object `Order` (field `tier: Tier`,
optional field `maybe: Tier`, ordered non-unique `[0, 3]` field `many: Tier`)
and a second variant type `Level` with cases `Gold` and `Silver`.

1. Select the package into one checked package under alias `Config` and read
   the type of each `Order` field, and of the `Tier` node and each case.
2. Apply `quire.op.enum.eq` and `quire.op.enum.ne` to two `Tier` operands, then
   `quire.op.structural.eq` and `quire.op.enum.lt` to them. Read an `enum_value`
   node of `Tier` whose case is `Gold`, then one whose case is `Silver`.
3. Select the same package into a second checked package under alias `Cfg` and
   compare the key of the `Tier` node, of `Gold` and of `Standard` with step 1.
4. Rename the case `Standard` to `Basic` in the package and recompute the keys.
5. Present a `model`/`variant_type` node owned by a record value type, by an
   object type and by a node the document does not declare, and one with no
   `owner`.

## Expected Results

1. `Order.tier` is the `Tier` node, `maybe` is `Option<Tier>` and `many` is
   `Sequence<Tier>[0, 3]`; none is `Reference<Tier>`. The `Tier` node has a
   `ModelOwner`, family `enum` and no `scalar_type`/`enum` unit-owned
   counterpart.
2. The enum equality and inequality are admitted; `quire.op.structural.eq` and
   the ordering application refuse `ill_typed`/`operator-ineligible`. The `Gold`
   node is admitted; the `Silver` node refuses `invalid_semantic_graph`.
3. The three keys equal those of step 1, and `Gold` of one package equals
   `Gold` of the other. The key of the `Gold` node of `Tier` differs from the
   key of the `Gold` node of `Level`.
4. `Basic` has a new key and `Standard` has none; the `Tier` key and the `Gold` key are unchanged (the node preimage names no case), and `Level`'s keys are unchanged.
5. The first three refuse `missing_declaration`/`missing-selection` in the owner
   join; the last fails the V2 schema (`malformed_wire`).

## Tuple value types and payload-bearing variant types (STD-519)

Scope: FR-322-AC-140 to FR-322-AC-147 and FR-322-AC-151 to FR-322-AC-154. Use `model-tuple-union-vectors.json`
and its selected document `domain-package-acme-shapes.json`:
the package `acme/shapes` declares the tuples `Pair` (`p0: Boolean`,
`p1: Integer`), `Swap` (`p0: Integer`, `p1: Boolean`) and `Nest` (`p0: Pair`,
`p1: Shape`), the variant type `Shape` (cases `Empty`, `Flag(Boolean)`,
`Both(Pair)`), the enumeration `Color` and the object `Holder`.

6. Validate a `model`/`tuple_value_type` node owned by `Pair` against both V2
   schemas and the `ModelDeclarationNode` preimage; remove its `owner`, give it a
   `SourceOwner`, a `declaration`, a `version` in its owner and a non-empty body.
   Own it by `Color`, by `Holder` and by an undeclared node.
7. Project position 0, position 1 and position 2 of `Pair`; project positions of
   `Nest`; read a `Holder` field typed `Pair`.
8. Select the package with `Swap` replacing `Pair` and with the positions of
   `Pair` swapped; compare node keys, the selection digest and the root key.
9. Read the family and members of `Shape` and `Color`, with `variants` reordered;
   build `union_value` nodes for `Empty`, `Flag` and `Both`, an `enum_value`
   over `Shape`, and structural and enum equality over `Shape`.
10. Present a tuple with no position, a gap, an absent `typeRef`, a field, an
    unresolved `typeRef`, a repeated ordinal, and a case with an unresolved
    `payloadType` and one typed `ix://quire/native/Clock`.
11. Read `fixtures/positive-model-tuple-union.json` with the full reader: the
    `union_value`, `enum_value`, `quire.op.tuple.project`, `case` and equality
    nodes it carries over `Pair`, `Shape`, `Color` and `List`, then each wire case
    `TW-11` to `TW-26` (a wrong case, payload or arity, a `union_value` over an
    enumeration, a missing or reordered `case` arm, a position beyond the tuple, a
    foreign `declaration`, a beyond-safe `KeyOrdinal` string, an `enum_value`
    over a union or with a foreign case, `enum.eq` over a union, `structural.eq`
    over an enumeration, a leaf supplied for a type that reaches no text).
12. Declare `Cell` (`Integer`, `List`) and `List` (`Nil`, `Cons(Cell)`) and walk
    them; compare the keys of `Cell` and `List` with their owner-only preimages.

13. Confirm the selected document's tuple construct uses FCD's `tuple` shape
    and every tuple position has its schema-required `origin` (FR-322-AC-151;
    filament-core-data FR-147-AC-2 and FR-147-AC-3). Derive the projection of
    `domain-package-acme-shapes.json`, compare it with `domain_package`, and
    hash the RFC 8785 bytes of the whole document against the lock row digest;
    swap the position types of `Pair` and reorder two declarations in the
    document.

Expected results:

6. The first is admitted with family `tuple`; the owner-less node, the
   `SourceOwner` node, the `declaration` node and the versioned owner fail the V2
   schema (`malformed_wire`); the non-empty body is refused; the three owners
   refuse `missing_declaration`/`missing-selection` in the owner join.
7. Position 0 is `Boolean`, position 1 `Integer`, position 2 refuses
   `ill_typed`/`operator-ineligible`; `Nest` gives the `Pair` and `Shape`
   nodes; the field is the `Pair` node.
8. The `Pair` key is unchanged by the swap; the selection digest, the root key
   and the `package_id` change; `Swap` has a key different from `Pair`.
9. `Shape` is `union` with members `Both`, `Empty`, `Flag`; `Color` is `enum`;
   the `union_value` nodes are admitted or refused as FR-322-AC-144 states; the
   `enum_value` refuses `invalid_semantic_graph`; structural equality is admitted
   over `Shape` and enum equality refuses.
10. Each refuses with the code and cause FR-322-AC-145 names, in the order
    "Intake refusal order" of FR-322 states.
11. The unmutated package is admitted; each application node recomputes its
    published application-node key; every wire case refuses with the code, locus
    and, for a code that carries a cause, the cause its vector states (a
    `malformed_wire` or `invalid_semantic_graph` case states none, FR-322-AC-153),
    `TW-05` as `invalid_package`/`stale-node-key` at `/semantic_graph/nodes/0/node_id`, the
    references dangling only through its rekey being deferred (FR-322-AC-154); `TW-18` to `TW-20` refuse at the
    application node `/semantic_graph/nodes/31` (FR-322-AC-152).
12. The cycle is admitted; neither node has a `recursion_group`; the walk and
    the empty leaf list are those of FR-322-AC-147.
13. The tuple construct shape is `tuple` and each tuple position carries an
    `origin`; the projection equals `domain_package`; the row digest equals the
    SHA-256 of the whole document; both mutations change that digest
    (FR-322-AC-151).

## Nominal rekey and refinement row nodes (STD-536, STD-538)

Scope: FR-322-AC-160 to FR-322-AC-162 (steps 14 to 15); FR-322-AC-163 to
FR-322-AC-168 (steps 16 to 18). Use `nominal-rekey-vectors.json` with
`fixtures/positive-compound-unit.json`, and `refinement-row-node-vectors.json`.

14. Read `fixtures/positive-compound-unit.json`, then each case `NK-00` to
    `NK-05` of `nominal-rekey-vectors.json`: a nominal Unit `U` rekeyed with a
    dependent compound unit holding the new key, holding `U`'s old key, and
    itself stale; only the compound stale; two stale units recomputing to one key.
15. Present a dangling reference that depends on no nominal rekey, alongside a
    nominal rekey.
16. Recompute the key of each vector `VN-01` to `VN-06` (history read,
    `pre` over it, whole-population value, `count`, `only`, `seq`) from the
    recorded world nodes and preimages.
17. Run each refusal case `RR-01` to `RR-08` through the refinement step and the
    operation stage.
18. Compare the catalog delta with the operations the vectors use.

Expected results:

14. The unmutated package is admitted; `NK-01` to `NK-03` and `NK-05` refuse
    `invalid_semantic_graph` (no cause) at the `node_id` of the first stale
    nominal node by retained digest; `NK-04` refuses `invalid_package`/
    `stale-node-key` at the compound (FR-322-AC-160, FR-322-AC-161).
15. The dangling reference is refused in the graph-shape stage, before the
    nominal key stage (FR-322-AC-162).
16. Every key equals the published digest; the history read carries no
    refinement in its key (FR-322-AC-163, 165, 166, 167).
17. Each case refuses with the code, cause and locus its vector states
    (FR-322-AC-164, 165, 166, 167).
18. The delta names exactly the three operations and the member kind `history`
    the vectors use (FR-322-AC-168).
