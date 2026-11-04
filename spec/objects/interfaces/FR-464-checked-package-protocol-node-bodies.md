---
id: FR-464
title: "Checked-package protocol node family: operation, keys, clause body, binders and reader order"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-466
    type: references
  - target: ix://agent-ix/quire-specification/FR-326
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/FR-228
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-464: Checked-package protocol node family: operation, keys, clause body, binders and reader order

## Description

When a checked package carries a protocol declaration, the producer SHALL
emit it as `quire.checked-semantic-graph/v2` nodes with `node_tag:
"protocol"`: one `protocol_clause` node for the declaration and one member
node for each of its roles, channels, queues, obligations, controls and
compensations
([FR-465](FR-465-checked-package-protocol-declaration-bodies.md),
[FR-466](FR-466-checked-package-protocol-control-bodies.md)). Every such body
is one application of `quire.op.protocol.control`, whose
`profile_operator` member names the construct. The nodes are the only
carrier of the protocol: a consumer reads the protocol from them and from
nothing else, and no separate `quire.checked-protocol/v1` document exists
([FR-326](FR-326-checked-protocol-artifact.md)). The reader SHALL refuse a
package whose protocol nodes do not satisfy this requirement, FR-465 and
FR-466, with the first refusal in the order below.

## Properties

- **body** — an `application` term with `operator: "protocol_control"` and
  `operation.identity: "quire.op.protocol.control"`, whose `laws` are exactly
  one `protocol_profile` law equal to the lock's `profile_selections` row of
  that role, `member` `{kind: "profile_operator", operator}` with `operator`
  one of the catalogued operators below, `mode` `null` and `leaves` empty.
  Its `arguments` are the fixed `binding` terms the operator's layout lists,
  in that order and with those names, each a Member of FR-322's body
  grammar. No application, aggregate or binding is nested beyond that
  grammar: every composite subterm is its own node, reached by `reference`.
- **semantic_type** — the `scalar_type`/`boolean` node; the application's
  `result_type` names the same node. A protocol node declares a Boolean
  truth in the sense of FR-322's `clause` result form, exactly as a
  `state_clause` and a `temporal_clause` do.
- **declaration** — the node's `declaration` member, `{qualified_name}`, as
  the catalog below fixes it, present exactly when an occurrence has role
  `declaration` (FR-322).
- **occurrences** — every occurrence has role `declaration`, one per
  declaring construct in the source, except a `queue` node, whose occurrences
  have role `generated`.
- **owner_model** — required on a `protocol_clause` node and absent on every
  other protocol node. It is a local `NodeRef` to the object-type node of the
  model that the source clause is declared over, which is the object type of
  its `over` input. That model is owned by a *selected declaration*: a
  declaration of a model package named by a `model_root` node, whose
  declaration node id is that package's node id for it (FR-322). A
  source-local declaration (a node with a source `owner`, such as
  `Config::ConfigVersion` of the published fixtures) is declared by no selected
  package and is never a valid target. The producer sets the edge from the
  clause's `over` object type; the edge is then authoritative for
  clause-to-model ownership, and a consumer SHALL NOT infer ownership from, or
  compare the edge with, the `over` parameter's type. It enters the node's
  exact dependency set. This edge continues to name the owner model's object-type
  node; it is not replaced by FR-322's canonical selected-package
  `model_root`. A refinement's `abstract`/`semantic_type` headers use the
  model root to identify a whole selected package, while protocol class
  ownership is joined through this exact declaration edge and its selected
  package.
- **dependencies** — exactly the unique node-id-ascending local `NodeRef`s
  reached by the body plus `owner_model` when present.
