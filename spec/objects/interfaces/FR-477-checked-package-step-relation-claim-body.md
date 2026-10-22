---
id: FR-477
title: "Checked-package step-relation claim body"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-397
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-402
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-451
    type: references
---

# FR-477: Checked-package step-relation claim body

## Description

When a checked package carries a `relation` clause whose execution variables
are bound to model operations
([FR-397](../../functional/analysis/FR-397-check-step-relations-over-model-operations.md)),
the producer SHALL emit it as one `quire.checked-semantic-graph/v2` node with
`node_tag: "claim"` and `semantic_form: "step_relation"`, whose body is the
aggregate below, and the reader SHALL refuse a node whose rules below do not
hold, with the first refusal in the reader order below. The node is what a
code-claim harness generator reads: it reaches every premise of the code
claim, and the body, through the node's references, and from nothing else.

## Properties

- **body** — `{term: "aggregate", members: [binding("subjects", …),
  binding("executions", …), binding("invariants", …), binding("body", …)]}`,
  in this order:
  - `subjects` — an aggregate of one `binding(alias, reference(M))` per
    distinct model alias the execution variables name, in order of first
    naming, each referencing the alias's `model` node, as FR-402's
    `subjects` does;
  - `executions` — an aggregate of one `binding(variable, aggregate)` per
    execution variable, in source order, whose aggregate holds, in this
    order:
    - `binding("alias", …)`, a `literal` text term naming the variable's
      model alias;
    - `binding("operation", …)`, a `reference` to the operation's
      `state`/`operation_anchor` node
      ([FR-342](FR-342-checked-package-operation-anchor-body.md)), whose
      `frame` member reaches the operation's `state`/`frame` node;
    - `binding("parameters", …)`, an aggregate of `reference`s to the
      execution's `value`/`parameter` nodes in the binder-level order
      [FR-341](FR-341-checked-package-state-clause-body.md) gives a
      postcondition: `self`, then `result` when the operation has one, then
      the operation's parameters in declared order;
    - `binding("preconditions", …)` and `binding("postconditions", …)`, each
      an aggregate of `reference`s, ascending by node-id digest, to every
      `state`/`state_clause` node of that clause kind whose anchor argument
      is the execution's `operation` node: the operation's effective
      precondition and postcondition;
    - `binding("implementation", …)`, a `reference` to the
      `correspondence`/`abstraction_relation` node
      ([FR-451](FR-451-checked-package-abstraction-relation-body.md)) whose
      `frames` hold the entry keyed by the operation, or the empty aggregate
      `{term: "aggregate", members: []}` when no relation binds it;
  - `invariants` — an aggregate of one `binding(alias, aggregate)` per
    `subjects` entry, in the same order, whose aggregate holds `reference`s,
    ascending by node-id digest, to every `state`/`state_clause` node of
    clause kind `invariant` anchored at an object type, process or state
    machine of that alias's model: the model's declared invariants;
  - `body` — a `reference` to the `expression` node of the relation's
    Boolean body, whose reads of an execution's arguments, receiver
    pre-state, receiver post-state and result are the reads of that
    execution's `parameters` nodes, as a postcondition's are.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **declaration** — `{qualified_name}`, the relation's declared name.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node. The model claim and, when requested, the code
  claim are two items over one occurrence, told apart by their capability
  kinds (FR-402 "Obligation identity").
- **dependencies** — exactly the unique, digest-ascending set of the body's
  `reference` targets.

Premises are reached through references, never copied: each precondition,
postcondition and invariant is its own `state`/`state_clause` node, the frame
is the anchor's, and each implementation binding is an entry of an
abstraction relation node.

## Code claim request

The clause yields the code claim exactly when every `executions` entry's
`implementation` is a reference. A node with any empty `implementation`
yields the model claim alone (FR-397 "Code claim").

## Identity

The node key follows FR-322's rules over the body, so the subjects, the
execution variables in order with their operations, the parameters, every
precondition, postcondition and invariant, the implementation bindings and
the body each enter the node id and the `package_id`. The obligation
identities of the model claim and the code claim are FR-402's.

