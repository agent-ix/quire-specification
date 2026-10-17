---
id: TC-945
title: "Composed documents declare no model node"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
---
# TC-945: Composed documents declare no model node

## Description

Verify FR-154's composed document rule: a document whose `type` is an artifact
type and that also carries `object:` is skipped by extraction because its `type` is an artifact-type id, is never
lifted, and is never named by a model reference. Scope: FR-154-AC-14.

## Test Procedure

The vectors are `proposals/checked-package-v2/composed-document-vectors.json`
over package `test/orders` with object types `entity`, `value_object`,
`enumeration`, `process` and `state_machine`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| C01 | `FR-204` (`type: FR`, `object: value_object`) and `FR-271` (`type: FR`, `object: enumeration`) | no node; no declaration key; the lifted package equals the one lifted without them |
| C02 | `Invoice` with `type: FR` and `object: entity` | no node: extraction decides by `type` alone, `object:` never selects an object type |
| C03 | `FR-100` with `type: FR` and no `object:` | no node |
| C04 | `Order` with `type: entity`, and `Money` with `type: value_object` and `object: value_object` | one node each, `ix://test/orders/Order` and `ix://test/orders/Money`; `object:` changes nothing |
| C05 | a relationship end naming `ix://test/orders/FR-204`; a supertype naming `ix://test/orders/FR-271` | each `refused { code: missing_declaration, cause: missing-name }`; a reference to `ix://test/orders/Order` is admitted |
| C06 | a node `ix://test/orders/FR-204` in the package | `refused { code: invalid_model_binding, cause: malformed-declaration }` at `TypeDefinition.identity`; `ix://test/orders/sys_pump` is admitted |

## Expected Results

Every result matches FR-154-AC-14; no composed document is lifted under a name
derived from its id.