- **node id** — FR-322's structural `node_id` `{package, path, ordinal}`
  for a protocol node, where `path` is the node's declaration path or, for a
  derived node outside a recursion group, FR-322's derived form; `owner_model` is carried only by a
  protocol clause. Under Option A below, a producer writes no `recursion_group` for a
  protocol node outside a cyclic reference component (a label there is admitted
  and means nothing, FR-322). Every node in a cyclic
  component, including nodes with another `node_tag`, uses FR-322's
  recursion-group encoding: one explicit shared `recursion_group`, and each
  reference to a member of that same component is an ordinary `reference` to
  the member's `node_id` on the checked wire (spelled `group_reference` in the
  written form). A `protocol_reference` carrier that closes the cycle is a
  member and carries the label; an absent or differing label on any member
  refuses at FR-322's cycle check (FR-322-AC-285). The id of a node outside every
  recursion group is a function of its declaration path and, for a derived
  node, of the ids of the nodes it references, and of nothing in the source
  text but those. A recursion-group member keeps its declaration path
  when it has a declaration, so a cyclic component of several declared nodes
  (a protocol, its predicate-join function, a holder) carries each node's own
  declaration path; an anonymous derived member carries the path of the group's
  declared member where there is exactly one (FR-322). No member's path is
  operand-derived, and nodes sharing a path are told apart by the producer-assigned
  `node_id.ordinal`.
  Reformatting the source, adding a comment or a second reference to a
  protocol changes no node id. Changing any name, member, reference or
  literal, or the order of an ordered list, yields a different (structurally
  unequal) node
  ([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)
  AC-4), and changes the id of each derived node outside every recursion
  group whose operand ids it changes; a recursion-group member keeps its
  producer-assigned id across such an edit (FR-322, "Structural node
  identity").
- **operation anchors** — an `attempt` resolves its operation through the
  `state`/`operation_anchor` of that operation (FR-342). A protocol-only
  anchor has no `execution_point`; if a state clause also names the operation
  with an accepted execution point, that clause-bound anchor has its own
  point-bound identity and the attempt continues to reference the no-point
  protocol anchor. The two anchors may share the same frame.

## Operator catalog

The protocol profile the lock selects (`protocol_profile`) defines exactly
these operators and the one state operator below. Each operator belongs to one `semantic_form`, and the form
fixes which operator a node's body root may apply. The argument layouts are
in FR-464 for the clause, FR-465 for the declarations and FR-466 for the
controls and the compensation.

| Operator | `semantic_form` | `declaration.qualified_name` | Layout |
| --- | --- | --- | --- |
| `protocol` | `protocol_clause` | the protocol's own qualified name `P` | "Clause body" |
| `role` | `role` | `P` followed by `role`, the role's name | FR-465 |
| `channel` | `channel` | `P` followed by `channel`, the channel's name | FR-465 |
| `queue` | `queue` | none | FR-465 |
| `finish` | `obligation` | `P` followed by `finish`, the finish node's name | FR-465 |
| `terminal` | `obligation` | `P` followed by `terminal` | FR-465 |
| `sequence`, `choice`, `parallel`, `repeat`, `await`, `check`, `send`, `receive`, `attempt`, `effect`, `event`, `commit`, `fence` | `control` | `P` followed by `run` and the control names from the root control down to this control | FR-466 |
| `compensate` | `compensation` | `P` followed by `compensate`, the template's name | FR-466 |

The profile also defines one operator for the `state`/`transition` node
form of [FR-341](FR-341-checked-package-state-clause-body.md) "State
transition body". It is applied by `quire.op.state.transition` only, never by
`quire.op.protocol.control`, and no operator of the table above is applied by
`quire.op.state.transition`:

| State operator | `semantic_form` | `declaration.qualified_name` | Layout |
| --- | --- | --- | --- |
| `transition` | `transition` | none | FR-341 |

The word after `P` is a fixed segment, so a role, a channel, a template and
a control of equal names have distinct declarations, and a name a
declaration segment cannot spell is not a protocol name. Two protocol nodes
with equal `declaration.qualified_name` refuse `ambiguous_declaration`/
`ambiguous-name`, as FR-322 refuses every pair of nodes with equal names,
naming both node ids in ascending node-id order.

A control's path is its parent's path followed by its own name. A branch
label, a case label and an `await`'s `then` and `timeout` positions add no
segment: the control a branch, case, `then` or `timeout` position holds is
named by its own name below the `parallel`, `choice` or `await` that
holds it.

## Term shapes

The layouts use these shapes of FR-322's body grammar. A **reference**
names a node of the stated tag and form by node id.

| Shape | Term |
| --- | --- |
| **text** | a `literal` with `value_kind: "text"`, whose `type` is the `scalar_type`/`text` node |
| **integer** | a `literal` with `value_kind: "integer"`, whose `type` is a `scalar_type` of family `integer` |
| **Boolean term** | a `literal` with `value_kind: "boolean"`, or a `reference` to a node whose `semantic_type` is the `scalar_type`/`boolean` node |
| **term** | any Leaf |
| **ordered list** | a Group `aggregate` whose members are `reference` terms in an order that carries meaning: execution order |
| **set** | a Group `aggregate` whose members are `reference` terms in strictly ascending node-id order; the list means a set, so one set has one encoding |
| **optional** | a Group `aggregate` with no member (absent) or exactly one member (present) |
| **interval** | a Group `aggregate` of the bindings `lower` and `upper`, two non-negative integers with `lower <= upper`; the upper bound is always present |
| **ordered records** | a Tuple `aggregate` of Groups, one Group per entry, in an order that carries meaning (capture evaluation order) |
| **sorted records** | a Tuple `aggregate` of Groups, one Group per entry, in strictly ascending order of the entry's sort key, which its layout states |

An enumerated text value (`origin`, `each`, `adversarial` and the values
FR-465 and FR-466 list) has the spellings its layout lists: another spelling
refuses `malformed_wire` at the schema stage, and the recorded locus is the node pointer `/semantic_graph/nodes/{i}` (FR-464-AC-24).

**Canonical order.** A set or a sorted-records list that is not in its
canonical order, or that repeats an entry, refuses `invalid_semantic_graph`
at the node, as FR-322-AC-10 refuses every noncanonical order. An ordered
list is in the order its layout states, and reordering it changes the node
key. So the sets of one protocol (roles, relationships, channels, queues,
requirements, compensations, `visible`, `contracts`, `related`, cases, branches)
have one encoding whatever order the source declares them in, and only the
lists where order is meaning (a sequence's children, captures)
distinguish two protocols by order.

## Clause body

A `protocol`/`protocol_clause` node's body applies the operator `protocol`
with thirteen arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `over` | a reference to the `value`/`parameter` node of the `over` parameter |
| 2 | `activation` | a Group: `kind`, a text `origin` or `each`; `trigger`, a reference to the trigger's `value`/`parameter` node for `each` and absent for `origin`; `guard`, a Boolean term for `each` with a guard and absent otherwise |
| 3 | `captures` | ordered records, one Group per capture, in evaluation order, `binder` (a reference to its `value`/`parameter` node) and `initializer` (a term) |
| 4 | `roles` | a set of `protocol`/`role` nodes |
| 5 | `relationships` | a set of `relation`/`relationship` nodes: the model relationship each `relationship` declaration of the protocol names; a target of another tag or form refuses `invalid_model_binding`/`wrong-export` |
| 6 | `channels` | a set of `protocol`/`channel` nodes |
| 7 | `queues` | a set of `protocol`/`queue` nodes, one per channel; the queues are generated and have no source order, so the set is ascending by each queue's own node id like every other set |
| 8 | `requirements` | a set of `temporal`/`temporal_clause` nodes: the temporal clauses the protocol requires |
| 9 | `compensations` | a set of `protocol`/`compensation` nodes |
| 10 | `terminal` | optional: a reference to the protocol's `terminal` `obligation` node |
| 11 | `scheduling` | optional: the text `adversarial` |
| 12 | `run` | a reference to the root `protocol`/`control` node |
| 13 | `finish` | a reference to the protocol's `finish` `obligation` node |

The clause's model owner is the separate node-level `owner_model` edge above;
it is not a fourteenth body argument. The edge targets the object-type node of the model
declaration that the clause's `over` input's object type names, a model owned
by a selected declaration. The reader checks that the target is such a
selected model declaration node and that it is an exact dependency of the
clause; it does not compare the target with the `over` parameter's type. FR-375 uses this identity-bound edge to decide
which concrete clauses contribute classes to a refinement catalog.

A relationship declaration has no node of its own: its name is a source
alias, and the `related by` annotations of FR-466 name the model
relationship node directly. The alias is not carried. A protocol with no
relationship, no required temporal clause, no compensation or no
`terminal` member has an empty set or an absent optional there. A protocol
without `scheduling adversarial` and one with it are two different
packages.

The `protocol_clause` node's occurrences have role `declaration`, at the
`protocol` declaration. Its `dependencies` are the nodes its body references
plus `owner_model`, so the clause depends on its exact owner model, its roles,
channels, queues,
requirements, compensations, `terminal` and `finish` nodes, its root control
and the `value`/`parameter` and `expression` nodes of its first three
arguments. A `requirements` entry whose temporal clause reaches the same
protocol through a `protocol_reference` carrier closes a dependency cycle
(clause, temporal clause, carrier, clause), which FR-322 refuses like every
ungrouped cycle.

## Activation relation

The control-to-temporal activation relation of a protocol states which
activation of the protocol requires which temporal clause. A consumer SHALL
derive it from the protocol's admitted `protocol_clause` node, from its
`activation` and `requirements` arguments and from nothing else:

- The relation holds one row for each `temporal`/`temporal_clause` node in
  the `requirements` set, in that set's order (ascending node-id).
  A row is the clause node's key, the activation (`origin`, or `each` with
  the key of its `trigger` node) and the temporal clause node's key.
- A protocol whose `requirements` set is empty has no row. A temporal
  clause named by two `requires temporal` members is one `requirements`
  entry and so one row.
- No separate artifact, document or table carries the relation. A control
  position, a temporal clause position, a name, a matching profile or clock,
  a timestamp or an observation adds, removes or substitutes no row, and a
  temporal clause the protocol does not require gives no row whatever it
  shares with the protocol.
- Two admitted packages that are structurally equal give equal relations; a
  change that leaves the clause node unchanged leaves the relation
  unchanged.
- The relation exists only for an admitted package. A `requirements` entry
  or an `each` `trigger` whose target is a node absent from the clause's
  `dependencies` refuses `invalid_semantic_graph` at the clause's
  `/dependencies` (graph stage, FR-322-AC-96, before the protocol step), and
  one that names no node refuses `missing_declaration`/`missing-name` at the
  target ("Membership and containment" and AC-6), and the reader grants no relation,
  partial or whole, for a package it refuses.
- The relation selects no truth and changes no temporal clause: the required
  clause keeps its own body, activation and evaluation
  ([FR-370](FR-370-checked-package-temporal-clause-body.md)).
- An obligation established through a row is bound to it only by an
  [FR-479](../protocol/FR-479-obligation-activation-binding.md) activation
  binding.

## Membership and containment

Every protocol member node belongs to exactly one clause.

- **Listing.** A role, channel, queue, compensation or obligation node
  belongs to the clause whose body lists it; a control node belongs to the
  clause whose `run` argument roots the control tree that holds it.
- **Declaration prefix.** A member node that has a `declaration` has
  the owning clause's qualified name as a prefix of its own, followed by the
  fixed segment its operator requires.
- **Control tree.** The root control and the controls it holds form a tree
  under *containment*: `sequence` holds its children, `choice` its case
  bodies, `parallel` its branch bodies, `repeat` its body and `exhausted`
  control, `await` its awaited `event` node and its `then` and `timeout`
  controls. A control is held by exactly one position. A control's
  declaration is the holder's declaration followed by the control's own
  name, and the root control's is `P`, `run`, its name. A control whose
  declaration is not that path, a control held twice and a control held by
  nothing refuse `invalid_semantic_graph` at the control node. A
  reference a control makes to another control that is not containment (an
  `await`'s anchor, a `receive`'s send, an `effect`'s attempt, an
  `event`'s compensation, a compensation's forward effect and commit) does
  not hold it.
- **Single listing.** A role, channel, queue, compensation or obligation
  node named by two arguments of one clause, or by two clauses, refuses
  `invalid_semantic_graph` at the node. This rule is about declarations: a
  node several nodes share by key (a `visible` entry, a contract, a model
  relationship, a binder) may be named by many.
- **Channels and queues.** Each channel is named by exactly one queue's
  `channel` argument, and each queue names a channel of the same clause.
- **Unattached nodes.** A protocol member node that no clause owns refuses
  `invalid_semantic_graph` at the node.

A reference that must name a node of a stated form and names another
refuses `invalid_model_binding`/`malformed-declaration`, located at the
target. A reference whose target is a node of the graph that is not among the
referencing node's `dependencies` is a graph-stage defect: it refuses
`invalid_semantic_graph` at the referencing node's `/semantic_graph/nodes/{i}/dependencies` (graph stage, FR-322-AC-96, before the protocol step), as for every other node form (FR-322 `dependencies`), and no
protocol step runs. A reference that names no node of the graph refuses
`missing_declaration`/`missing-name`, located at the target. The same graph-stage join precedes FR-341's state clause steps (FR-341
"Reader order").

