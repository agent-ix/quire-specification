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
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-464
    type: references
---

# FR-341: Checked-package state clause body

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "state"` and
`semantic_form: "state_clause"`, the producer SHALL encode its `body` as one
application of `quire.op.state.clause`, the clause operation of an
invariant, a state constraint
([FR-382](../../functional/analysis/FR-382-select-state-space-reductions.md)),
a precondition or a postcondition, whose `state_clause` member
names the clause kind and whose three arguments are, in this fixed order,
the clause's parameter aggregate, its anchor and its Boolean condition. The
reader SHALL refuse a node whose joins below do not hold with the first
refusal in the order below.

A `quire.checked-semantic-graph/v2` node with `node_tag: "state"` and
`semantic_form: "transition"` is the checked claim that one operation
transitions one Boolean model field. The producer SHALL encode its `body` as
one application of `quire.op.state.transition` as "State transition body"
below fixes, and the reader SHALL refuse a node whose joins there do not
hold.

## Properties

- **body** — an `application` term with `operator: "state_clause"` and
  `operation.identity: "quire.op.state.clause"`, whose `operation.member` is
  `{kind: "state_clause", clause, execution_point?}` with `clause` one of `invariant`,
  `constraint`, `precondition` or `postcondition`, `laws` empty, `mode` `null` and
  `leaves` empty. Its `arguments` are exactly:
  1. **the parameter aggregate** — an `aggregate` whose members are
     `reference` terms to `value`/`parameter` nodes, one per value the
     clause binds, in binder-level order: `self` at level 0, then `result`
     when present, then the operation's parameters in declared order;
  2. **the anchor** — a `reference` to the node the clause is anchored at:
     the context's node for an invariant or a constraint (a `model`/`object_type`,
     `model`/`process` or `model`/`state_machine` node), and the
     operation's `state`/`operation_anchor` node
     ([FR-342](FR-342-checked-package-operation-anchor-body.md)) for a
     precondition or postcondition;
  3. **the condition** — the Boolean term the clause asserts.
- **execution point** — the optional `execution_point` member of the
  `state_clause` operation member. When present it is the accepted
  `ClauseBinding.execution_point`, in FR-322's closed `ExecutionPoint` shape.
  It is absent only when the checked clause has no execution point. Because
  the operation member is part of the application body, the point participates
  in the state-clause node's identity.
- **semantic_type** — the `scalar_type`/`boolean` node. The application's
  `result_type` names the same node: the catalog's `clause` result form
  declares a Boolean result, and FR-322's rule that a node whose body root
  is an application has `semantic_type` equal to its `result_type` then
  fixes the node's type.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node.
- **dependencies** — FR-322's application-node rule: exactly the unique
  node-id-ascending `reference` targets of the body, so the anchor, each
  parameter node and every node the condition references.

The parameters a clause binds follow the state contract's "Operation
anchors, aliases and captures" table:

| Clause | `self` | `result` | Operation parameters |
| --- | --- | --- | --- |
| `invariant` | `Reference<C>`, `C` the anchor type | absent | none |
| `constraint` | `Reference<C>`, `C` the anchor type | absent | none |
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
| member | `state_clause`: `{kind: "state_clause", clause: "invariant" \| "constraint" \| "precondition" \| "postcondition", execution_point?: ExecutionPoint}` |
| constraints | none |
| leaves | none |

The anchor fits the `object` position: the node of an object type, a process
or a state machine has family `object`, and an `operation_anchor` node's
`semantic_type` is its context node (FR-342), so a reference to it has family
`object` too. The
clause kind decides which clause an application. The optional execution point
is also part of the member and application identity; equal clause conditions
at different accepted points are distinct nodes.

## Node forms and body roots

- A `quire.op.state.clause` application is the body root of a
  `state`/`state_clause` node, and stands nowhere else.
- A `quire.op.state.transition` application is the body root of a
  `state`/`transition` node (see "State transition body"), and stands
  nowhere else.
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

## State transition body

A `state`/`transition` node states that an operation, from a pre-state its
precondition admits, leaves one Boolean model field in the state its
postcondition describes. It supports the Boolean model-field transition (for
example `Switch.set(flag: Boolean)` with the required Boolean field `on`); other field types and
other transition forms are outside this body and this version.

- **body** — an `application` term with `operator: "state_transition"` and
  `operation.identity: "quire.op.state.transition"`, whose `laws` are exactly
  one `protocol_profile` law equal to the lock's `profile_selections` row of
  that role (so a package carrying a transition selects the protocol profile),
  `member` `{kind: "profile_operator", operator: "transition"}`, `mode` `null`
  and `leaves` empty. The operator `transition` is the one state operator the
  finite-global profile defines
  ([FR-464](FR-464-checked-package-protocol-node-bodies.md)). Its `arguments`
  are exactly these five `binding` terms, in this order:

  | # | Binding | Value |
  | --- | --- | --- |
  | 1 | `anchor` | a `reference` to the `state`/`operation_anchor` node ([FR-342](FR-342-checked-package-operation-anchor-body.md)) of the operation that effects the transition |
  | 2 | `subject` | a `reference` to the `expression`/`query` node, a `quire.op.record.project` application with member `{kind: "field", name, declaration}` over a `quire.op.model.deref` application of the `self` `value`/`parameter` node, that reads the transitioned Boolean field of the anchor's context |
  | 3 | `inputs` | an ordered list: a Group `aggregate` of `reference` terms to the `value`/`parameter` nodes of the operation's parameters, in declared order, empty when the operation has none |
  | 4 | `pre` | optional: a Group `aggregate` of no member or of one `reference` to the `state`/`state_clause` precondition of the operation |
  | 5 | `post` | a `reference` to the `state`/`state_clause` postcondition of the operation |

- **subject binding** — the subject names the model field by the field read
  itself, so its `member.declaration` and `member.name` are the field's
  declaring node and name, resolved by FR-322's "Model-owned members" among the
  exposed effective members of the anchor's context. The field is a
  required-presence, single-valued Boolean field (FR-322 "Model-owned members"
  step 4 gives it the type `scalar_type`/`boolean`): the subject node's
  `semantic_type` is the `scalar_type`/`boolean` node. An optional Boolean
  field has type `Option<Boolean>` (FR-322-AC-31) and a multi-valued one a
  collection type, so neither is a transition subject: a subject node of
  another type refuses `ill_typed`/`operator-ineligible` at the transition
  node, and a projection that declares `Boolean` for such a field refuses the
  same way at the projection (the operation step). The
  anchor's frame ([FR-340](FR-340-checked-package-frame-location.md))
  `modifies` the field, with a `field` entry of that declaring node and name.
  A read of the subject inside the `pre` clause denotes the pre-state value of
  the field and inside the `post` clause its post-state value, as the clause
  position fixes ("Operation anchors, aliases and captures"); the transition
  states no third state.
- **inputs and result** — each input is a Boolean parameter; the transition's
  pre-state Boolean inputs are the Boolean terms of the `pre` clause over the
  subject and the inputs, and its post-state Boolean result is the term of the
  `post` clause. The application's `result_type` is the `scalar_type`/`boolean`
  node and equals the node's `semantic_type`: the claim is a Boolean truth in
  the sense of the catalog's `clause` result form, true when the operation
  carries a state satisfying `pre` (when present) to one satisfying `post`.
- **clause joins** — `pre` and `post` each target a `state_clause` node whose
  anchor is an `operation_anchor` of the same context and operation name as the
  transition's anchor. An execution-point-bearing clause names its own
  point-bound anchor ([FR-342](FR-342-checked-package-operation-anchor-body.md)),
  so the clauses' anchors may differ from the transition's anchor and from each
  other by the execution point only. The transition's `anchor` is the no-point
  anchor of the operation, as an `attempt`'s is (FR-464-AC-17), whether or not
  its clauses carry points; when only one of `pre` and `post` is present and
  carries a point, the no-point anchor still stands for the transition. A
  context-and-operation pair with no no-point anchor in the package therefore
  has no transition.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node.
- **dependencies** — FR-322's application-node rule: the unique
  node-id-ascending `reference` targets of the body, so the anchor, subject,
  each input, the `pre` clause when present and the `post` clause.
- **identity** — the node is an application with no `declaration`, so its
  `node_id` is the structural path of FR-322's derived-node form: the
  operator, the operation identity, the profile law's definition, the member
  operator and the five bindings (every referenced node's `node_id`, therefore
  the anchor's context, operation name and execution point, the subject's
  field and the two clauses' conditions). It holds no source region,
  occurrence or source-map entry: re-spelling the source, moving a region or
  adding an occurrence leaves the node id unchanged, while changing the
  field, the operation, an input, `pre`, `post` or the profile selection
  yields a different node.

Placement. A `quire.op.state.transition` application is the body root of a
`state`/`transition` node and nowhere else. The state step checks the
placement of every such application first, in ascending node-id order,
before the anchor checks: an application at the body root of a node that is not
a `state`/`transition` node refuses `ill_typed`/`operator-ineligible` at that
node; an application nested inside a node body or another term is outside
FR-322's body grammar and refuses `malformed_wire` at the nested application at
strict wire validation; and a `state`/`transition` node whose body root is not a
`quire.op.state.transition` application fails the published schema.

Reader order for transition nodes: the state step checks every
`state`/`transition` node after every `state`/`state_clause` node, in
ascending node-id order, and the first failing check is its defect:

1. **Anchor.** The `anchor` target is a declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else
   `missing_declaration`/`missing-name` for a target that names no node, and a `state`/`operation_anchor` node,
   else `invalid_model_binding`/`malformed-declaration`. The locus is the
   target.
2. **Subject.** The `subject` target is a declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else
   `missing_declaration`/`missing-name` for a target that names no node, with the shape above (a field
   projection of a dereferenced `self` whose `member.declaration` is the
   declaring node of a field of the anchor's context and whose `self` is of
   type `Reference<C>` for the anchor's context `C`) and a field the anchor's
   frame `modifies`, else `invalid_model_binding`/`malformed-declaration`. A
   subject whose `semantic_type` is not the `scalar_type`/`boolean` node
   refuses `ill_typed`/`operator-ineligible` at the transition node. The
   locus is the subject's target, except for the type refusal.
3. **Inputs.** Each `inputs` target is a declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else
   `missing_declaration`/`missing-name` for a target that names no node, and a `value`/`parameter` node, else
   `invalid_model_binding`/`malformed-declaration`; the list equals the
   resolved operation's parameters in declared order, each Boolean, else
   `ill_typed`/`operator-ineligible` at the transition node.
4. **Clauses.** Each of `pre` and `post` targets a declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else
   `missing_declaration`/`missing-name` for a target that names no node, that is a `state`/`state_clause` node
   of clause `precondition` and `postcondition` respectively and satisfies
   "clause joins", else `invalid_model_binding`/`malformed-declaration` at its
   target.
5. **Uniqueness.** No two transition nodes join to one (`anchor`, `subject`)
   pair; the second in ascending node-id order refuses
   `ambiguous_declaration`/`ambiguous-name`.

The profile law, the member operator, the `result_type` and the argument term
families are the application's catalog checks at the operation step, in
FR-322's order ([FR-322-AC-22](FR-322-checked-package-artifact.md)): a missing
or non-protocol law, an undefined operator and a non-Boolean `result_type`
refuse as there.

### Stage boundary for a mismatched profile operator

FR-322's reader order applies the published V2 schema before any step. The
schema pins the member operator of a `quire.op.state.transition` application
to `transition` and enumerates the member operators of a
`quire.op.protocol.control` application without `transition`. A complete
reader therefore refuses these inputs at the schema stage and never reaches
the operation step for them:

- a `quire.op.state.transition` application naming any other operator, and a
  `quire.op.protocol.control` application naming `transition`, refuse
  `malformed_wire` at the application's `operation/member/operator`;
- the operation step still defines `operation-member-mismatch` for the same
  inputs, as the catalog's member check, and a component that runs that step
  alone on an unvalidated document (an operation oracle, a partial reader)
  SHALL refuse them with it. A complete reader never reports it for these
  pairs, because the schema stage precedes it. The schema is deliberately not
  widened: a wider enum would admit a state operator into a protocol node
  and leave the refusal to a later stage for no gain.

`operation-member-mismatch` stays reachable through a complete reader for a
member the schema admits and the lock's profile does not define.

### Several postconditions and preconditions on one operation

A `state`/`transition` node is a Boolean model-field claim over one
(`anchor`, `subject`) pair. It is not the summary of an operation's
postconditions or of its preconditions. An operation anchor may have any
number of `state_clause` nodes of clause `postcondition` and of clause
`precondition` (QSL FR-105-AC-4 adds a second post as one more node on the
same anchor; QSL allows several preconditions, evaluated as a conjunction),
each checked as FR-341 states, and an operation's postconditions (and,
likewise, its preconditions) are the conjunction of its clauses of that kind
whether or not any transition names one of them. A transition's `post` binds
exactly one postcondition and its `pre` at most one precondition, the ones the
producer states for its subject. The producer SHALL emit at most one
transition per (`anchor`, `subject`). A consumer SHALL NOT read a
transition's `post` as the operation's complete postcondition, SHALL NOT read
its `pre` as the operation's complete precondition, and SHALL NOT infer from a
transition that no further postcondition or precondition exists. No
conjunction node is synthesized, so no new canonical order or identity exists
to rule, and the single `post` binding, the at-most-one `pre` and the uniqueness rule are unchanged.

One producer option, not a requirement of this specification: where several
clauses of one kind constrain one subject, the producer that wants a
transition for it states one clause for that subject (the source's own
`post A and B`) and binds that, and the others remain plain clause nodes.
Which option QSL takes is QSL-owned and OPEN (QSL-1163, the first transition
emitter, escalation e38). QSL FR-105 and ADR-012 section 15.4 currently say
QSL emits no `state`/`transition` node from a state clause, so the QSL-authored
part of IR-1052 AC-7 is not closed by this specification.

## Reader order

The state step runs after the frame step (FR-340) and before any operation
refusal ([FR-322](FR-322-checked-package-artifact.md)), after every
graph-shape and declaration refusal. Within it, the
reader first checks the placement of every `quire.op.state.clause`
application, in ascending node-id order; then every
`state`/`operation_anchor` node (FR-342), in ascending node-id
order; then every `state`/`state_clause` node, in ascending node-id
order; then every `state`/`transition` node, in ascending node-id order
("State transition body"). It reports the first defect. For one clause node, the checks run in
this order and the first failing check is its defect:

1. **Anchor.** The anchor's target is a declared dependency of the clause
   node (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else `missing_declaration`/`missing-name` for a target that names no node; for an invariant or a
   constraint it is a `model`/`object_type`, `model`/`process` or
   `model`/`state_machine` node and
   for a precondition or postcondition a `state`/`operation_anchor` node, else
   `invalid_model_binding`/`malformed-declaration`. The locus is the anchor's
   target.
2. **Parameters.** Each parameter reference, in aggregate order, targets a
   declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-322-AC-96, before this step), else `missing_declaration`/`missing-name` for a target that names no node, that is
   a `value`/`parameter` node, else `invalid_model_binding`/
   `malformed-declaration`. The locus is the offending target.
3. **Signature.** The parameter count and each parameter's type equal the
   table above, else `ill_typed`/`operator-ineligible` at the clause node.
   `self` is checked first, then `result`, then the operation's parameters
   in order. A constraint is checked as an invariant.
4. **Execution point.** When present, the point has FR-322's exact closed
   shape. `invariant` and `constraint` take `initialization{name}` or
   `handler{name}`; `precondition` and `postcondition` take `pre{operation}`
   or `post{operation}`. The name equals the checked source anchor, and an
   operation clause's point equals its `operation_anchor` point. A malformed,
   mismatched or inapplicable point refuses
   `invalid_model_binding`/`malformed-declaration` at the clause node.

The Boolean condition, the operand families and the `result_type` are the
application's catalog checks, reported at the operation step in FR-322's
order: a condition of a non-Boolean family and a `result_type` naming a
non-Boolean node each refuse `ill_typed`/`operator-ineligible`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-341-AC-1 | A `state`/`state_clause` node's body is a `quire.op.state.clause` application with operator class `state_clause`, a `{kind: "state_clause", clause}` member whose `clause` is `invariant`, `constraint`, `precondition` or `postcondition`, and three arguments: an `aggregate` of one or more `reference` terms, a `reference`, and the condition. A body that is another clause application, names another identity or operator class, carries another member kind or clause value, or whose first or second argument has another term shape fails the published schema. | Test (TC-233) |
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
| FR-341-AC-13 | The published clause package's invariant node with its kind member's `clause` rewritten to `constraint`, rekeyed, validates against both V2 schemas and is admitted by the state step, as a constraint is checked as an invariant; the same constraint anchored at an operation anchor refuses `invalid_model_binding`/`malformed-declaration` at the anchor's target, and one binding a second parameter refuses `ill_typed`/`operator-ineligible` at the clause node. An invariant and a constraint with equal bodies have different node ids, because the kind member is part of the body. | Test (TC-233) |
| FR-341-AC-14 | The state-clause member admits each closed `execution_point` variant: initialization/handler on an invariant, pre/post on an operation clause; it refuses an unknown variant, extra member, missing anchor name, an inapplicable point, or a name that disagrees with the checked source/operation anchor. Equal clause bodies with different accepted points have different node identities and package ids; the same point round-trips unchanged. | Test (TC-233) |
| FR-341-AC-15 | A `state`/`transition` node's body is a `quire.op.state.transition` application with operator class `state_transition`, one `protocol_profile` law, `member` `{kind: "profile_operator", operator: "transition"}`, `null` mode, empty leaves, a Boolean `result_type`, and exactly the five bindings `anchor`, `subject`, `inputs`, `pre`, `post` in that order, where `anchor`, `subject` and `post` are references, `inputs` an aggregate of references and `pre` an aggregate of at most one reference; every occurrence has role `claim`. A body with the bindings reordered, one missing or added, `pre` holding two references, an aggregate `subject`, another member operator, another law role, a mode, or an occurrence of another role fails the published schema (`malformed_wire`); the operation step, run alone on a document the schema was not applied to, refuses another profile operator as `operation-member-mismatch` (FR-341-AC-19). | Test (TC-233) |
| FR-341-AC-16 | The state step refuses a transition node with the first defect in this order, each at its stated locus: a transition application at the root of another node (`ill_typed`/`operator-ineligible` at that node); an `anchor` that is not a declared `operation_anchor` dependency; a `subject` that is not a declared field projection of a dereferenced `Reference<C>` `self` of the anchor's context, or whose field the anchor's frame does not modify (`invalid_model_binding`/`malformed-declaration`), or whose type is not the required single-valued Boolean node, such as an optional Boolean field typed `Option<Boolean>` or an Integer field (`ill_typed`/`operator-ineligible` at the node); an `inputs` list that is not the operation's Boolean parameters in declared order; a `pre` or `post` that is not the operation's precondition or postcondition state clause, including one anchored at another context or operation; and a second transition over the same `anchor` and `subject` (`ambiguous_declaration`/`ambiguous-name`). | Analysis |
| FR-341-AC-17 | The published `positive-state-transition.json` package, whose transition is `Switch.set(flag: Boolean)` over the required Boolean field `on` of `acme/orders` with precondition `flag || on` and postcondition `on`, validates against `schema.json`, is admitted by the operation step for every application, the subject projection included. Rewriting the anchor's operation name changes the ids of the anchor, both clauses and the transition; rewriting the subject's field, `pre`, `post` or the profile selection changes the transition id; moving a source-map region leaves every node id unchanged. | Test (TC-233) |
| FR-341-AC-18 | A `quire.op.state.transition` application nested inside a node body or another term fails the V2 schema (`malformed_wire` at the nested application), and a `state`/`transition` node whose body root is another application or term fails the V2 schema; the state step refuses a transition application at the root of any other node as `ill_typed`/`operator-ineligible` at that node, before any anchor check. | Test (TC-233) |
| FR-341-AC-19 | For the published `positive-clause-operations.json` and `positive-state-transition.json` packages, a `quire.op.protocol.control` application naming operator `transition` fails the published schema and a complete reader refuses it `malformed_wire` at the application's `operation/member/operator`; a `quire.op.state.transition` application naming `sequence` fails the schema likewise; the operation step run alone on either edited document refuses `invalid_package`/`operation-member-mismatch` at the application. The schema is not widened. | Test (TC-233) |
| FR-341-AC-20 | A package whose operation anchor has two `postcondition` `state_clause` nodes and one transition binding either one as `post` is admitted, and likewise for two `precondition` nodes and `pre`; a transition's `post` and `pre` do not stand for the operation's complete postcondition or precondition, and two transitions over one (`anchor`, `subject`) refuse `ambiguous_declaration`/`ambiguous-name` whichever clauses each binds. How QSL's first transition emitter chooses its clause (for example one source-stated `post A and B`) is QSL-owned and OPEN (QSL-1163, e38); this criterion does not close the QSL-authored part of IR-1052 AC-7. | Analysis |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`,
  the operation catalog, the application-node dependency rule and the reader's
  refusal order this artifact adds the state step to.
- [FR-342](FR-342-checked-package-operation-anchor-body.md) owns the
  `state`/`operation_anchor` body a precondition or postcondition is
  anchored at, and the operation it resolves.
- [FR-340](FR-340-checked-package-frame-location.md) owns the
  `state`/`frame` body.
- [FR-464](FR-464-checked-package-protocol-node-bodies.md) owns the
  protocol profile's operator catalog, which names the one `transition`
  operator a state transition applies.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns
  invariants, preconditions and postcondition anchors as a model feature,
  and [FR-382](../../functional/analysis/FR-382-select-state-space-reductions.md)
  owns the state constraint;
  `proposals/quire-v1/state-contract.md`'s "Operation anchors, aliases and
  captures" fixes which values each clause binds.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Accepted complete-V1 boundary value contract for the `state`/`state_clause`
node body, the `value`/`parameter` node body and the Boolean model-field
`state`/`transition` node body above.
