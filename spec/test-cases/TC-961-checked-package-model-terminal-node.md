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
counter model `M` (one object type `C` with field `value` and its one
population `M::Counters`), read the `temporal`/`terminal` node, and compare
node ids, `package_id`s and the deadlock-freedom item's obligation identity.
For MT-07 to MT-15, edit the node of MT-01 or MT-02 as stated, adding to the
graph any node the edit references; then set the node's `dependencies` to the
body's targets, recompute its node key, rewrite its `identity_projection`
entry and recompute the `package_id`, as TC-233 does for its patched
vectors, and feed the package to the reader.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| MT-01 | `terminal Done using v on M when { self.value = 3 };` | one node with no `declaration`; `mode` `when`; `parameters` holds one reference, to the `self` parameter node at level 0 typed `Reference<M::C>`; `condition` references the `expression` node of `self.value = 3`; occurrence role `declaration` |
| MT-02 | `terminal Halt using v on M any;` | `mode` `any`; `parameters` and `condition` empty aggregates; `dependencies` the `model` node only |
| MT-03 | `terminal Done using v on M when { forall(c in M::Counters : c.value = 3) };` | `parameters` empty |
| MT-04 | MT-01 and MT-01 with `Done` renamed `Stop` | one node id and one `package_id` |
| MT-05 | MT-01 and MT-02; MT-01 and MT-01 with condition `self.value = 2` | different node ids and `package_id`s for each pair |
| MT-06 | the deadlock-freedom item over `M` with MT-01, and with no `terminal` declaration | different `package_id`s and so different obligation identities |
| MT-07 | MT-01 with `mode` and `condition` swapped in order | `invalid_semantic_graph` at the body |
| MT-08 | MT-01 with `mode` `some` | `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/1` |
| MT-09 | MT-02 with `condition` a reference to the added `expression` node of `self.value = 3`; MT-01 with `condition` the empty aggregate | `invalid_semantic_graph` at the `condition` binding, each |
| MT-10 | MT-01 with `model` a reference to the `self` parameter node | `invalid_model_binding`/`malformed-declaration` at the target |
| MT-11 | MT-01 with `condition` a reference to the added integer-typed `expression` node of `self.value + 1` | `ill_typed`/`non-boolean-root` at the target |
| MT-12 | a package holding MT-01's and MT-02's nodes for `M` | `ambiguous_declaration`/`ambiguous-name` at the second node in ascending node-id digest order, naming both |
| MT-13 | MT-01 with `parameters` holding the `self` node twice | `invalid_semantic_graph` at the `parameters` binding |
| MT-14 | MT-02 with `parameters` holding MT-01's `self` node and `condition` a reference to the added node of `self.value = 3` | `invalid_semantic_graph` at the `parameters` binding |
| MT-15 | MT-01 with its `self` entry replaced by an added parameter node named `result` at level 1; then by an added `self` node typed `Reference<M::D>` for a second object type `D` of `M` | `ill_typed`/`operator-ineligible` at the target, each |

## Expected Results

Each mode has its own body, the written name never changes identity, the
deadlock-freedom identity reads the declaration, and each malformed node
refuses at the first failing reader step.