## Binders

A binder is a `value`/`parameter` node whose body and occurrences are
[FR-341](FR-341-checked-package-state-clause-body.md)'s: an `aggregate` of
the bindings `name`, a text, and `level`, an integer; every occurrence has
role `expression`. Its `semantic_type` is the type the source declares for
the binder.

**Sharing.** A binder node is shared by every place whose name, level and type
are equal. A read of a binder at a node therefore denotes the *introducing
construct* readable at that node: the construct that introduced a binder
of that node's key and that is in the node's readable set. Two such
constructs readable at one node cannot occur, because two readable binders of
one name refuse `ambiguous_declaration`/`ambiguous-name`. Eligibility, the
owner, `foreign` reads and levels are decided for that introducing
construct.

**Level.** A construct that introduces binders has an *entry count*: the
number of binders readable at its entry (`0` for the clause; `size(C0)` for a
role, a channel and a compensation template; `size(In(n))` for a control; the
binders readable after the root control for `finish`). It introduces its
binders in the order its layout lists them, at consecutive levels from the
entry count. So `over` is `0`, the `each` trigger `1`, the first capture the
count of `over` and the trigger, and each later capture the next level; a
`send`'s binder is `size(In(send))`. Two binders of one name readable at one
place refuse `ambiguous_declaration`/`ambiguous-name` at the node that
introduces the second. A binder whose level is not its entry count refuses
`invalid_semantic_graph` at the binder.

**The `over` input is model state.** The protocol input `over` denotes the
model observation at the protocol's evaluation point. A slot that bars model
state (below) refuses a read of `over` with `ill_typed`/`operator-ineligible`
at the read, naming the slot. This refusal precedes the stage table: the
table's `missing_declaration`/`missing-name` applies to binders and captures
only. A protocol capture, a registration capture and an activation capture
may read `over` (their initializers may read model state), as may an event
constraint, a `related` end and the `finish` constraint.

**Readable binders.** Each expression a protocol node references is read at a
*stage* and may read only the binders and captures that stage lists. The
clause-level set `C0` is `over`, the activation binder (the `each` trigger)
when present, and every protocol capture binder.

| Stage | Where | Binders and captures it may read |
| --- | --- | --- |
| `protocol-activation` | `activation.guard` | the activation binder |
| `protocol-capture` | a clause capture `initializer` | `over`, the activation binder and the protocol captures before it |
| `role-lifetime` | a replicated role's `until` | `C0` and the role's binder |
| `channel-key` | a FIFO channel's `key` | the channel's message binder |
| `run` | a slot in the `run` tree | FR-466 "Binder scope" |
| `finish` | `finish.constraint` | the binders readable after the root control, and the finish binder |
| `terminal` | `terminal.condition` | `over`, and the protocol captures when the activation is `origin`; under `each` no capture and no binder |
| `registration-capture`, `trigger-guard`, `activation-capture`, `retry`, `recovery` | the slots of a compensation | FR-466 "Compensation scope" |

Every `run` slot and `finish` may also read the protocol captures through
`C0`. A read outside the stage's list refuses `missing_declaration`/
`missing-name`, located at the node whose slot holds the expression,
retaining `name`, `stage` (the identifier above), `owner` (the clause or the
template) and `read`: `forward` for a capture of the same stage written
later than the read or the capture itself, `foreign` for a binder a template
declares, read outside that template, and `stage` for any other binder or
capture that exists but that the stage may not read. What an expression
reads is found over its dependency closure: the `value`/`parameter` nodes
reachable from the expression node through `dependencies`, stopping at a
`value`/`parameter` node and at a `function` node.

