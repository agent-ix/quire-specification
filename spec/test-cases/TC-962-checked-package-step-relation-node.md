---
id: TC-962
title: "Checked-package step-relation claim node"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-477
    type: verifies
---

# TC-962: Checked-package step-relation claim node

## Description

Check the `claim`/`step_relation` node a step relation produces, that it
references every premise of the code claim, that every premise enters its
identity, and the reader's refusals of a malformed node.

## Test Procedure

Compile each vector through the real local Rust boundary, read the
`claim`/`step_relation` node and the items the clause yields, and compare node
ids. For RN-07 to RN-12, RN-14, RN-15's second case and RN-16, edit RN-01's node as
stated, adding to the graph any node the edit references; then set its
`dependencies` to the body's targets, as TC-233 does
for its patched vectors, and feed the package to the reader. The vault declares
`step(i)` with a precondition `pre StepOk using v on V::Vault::step { self.h
>= 0 }`, a postcondition, and the invariant `self.l <= self.h`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RN-01 | TC-343 SR-05: TC-337 HB-11 with `V::Vault::step` bound by an abstraction relation | one node; body members `subjects`, `executions`, `parameters`, `preconditions`, `postconditions`, `invariants`, `body` in order; `subjects` holds `V` once; `executions` holds two execution Groups, `x` then `y`, each binding `variable`, alias `V`, `step`'s `operation_anchor` node and the abstraction relation node as `implementation`; `parameters` Group `k` holds execution `k`'s own `self` and `i` parameter nodes in that order (`step` takes `i` and has no result; `x`'s at levels 0 and 1, `y`'s at levels 2 and 3); each `preconditions` Group holds `StepOk` and each `postconditions` Group the postcondition node; `invariants` holds one Group with the `self.l <= self.h` node; `declaration` `Det` with its `owner`; one `declaration` and one `claim` occurrence |
| RN-02 | HB-11 with no abstraction relation | neither execution Group holds an `implementation` binding; one item, the model claim |
| RN-03 | RN-01's two items | model claim `temporal-satisfaction`, code claim `operation-contract`, different obligation identities |
| RN-04 | RN-01 with a second invariant added to `Vault`; RN-01 with `StepOk` changed to `self.h >= 1` | each a structurally unequal body from RN-01; the node id is unchanged |
| RN-05 | RN-01 with `x` and `y` swapped in the binding list | a structurally unequal body; the node id is unchanged |
| RN-06 | RN-01 with different whitespace and comments | RN-01's node id |
| RN-07 | RN-01 with `x`'s execution Group holding `operation` before `alias`; RN-01 with `parameters` holding only `x`'s Group | `invalid_semantic_graph` at `x`'s execution Group; `invalid_semantic_graph` at the `parameters` binding, `/semantic_graph/nodes/{n}/body/members/2` |
| RN-08 | RN-01 with `x`'s `alias` `W` | `missing_declaration`/`missing-name` at the `alias` binding |
| RN-09 | RN-01 with `x`'s `operation` a reference to the `Vault` object type node; RN-01 with `subjects.V` a reference to the `Vault` object type node | `invalid_model_binding`/`malformed-declaration` at the target |
| RN-10 | RN-01 with `implementation` naming an added abstraction relation with no `frames` entry for `step` | `invalid_model_binding`/`malformed-declaration` at the target |
| RN-11 | RN-01 with `StepOk` removed from `x`'s `preconditions` Group; RN-01 with the invariant removed from the `invariants` Group | `invalid_model_binding`/`malformed-declaration` at that Group, each |
| RN-12 | RN-01 with `body` a reference to the `expression` node of `x.self.l` | `ill_typed`/`non-boolean-root` at the target |
| RN-13 | RN-01's `x` and `y` binder nodes and its body | `x`'s `self` node differs from `y`'s, and `x`'s `i` node from `y`'s; the body's `pre(x.self.l) = pre(y.self.l)` reads two different nodes, so the body is not a tautology |
| RN-14 | RN-01 with `y`'s `parameters` Group referencing `x`'s `self` and `i` nodes | `invalid_model_binding`/`malformed-declaration` at `y`'s `parameters` Group, `/semantic_graph/nodes/{n}/body/members/2/value/members/1` |
| RN-15 | RN-01's package through strict wire validation; RN-01 with `x`'s execution Group holding a nested `parameters` aggregate of its parameter references (the shape before QSL-1093) | admitted with no `malformed_wire`: every body member is a Group, a Tuple of Groups, or a Leaf, and the deepest path is binding, Tuple, Group, binding, Leaf; the nested shape refuses `malformed_wire` |
| RN-16 | RN-01 with `x`'s `implementation` binding removed, the abstraction relation node and its `frames` entry for `step` kept | `invalid_model_binding`/`malformed-declaration` at `x`'s execution Group |

## Expected Results

The node reaches every premise and the body by reference, each premise changes
its identity, and each malformed node refuses at the first failing reader step.
