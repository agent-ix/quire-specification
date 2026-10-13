---
id: TC-942
title: "Variant case names are any non-empty string"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-141
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-144
    type: verifies
---
# TC-942: Variant case names are any non-empty string

## Description

Verify that a variant type whose case names are not identifiers is admitted,
that each case has a distinct member key, and that a set orders cases by the
UTF-8 bytes of the case name, while an ordinary enumeration still requires an
identifier. Scope: FR-141-AC-8, FR-144.

## Test Procedure

Use the M24 package with variant type `Tier` given cases `Gold`, `Gold Plus`,
`2x` and `Café`, each with its identity, and object `Order` with a field
`tiers: Tier` of multiplicity `[0, unbounded]`, unordered and unique.

1. Admit the package and compute the member key of each case over the `Tier`
   node key and the exact case name.
2. Compare `Gold` with `Gold Plus`.
3. Build the set `{Gold Plus, Café, Gold, 2x}` and read its canonical element
   order.
4. Give an ordinary `scalar_type`/`enum` node a member whose case is `Gold Plus`.

## Expected Results

1. Admitted; four member keys, pairwise distinct.
2. Unequal.
3. The order is `2x`, `Café`, `Gold`, `Gold Plus`: ascending UTF-8 bytes, with
   `2` (0x32) before `C` (0x43) before `G` (0x47), and `Gold` before `Gold Plus`
   as a proper prefix.
4. Refused: an ordinary enum member's case must be an identifier.
