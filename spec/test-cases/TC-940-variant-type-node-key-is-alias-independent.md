---
id: TC-940
title: "A model-owned variant type node and its case keys do not depend on the alias"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-141
    type: verifies
---
# TC-940: A model-owned variant type node and its case keys do not depend on the alias

## Description

Verify the `model`/`variant_type` declaration node, the type a variant-typed
member derives, and the identity of the node and of its cases. Scope:
FR-322-AC-71, FR-141-AC-7.

## Test Procedure

Use the M24 package extended with object `Account` (field `tier: Tier`,
optional field `maybe: Tier`, ordered non-unique `[0, 3]` field `many: Tier`)
and record value type `Level` with cases `Gold` and `Silver`.

1. Select the package into one checked package under alias `Config` and read
   the type of each `Account` field, and of the `Tier` node and each case.
2. Apply `quire.op.structural.eq` to two `Tier` operands, then an ordering
   application to them.
3. Select the same package into a second checked package under alias `Cfg` and
   compare the key of the `Tier` node, of `Gold` and of `Standard` with step 1.
4. Rename the case `Standard` to `Basic` in the package and recompute the keys.
5. Present a `model`/`variant_type` node owned by a record value type, by an
   object type and by a node the document does not declare, and one with no
   `owner`.

## Expected Results

1. `Account.tier` is the `Tier` node, `maybe` is `Option<Tier>` and `many` is
   `Sequence<Tier>[0, 3]`; none is `Reference<Tier>`. The `Tier` node has a
   `ModelOwner`, family `enum` and no `scalar_type`/`enum` unit-owned
   counterpart.
2. The equality is admitted; the ordering application refuses
   `ill_typed`/`operator-ineligible`.
3. The three keys equal those of step 1, and `Gold` of one package equals
   `Gold` of the other. The key of the `Gold` node of `Tier` differs from the
   key of the `Gold` node of `Level`.
4. `Basic` has a new key and `Standard` has none; the `Tier` key and the `Gold` key are unchanged (the node preimage names no case), and `Level`'s keys are unchanged.
5. The first three refuse `missing_declaration`/`missing-selection` in the owner
   join; the last fails the V2 schema (`malformed_wire`).