**Slots that bar model state.** The activation guard, the trigger guard, the
retry relation, the recovery predicate, a choice or repeat guard, a `visible`
entry, a `check` condition, a FIFO `key` read binders, captures and constants, never model state. They refuse,
at the read and naming the slot, `ill_typed`/`operator-ineligible` when the
closure holds an application of `quire.op.state.pre`, `quire.op.model.deref`,
`quire.op.model.navigate`, `quire.op.model.reaches`,
`quire.op.model.reaches_field`, `quire.op.model.lookup`,
`quire.op.model.all_instances` or `quire.op.model.dispatch_call`, or a read of
`over`. A non-Boolean guard, relation, predicate or condition refuses
`ill_typed`/`non-boolean-root` at the slot, and a binder whose type is not the
type its construct requires refuses `ill_typed`/`type-mismatch`, retaining
`expected` and `actual`.

## Derived causal relation

A clause's causal relation is **derived** from its control tree and the
cross-references, and is carried by no node. Each control has an *enter* and
an *exit* port; a leaf control (`send`, `receive`, `attempt`, `effect`,
`event`, `commit`, `fence`, `check`) has enter before exit. The edges are:

| Kind | Edges |
| --- | --- |
| `sequence` | enter of the `sequence` to enter of its first child; exit of each child to enter of the next; exit of the last child to exit of the `sequence`; for no child, enter to exit |
| `branch` | enter of a `choice` to enter of each case body; enter of a `parallel` to enter of each branch body; exit of each case body to exit of its `choice` |
| `join` | exit of each branch body the `parallel`'s `joined` list names to exit of the `parallel` |
| `repeat_progress` | enter of the `repeat` to enter of its body; exit of the body to exit of the `repeat`; enter of the `repeat` to enter of its `exhausted` control and its exit to exit of the `repeat`; and the back edge, exit of the body to enter of the body |
| `await_success`, `await_timeout` | enter of the `await` to enter of its `event`; exit of the `event` to enter of `then`; enter of the `await` to enter of `timeout`; exit of `then` and of `timeout` to exit of the `await` |
| `await_anchor` | exit of the control (or enter of the compensation's forward effect) the `after` argument names to enter of the `await` |
| `message` | exit of a `send` to enter of each `receive` whose `of` argument names it |
| `effect` | exit of an `attempt` to enter of each `effect` whose `of` argument names it |

The relation without the `repeat_progress` back edges SHALL be acyclic. A
cycle refuses `invalid_package`/`definition-cycle`, naming the controls on
the cycle in tree order, so a `receive` sequenced before the `send` it
receives refuses and the same two controls on different branches of a
`parallel` do not. A sequence's edges follow the authored order of its `children`, which is
meaning; beyond that no edge is derived from source order, a node's position in
the node array, timestamps, queue order or equal labels
([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)).

**Progress.** A control *necessarily steps* when every path through it
passes a step: `send`, `receive`, `attempt`, `effect`, `event`, `commit` and
`fence` do; `await` and `parallel` do (the match or the `timeout` step, the
`fork` and the `join`); a `sequence` does when some child does; a `choice`
does when every case body does; `check` does not. A `repeat` is skipped when
its guard is false at its entry, so it is a step in an enclosing obligation
only when that exit cannot happen (the *nested `repeat` rule* below). A
`repeat` whose body does not necessarily step refuses
`unsupported_construct`/`declaration-form` at the `repeat` (the first such
`repeat` in tree order), naming it and the selected protocol profile
([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)-AC-3).

**Nested `repeat` rule.** Repeat-through-repeat is a supported capability: a
`repeat` whose body progresses only through a nested `repeat` is admitted when
the nested `repeat` necessarily steps, and refused otherwise. A `repeat` that
sits inside the body of another `repeat` (at any depth, through any control)
*necessarily steps* for that enclosing `repeat` exactly when all three hold:

1. its exit is *infeasible*: its `guard`, read over the independent-atom
   abstraction below, is true under every valuation of its atoms;
2. its `maximum` is absent or at least 1 (a `repeat` with `maximum` 0 never
   enters its body, so it contributes nothing whatever its guard is);
3. its own `body` necessarily steps (a `repeat` whose own body does not is
   refused at the first such `repeat` in tree order, as above).

A `repeat` whose exit is feasible, that is, whose guard is false under some
valuation, contributes no step to an enclosing `repeat`: a feasible exit
contributes no enclosing progress, and an infeasible exit derives the
`repeat`'s contribution from the entered body's independently admitted
progress (QSL FR-048, the source language's choreography requirement, in the
repository agent-ix/quire-spec-language). The `exhausted` control is never counted. The
encoding adds no field: the wire's `guard` term, its `visible` entries and its
`maximum` are the whole input, and the reader derives the contribution from
them.

*Independent-atom abstraction.* A **Boolean constant** is a `literal` term of
`value_kind` `boolean`, written inline or reached by a `reference` to a
`value`/`literal` node of that kind; it is a constant and never an atom. The
**atoms** of a nested `repeat`'s `guard` are, first, each node its `visible`
set lists, in the set's order, whether or not the `guard` reads it, and then
every other maximal subterm of the `guard` that is neither a Boolean constant
nor an application of `quire.op.boolean.not`, `.and`, `.or` or `.implies`,
in order of first occurrence in a depth-first walk that visits the last
operand of a connective first. A node that is one of the listed entries is an
atom and the walk does not descend into it: with the entry `E` listing
`a and b`, the guard `E or not a or not b` has the three atoms `E`, `a`
and `b`, and a feasible exit. A `reference` (to a listed entry, an unlisted
expression or a capture) and an application of any other operator (for
example `quire.op.boolean.eq`) is one atom as a whole, its operands are not
read; two subterms that are the same node (the same node id, which is
structural) are one atom, and `quire.op.boolean.eq(a, b)` and
`quire.op.boolean.eq(b, a)` are two atoms, the reader being commutativity-
unaware. The atoms are independent: no valuation is excluded by any relation
between them, so `a` and `a > 0` are two atoms and a guard over atoms that
are in fact related is judged as if they were not. A **valuation** assigns
`true` or `false` to each of the `n` distinct atoms. Valuations are
enumerated from the all-false valuation as a little-endian binary counter
over the atoms in the order above (the first atom is the least significant
bit, each step adds one), and the guard is evaluated with the Boolean
constants, `not`, `and`, `or` and `implies` taking their Boolean meaning. The
exit is infeasible exactly when all `2^n` valuations make the guard true (`n`
may be 0: the guard `true` is infeasible, `false` is feasible); the reader
stops at the first valuation that makes the guard false. No guard is ever
compared with the body or with another guard, and an atom's value is never
read from a binder, a capture or the model. Collecting and comparing atoms
charges nothing.

*Work.* The check runs for every `repeat` that sits inside the `body` of
another `repeat` (a `repeat` inside an `exhausted` control is not), whatever
its `maximum`, after the causal-cycle check and before the progress refusals.
The reader charges one unit of the `work` limit
(`CheckedPackageReadLimits.work`) before evaluating each valuation, and
nothing else for the check. A valuation whose charge the limit denies stops
the read as the reader's `incomplete` result
`stage_limit_exceeded`/`work-budget-exceeded`
([FR-330](FR-330-contract-package.md) "Reader results as FR-272 diagnostics"),
located at the nested `repeat`'s node, and no progress refusal is reported
for the enclosing `repeat`. Let *V* be the reader's total `work` consumption
through the last valuation charge of the nested `repeat` and *T* its total
consumption for the whole package: a limit of *V* minus 1 or less that denies
a valuation's charge yields the incomplete result above, and a limit of *T*
admits the package. There is no ceiling on the number of atoms other than the
caller's `work` limit.

**Soundness.** Progress stays structural: a `repeat` with a feasible exit
adds no edge and no step to the enclosing cycle, so a cycle of the enclosing
`repeat` passes a step only through a step that every path takes. An
infeasible exit is the one case where the first guard test at the `repeat`'s
entry is true on every valuation, so the entered body runs at least once
(`maximum` at least 1 or absent) and its necessary step is taken before the
enclosing back edge. A `repeat` is pre-test (FR-466 "No post-test `repeat`"),
so its guard reads only binders in scope at its entry and the first test is
the one the abstraction covers. A nested `repeat`'s iteration counter is
reset to 0 on each entry from outside it (FR-425, FR-426), so each entry
reaches its first guard test with the counter below any `maximum` of at least
1: `repeat max 1 while true { attempt }` nested in an outer `repeat` takes its
`attempt` on every iteration of the outer `repeat`, and is a step of the
outer `repeat`. The reader never decides that a feasible exit cannot happen,
and never decides it from a binder's runtime value.

## Placement and profile fit

- A `quire.op.protocol.control` application is the body root of a
  `protocol` node whose `semantic_form` the operator's row in the catalog
  names, and stands nowhere else. The application at the body root of a node
  of another tag or of another form of the same tag refuses
  `ill_typed`/`operator-ineligible` at that node. A `protocol_reference`
  carrier's body is exactly a `reference` (FR-322): an application there
  refuses `malformed_wire`.
- The same application nested inside a node body or inside another term is
  outside FR-322's body grammar and refuses `malformed_wire` at the nested
  application, at strict wire validation.
- A reference to a protocol member node (any form other than
  `protocol_clause` and `protocol_reference`) stands only in the argument
  FR-464, FR-465 or FR-466 places it. A reference from a state clause, a
  temporal clause or a diagnostic's `details` refuses
  `ill_typed`/`operator-ineligible` at the node whose body holds it, or at
  `/diagnostics/entries/{e}/details/{d}`. A reference to a `protocol_clause`
  node is FR-322's `protocol_reference` carrier and stands wherever that
  requirement places it.
- An operator the lock's `protocol_profile` definition does not define, or a
  `member` that is not a `profile_operator`, refuses `invalid_package`/
  `operation-member-mismatch` at the application. A law that is absent,
  of another role or not the lock's row refuses as FR-322 states
  (`operation-law-missing`, `operation-law-mismatch`,
  `operation-law-unselected`).
- An `await` or a `compensate` reads a clock role, so the lock SHALL select a
  `temporal_profile`; without one the node refuses
  `missing_declaration`/`missing-selection`.

## Reader order

This is the one reader order for the protocol nodes; FR-465 and FR-466 name
the checks each member runs and defer the order to this section. The
`dependencies` list of every protocol node is checked at the graph stage
(FR-322 `dependencies`, FR-322-AC-96), before any body step: a body-referenced
node absent from the referencing node's `dependencies` refuses
`invalid_semantic_graph` at that node's `/dependencies` and no protocol step
runs, as the `owner_model` target does (AC-20). `missing_declaration`/
`missing-name` is left to a name that resolves to no node, to an
out-of-scope binder read and to a list-membership miss (FR-466-AC-4,
FR-466-AC-9). The protocol
step runs after the state step ([FR-341](FR-341-checked-package-state-clause-body.md)
"Reader order") and every other body step of FR-322 "Body steps" (the
state-graph, temporal ([FR-370](FR-370-checked-package-temporal-clause-body.md))
and abstraction steps among them), and before any operation refusal
([FR-322](FR-322-checked-package-artifact.md)). Strict wire validation, which
refuses a nested protocol application and, for a protocol application, a
layout, term shape or enumerated value outside its operator's layout
(`malformed_wire`), precedes it. The V2 schema carries that wire layer in its
`ProtocolNode` body rule, which applies `ProtocolControlApplication` to every
protocol node except a `protocol_reference` carrier: the operator's argument
names, order and count, the references the layout names (`over`, a `trigger`,
a `binder`), the Tuple-of-Groups records (`captures`, `cases`, `branches`,
`related`, the `compensate` captures), the minimum of two `cases` and
`branches`, every enumerated text, the exactly-when presence of `count`,
`predicate`, `key`, `loss`, `condition`, `exhausted` and the `proof` pair, and
a single `protocol_profile` law. The locus of such a refusal is the first error
path of the strict schema validator. The step reports the first defect, in this
order:

1. **Placement** of every `quire.op.protocol.control` application and of
   every reference to a protocol member node, in ascending node-id
   order, then every diagnostic `details` term.
2. **Operator and law** of every protocol node, in ascending node-id
   order.
3. **Declarations**: equal `declaration.qualified_name` values, then the
   declaration prefix and control paths ("Membership and containment").
4. **Member nodes**, form by form in this order, each in ascending node-id
   node-id order: `role`, `channel`, `queue`, `obligation`, `control`,
   `compensation`. For one node the local checks of its form's section run in
   the order that section lists: references and forms, then shapes and
   canonical order, then types, then bounds. Binder scope, the model-state
   refusals and the decision facts are not local checks; they run at step 6.
5. **Membership**: containment, listing and unattached nodes.
6. **Clause nodes**, each in ascending node-id order. For one clause:
   first the **owner-model edge** ("Owner-model edge phase" below), then its
   other references and their forms, the activation, the captures, the derived
   causal relation (cycle), progress, and then the **slots**, in this order:
   the clause (activation guard, captures), the roles, the channels, the
   `run` tree in pre-order, `finish`, `terminal`, then the compensations. For
   one slot the checks run in this order: the `over` and model-state
   refusals, then the binder-scope refusals, then (for a `choice` or `repeat`)
   the decision facts (FR-466 "Decision facts").
7. **Refinement class catalogs.** After all protocol nodes pass steps 1 to 6,
   visit `relation`/`refinement` nodes in ascending node-id order.
   For each present `class_catalog`, reconstruct the complete concrete and
   abstract class lists from its selected model declarations and the now
   admitted local or directly selected dependency protocol graph, using
   FR-432's class table and binder records. Compare `concrete` before
   `abstract`, in class order; within one equal class compare node
   association, then binder types in order, then source-default memory
   model (a memory class is catalogued once per model, so two entries
   of one `memory` spelling are distinct by model, and the `activate`
   class of an `on each` clause compares its clause node and trigger binder
   types as FR-375 states). FR-375 specifies the refusal and locus for each mismatch. A
   protocol-body defect therefore precedes a catalog-versus-graph defect,
   while any refinement-step row/catalog defect precedes both. This check
   runs even when the package has no local protocol node and the selected
   abstract clause is in an admitted direct dependency.

Within one member node's body references (the clause's `owner_model` edge excepted:
"Owner-model edge phase" states its stages and codes), a reference defect (a name that resolves to no node, then wrong form; a target
absent from `dependencies` is already refused at the graph stage, before this step) precedes a type defect, which precedes a bound defect. Operand
families and `result_type` are the application's catalog checks at the
operation step.

