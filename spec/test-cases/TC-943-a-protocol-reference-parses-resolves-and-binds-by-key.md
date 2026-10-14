---
id: TC-943
title: "A protocol reference parses, resolves and binds by typed key"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-463
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-66
    type: verifies
---
# TC-943: A protocol reference parses, resolves and binds by typed key

## Description

Verify that `protocol qualified-name` parses as a `primary`, resolves to a
local protocol by typed node key, refuses a missing, import-only or non-protocol
target, and is carried as the FR-322-AC-66 `protocol_reference` node. Scope:
FR-463-AC-1 to FR-463-AC-5.

## Test Procedure

1. Parse a unit that declares `protocol Fill` and holds `protocol Fill` and
   `protocol M::Fill` as expressions. Inspect the parse.
2. Parse `protocol`, `protocol 3` and `protocol (Fill)` as expressions.
3. Check a unit that declares `protocol Fill` and holds `protocol Fill`.
   Inspect the bound key and type, then the checked package.
4. Check a unit holding `protocol Missing`; one holding `protocol Lib::Fill`
   where `Fill` exists only in an imported library; and one holding a name that
   resolves to a declared function.
5. Change `Fill` so its node key changes and check again.

## Expected Results

1. Two `protocol-reference` nodes in source order with targets `Fill` and
   `M::Fill`, no package selector, and the `protocol-clause` declaration parsed
   unchanged.
2. Each refuses `invalid_syntax` at the token that is not a `qualified-name`.
3. The reference binds `Fill`'s `protocol`/`protocol_clause` node key and has
   its semantic type. The package holds one `protocol`/`protocol_reference` node
   with body exactly `{term: "reference", target: <Fill key>}`.
4. `protocol Missing` and the import-only name refuse
   `missing_declaration`/`missing-name`; the function refuses
   `ill_typed`/`operator-ineligible`. No refusal produces a package.
5. The reference node's key changes with `Fill`'s key.
Assertions compare typed fields, causes and keys, never message text.