## Reader order

The step-relation step runs after the abstraction step of FR-451 and before
the operation step, over every `claim`/`step_relation` node in ascending
node-id digest order. For one node, the checks run in this order and the
first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the four bindings in
   order, and each `executions` entry of exactly the six bindings in order,
   else `invalid_semantic_graph` at the first binding that breaks the
   layout.
2. **Aliases.** Each `executions` entry's `alias` names a `subjects`
   binding, and each `subjects` binding is named by some entry, else
   `missing_declaration`/`missing-name` at the `alias` binding, or
   `invalid_semantic_graph` at the unnamed `subjects` binding.
3. **Targets**, entry by entry in body order: a `subjects` target that is
   not a `model` node, an `operation` target that is not a
   `state`/`operation_anchor` node whose `context` is a declaration of the
   alias's model, or an `implementation` target that is not a
   `correspondence`/`abstraction_relation` node holding a `frames` entry
   keyed by the operation, refuses `invalid_model_binding`/
   `malformed-declaration` at the target.
4. **Premises.** An execution's `parameters` that are not the FR-341
   postcondition binders of its operation, a `preconditions` or
   `postconditions` aggregate, or an `invariants` aggregate, that is not
   exactly the set the Properties define over the package, refuses
   `invalid_model_binding`/`malformed-declaration` at that aggregate.
5. **Body.** A `body` target that is not an `expression` node whose
   `semantic_type` is the `scalar_type`/`boolean` node refuses
   `ill_typed`/`non-boolean-root` at the target.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-477-AC-1 | A determinism relation `x: V::Vault::step, y: V::Vault::step` over a vault whose `step` is bound by an abstraction relation produces one `claim`/`step_relation` node whose body has `subjects`, `executions`, `invariants` and `body` in order; `subjects` holds `V` once; each of the two `executions` entries holds alias `V`, a reference to `step`'s `state`/`operation_anchor` node, the `self`, `result` and declared-parameter nodes in FR-341 order, `step`'s precondition and postcondition nodes and a reference to the abstraction relation node; `invariants` holds `Vault`'s invariant nodes; every occurrence has role `claim`. | Test (TC-962) |
| FR-477-AC-2 | The same relation over a vault whose `step` has no binding has empty `implementation` members and yields the model claim alone; with the binding it yields both claims, with different obligation identities (FR-402-AC-4). | Test (TC-962) |
| FR-477-AC-3 | Adding a declared invariant to `Vault`, changing `step`'s precondition, or swapping `x` and `y` changes the node id and the `package_id`; the same relation respelled with different whitespace and comments has the same node id. | Test (TC-962) |
| FR-477-AC-4 | An `executions` entry missing `implementation` refuses `invalid_semantic_graph`; an `alias` naming no `subjects` binding refuses `missing_declaration`/`missing-name`; an `operation` naming an object type node, and an `implementation` naming an abstraction relation with no `frames` entry for `step`, refuse `invalid_model_binding`/`malformed-declaration`; a `preconditions` aggregate missing `step`'s precondition and an `invariants` aggregate missing one of `Vault`'s invariants refuse `invalid_model_binding`/`malformed-declaration`; a `body` naming an integer-typed expression node refuses `ill_typed`/`non-boolean-root`. | Test (TC-962) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `step_relation` among the
`claim` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the node form, keys,
  `dependencies` and the reader's refusal order.
- [FR-397](../../functional/analysis/FR-397-check-step-relations-over-model-operations.md)
  states the model claim and code claim this node carries.
- [FR-402](FR-402-hyperproperty-obligation-identity.md) owns both claims'
  obligation identities and the `subjects` binding shape.
- [FR-341](FR-341-checked-package-state-clause-body.md),
  [FR-342](FR-342-checked-package-operation-anchor-body.md) and
  [FR-451](FR-451-checked-package-abstraction-relation-body.md) own the
  clause, anchor and abstraction relation nodes the body references.

## References

- STD-291 (this requirement); quire-spec-language FR-180 and ADR-023 XC-1,
  XC-2.
