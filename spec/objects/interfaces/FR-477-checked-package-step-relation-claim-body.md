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
  binding("executions", …), binding("parameters", …),
  binding("preconditions", …), binding("postconditions", …),
  binding("invariants", …), binding("body", …)]}`, in this order. Every
  member is a FR-322 "Body grammar" Member: a `binding` whose value is a
  Group, a Tuple or a Leaf. Where a Tuple holds one Group per execution, its
  Groups are in `executions` order, so Group `k` of `parameters`,
  `preconditions` and `postconditions` belongs to execution `k`:
  - `subjects` — a Group of one `binding(alias, reference(M))` per
    distinct model alias the execution variables name, in order of first
    naming, each referencing the `model`/`model_root` node of the domain
    package the alias selects, as FR-402's `subjects` does;
  - `executions` — a Tuple of one **execution Group** per execution
    variable, in source order. An execution Group is an aggregate whose
    members are, in this order:
    1. `binding("variable", …)`, a `literal` text term, the execution
       variable's name;
    2. `binding("alias", …)`, a `literal` text term naming the variable's
       model alias;
    3. `binding("operation", …)`, a `reference` to the operation's
       `state`/`operation_anchor` node
       ([FR-342](FR-342-checked-package-operation-anchor-body.md)), whose
       `frame` member reaches the operation's `state`/`frame` node;
    4. `binding("implementation", …)`, a `reference` to the
       `correspondence`/`abstraction_relation` node
       ([FR-451](FR-451-checked-package-abstraction-relation-body.md)) whose
       `frames` hold the entry keyed by the operation; this binding is
       absent when no relation binds the operation;
  - `parameters` — a Tuple of one Group per execution, each Group holding
    `reference`s to the execution's own `value`/`parameter` nodes, with the
    names and types
    [FR-341](FR-341-checked-package-state-clause-body.md) gives a
    postcondition's binders, in its binder order: `self`, then `result`
    when the operation has one, then the operation's parameters in
    declared order. Levels continue across executions: the first
    execution's binders take levels 0, 1, … in that order, and each later
    execution's binders take the next levels after the previous
    execution's last. A `value`/`parameter` node is keyed by its name,
    level and type (FR-322), so two executions of one operation have
    distinct binder nodes, and the body tells their reads apart;
  - `preconditions` and `postconditions` — each a Tuple of one Group per
    execution, each Group holding `reference`s, ascending by node id, to
    every `state`/`state_clause` node of that clause kind whose anchor
    argument is the execution's `operation` node: the operation's effective
    precondition and postcondition. A Group with no such clause is the
    empty aggregate;
  - `invariants` — a Tuple of one Group per `subjects` binding, in the same
    order, each Group holding `reference`s, ascending by node id, to every
    `state`/`state_clause` node of clause kind `invariant` anchored at an
    object type, process or state machine of that alias's model: the
    model's declared invariants;
  - `body` — a `reference` to the `expression` node of the relation's
    Boolean body, whose reads of an execution's arguments, receiver
    pre-state, receiver post-state and result are the reads of that
    execution's own `parameters` nodes, as a postcondition's are of its
    binders.

The deepest path in the body is a Member `binding`, a Tuple, a Group, a
`binding` and a Leaf (`executions`), the longest chain FR-322's grammar
admits; no Group holds an aggregate and no Tuple holds a `binding`. An
execution's lists are therefore positional Groups of sibling Tuples, joined
to the execution by index, never aggregates nested inside its execution
Group.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **declaration** — `{qualified_name}`, the relation's declared name.
- **owner** — the declaring source unit's `SourceOwner`
  ([FR-322](FR-322-checked-package-artifact.md) `owner`), required for a
  declared node whose body holds no application; it enters the node id (FR-322
  "Structural node identity").
- **occurrences** — one occurrence of role `declaration`, at the relation's
  declaration, and one of role `claim`, which provider requests name. The
  model claim and, when requested, the code claim are two items over that
  `claim` occurrence, told apart by their capability kinds (FR-402
  "Obligation identity").
- **dependencies** — exactly the unique, node-id-ascending set of the body's
  `reference` targets.

Premises are reached through references, never copied: each precondition,
postcondition and invariant is its own `state`/`state_clause` node, the frame
is the anchor's, and each implementation binding is an entry of an
abstraction relation node.

## Code claim request

The clause yields the code claim exactly when every execution Group holds an
`implementation` binding. A node with any execution Group lacking it yields
the model claim alone (FR-397 "Code claim").

## Identity

The node is declared, so its id is FR-322's structural `{package, path, ordinal}`:
the declaration path (with the owner) and a producer-assigned ordinal. The
subjects, the execution variables in order with their operations, the
parameters, every precondition, postcondition and invariant, the implementation
bindings and the body are part of the body, compared structurally, and none
of them enters the node id. The obligation
identities of the model claim and the code claim are FR-402's.

## Reader order

The step-relation step is a body step of
[FR-322](FR-322-checked-package-artifact.md) "Body steps": it runs after the
terminal step and before the protocol step. It visits every
`claim`/`step_relation` node in ascending node-id order. For one node, the checks run in this order and the
first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the seven bindings in
   order; each `subjects` member is a `binding(alias, reference)`; each
   `executions` member is an execution Group whose bindings are
   `variable`, `alias`, `operation` and, optionally, `implementation`, in
   that order and nothing else, with `variable` and `alias` text literals
   and no two execution Groups holding one `variable` text; each
   `parameters`, `preconditions`, `postconditions` and `invariants` Group
   holds `reference`s only; else `invalid_semantic_graph` at the first
   member that breaks the layout. A `parameters`, `preconditions` or
   `postconditions` Tuple whose Group count is not the number of execution
   Groups, or an `invariants` Tuple whose Group count is not the number of
   `subjects` bindings, refuses `invalid_semantic_graph` at that Tuple's
   binding, `B/members/{i}`.
2. **Aliases.** Each execution Group's `alias` names a `subjects`
   binding, and each `subjects` binding is named by some execution Group,
   else `missing_declaration`/`missing-name` at the `alias` binding, or
   `invalid_semantic_graph` at the unnamed `subjects` binding.
3. **Targets**, execution Group by execution Group in body order, after the
   `subjects` targets: a `subjects` target that is
   not a `model`/`model_root` node, an `operation` target that is not a
   `state`/`operation_anchor` node whose `context` is a declaration of the
   alias's model, or an `implementation` target that is not a
   `correspondence`/`abstraction_relation` node holding a `frames` entry
   keyed by the operation, refuses `invalid_model_binding`/
   `malformed-declaration` at the target.
4. **Premises.** A `parameters` Group whose targets' names and types, in
   order, are not the FR-341 postcondition binders of its execution's
   operation, or whose levels do not continue from the previous
   execution's as the Properties state, a `preconditions` or
   `postconditions` Group, or an `invariants` Group, that is not exactly
   the set the Properties define over the package, refuses
   `invalid_model_binding`/`malformed-declaration` at that Group. An
   execution Group with no `implementation` binding while the package holds
   a `correspondence`/`abstraction_relation` node with a `frames` entry
   keyed by its operation refuses `invalid_model_binding`/
   `malformed-declaration` at that execution Group, so a bound operation's
   code claim is never dropped silently.
5. **Body.** A `body` target that is not an `expression` node whose
   `semantic_type` is the `scalar_type`/`boolean` node refuses
   `ill_typed`/`non-boolean-root` at the target.

With `B` = `/semantic_graph/nodes/{n}/body`, the execution Group `k` is at
`B/members/1/value/members/{k}` and its Group `k` of `parameters`,
`preconditions` and `postconditions` at `B/members/2/value/members/{k}`,
`B/members/3/value/members/{k}` and `B/members/4/value/members/{k}`; the
`invariants` Group of subject `j` is at `B/members/5/value/members/{j}`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-477-AC-1 | A determinism relation `x: V::Vault::step, y: V::Vault::step` over a vault whose `step` is bound by an abstraction relation produces one `claim`/`step_relation` node whose body has `subjects`, `executions`, `parameters`, `preconditions`, `postconditions`, `invariants` and `body` in order; `subjects` holds `V` once; `executions` holds two execution Groups, `x`'s then `y`'s, each binding `variable`, alias `V`, a reference to `step`'s `state`/`operation_anchor` node and a reference to the abstraction relation node; Group `k` of `parameters` holds execution `k`'s own `self` and `i` parameter nodes in FR-341 order (`step` has no result), `x`'s at levels 0 and 1 and `y`'s at levels 2 and 3; Group `k` of `preconditions` and of `postconditions` holds `step`'s precondition and postcondition nodes; `invariants` holds one Group with `Vault`'s invariant nodes; the node carries `declaration` `Det`, its `owner`, and one `declaration` and one `claim` occurrence. | Test (TC-962) |
| FR-477-AC-2 | The same relation over a vault whose `step` has no binding has execution Groups with no `implementation` binding and yields the model claim alone; with the binding it yields both claims, with different obligation identities (FR-402-AC-4). | Test (TC-962) |
| FR-477-AC-3 | Adding a declared invariant to `Vault`, changing `step`'s precondition, or swapping `x` and `y` changes the relation's body, so the package is structurally unequal, and no node id changes; renaming the relation or changing its owner changes the node id; the same relation respelled with different whitespace and comments has the same node id. | Test (TC-962) |
| FR-477-AC-4 | An execution Group with `operation` before `alias` refuses `invalid_semantic_graph` at that Group, and a `parameters` Tuple holding one Group for two executions refuses `invalid_semantic_graph` at `B/members/2`; an execution Group missing `implementation` while the package's abstraction relation has a `frames` entry for `step` refuses `invalid_model_binding`/`malformed-declaration` at that Group; an `alias` naming no `subjects` binding refuses `missing_declaration`/`missing-name`; a `subjects` target that is a `model`/`object_type` node, an `operation` naming an object type node, and an `implementation` naming an abstraction relation with no `frames` entry for `step`, refuse `invalid_model_binding`/`malformed-declaration`; a `preconditions` Group missing `step`'s precondition and an `invariants` Group missing one of `Vault`'s invariants refuse `invalid_model_binding`/`malformed-declaration` at that Group; a `body` naming an integer-typed expression node refuses `ill_typed`/`non-boolean-root`. | Test (TC-962) |
| FR-477-AC-5 | In AC-1's node, `x`'s and `y`'s `self` nodes are different nodes, as are their `i` nodes, and the body's `pre(x.self.l) = pre(y.self.l)` reads both; a `parameters` Group for `y` that references `x`'s `self` and `i` nodes refuses `invalid_model_binding`/`malformed-declaration` at `y`'s `parameters` Group, `B/members/2/value/members/1`. | Test (TC-962) |
| FR-477-AC-6 | AC-1's body passes strict wire validation of FR-322 "Body grammar" with no `malformed_wire`: `subjects` is a Group, `executions` a Tuple of execution Groups whose members are `binding`s to Leaves, and `parameters`, `preconditions`, `postconditions` and `invariants` Tuples of Groups of `reference`s; no Group holds an aggregate and no Tuple holds a `binding`; and the reader admits the package. A body whose execution Group nests a `parameters` aggregate inside it, the shape before QSL-1093, refuses `malformed_wire`. | Test (TC-962) |

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
  XC-2; QSL-1093 (the grammar-legal body: execution Groups and positional
  Tuples of premise Groups, as FR-402's quantifier Groups).