### Owner-model edge phase

The typed check of a clause's `owner_model` target has one phase: the first
check of that clause at step 6, after every state, temporal, abstraction and
refinement step and after steps 1 to 5. It is neither a refinement-step check
nor part of the step 7 catalog association. Every defect of the edge falls in
exactly one earlier or later stage, in this order of precedence:

1. **Wire** (strict V2 schema, `malformed_wire` at the node): an omitted
   `owner_model` on a `protocol_clause`, an external `{package, node}` key or any
   other non-`NodeRef` value, and an `owner_model` on another node.
2. **Graph shape** (FR-322 graph stage, before any body step): the
   target's own **owner join**. A `model`/`object_type` node whose
   `ModelOwner` joins no `model_root` node, or whose admitted document
   does not declare that node as an object type, is a genuinely unselected
   target. It refuses `missing_declaration`/`missing-selection` at the
   target's own `node_id` (FR-322 owner join), whether or not any clause names
   it, so a clause never reaches the typed check with such a target. In the
   same stage a clause whose `dependencies` lack its `owner_model` target
   refuses `invalid_semantic_graph` at the clause's `/dependencies`
   (FR-322-AC-96).
3. **Typed edge check** (step 6, first check of the clause), at
   `/semantic_graph/nodes/{i}/owner_model`, retaining the clause `node_id`,
   the target key and the target's form: a local target that names no node of
   the graph refuses `missing_declaration`/`missing-name`; a target that is
   not the `model`/`object_type` node of a selected declaration refuses
   `invalid_model_binding`/`malformed-declaration`. The second case holds for a
   non-model node, a non-model declared node with a source `owner` (a
   model-tagged node with a `SourceOwner` is `malformed_wire` already,
   FR-322-AC-51), and a model-tagged node of a selected declaration of any
   other form (`state_machine`, `process`, `record_value_type`, `event_type`,
   a relation node) or the selected package's `model_root` node (not a
   declaration, but a model node). The check does not compare the target with the `over`
   parameter's type: a selected second `object_type` is admitted as a
   different clause identity.
