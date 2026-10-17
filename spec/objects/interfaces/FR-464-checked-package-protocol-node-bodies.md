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
  protocol changes no key. Changing any name, member, reference, literal or
  list order changes the node key and, through the package's
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
| **list** | a Group `aggregate` whose members are `reference` terms, in the order the source writes the construct |
| **optional** | a Group `aggregate` with no member (absent) or exactly one member (present) |
| **interval** | a Group `aggregate` of the bindings `lower` and `upper`, two non-negative integers with `lower <= upper`; the upper bound is always present |
| **records** | a Tuple `aggregate` of Groups, one Group per entry, in source order |

An enumerated text value (`origin`, `each`, `adversarial` and the values
FR-465 and FR-466 list) is fixed by the published schema: another spelling
fails it. A list's order is authored order and part of the node key. A
node listed twice in one list, or in two lists of one protocol, refuses
`invalid_semantic_graph`.

## Clause body

A `protocol`/`protocol_clause` node's body applies the operator `protocol`
with thirteen arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `over` | a reference to the `value`/`parameter` node of the `over` parameter |
| 2 | `activation` | a Group: `kind`, a text `origin` or `each`; `trigger`, a reference to the trigger's `value`/`parameter` node for `each` and absent for `origin`; `guard`, a Boolean term for `each` with a guard and absent otherwise |
| 3 | `captures` | records, one Group per capture, `binder` (a reference to its `value`/`parameter` node) and `initializer` (a term) |
| 4 | `roles` | a list of `protocol`/`role` nodes |
| 5 | `relationships` | a list of `relation`/`relationship` nodes: the model relationship each `relationship` declaration of the protocol names; a target of another tag or form refuses `invalid_model_binding`/`wrong-export` |
| 6 | `channels` | a list of `protocol`/`channel` nodes |
| 7 | `queues` | a list of `protocol`/`queue` nodes, one per channel |
| 8 | `requirements` | a list of `temporal`/`temporal_clause` nodes: the temporal clauses the protocol requires |
| 9 | `compensations` | a list of `protocol`/`compensation` nodes |
| 10 | `terminal` | optional: a reference to the protocol's `terminal` `obligation` node |
| 11 | `scheduling` | optional: the text `adversarial` |
| 12 | `run` | a reference to the root `protocol`/`control` node |
| 13 | `finish` | a reference to the protocol's `finish` `obligation` node |

A relationship declaration has no node of its own: its name is a source
alias, and the `related by` annotations of FR-466 name the model
relationship node directly. The alias is not carried. A protocol with no
relationship, no required temporal clause, no compensation or no
`terminal` member has an empty list or an absent optional there. A protocol
without `scheduling adversarial` and one with it are two different
packages.

The `protocol_clause` node's occurrences have role `declaration`, at the
`protocol` declaration. Its `dependencies` are the nodes its body
references, so the clause depends on its roles, channels, queues,
requirements, compensations, `terminal` and `finish` nodes, its root control
and the `value`/`parameter` and `expression` nodes of its first three
arguments.

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
the binder. A protocol binder is introduced at one place, and a binder
node is shared by every place whose name, level and type are equal.

**Level.** A construct that introduces binders has an *entry count*: the
number of binders readable at its entry (`0` for the clause; `size(C0)` for a
role, a channel and a compensation template; `size(In(n))` for a control; the
binders readable after the root control for `finish`). It introduces its binders in
the order its layout lists them, at consecutive levels from the entry
count. So `over` is `0`, the `each` trigger `1`, the first capture the
count of `over` and the trigger, and each later capture the next level; a
`send`'s binder is `size(In(send))`. Two binders of one name readable at one
place refuse `ambiguous_declaration`/`ambiguous-name` at the node that
introduces the second.

**Readable binders.** Each expression a protocol node references is read
at a *stage*, and may read only the binders that stage lists. The clause-level
set `C0` is `over`, the activation binder (the `each` trigger) when present,
and every protocol capture binder.

| Stage | Where | Binders it may read |
| --- | --- | --- |
| `protocol-activation` | `activation.guard` | the activation binder |
| `protocol-capture` | a clause capture `initializer` | `over`, the activation binder and the protocol captures before it |
| `role-lifetime` | a replicated role's `until` | `C0` and the role's binder |
| `channel-key` | a FIFO channel's `key` | the channel's message binder |
| `run` | a slot in the `run` tree | FR-466 "Binder scope" |
| `finish` | `finish.constraint` | the binders readable after the root control, and the finish binder |
| `terminal` | `terminal.condition` | `C0` |
| `registration-capture`, `activation-guard`, `activation-capture`, `retry`, `recovery` | the slots of a compensation | FR-466 "Compensation scope" |

