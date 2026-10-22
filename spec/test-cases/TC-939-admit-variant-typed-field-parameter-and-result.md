---
id: TC-939
title: "Admit a field, parameter and result typed by a variant type"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
---
# TC-939: Admit a field, parameter and result typed by a variant type

## Description

Verify that a field, an operation parameter and an operation result whose
`typeRef` names a variant type are admitted, and that the placeholder refusals
FR-208 and FR-154 gave them no longer apply. Scope: FR-208-AC-15, FR-208-AC-16,
FR-154-AC-11.

## Test Procedure

Use TC-235's fixture K2 with the M24 package (variant type `Tier`, cases `Gold`
and `Standard`).

1. Give object `Order` the field `tier: Tier`, record value type `Money` the
   field `tier: Tier`, state machine `OrderLifecycle` a context field
   `tier: Tier`, process `Fulfilment` a field `tier: Tier`, and `Order` the
   operation `setTier` with parameter `t: Tier` and result `Tier`, no contract.
2. Give event type `OrderPlaced` a non-occurrence field `tier: Tier`; then
   name `Tier` as its `occurrenceField`.
3. Empty `Tier.variants`, keeping the field of step 1 naming `Tier`.
4. Give `Order` a field `ref` typed, in turn, `OrderPlaced`, `OrderLifecycle`,
   `Fulfilment`, `OrderStore` and `OrderManagement`.
5. Add to `Money` a `quire` clause that names the variant type and a case
   literal: `self.tier = OrderManagement::Tier::Gold`.

## Expected Results

1. Admitted with no refusal. Each field and the parameter and result are
   members typed by `Tier`.
2. The non-occurrence field is admitted. Naming `Tier` as the occurrence field
   is `invalid_model_binding`/`malformed-declaration` at the event type node,
   path `TypeDefinition.occurrenceField`.
3. One refusal, `invalid_model_binding`/`malformed-declaration` at the variant
   type, path `TypeDefinition.variants`; the field naming `Tier` raises none of
   its own.
4. Each of the five fields keeps `invalid_model_binding`/`malformed-declaration`
   at `Field.typeRef`.
5. Admitted, with no `unsupported_construct`/`declaration-form`: the clause names
   `Tier` and its case `Gold`.