4. **Catalog association** (step 7) reads only an edge that passed 1 to 3.

A defect of an earlier stage in the same package is reported instead (the
refinement step's row and catalog-shape defects, the member defects of step
4, membership at step 5); a defect of the same clause's activation, captures,
causal relation or slots is reported only after the edge passes.

## Cyclic protocol reference components (Option A)

A predicate-join function that refers to its own protocol is admitted when
the resulting checked-node references form a cycle. The reader uses FR-322's
existing recursion-group mechanism for the entire cyclic strongly connected
component; the group is not restricted to `protocol` nodes. FR-322's generic
graph-cycle validation assigns and validates the group before the
protocol-specific body step. The `protocol_reference` carrier that closes the
cycle, the node through which the function reaches its own protocol, is part
of the component and MUST carry the group (FR-322, "Carrier in a cyclic
component"); the carrier join and the reader order of this artifact follow
the cycle check unchanged. Acyclic protocol nodes carry no
`recursion_group` from a producer; a label on one is admitted and means nothing. This adds no protocol-specific wire variant or
alternate cycle encoding.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-464-AC-1 | Every protocol node's body is a `quire.op.protocol.control` application with operator class `protocol_control`, one `protocol_profile` law equal to the lock's row, a `profile_operator` member naming an operator of the catalog, `null` mode, empty leaves, `result_type` the Boolean node and `semantic_type` the same node; a body that applies another identity, carries two laws, another member kind or a `null` member, or whose arguments are not the operator's bindings in order, refuses `malformed_wire` or `operation-member-mismatch`. | Test (TC-946) |
| FR-464-AC-2 | Each operator belongs to the one `semantic_form` the catalog names; an application at the body root of a node of another form or tag refuses `ill_typed`/`operator-ineligible` at that node, and the same application nested in a body or term refuses `malformed_wire` at the nested application. | Test (TC-946) |
| FR-464-AC-3 | A producer writes no `recursion_group` on an acyclic protocol node (a label there is admitted and means nothing): reformatting the source, a comment, a second reference to the protocol and declaring the roles in another source order leave every node id unchanged; renaming a role, reordering two `children`, changing one literal, retargeting one reference or changing a clause's `owner_model` yields a structurally unequal affected node, and changes a node id only for a rename (a declared node's id is its declaration path and producer-assigned ordinal) or for a node whose id embeds the changed operand ids; a set out of ascending node-id order or repeating an entry refuses `invalid_semantic_graph`. | Test (TC-946) |
| FR-464-AC-4 | A role, a channel, a template and a control named `X` in one protocol have four distinct declarations; two controls on one path, or two roles of one name, refuse `ambiguous_declaration`/`ambiguous-name` naming both keys; a control whose declaration is not its holder's declaration followed by its name, a control held twice and a control held by nothing refuse `invalid_semantic_graph` at the control. | Test (TC-946) |
| FR-464-AC-5 | The clause body is thirteen bindings in the order the "Clause body" table lists; `activation` is `origin` with no trigger or `each` with a trigger and an optional guard; a protocol with no relationship, requirement, compensation, `terminal` member or `scheduling` has empty sets and absent optionals and is admitted; a body with arguments reordered, missing or of another term shape refuses `malformed_wire`. | Test (TC-946) |
| FR-464-AC-6 | A listed role, channel, queue, compensation or obligation node that is not of the stated tag and form refuses `invalid_model_binding`/`malformed-declaration` at the target, and a `relationships` target that is not a `relation`/`relationship` node `invalid_model_binding`/`wrong-export`; a target that is a node of the graph but absent from `dependencies` refuses `invalid_semantic_graph` at the referencing node's `/dependencies` (graph stage, FR-322-AC-96, before any protocol step), and a target that names no node `missing_declaration`/`missing-name`; a declaration node named by two arguments of one clause or by two clauses, a node owned by no clause, a channel with no queue or with two, and a queue naming another clause's channel refuse `invalid_semantic_graph`. | Test (TC-946) |
| FR-464-AC-7 | Binder levels run from the introducing construct's entry count in layout order: `over` `0`, the `each` trigger `1`, the captures `2` and up in order, a control's binder `size(In(n))`; a wrong level, a binder body of another shape and two readable binders of one name refuse (`invalid_semantic_graph`, `malformed_wire`, `ambiguous_declaration`/`ambiguous-name`). | Test (TC-946) |
| FR-464-AC-8 | An expression that reads a binder or capture outside its stage's list refuses `missing_declaration`/`missing-name` at the node whose slot holds it, retaining name, stage, owner and `read` (`forward`, `foreign` or `stage`); a read of `over` in an activation guard, a trigger guard, a retry relation, a recovery predicate, a choice or repeat guard, a `visible` entry, a `check` condition or a FIFO key refuses `ill_typed`/`operator-ineligible` before any stage-table refusal, while a protocol, registration or activation capture, an event constraint and `finish` may read it; a `finish` constraint reading a binder of a branch not joined is refused `missing-name`; a slot that bars model state whose closure holds a model-state operation refuses `ill_typed`/`operator-ineligible` at the read; a non-Boolean guard refuses `ill_typed`/`non-boolean-root`. | Test (TC-946) |
| FR-464-AC-9 | The derived causal relation is acyclic: a `receive` sequenced before the `send` it names, a send/receive pair in one sequence in the wrong order and an `await` anchored at a later control each refuse `invalid_package`/`definition-cycle` naming the controls on the cycle; the same pair on different branches of a `parallel` is admitted; a `repeat` back edge is not a cycle. | Test (TC-946) |
| FR-464-AC-10 | A `repeat` whose body is only a `check`, or a `choice` with a step in one case and an empty `sequence` in the other, or a nested `repeat` with a feasible exit in one case, refuses `unsupported_construct`/`declaration-form` at the `repeat`; a `repeat` whose body is a step, a `parallel` or an `await` is admitted with a maximum, with a loop proof, with both and with neither. | Test (TC-946) |
| FR-464-AC-11 | An operator the profile does not define refuses `operation-member-mismatch`; a protocol application with the law missing, of the temporal role or not the lock's row refuses as FR-322-AC-22 states; an `await` or `compensate` in a package whose lock selects no `temporal_profile` refuses `missing_declaration`/`missing-selection`. | Test (TC-946) |
| FR-464-AC-12 | The protocol step runs after the state, temporal, abstraction and refinement steps and before the operation step and reports, in order, placement, operator, declaration, member, membership, clause and refinement class-catalog graph-join defects; within a clause the cycle and progress defects precede the slot checks, and within a slot the `over`/model-state refusal precedes the binder-scope refusal, which precedes the decision-fact refusal; a state-step defect and a protocol defect in one package report the state-step defect. | Test (TC-946, TC-310) |
| FR-464-AC-13 | The activation relation of an admitted protocol is derived from its clause node's `activation` and `requirements` alone: `activation on each (opened)` with `requires temporal OrderClosureBounded` gives exactly one row, the clause key, `each` with `opened`'s node id and `OrderClosureBounded`'s node id; under `activation on origin` the one row carries `origin` and no trigger; a protocol with no requirement has no row; a second `requires temporal OrderClosureBounded` leaves one row; a second required temporal clause gives two rows in ascending node-id order; a temporal clause that shares the protocol's clock but is not required gives no row; a `requirements` entry or an `each` trigger that is a node absent from the clause's `dependencies` refuses `invalid_semantic_graph` at the clause's `/dependencies` (graph stage, FR-322-AC-96, before the protocol step) and one that names no node refuses `missing_declaration`/`missing-name` at the target, each with no relation. | Test (TC-946) |
| FR-464-AC-14 | A complete refinement class catalog whose rows agree with it is admitted after its selected protocol graph passes the protocol step. When one otherwise valid package has both an independent protocol-body member defect and a rekeyed catalog class associated with the wrong existing control node, the protocol-body defect refuses first; after repairing only that defect and rebuilding the changed keys and package identity, the catalog association refuses. A refinement row/catalog mismatch instead refuses at the earlier refinement step before either protocol defect. | Test (TC-310) |
| FR-464-AC-15 | When a predicate-join function refers to its own protocol and the checked-node references form a cycle, the reader admits the package using FR-322's recursion-group encoding for the entire cyclic component, including non-protocol nodes: all component members, a `protocol_reference` carrier that closes the cycle among them, share the group, and in-component references are ordinary `reference`s to member `node_id`s (spelled `group_reference` in the written form). Membership refusals are FR-322-AC-285's and an acyclic carrier's label is FR-322-AC-284's. Only cycle members are constrained to share one explicit `recursion_group`; an acyclic protocol node carries none from a producer, may carry a label with no consequence, and retains its existing key inputs. | Test (TC-946) |
| FR-464-AC-16 | Every `protocol_clause` node has one `owner_model` local `NodeRef` to its exact model declaration node; the edge is present in both V2 schemas, included in the clause's exact dependencies, and absent from every other node. A missing edge, external key, non-model target or missing dependency refuses at the edge or dependency before clause interpretation, in the stage and with the code that FR-464-AC-20 states. Retargeting only the edge yields a different clause node. FR-375 includes every clause whose edge is the object-type node of a model owned by the selected concrete declaration in its class catalog, even when no step row names its classes, and excludes a clause whose edge names a model owned by another declaration. | Test (TC-946, TC-310) |
| FR-464-AC-17 | An `attempt` references the no-point `operation_anchor` for its operation. When a state clause names that same operation with a `ClauseBinding.execution_point`, the package has a distinct point-bearing anchor for the clause and a no-point anchor for the attempt; both may reference the operation's shared frame. Altering either point-bearing anchor changes its identity without changing the protocol attempt's anchor reference. | Test (TC-946) |
| FR-464-AC-18 | A protocol clause retains `owner_model` as the owner model's object-type node even when its package also has a `model_root`; the root identifies the selected package as a whole and MUST NOT replace or alter the clause's owner edge or dependencies. Refinement class-catalog joins resolve exact clause owners first, then associate them with the selected package identity. | Test (TC-946, TC-310) |
| FR-464-AC-19 | The protocol profile defines the operator `transition` for the `state`/`transition` form only: a `quire.op.state.transition` application naming `transition` is admitted by the operation step, a `quire.op.state.transition` application naming any operator of the protocol-node table (for example `sequence`) and a `quire.op.protocol.control` application naming `transition` fail the published schema and a complete reader refuses them `malformed_wire` at the application's `operation/member/operator`; the operation step run alone on the same documents refuses `invalid_package`/`operation-member-mismatch` (FR-341-AC-19). | Test (TC-233) |
| FR-464-AC-20 | The `owner_model` target is checked in one phase ("Owner-model edge phase"), with a fail-closed outcome for each case: an omitted edge and an external key refuse `malformed_wire` at the node; a genuinely unselected `model`/`object_type` target (owner joining no selection row, or not declared as an object type by the selected document) refuses `missing_declaration`/`missing-selection` at the target's own `node_id` in FR-322's owner join before any body step and before the clause's edge is read; a target missing from the clause's `dependencies` refuses `invalid_semantic_graph` at `/dependencies`; a local target naming no node refuses `missing_declaration`/`missing-name`, and a model node of a selected declaration of another form, a selected package's `model_root` node, a non-model declared node with a source owner and a non-model node refuse `invalid_model_binding`/`malformed-declaration`, both at `/semantic_graph/nodes/{i}/owner_model` as the first check of the clause at step 6; a valid selected second `object_type` is admitted and is not compared with the `over` type. An earlier-stage fault wins when paired: the owner join of the target precedes every body step, the refinement-step defects and the step 4 and 5 defects precede the typed edge check, and the typed edge check precedes the same clause's activation, capture and slot defects. Reader evidence is owned by IR-1053 (IR FR-064-AC-4/TC-048); no QSpec reader test exists, so the vectors TC-946 OM-03, OM-05..OM-11 and O10..O12 are the published oracle. | Analysis |
| FR-464-AC-21 | Repeat-through-repeat: an outer `repeat` whose body steps only through a nested `repeat` that necessarily steps (a stepping body, `maximum` absent or at least 1) is admitted when the nested guard's exit is infeasible over the independent atoms (the guard `true`, the tautology `a or not a or b or not b`, with or without a `maximum` of at least 1) and refuses `unsupported_construct`/`declaration-form` at the outer `repeat` when the exit is feasible (the guard `a`, `not a`, `a and not a`, `false`, `a or b`) or when the nested `maximum` is 0 whatever its guard is; a nested `repeat` that is the only stepping part of a `choice` case follows the same rule (the `choice` necessarily steps only when the nested `repeat` necessarily steps and every other case steps), and a nested `repeat` whose own body does not step refuses at the first such `repeat` in tree order; a body that holds a step beside the nested `repeat` (a `sequence` with a stepping sibling) or a `parallel` or `await` around it is admitted whatever the nested guard is. | Test (TC-946) |
| FR-464-AC-22 | The exit-feasibility derivation adds no field and compares no guard. Atoms: every listed `visible` entry, in set order and read or not, then every other maximal subterm that is neither a Boolean constant (an inline Boolean `literal` or a reference to a Boolean `value`/`literal` node) nor an application of `quire.op.boolean.not`, `.and`, `.or` or `.implies`, by first occurrence in a depth-first walk visiting the last operand first; a node equal to a listed entry is an atom and is not descended into (the entry `E` listing `a and b` with the guard `E or not a or not b` has three atoms and a feasible exit, so the outer `repeat` refuses); equal nodes are one atom and `quire.op.boolean.eq(a, b)` and `quire.op.boolean.eq(b, a)` are two (`eq(a, b) or not eq(a, b)` is infeasible, `eq(a, b) or not eq(b, a)` is feasible); a guard `true` written as a reference to a Boolean literal node is the constant, as an inline literal is. Valuations are enumerated from the all-false valuation as a little-endian counter over the atoms in that order. One `work` unit is charged before each valuation: a two-atom tautology charges four units, a feasible guard stops at its first false valuation (`a or b` charges one), a nested `repeat` is charged whatever its `maximum`, a `repeat` that is not inside another `repeat`'s `body` is charged nothing (its total consumption equals that with the guard `true`) and the collection and comparison of atoms charges nothing. With *V* the total consumption through the nested `repeat`'s last valuation charge and *T* the total for the package, a limit of *V* minus 1 that denies a valuation yields `stage_limit_exceeded`/`work-budget-exceeded` located at the nested `repeat` with no progress refusal for the enclosing `repeat`, and a limit of *T* admits the package. | Test (TC-946) |
| FR-464-AC-23 | A nested `repeat`'s counter is reset on each entry from outside it, so `repeat max 1 while true { attempt }` with an `exhausted` control, nested as the only body of an outer `repeat`, is admitted: its exit is infeasible, its `maximum` is 1 and its body steps, and the outer `repeat` takes a step on every iteration. A nested `repeat` is never counted through its `exhausted` control: a nested `repeat max 1 while true` whose body is only a `check` and whose `exhausted` control is an `attempt` leaves the outer `repeat` refused (`unsupported_construct`/`declaration-form` at the first `repeat` in tree order, the outer one). | Test (TC-946) |
| FR-464-AC-24 | The V2 schema decides a protocol node's layout: every protocol node except a `protocol_reference` carrier must satisfy `ProtocolControlApplication`, so an operator's argument names, order and count, a Tuple-of-Groups record, and an enumerated value outside its set (`delivery`, `overflow`, `loss`, `memory`, `outstanding`, `policy`, `ordering`, `scheduling`, `mode`) each refuse `malformed_wire` at the schema stage, and the recorded locus is the node-level pointer `/semantic_graph/nodes/{i}` (the first error path of the strict validator); every spelling an enumeration admits, and an appended `fence`, `event` and `await` with their layouts, stay schema-valid. | Test (TC-946) |

