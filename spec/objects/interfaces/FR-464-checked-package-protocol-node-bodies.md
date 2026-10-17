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
- **dependencies** — FR-322's application-node rule: exactly the unique
  digest-ascending `reference` targets of the body.
- **node key** — FR-322's `quire.application-node/v1` preimage
  `{version, node_tag, semantic_form, semantic_type, declaration, recursion,
  body}`, with `recursion` `null`: a protocol node carries no
  `recursion_group`. Because the body, its references and the `declaration`
  path enter the preimage and occurrences do not, a member node's key is a
  function of its own content, the content of every node it references and
  its place in its protocol, and of nothing in the source text but those.
  Reformatting the source, adding a comment or a second reference to a
  protocol changes no key. Changing any name, member, reference or literal, or the order of an
  ordered list, changes the node key and, through the package's
  `identity_projection`, the `package_id` ([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)
  AC-4).

## Operator catalog

The protocol profile the lock selects (`protocol_profile`) defines exactly
these operators. Each operator belongs to one `semantic_form`, and the form
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

The word after `P` is a fixed segment, so a role, a channel, a template and
a control of equal names have distinct declarations, and a name a
declaration segment cannot spell is not a protocol name. Two protocol nodes
with equal `declaration.qualified_name` refuse `ambiguous_declaration`/
`ambiguous-name`, as FR-322 refuses every pair of nodes with equal names,
naming both node keys in ascending digest order.

A control's path is its parent's path followed by its own name. A branch
label, a case label and an `await`'s `then` and `timeout` positions add no
segment: the control a branch, case, `then` or `timeout` position holds is
named by its own name below the `parallel`, `choice` or `await` that
holds it.

## Term shapes

The layouts use these shapes of FR-322's body grammar. A **reference**
names a node of the stated tag and form by node key.

| Shape | Term |
| --- | --- |
| **text** | a `literal` with `value_kind: "text"`, whose `type` is the `scalar_type`/`text` node |
| **integer** | a `literal` with `value_kind: "integer"`, whose `type` is a `scalar_type` of family `integer` |
| **Boolean term** | a `literal` with `value_kind: "boolean"`, or a `reference` to a node whose `semantic_type` is the `scalar_type`/`boolean` node |
| **term** | any Leaf |
| **ordered list** | a Group `aggregate` whose members are `reference` terms in an order that carries meaning: execution order or branch order |
| **set** | a Group `aggregate` whose members are `reference` terms in strictly ascending node-id digest order (unsigned bytes of the digest); the list means a set, so one set has one encoding |
| **optional** | a Group `aggregate` with no member (absent) or exactly one member (present) |
| **interval** | a Group `aggregate` of the bindings `lower` and `upper`, two non-negative integers with `lower <= upper`; the upper bound is always present |
| **ordered records** | a Tuple `aggregate` of Groups, one Group per entry, in an order that carries meaning (capture evaluation order, branch order) |
| **sorted records** | a Tuple `aggregate` of Groups, one Group per entry, in strictly ascending order of the entry's sort key, which its layout states |

An enumerated text value (`origin`, `each`, `adversarial` and the values
FR-465 and FR-466 list) has the spellings its layout lists: another spelling
refuses `malformed_wire` at the term.

