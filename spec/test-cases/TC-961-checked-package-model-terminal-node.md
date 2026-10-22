---
id: TC-961
title: "Checked-package model terminal declaration node"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-476
    type: verifies
---

# TC-961: Checked-package model terminal declaration node

## Description

Check the `temporal`/`terminal` node of a model `terminal` declaration in
both modes, that mode and condition enter identity while the written name does
not, and the reader's refusals of a malformed node.

## Test Procedure

Compile each vector through the real local Rust boundary over FR-366's
counter model `M` (one object type `C` with field `value` and its one population `M::Counters`),
read the `temporal`/`terminal` node, and compare node ids, `package_id`s and
the deadlock-freedom item's obligation identity. For MT-07 to MT-12, edit the
node of MT-01 or MT-02 as stated, recompute its key, and feed the package to
the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| MT-01 | `terminal Done using v on M when { self.value = 3 };` | one node; `mode` `when`; `parameters` holds the `self` parameter node typed `Reference<M::C>`; `condition` references the `expression` node of `self.value = 3`; `declaration` path `M`, `terminal`; occurrence role `declaration` |
| MT-02 | `terminal Halt using v on M any;` | `mode` `any`; `parameters` and `condition` empty aggregates; `dependencies` the `model` node only |
| MT-03 | `terminal Done using v on M when { forall(c in M::Counters : c.value = 3) };` | `parameters` empty |
| MT-04 | MT-01 and MT-01 with `Done` renamed `Stop` | one node id and one `package_id` |
| MT-05 | MT-01 and MT-02; MT-01 and MT-01 with condition `self.value = 2` | different node ids and `package_id`s for each pair |
| MT-06 | the deadlock-freedom item over `M` with MT-01, and with no `terminal` declaration | different obligation identities |
| MT-07 | MT-01 with `mode` and `condition` swapped in order | `invalid_semantic_graph` at the body |
| MT-08 | MT-01 with `mode` `some` | `invalid_package`/`invalid-value` at the `mode` binding |
| MT-09 | MT-02 with `condition` a reference to an expression node; MT-01 with `condition` the empty aggregate | `invalid_semantic_graph` at the `condition` binding, each |
| MT-10 | MT-01 with `model` a reference to the `self` parameter node | `invalid_model_binding`/`malformed-declaration` at the target |
| MT-11 | MT-01 with `condition` a reference to the `expression` node of `self.value` | `ill_typed`/`non-boolean-root` at the target |
| MT-12 | a package holding MT-01's and MT-02's nodes for `M` | `ambiguous_declaration`/`ambiguous-name` naming both nodes |

## Expected Results

Each mode has its own body, the written name never changes identity, the
deadlock-freedom identity reads the declaration, and each malformed node
refuses at the first failing reader step.