## Published artifacts and the operation catalog

- The operation catalog `quire.checked-operation-catalog/v1`, owned by
  quire-verification-contracts, keeps its `quire.op.protocol.control` entry:
  operator class `protocol_control`, no operand position, rest `any_term`,
  result `clause`, law role `protocol_profile`, member kind `profile_operator`.
  The entry admits every argument list this requirement fixes, so it needs no
  new entry.
- `proposals/checked-package-v2/schema.json` enumerates the `protocol` node forms and
  the `quire.op.protocol.control` identity, and fix, per operator, the
  argument layout, the enumerated texts and the term shapes this requirement
  lists. A layout violation is `malformed_wire`.
- Every published package that carries a `protocol_clause` node encodes it in
  the clause body above.
- The set of `profile_operator` operators the protocol profile defines is the
  operator catalog above and the one state operator `transition`. The
  `quire.op.state.transition` catalog entry (operator class `state_transition`,
  no operand position, rest `any_term`, result `clause`, law role
  `protocol_profile`, member kind `profile_operator`) admits the argument list
  FR-341 fixes, so it needs no new entry.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`, the
  body grammar, the application-node dependency rule, the operation catalog and the
  reader's refusal order this artifact adds the protocol step to.
- [FR-465](FR-465-checked-package-protocol-declaration-bodies.md) and
  [FR-466](FR-466-checked-package-protocol-control-bodies.md) own the
  declaration, control and compensation bodies.
- [FR-341](FR-341-checked-package-state-clause-body.md) owns the
  `value`/`parameter` body binders use, and
  [FR-342](FR-342-checked-package-operation-anchor-body.md) the
  `state`/`operation_anchor` node an `attempt` and a compensation name.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) and
  [FR-228](../../objects/protocol/FR-228-bounded-control-construct.md) own the
  control constructs, their causal rules and the progress rule.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Proposed complete-V1 boundary contract for the protocol node family, including
the approved Option A cycle handling.