**Canonical order.** A set or a sorted-records list that is not in its
canonical order, or that repeats an entry, refuses `invalid_semantic_graph`
at the node, as FR-322-AC-10 refuses every noncanonical order. An ordered
list is in the order its layout states, and reordering it changes the node
key. So the sets of one protocol (roles, relationships, channels, queues,
requirements, compensations, `visible`, `contracts`, `related`, case guards)
have one encoding whatever order the source declares them in, and only the
lists where order is meaning (a sequence's children, captures, branches)
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
| 7 | `queues` | a set of `protocol`/`queue` nodes, one per channel; the queues are generated, so their order is the ascending digest order of the channel nodes they name, not a source order |
| 8 | `requirements` | a set of `temporal`/`temporal_clause` nodes: the temporal clauses the protocol requires |
| 9 | `compensations` | a set of `protocol`/`compensation` nodes |
| 10 | `terminal` | optional: a reference to the protocol's `terminal` `obligation` node |
| 11 | `scheduling` | optional: the text `adversarial` |
| 12 | `run` | a reference to the root `protocol`/`control` node |
| 13 | `finish` | a reference to the protocol's `finish` `obligation` node |

A relationship declaration has no node of its own: its name is a source
alias, and the `related by` annotations of FR-466 name the model
relationship node directly. The alias is not carried. A protocol with no
relationship, no required temporal clause, no compensation or no
`terminal` member has an empty set or an absent optional there. A protocol
without `scheduling adversarial` and one with it are two different
packages.

The `protocol_clause` node's occurrences have role `declaration`, at the
`protocol` declaration. Its `dependencies` are the nodes its body
references, so the clause depends on its roles, channels, queues,
requirements, compensations, `terminal` and `finish` nodes, its root control
and the `value`/`parameter` and `expression` nodes of its first three
arguments. A `requirements` entry whose temporal clause reaches the same
protocol through a `protocol_reference` carrier closes a dependency cycle
(clause, temporal clause, carrier, clause), which FR-322 refuses like every
ungrouped cycle.

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
target, and a reference whose target is not among the referencing node's
`dependencies` refuses `missing_declaration`/`missing-name`, located at
the target. These are the same joins FR-341 states for a state clause's
parameters.

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
| `terminal` | `terminal.condition` | none: constants and the model observation only |
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
entry, a `check` condition, a FIFO `key` and the terminal condition's
binders read binders, captures and constants, never model state. They refuse,
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
does when every case body does; `check` and `repeat` do not (a `repeat` can
be skipped by a false guard). A `repeat` whose body does not necessarily
step refuses `unsupported_construct`/`declaration-form` at the `repeat`,
naming it and the selected protocol profile
([FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)-AC-3).

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
the checks each member runs and defer the order to this section. The protocol
step runs after the state step ([FR-341](FR-341-checked-package-state-clause-body.md)
"Reader order"), the temporal step ([FR-370](FR-370-checked-package-temporal-clause-body.md))
and the abstraction step, and before any operation refusal
([FR-322](FR-322-checked-package-artifact.md)). Strict wire validation, which
refuses a nested protocol application and, for a protocol application, a
layout, term shape or enumerated value outside its operator's layout
(`malformed_wire`), precedes it. The step reports the first defect, in this
order:

1. **Placement** of every `quire.op.protocol.control` application and of
   every reference to a protocol member node, in ascending node-id digest
   order, then every diagnostic `details` term.
2. **Operator and law** of every protocol node, in ascending node-id digest
   order.
3. **Declarations**: equal `declaration.qualified_name` values, then the
   declaration prefix and control paths ("Membership and containment").
4. **Member nodes**, form by form in this order, each in ascending node-id
   digest order: `role`, `channel`, `queue`, `obligation`, `control`,
   `compensation`. For one node the local checks of its form's section run in
   the order that section lists: references and forms, then shapes and
   canonical order, then types, then bounds. Binder scope, the model-state
   refusals and the decision facts are not local checks; they run at step 6.
5. **Membership**: containment, listing and unattached nodes.
6. **Clause nodes**, each in ascending node-id digest order. For one clause:
   its references and their forms, the activation, the captures, the derived
   causal relation (cycle), progress, and then the **slots**, in this order:
   the clause (activation guard, captures), the roles, the channels, the
   `run` tree in pre-order, `finish`, `terminal`, then the compensations. For
   one slot the checks run in this order: the `over` and model-state
   refusals, then the binder-scope refusals, then (for a `choice` or `repeat`)
   the decision facts (FR-466 "Decision facts").

Within one node, a reference defect (absent from `dependencies`, then wrong
form) precedes a type defect, which precedes a bound defect. Operand
families and `result_type` are the application's catalog checks at the
operation step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-464-AC-1 | Every protocol node's body is a `quire.op.protocol.control` application with operator class `protocol_control`, one `protocol_profile` law equal to the lock's row, a `profile_operator` member naming an operator of the catalog, `null` mode, empty leaves, `result_type` the Boolean node and `semantic_type` the same node; a body that applies another identity, carries two laws, another member kind or a `null` member, or whose arguments are not the operator's bindings in order, refuses `malformed_wire` or `operation-member-mismatch`. | Test (TC-946) |
| FR-464-AC-2 | Each operator belongs to the one `semantic_form` the catalog names; an application at the body root of a node of another form or tag refuses `ill_typed`/`operator-ineligible` at that node, and the same application nested in a body or term refuses `malformed_wire` at the nested application. | Test (TC-946) |
| FR-464-AC-3 | A protocol node's key is the `quire.application-node/v1` digest of its tag, form, type, declaration, `null` recursion and body: reformatting the source, a comment, a second reference to the protocol and declaring the roles in another source order leave every key and the `package_id` unchanged; renaming a role, reordering two `children`, changing one literal or retargeting one reference changes the node key and the `package_id`; a set out of ascending digest order or repeating an entry refuses `invalid_semantic_graph`. | Test (TC-946) |
| FR-464-AC-4 | A role, a channel, a template and a control named `X` in one protocol have four distinct declarations; two controls on one path, or two roles of one name, refuse `ambiguous_declaration`/`ambiguous-name` naming both keys; a control whose declaration is not its holder's declaration followed by its name, a control held twice and a control held by nothing refuse `invalid_semantic_graph` at the control. | Test (TC-946) |
| FR-464-AC-5 | The clause body is thirteen bindings in the order the "Clause body" table lists; `activation` is `origin` with no trigger or `each` with a trigger and an optional guard; a protocol with no relationship, requirement, compensation, `terminal` member or `scheduling` has empty sets and absent optionals and is admitted; a body with arguments reordered, missing or of another term shape refuses `malformed_wire`. | Test (TC-946) |
| FR-464-AC-6 | A listed role, channel, queue, compensation or obligation node that is not of the stated tag and form refuses `invalid_model_binding`/`malformed-declaration` at the target, and a `relationships` target that is not a `relation`/`relationship` node `invalid_model_binding`/`wrong-export`; a target absent from `dependencies` refuses `missing_declaration`/`missing-name`; a declaration node named by two arguments of one clause or by two clauses, a node owned by no clause, a channel with no queue or with two, and a queue naming another clause's channel refuse `invalid_semantic_graph`. | Test (TC-946) |
| FR-464-AC-7 | Binder levels run from the introducing construct's entry count in layout order: `over` `0`, the `each` trigger `1`, the captures `2` and up in order, a control's binder `size(In(n))`; a wrong level, a binder body of another shape and two readable binders of one name refuse (`invalid_semantic_graph`, `malformed_wire`, `ambiguous_declaration`/`ambiguous-name`). | Test (TC-946) |
| FR-464-AC-8 | An expression that reads a binder or capture outside its stage's list refuses `missing_declaration`/`missing-name` at the node whose slot holds it, retaining name, stage, owner and `read` (`forward`, `foreign` or `stage`); a read of `over` in an activation guard, a trigger guard, a retry relation, a recovery predicate, a choice or repeat guard, a `visible` entry, a `check` condition or a FIFO key refuses `ill_typed`/`operator-ineligible` before any stage-table refusal, while a protocol, registration or activation capture, an event constraint and `finish` may read it; a `finish` constraint reading a binder of a branch not joined is refused `missing-name`; a slot that bars model state whose closure holds a model-state operation refuses `ill_typed`/`operator-ineligible` at the read; a non-Boolean guard refuses `ill_typed`/`non-boolean-root`. | Test (TC-946) |
| FR-464-AC-9 | The derived causal relation is acyclic: a `receive` sequenced before the `send` it names, a send/receive pair in one sequence in the wrong order and an `await` anchored at a later control each refuse `invalid_package`/`definition-cycle` naming the controls on the cycle; the same pair on different branches of a `parallel` is admitted; a `repeat` back edge is not a cycle. | Test (TC-946) |
| FR-464-AC-10 | A `repeat` whose body is only a `check`, or a `choice` with a step in one case and an empty `sequence` in the other, or a nested `repeat` in one case, refuses `unsupported_construct`/`declaration-form` at the `repeat`; a `repeat` whose body is a step, a `parallel` or an `await` is admitted with a maximum, with a loop proof, with both and with neither. | Test (TC-946) |
| FR-464-AC-11 | An operator the profile does not define refuses `operation-member-mismatch`; a protocol application with the law missing, of the temporal role or not the lock's row refuses as FR-322-AC-22 states; an `await` or `compensate` in a package whose lock selects no `temporal_profile` refuses `missing_declaration`/`missing-selection`. | Test (TC-946) |
| FR-464-AC-12 | The protocol step runs after the state, temporal and abstraction steps and before the operation step and reports, in order, placement, operator, declaration, member, membership and clause defects; within a clause the cycle and progress defects precede the slot checks, and within a slot the `over`/model-state refusal precedes the binder-scope refusal, which precedes the decision-fact refusal; a state-step defect and a protocol defect in one package report the state-step defect. | Test (TC-946) |

## Published artifacts and the operation catalog

What is true at the head that adds this requirement:

- The operation catalog `quire.checked-operation-catalog/v1`, owned by
  quire-verification-contracts, keeps its `quire.op.protocol.control` entry
  unchanged: operator class `protocol_control`, no operand position, rest
  `any_term`, result `clause`, law role `protocol_profile`, member kind
  `profile_operator`. The entry admits every argument list this requirement
  fixes, so it needs no new entry.
- `proposals/checked-package-v2/schema.json` and
  `node-identity-preimage.schema.json` enumerate the `protocol` node forms and
  the `quire.op.protocol.control` identity. They do not yet fix the per-operator
  argument layouts, enumerated texts or term shapes, and this requirement adds
  none of them. A reader enforces the layouts itself and refuses a violation as
  `malformed_wire` until the schemas encode them.
- The published packages `positive-all-families.json` and
  `positive-clause-operations.json` carry a `protocol_clause` node that applies
  `sequence` with no arguments. That node predates this requirement and is not
  a conforming clause body: a reader implementing this requirement refuses it.
  FR-322-AC-66's worked example uses that node only as the target of a
  `protocol_reference` carrier, and the carrier's rules depend on its tag, form
  and type, not its body. Regenerating the published packages with conforming
  clause bodies (new node keys and `package_id`s) is follow-up work for the
  IR owner; until then the example and the published packages are exempt from
  the clause body rules of this requirement and the vectors of TC-946 to TC-948
  are the conformance reference.
- The set of `profile_operator` operators the protocol profile defines is the
  operator catalog above. FR-322 names that set.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`, the
  body grammar, the application-node preimage, the operation catalog and the
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

Proposed complete-V1 boundary contract for the protocol node family.
Implementation and qualification remain tracked separately.