Every `run` slot and `finish` may also read the protocol captures through
`C0`. A binder read outside its stage's list refuses
`missing_declaration`/`missing-name`, located at the node whose slot holds
the expression, retaining `name`, `stage` (the stage identifier above),
`owner` (the clause or the template) and `read`: `forward` for a capture
of the same stage written later than the read or the capture itself,
`foreign` for a binder a template declares, read outside that template,
and `stage` for any other binder that exists but that the stage may not
read. The protocol input `over` is readable in the two capture stages and
in the `run` tree, and by none of the activation guard, the activation
captures, the retry relation and the recovery predicate. What an expression
reads is found over its dependency closure: the `value`/`parameter` nodes
reachable from the expression node through `dependencies`, stopping at a
`value`/`parameter` node and at a `function` node.

A slot that FR-465 and FR-466 mark **binders-only** (a choice or repeat
guard, a `check` condition, a FIFO `key`, the activation guard) is
additionally refused `ill_typed`/`operator-ineligible` at the read, naming
the slot, when the closure holds an application of `quire.op.state.pre`,
`quire.op.model.deref`, `quire.op.model.navigate`,
`quire.op.model.reaches`, `quire.op.model.reaches_field`,
`quire.op.model.lookup`, `quire.op.model.all_instances` or
`quire.op.model.dispatch_call`: such a slot reads binders and constants,
never model state.

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
`parallel` do not. Reading a name never adds an edge: no edge is derived
from source order, array order, timestamps, queue order or equal labels
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
  of another tag, of another form of the same tag, or a `protocol_reference`
  carrier, refuses `ill_typed`/`operator-ineligible` at that node.
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

The protocol step runs after the state step ([FR-341](FR-341-checked-package-state-clause-body.md)
"Reader order"), the temporal step ([FR-370](FR-370-checked-package-temporal-clause-body.md))
and the abstraction step, and before any operation refusal
([FR-322](FR-322-checked-package-artifact.md)). Strict wire validation, which
refuses a nested protocol application, a layout, term-shape or enumerated
value the schema fixes, and a negative bound, precedes it. It reports the
first defect, in this order:

1. **Placement** of every `quire.op.protocol.control` application and of
   every reference to a protocol member node, in ascending node-id digest
   order, then every diagnostic `details` term.
2. **Operator and law** of every protocol node, in ascending node-id digest
   order.
3. **Declarations**: equal `declaration.qualified_name` values, then the
   declaration prefix and control paths ("Membership and containment").
4. **Member nodes**, form by form in this order, each in ascending node-id
   digest order: `role`, `channel`, `queue`, `obligation`, `control`,
   `compensation`. For one node, the checks of its form's section run in
   the order that section lists.
5. **Membership**: containment, listing and unattached nodes.
6. **Clause nodes**, each in ascending node-id digest order. For one clause:
   its references and their forms, the activation, the captures, then the
   derived causal relation (cycle), then progress, then the binder scope of
   every slot in the order the clause, the roles, the channels, the run tree
   in pre-order, `finish` and the compensations list them.

