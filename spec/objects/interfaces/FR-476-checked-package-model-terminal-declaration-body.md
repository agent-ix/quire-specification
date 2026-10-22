---
id: FR-476
title: "Checked-package model terminal declaration body"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
---

# FR-476: Checked-package model terminal declaration body

## Description

When a checked package carries a model `terminal` declaration
([FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md)), the
producer SHALL emit it as one `quire.checked-semantic-graph/v2` node with
`node_tag: "temporal"` and `semantic_form: "terminal"`, whose body is the
aggregate below, and the reader SHALL refuse a node whose rules below do not
hold, with the first refusal in the reader order below. The node is the only
carrier of the declaration: a deadlock-freedom item, its classifier and its
replay read the model's intended terminal states from this node and from
nothing else.

## Properties

- **body** — `{term: "aggregate", members: [binding("model", …),
  binding("mode", …), binding("parameters", …), binding("condition", …)]}`,
  in this order:
  - `model` — a `reference` to the `model` node of the model alias the
    declaration's `on` names, the node an FR-402 `subjects` binding names for
    that alias;
  - `mode` — a `literal` text term, `when` or `any`;
  - `parameters` — an aggregate of `reference`s to `value`/`parameter`
    nodes: the `self` parameter, typed `Reference<M::T>`, when the condition
    reads `self`, and otherwise empty. It is empty for `any`;
  - `condition` — for `when`, a `reference` to the `expression` node of the
    Boolean state predicate `P`; for `any`, the empty aggregate
    `{term: "aggregate", members: []}`.
- **declaration** — `{qualified_name}` whose path is the model alias's path
  followed by the segment `terminal`. The written name `T` of `terminal T
  using v on M …` is presentation, carried by the node's `declaration`
  occurrence: renaming `T` changes neither the node id nor any obligation
  identity.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **occurrences** — one occurrence of role `declaration` per source
  declaration that checks to this node, at the `terminal-decl` it was
  produced from.
- **dependencies** — exactly the unique, digest-ascending set of the body's
  `reference` targets: the `model` node, each `parameters` node and, for
  `when`, the condition node.

The body carries no `using` profile alias: the profile is the unit's value
profile, which the lock already selects.

## Identity

The node key follows FR-322's rules over the body and the `declaration`, so
the mode, the model and the condition each enter the node id and the
`package_id`. Two packages whose `terminal` declarations differ only in mode,
or only in the condition, have different `package_id`s. A model subject's
identity, and so the obligation identity of every item over the subject, the
derived deadlock-freedom item included, binds the node id of the model's
`terminal` node when the package holds one, and binds no terminal node when it
holds none (FR-366: a model with no declaration intends no terminal state).

## Reader order

The terminal step runs after the temporal step of
[FR-370](FR-370-checked-package-temporal-clause-body.md) "Reader order" and
before the operation step, over every `temporal`/`terminal` node in ascending
node-id digest order. For one node, the checks run in this order and the first
failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the four bindings in order,
   else `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/body`.
2. **Mode.** `mode` is `when` or `any`, else `invalid_package`/`invalid-value`
   at the `mode` binding.
3. **Mode fit.** `condition` is a `reference` exactly when `mode` is `when`,
   and `parameters` is empty when `mode` is `any`, else
   `invalid_semantic_graph` at the binding that breaks the rule.
4. **Targets**, in body order: a `model` target that is not a `model` node
   refuses `invalid_model_binding`/`malformed-declaration`; a `parameters`
   target that is not a `value`/`parameter` node, or a `condition` target that
   is not an `expression` node, refuses `ill_typed`/`operator-ineligible`; a
   `condition` target whose `semantic_type` is not the `scalar_type`/`boolean`
   node refuses `ill_typed`/`non-boolean-root`. The locus is the target.

Once every node's own checks hold, the reader checks that no two
`temporal`/`terminal` nodes reference one `model` node; the second, in
ascending node-id digest order, refuses `ambiguous_declaration`/
`ambiguous-name` naming both nodes (FR-366).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-476-AC-1 | `terminal Done using v on M when { P }` produces one `temporal`/`terminal` node whose body has the bindings `model`, `mode`, `parameters` and `condition` in that order, with `mode` `when` and `condition` a reference to `P`'s `expression` node; `terminal Halt using v on M any` produces `mode` `any`, an empty `parameters` and an empty `condition`; each node's `declaration` path ends with `terminal`, its occurrences have role `declaration` and its `dependencies` are exactly the body's targets. | Test (TC-961) |
| FR-476-AC-2 | A `when` condition that reads `self` binds the `self` parameter node in `parameters`; one that reads no `self` has empty `parameters`. | Test (TC-961) |
| FR-476-AC-3 | Changing the mode or the condition changes the node id and the `package_id`; renaming `T` changes neither; the deadlock-freedom item over a subject whose model has a `terminal` node has a different obligation identity from the same subject whose model has none. | Test (TC-961) |
| FR-476-AC-4 | A body with `mode` and `condition` swapped in order refuses `invalid_semantic_graph`; `mode` `some` refuses `invalid_package`/`invalid-value`; an `any` node with a `condition` reference, and a `when` node with an empty `condition`, refuse `invalid_semantic_graph`; a `model` target that is a `value`/`parameter` node refuses `invalid_model_binding`/`malformed-declaration`; a `condition` naming an integer-typed expression node refuses `ill_typed`/`non-boolean-root`; two `terminal` nodes on one `model` node refuse `ambiguous_declaration`/`ambiguous-name` naming both. | Test (TC-961) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `terminal` among the
`temporal` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the node form, keys,
  `declaration`, `dependencies` and the reader's refusal order.
- [FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md) owns
  the declaration's grammar and meaning and the derived deadlock-freedom item.
- [FR-465](FR-465-checked-package-protocol-declaration-bodies.md) "Terminal"
  is the protocol member with the same `when`/`any` layout.
- [FR-341](FR-341-checked-package-state-clause-body.md) binds `self` for a
  state predicate the same way.

## References

- STD-303 (this requirement); quire-spec-language FR-124 and ADR-018 DL-1 to
  DL-3.
