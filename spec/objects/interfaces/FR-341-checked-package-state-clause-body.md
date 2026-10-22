---
id: FR-341
title: "Checked-package state clause body"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-341: Checked-package state clause body

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "state"` and
`semantic_form: "state_clause"`, the producer SHALL encode its `body` as one
application of `quire.op.state.clause`, the clause operation of an
invariant, a precondition or a postcondition, whose `state_clause` member
names the clause kind and whose three arguments are, in this fixed order,
the clause's parameter aggregate, its anchor and its Boolean condition. The
reader SHALL refuse a node whose joins below do not hold with the first
refusal in the order below.

## Properties

- **body** — an `application` term with `operator: "state_clause"` and
  `operation.identity: "quire.op.state.clause"`, whose `operation.member` is
  `{kind: "state_clause", clause}` with `clause` one of `invariant`,
  `precondition` or `postcondition`, `laws` empty, `mode` `null` and
  `leaves` empty. Its `arguments` are exactly:
  1. **the parameter aggregate** — an `aggregate` whose members are
     `reference` terms to `value`/`parameter` nodes, one per value the
     clause binds, in binder-level order: `self` at level 0, then `result`
     when present, then the operation's parameters in declared order;
  2. **the anchor** — a `reference` to the node the clause is anchored at:
     the context's node for an invariant (a `model`/`object_type`,
     `model`/`process` or `model`/`state_machine` node), and the
     operation's `state`/`operation_anchor` node
     ([FR-342](FR-342-checked-package-operation-anchor-body.md)) for a
     precondition or postcondition;
  3. **the condition** — the Boolean term the clause asserts.
- **semantic_type** — the `scalar_type`/`boolean` node. The application's
  `result_type` names the same node: the catalog's `clause` result form
  declares a Boolean result, and FR-322's rule that a node whose body root
  is an application has `semantic_type` equal to its `result_type` then
  fixes the node's type.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node.
- **dependencies** — FR-322's application-node rule: exactly the unique
  digest-ascending `reference` targets of the body, so the anchor, each
  parameter node and every node the condition references.

The parameters a clause binds follow the state contract's "Operation
anchors, aliases and captures" table:

| Clause | `self` | `result` | Operation parameters |
| --- | --- | --- | --- |
| `invariant` | `Reference<C>`, `C` the anchor type | absent | none |
| `precondition` | `Reference<C>`, `C` the anchor's `context` | absent | the resolved operation's parameters, in order |
| `postcondition` | `Reference<C>`, `C` the anchor's `context` | the resolved operation's result type, present exactly when the operation has a result | the resolved operation's parameters, in order |

`C` is an object type, a process or a state machine, and `self` is spelled
`Reference<C>` for each, the `composite_type`/`reference` node over `C`'s
model declaration node ([FR-322](FR-322-checked-package-artifact.md) "Derived
type forms and forwarding"). It stands for the object of an object type, for
the run of a process at the run's declared object identity, and for the one
context of a state machine
([FR-208](../foundation/FR-208-quire-meaning-vocabulary.md)). A clause on a
process or state machine binds its `result` and parameters from its operation
exactly as an object type's clause does.

A parameter's type is the `semantic_type` of its `value`/`parameter` node.
A result and parameter type is the type
[FR-322](FR-322-checked-package-artifact.md)'s "Model-owned members" step 4
derives from the operation record the anchor resolves (FR-342-AC-4), and a
parameter node's `semantic_type` names that type's node.

## Operation catalog entry

`quire.op.state.clause` is the catalogued operation (FR-322 "Operation
families, groups and constraints"):

| Member | Value |
| --- | --- |
| operator class | `state_clause` |
| operands | `aggregate`, `object`, `boolean` |
| rest | none |
| result | `clause` (declared Boolean) |
| laws | none |
| mode | none |
| member | `state_clause`: `{kind: "state_clause", clause: "invariant" \| "precondition" \| "postcondition"}` |
| constraints | none |
| leaves | none |

The anchor fits the `object` position: the node of an object type, a process
or a state machine has family `object`, and an `operation_anchor` node's
`semantic_type` is its context node (FR-342), so a reference to it has family
`object` too. The
clause kind is the only difference between the three clauses' spellings, so
the kind member decides which clause an application is.

## Node forms and body roots

- A `quire.op.state.clause` application is the body root of a
  `state`/`state_clause` node, and stands nowhere else.
- A `quire.op.state.transition` application is the body root of a
  `state`/`transition` node.
- A `state`/`frame` node's body is
  [FR-340](FR-340-checked-package-frame-location.md)'s frame term and holds
  no clause application.
- A `state`/`operation_anchor` node's body is FR-342's binding aggregate
  and holds no application.

The schema fixes each body root. A `quire.op.state.clause` application
standing as the body root of a node that is not `state`/`state_clause`
refuses `ill_typed`/`operator-ineligible` at that node. A
`quire.op.state.clause` application nested inside a node body or inside
another term is outside [FR-322](FR-322-checked-package-artifact.md)'s body
grammar and refuses `malformed_wire` at the nested application, at strict
wire validation, like any other nested application other than `case`.

## `value`/`parameter` node body

A `value`/`parameter` node's `body` is an `aggregate` of exactly two
bindings, in order: `name`, a `text` literal naming the value (`"self"`,
`"result"`, or the operation parameter's declared name); and `level`, an
`integer` literal, the value's binder level in the "Operation anchors,
aliases and captures" table above (`0` for `self`, ascending for `result`
and each operation parameter in bound order). Every occurrence has role
`expression`. The node's `semantic_type` is the value's type, as that table
and [FR-322](FR-322-checked-package-artifact.md)'s "Model-owned members"
step 4 derive it. A step relation's execution binders are the one exception to
the table's levels: they take the levels that continue across executions
([FR-477](FR-477-checked-package-step-relation-claim-body.md)
`parameters`), so a later execution's `self` has a level above `0`.

## Reader order

The state step runs after the frame step (FR-340) and before any operation
refusal ([FR-322](FR-322-checked-package-artifact.md)), after every
graph-shape, `stale-node-key` and declaration refusal. Within it, the
reader first checks the placement of every `quire.op.state.clause`
application, in ascending node-id digest order; then every
`state`/`operation_anchor` node (FR-342), in ascending node-id digest
order; then every `state`/`state_clause` node, in ascending node-id digest
order. It reports the first defect. For one clause node, the checks run in
this order and the first failing check is its defect:

1. **Anchor.** The anchor's target is a declared dependency of the clause
   node, else `missing_declaration`/`missing-name`; for an invariant it is a
   `model`/`object_type`, `model`/`process` or `model`/`state_machine` node and
   for a precondition or postcondition a `state`/`operation_anchor` node, else
   `invalid_model_binding`/`malformed-declaration`. The locus is the anchor's
   target.
2. **Parameters.** Each parameter reference, in aggregate order, targets a
   declared dependency, else `missing_declaration`/`missing-name`, that is
   a `value`/`parameter` node, else `invalid_model_binding`/
   `malformed-declaration`. The locus is the offending target.
3. **Signature.** The parameter count and each parameter's type equal the
   table above, else `ill_typed`/`operator-ineligible` at the clause node.
   `self` is checked first, then `result`, then the operation's parameters
   in order.

The Boolean condition, the operand families and the `result_type` are the
application's catalog checks, reported at the operation step in FR-322's
order: a condition of a non-Boolean family and a `result_type` naming a
non-Boolean node each refuse `ill_typed`/`operator-ineligible`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-341-AC-1 | A `state`/`state_clause` node's body is a `quire.op.state.clause` application with operator class `state_clause`, a `{kind: "state_clause", clause}` member whose `clause` is `invariant`, `precondition` or `postcondition`, and three arguments: an `aggregate` of one or more `reference` terms, a `reference`, and the condition. A body that is another clause application, names another identity or operator class, carries another member kind or clause value, or whose first or second argument has another term shape fails the published schema. | Test (TC-233) |
| FR-341-AC-2 | Every occurrence of a `state`/`state_clause` node has role `claim`; an occurrence of another role fails the published schema. | Test (TC-233) |
| FR-341-AC-3 | The anchor of an invariant is a declared `model`/`object_type`, `model`/`process` or `model`/`state_machine` dependency and the anchor of a precondition or postcondition a declared `state`/`operation_anchor` dependency; an invariant anchored at an operation anchor and a precondition anchored at an object type refuse `invalid_model_binding`/`malformed-declaration` at the anchor's target. | Test (TC-233) |
| FR-341-AC-4 | Each parameter reference targets a declared `value`/`parameter` dependency, else it refuses `invalid_model_binding`/`malformed-declaration` at its target; `self` has type `Reference<C>` for the clause's context `C` (an object type, a process or a state machine), and an invariant binds `self` alone; a `self` of another type and an invariant binding a second parameter refuse `ill_typed`/`operator-ineligible` at the clause node. | Test (TC-233) |
| FR-341-AC-5 | A precondition binds `self` and then the resolved operation's parameters; a postcondition binds `self`, then the operation's result when it has one, then its parameters; for `Order.scaled(n: Integer): Integer`, `[Reference<Order>, Integer]` is the precondition's list and `[Reference<Order>, Integer, Integer]` the postcondition's, and `Order.reset()` with no result gives `[Reference<Order>]`. A missing parameter, a missing result, a `self` of another object type and a parameter of another type each refuse `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-341-AC-6 | A `quire.op.state.clause` application at the body root of a node other than a `state`/`state_clause` node refuses `ill_typed`/`operator-ineligible` at that node; a `quire.op.state.clause` application nested inside a node body or inside another term fails the V2 schema and refuses `malformed_wire` at the nested application. | Test (TC-233) |
| FR-341-AC-7 | The published clause package's invariant and precondition are admitted by the state step; the state step reports anchor defects before clause defects, and runs after the frame step and before the operation step. | Test (TC-233) |
| FR-341-AC-8 | A `state`/`transition` node's body root is a `quire.op.state.transition` application; a `state`/`transition` node whose body root is another clause application fails the published schema. | Test (TC-233) |
| FR-341-AC-9 | The operation catalog holds `quire.op.state.clause` with operator class `state_clause`, operands `aggregate`, `object`, `boolean`, result `clause`, no laws, no mode and the `state_clause` member kind, and both V2 schemas enumerate the identity, the operator class and the member kind; each published state clause application is admitted by the operation step. | Test (TC-233) |
| FR-341-AC-10 | A `value`/`parameter` node's body is an `aggregate` of exactly two bindings, `name` (a `text` literal) and `level` (an `integer` literal), in that order, and every occurrence has role `expression`; a body with the bindings reordered, a non-text `name`, a non-integer `level`, a missing binding and an occurrence of another role each fail the published schema. | Test (TC-233) |
| FR-341-AC-11 | A process `Flow` with field `n` and operation `go(): Integer` admits an invariant anchored at the `model`/`process` node binding `[Reference<Flow>]`, a precondition of `go` binding `[Reference<Flow>]` and a postcondition of `go` binding `[Reference<Flow>, Integer]`, whose anchor is the `go` operation anchor of context `Flow`; a state machine `Gate` with operation `toggle()` admits an invariant anchored at the `model`/`state_machine` node binding `[Reference<Gate>]` and a postcondition of `toggle` binding `[Reference<Gate>]`; the `Reference<Flow>` and `Reference<Gate>` nodes are `composite_type`/`reference` nodes over the process and state-machine declaration nodes. | Test (TC-233, TC-280) |
| FR-341-AC-12 | A clause on a process or a state machine refuses as one on an object type does: an invariant anchored at an operation anchor, and a precondition anchored at the process node, refuse `invalid_model_binding`/`malformed-declaration` at the anchor's target; a `self` of `Reference<Other>` for another declaring type, a missing operation parameter and a missing result each refuse `ill_typed`/`operator-ineligible` at the clause node; an invariant anchored at a record value type, an event type, a persistence interface or a systems interface refuses `invalid_model_binding`/`malformed-declaration` at the anchor's target. | Test (TC-233, TC-280) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`,
  the operation catalog, the application-node preimage and the reader's
  refusal order this artifact adds the state step to.
- [FR-342](FR-342-checked-package-operation-anchor-body.md) owns the
  `state`/`operation_anchor` body a precondition or postcondition is
  anchored at, and the operation it resolves.
- [FR-340](FR-340-checked-package-frame-location.md) owns the
  `state`/`frame` body.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns
  invariants, preconditions and postcondition anchors as a model feature;
  `proposals/quire-v1/state-contract.md`'s "Operation anchors, aliases and
  captures" fixes which values each clause binds.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Accepted complete-V1 boundary value contract for the `state`/`state_clause`
node body and the `value`/`parameter` node body above. Implementation and
qualification remain tracked separately.