Within one node, a reference defect (absent from `dependencies`, then wrong
form) precedes a type defect, which precedes a scope defect. Operand
families and `result_type` are the application's catalog checks at the
operation step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-464-AC-1 | Every protocol node's body is a `quire.op.protocol.control` application with operator class `protocol_control`, one `protocol_profile` law equal to the lock's row, a `profile_operator` member naming an operator of the catalog, `null` mode, empty leaves, `result_type` the Boolean node and `semantic_type` the same node; a body that applies another identity, carries two laws, another member kind or a `null` member, or whose arguments are not the operator's bindings in order, fails the published schema or refuses `operation-member-mismatch`. | Test (TC-946) |
| FR-464-AC-2 | Each operator belongs to the one `semantic_form` the catalog names; an application at the body root of a node of another form or tag refuses `ill_typed`/`operator-ineligible` at that node, and the same application nested in a body or term fails the V2 schema and refuses `malformed_wire` at the nested application. | Test (TC-946) |
| FR-464-AC-3 | A protocol node's key is the `quire.application-node/v1` digest of its tag, form, type, declaration, `null` recursion and body: reformatting the source, a comment and a second reference to the protocol leave every key and the `package_id` unchanged; renaming a role, reordering two list entries, changing one literal or retargeting one reference changes the node key and the `package_id`. | Test (TC-946) |
| FR-464-AC-4 | A role, a channel, a template and a control named `X` in one protocol have four distinct declarations; two controls on one path, or two roles of one name, refuse `ambiguous_declaration`/`ambiguous-name` naming both keys; a control whose declaration is not its holder's declaration followed by its name, a control held twice and a control held by nothing refuse `invalid_semantic_graph` at the control. | Test (TC-946) |
| FR-464-AC-5 | The clause body is thirteen bindings in the order the "Clause body" table lists; `activation` is `origin` with no trigger or `each` with a trigger and an optional guard; a protocol with no relationship, requirement, compensation, `terminal` member or `scheduling` has empty lists and absent optionals and is admitted; a body with arguments reordered, missing or of another term shape fails the published schema. | Test (TC-946) |
| FR-464-AC-6 | A listed role, channel, queue, compensation or obligation node that is not of the stated tag and form refuses `invalid_model_binding`/`malformed-declaration` at the target, and a `relationships` target that is not a `relation`/`relationship` node `invalid_model_binding`/`wrong-export`; a target absent from `dependencies` refuses `missing_declaration`/`missing-name`; a node listed twice, a node owned by no clause, a channel with no queue or with two, and a queue naming another clause's channel refuse `invalid_semantic_graph`. | Test (TC-946) |
| FR-464-AC-7 | Binder levels run from the introducing construct's entry count in layout order: `over` `0`, the `each` trigger `1`, the captures `2` and up in order, a control's binder `size(In(n))`; a wrong level, a binder body of another shape and two readable binders of one name refuse (`invalid_semantic_graph`, the FR-341 body schema, `ambiguous_declaration`/`ambiguous-name`). | Test (TC-946) |
| FR-464-AC-8 | An expression that reads a binder outside its stage's list refuses `missing_declaration`/`missing-name` at the node whose slot holds it, retaining name, stage, owner and `read` (`forward`, `foreign` or `stage`); an activation guard reading a capture or `over`, a capture reading a later capture of its stage and a `finish` constraint reading a binder of a branch not joined are refused; `over` is readable in a protocol capture and a registration capture and not in an activation guard, an activation capture, a retry relation or a recovery predicate; an activation guard, a choice guard, a repeat guard, a `check` condition or a FIFO key whose closure holds a model-state operation refuses `ill_typed`/`operator-ineligible` at the read. | Test (TC-946) |
| FR-464-AC-9 | The derived causal relation is acyclic: a `receive` sequenced before the `send` it names, a send/receive pair in one sequence in the wrong order and an `await` anchored at a later control each refuse `invalid_package`/`definition-cycle` naming the controls on the cycle; the same pair on different branches of a `parallel` is admitted; a `repeat` back edge is not a cycle. | Test (TC-946) |
| FR-464-AC-10 | A `repeat` whose body is only a `check`, or a `choice` with a step in one case and an empty `sequence` in the other, or a nested `repeat` in one case, refuses `unsupported_construct`/`declaration-form` at the `repeat`; a `repeat` whose body is a step, a `parallel` or an `await` is admitted with a maximum, with a loop proof, with both and with neither. | Test (TC-946) |
| FR-464-AC-11 | An operator the profile does not define refuses `operation-member-mismatch`; a protocol application with the law missing, of the temporal role or not the lock's row refuses as FR-322-AC-22 states; an `await` or `compensate` in a package whose lock selects no `temporal_profile` refuses `missing_declaration`/`missing-selection`. | Test (TC-946) |
| FR-464-AC-12 | The protocol step runs after the state, temporal and abstraction steps and before the operation step, reports placement before operator before declaration before member before membership before clause defects, and reports a state-step defect and a protocol defect in one package as the state-step defect. | Test (TC-946) |

## Published artifacts and the operation catalog

The operation catalog `quire.checked-operation-catalog/v1`, owned by
quire-verification-contracts, keeps its `quire.op.protocol.control` entry:
operator class `protocol_control`, no operand position, rest `any_term`,
result `clause`, law role `protocol_profile`, member kind `profile_operator`.
This requirement adds no catalog entry: the `protocol` clause, every
declaration and every control is an application of that one entry, and the
`profile_operator` operator set the protocol profile defines is the operator
catalog above. FR-322's phrase "the FR-052 bounded control constructs" names
that set.

`proposals/checked-package-v2/schema.json` and
`node-identity-preimage.schema.json` fix, per operator, the argument
layout the clause, FR-465 and FR-466 list, with the enumerated text values,
and keep the `ProtocolNode` form enumeration (`protocol_clause`,
`protocol_reference`, `role`, `channel`, `queue`, `control`, `obligation`,
`compensation`). The published packages that carry a `protocol_clause` node
encode it in the clause body above.

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
