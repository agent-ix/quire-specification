---
id: FR-465
title: "Checked-package protocol role, channel, queue and obligation bodies"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-464
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-050
    type: references
  - target: ix://agent-ix/quire-specification/FR-054
    type: references
  - target: ix://agent-ix/quire-specification/FR-170
    type: references
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
  - target: ix://agent-ix/quire-specification/FR-428
    type: references
  - target: ix://agent-ix/quire-specification/FR-429
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-465: Checked-package protocol role, channel, queue and obligation bodies

## Description

When a checked package carries a protocol declaration's roles, channels,
queue bounds, finish and terminal members, the producer SHALL encode each as
one `protocol` node whose body is the
[FR-464](FR-464-checked-package-protocol-node-bodies.md) application with the
operator and argument layout below, and the reader SHALL refuse a node whose
checks below do not hold, with the first refusal in the order each section
lists. The node family's operation, keys, declarations, membership, binders
and reader order are FR-464's; this requirement fixes the member set, order and
checks of the declarations.

## Role

A `protocol`/`role` node (operator `role`) has two arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `model` | a reference to the `model` node the role is played against |
| 2 | `replication` | a Group; empty for a static role |

A static role is one responsibility with one role-instance binding per
assessed workflow. A replicated role (`role r each T from pop max n lifetime L`)
is a template of role instances, one per object of a population, and its
`replication` Group has these bindings, in this order:

| Binding | Value |
| --- | --- |
| `population` | a text: the name of the population the instances are drawn from |
| `max` | an integer, at least 1: the authored limit on live instances at once |
| `binder` | a reference to the `value`/`parameter` node that stands for one instance |
| `lifetime` | a text, `workflow`, `scope` or `until` |
| `until` | a Boolean term, present exactly when `lifetime` is `until` |

- **semantic_type** — the Boolean node (FR-464).
- **dependencies** — the `model` target, and the `binder` target and the
  nodes the `until` term references for a replicated role.

The reader checks, for one role, in this order:

1. **Model.** The `model` target is a declared dependency, else
   `missing_declaration`/`missing-name`, and a `model` node of form
   `object_type`, `process`, `state_machine`, `systems_interface` or
   `persistence_interface`, else `invalid_model_binding`/
   `malformed-declaration`. A role is a participant contract: a
   `record_value_type`, `event_type`, `variant_type` or any node that is not
   a `model` node is not a role's model. The locus is the target.
2. **Replication shape.** `until` is present exactly when `lifetime` is
   `until`, else the layout is violated (`malformed_wire`). `max` is at least 1, else
   `ill_typed`/`type-mismatch` at `max`, retaining `expected` and `actual`.
3. **Binder.** The `binder` target is a declared dependency, else
   `missing_declaration`/`missing-name`, and a `value`/`parameter` node, else
   `invalid_model_binding`/`malformed-declaration`; its `semantic_type` is the
   `composite_type`/`reference` node over the `model` target, else
   `ill_typed`/`type-mismatch` at the role node, retaining `expected` and
   `actual`.
4. **Lifetime.** The `until` term is a Boolean term, else
   `ill_typed`/`non-boolean-root` at the role node. Its reads are a slot
   check (FR-464 step 6, stage `role-lifetime`).

A role has no internality member: neither the source nor this body spells
`internal` or `external`, so a role carries none.

## Channel

A `protocol`/`channel` node (operator `channel`) has five arguments, in this
order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `from` | a reference to the sending `protocol`/`role` node |
| 2 | `to` | a reference to the receiving `protocol`/`role` node |
| 3 | `carries` | a reference to the message type node |
| 4 | `ordering` | a Group: `kind`, a text `unordered` or `fifo`; for `fifo`, `binder`, a reference to the `value`/`parameter` node of the message, and `key`, a term: the FIFO key expression |
| 5 | `delivery` | a text: `unknown`, `at-most-once`, `at-least-once` or `exactly-once-premise` |

- **semantic_type** — the Boolean node.
- **dependencies** — the two roles, the message type node, and for `fifo`
  the `binder` and the nodes `key` references.
- **Premises, not guarantees.** Every channel states its sender, receiver,
  message type, ordering and delivery premise explicitly. `unordered` adds no
  order. `fifo` orders only the messages of this channel that carry equal
  values of `key`; it never orders messages across channels
  ([FR-054](../../functional/protocol/FR-054-bind-channel-premises.md),
  [FR-170](../../functional/protocol/FR-170-evaluate-channel-queue-semantics.md)).
  `exactly-once-premise` is an authored premise, never a transport
  guarantee, and `unknown` supports none of the delivery conclusions. No
  premise defaults.

The reader checks, for one channel, in this order:

1. **Endpoints.** `from` and `to` are declared dependencies, else
   `missing_declaration`/`missing-name`, and `protocol`/`role` nodes of the
   same clause, else `invalid_model_binding`/`malformed-declaration`; a node
   that is a `queue`, a model component or an endpoint is not a role. The
   locus is the target.
2. **Message type.** `carries` is a declared dependency, else
   `missing_declaration`/`missing-name`, and a `scalar_type` node, a
   `composite_type` node, or a `model` node of form `record_value_type`,
   `event_type` or `variant_type`, else `invalid_model_binding`/
   `malformed-declaration`.
3. **Ordering.** For `fifo`, `binder` and `key` are present, else the
   layout is violated (`malformed_wire`); `binder` is a declared `value`/`parameter`
   dependency whose `semantic_type` is the `carries` target, else
   `ill_typed`/`type-mismatch` at the channel node, retaining `expected` and
   `actual`. The type of `key` is equality-stable (FR-322 "Equality-stable
   types"), else `ill_typed`/`operator-ineligible` at the key, retaining the
   operator `=` and the type. `key` is a slot that bars model state at the
   `channel-key` stage (FR-464 step 6): it reads the message binder and
   constants, so a read of a capture is `missing_declaration`/`missing-name`
   and a read of `over` or of model state `ill_typed`/`operator-ineligible`.
   For `unordered`, `binder` and `key` are absent.

## Queue

A `protocol`/`queue` node (operator `queue`) states the bound and overflow
behaviour of one channel and has four arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `channel` | a reference to the `protocol`/`channel` node it bounds |
| 2 | `capacity` | a Group: `kind`, a text `literal` or `symbolic`; `value`, a non-negative integer for `literal`; `name`, a text for `symbolic` |
| 3 | `overflow` | a text: `reject`, `block` or `loss` |
| 4 | `loss` | optional: the text `head`, `tail` or `new`, present exactly when `overflow` is `loss` |

- **declaration** — none: a queue is declared by its channel's `capacity` and
  `overflow` text, so its occurrences have role `generated`.
- **semantic_type** — the Boolean node.
- **dependencies** — the `channel` target.
- **Meaning.** A `literal` capacity is a concrete bound. A `symbolic`
  capacity names a bound that execution discharges before a finite run: the
  package carries the name and no value. At full capacity `reject` leaves
  the queue unchanged and records a rejected send, `block` leaves a pending
  send, and `loss` applies the loss selector: `head` drops the oldest queued message, `tail` the newest queued message and `new` the arriving message, the queue otherwise unchanged ([FR-170](../../functional/protocol/FR-170-evaluate-channel-queue-semantics.md)); the grammar spells the selector after `loss`.
  There is one queue per channel and no queue without a channel. The delivery
  policy governs transport duplication and loss and the overflow policy governs
  a full queue, so a channel with `at-least-once` delivery and overflow `loss`
  is admitted: an overflow loss is not a transport loss.

The reader checks, for one queue: the `channel` target is a declared
dependency, else `missing_declaration`/`missing-name`, and a
`protocol`/`channel` node of the same clause, else
`invalid_model_binding`/`malformed-declaration`; then `capacity` has `value`
for `literal` and `name` for `symbolic` and no other member, else the
layout is violated (`malformed_wire`). The one-queue-per-channel rule is FR-464's
membership check.

## Finish

A `protocol`/`obligation` node with operator `finish` is the protocol's final
predicate and has two arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `binder` | a reference to the `value`/`parameter` node of the closing record |
| 2 | `constraint` | a Boolean term over the closing record |

- **declaration** — `P`, `finish`, the finish node's name.
- **semantic_type** — the Boolean node.
- **Meaning.** The finish obligation holds when the closing record exists and
  satisfies `constraint`. It is met only by the workflow-closure observation
  after the root control completes; a matched record, an end of input or the
  absence of a violation does not meet it.
- **Readable binders.** `constraint` reads the binders readable after the
  root control ([FR-466](FR-466-checked-package-protocol-control-bodies.md)
  "Binder scope") and `binder`; it may read `over`. The closing record's
  type is the type the source declares and no check constrains it.

The reader checks that `binder` is a declared `value`/`parameter`
dependency, else `missing_declaration`/`missing-name` or
`invalid_model_binding`/`malformed-declaration`; and that `constraint` is a
Boolean term, else `ill_typed`/`non-boolean-root` at the node. Its reads are
a slot check (FR-464 step 6, stage `finish`). A clause names
exactly one `finish` node, in its `finish` argument.

## Terminal

A `protocol`/`obligation` node with operator `terminal` declares the
intended terminal states and has two arguments, in this order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `mode` | a text, `when` or `any` |
| 2 | `condition` | optional: a Boolean term, present exactly when `mode` is `when` |

- **declaration** — `P`, `terminal`.
- **semantic_type** — the Boolean node.
- **Meaning.** `when` declares a terminal state intended when `condition`
  holds there; `any` declares every terminal state intended
  ([FR-429](../../functional/protocol/FR-429-report-protocol-deadlocks.md)).
  A clause carries at most one `terminal` node; a second `terminal` node
  with the declaration `P`, `terminal` refuses `ambiguous_declaration`/
  `ambiguous-name`, as FR-464 refuses equal declarations.

The reader checks that `condition` is present exactly when `mode` is `when`,
else the layout is violated (`malformed_wire`); and that
it is a Boolean term, else `ill_typed`/`non-boolean-root` at the node. Its reads
are a slot check (FR-464 step 6, stage `terminal`): it reads the model
observation (`over`), constants and, for an `origin` activation, the protocol
captures, as
[FR-429](../../functional/protocol/FR-429-report-protocol-deadlocks.md) states,
and no protocol binder, so a read of the activation binder, of a `run` binder
or of a capture of an `each` protocol (a per-instance value the package-level
condition has no binder for) refuses `missing_declaration`/`missing-name`. Under `activation on
each` a terminal state can have no live instance, which is why no instance
binder is readable there.

## Runtime subjects

A package states what a later runtime must bind and states nothing it can
only learn at runtime. The runtime subject kinds
([FR-210](../protocol/FR-210-protocol-subject-vocabulary.md)) a consumer binds
are a function of the operators in the package, and no node carries them:

| Operator | Subject kinds bound at runtime |
| --- | --- |
| `protocol` | `workflow-instance` |
| `role` | `role-instance`, one per assessed workflow, or one per spawned object for a replicated role |
| `send` | `message`, `send` |
| `receive` | `receive`, `delivery` |
| `attempt` | `operation-attempt` |
| `effect` | `business-effect` |
| `commit` | `commit` |
| `compensate` | `compensation`, `compensation-attempt`, `compensation-effect` |
| `finish` | the workflow-closure observation |

The authority that supplies each binding and the contract it is checked
against are the package lock's `binding_contract` selection; the package
fabricates no instance, ordinal, delivery or observation, and compiling
needs none of them. The enclosing repeat ordinals of an occurrence are a
runtime dimension of the control's key and are not a member of any node.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-465-AC-1 | A static role is admitted with an empty `replication` Group and a `model` target that is an object type, process, state machine, systems interface or persistence interface; a role on a `record_value_type`, `event_type`, `variant_type` or non-`model` node refuses `invalid_model_binding`/`malformed-declaration` at the target, and an absent target `missing_declaration`/`missing-name`. | Test (TC-947) |
| FR-465-AC-2 | A replicated role is admitted with `population`, `max` of at least 1, a `binder` of type `Reference<model>` and each lifetime; `until` is present exactly with `lifetime: until`; `max: 0` refuses `ill_typed`/`type-mismatch`, a binder of another type `ill_typed`/`type-mismatch`, a non-Boolean `until` `ill_typed`/`non-boolean-root`, and an `until` term reading a run-tree binder `missing_declaration`/`missing-name`. | Test (TC-947) |
| FR-465-AC-3 | A channel is admitted with two roles of its clause, a message type, `unordered` or `fifo` with its message binder and key, and one of the four delivery premises; an endpoint that is not a role of the clause, a message type that is not a type node, a FIFO binder of a type other than `carries` (`ill_typed`/`type-mismatch`), a key that reads a capture (`missing_declaration`/`missing-name`), `over` or model state (`ill_typed`/`operator-ineligible`), a key of a type without stable equality (a float or a type holding one or an event) such as a `float64`, a `fifo` without `key` and an `unordered` with one refuse as the reader order states. | Test (TC-947) |
| FR-465-AC-4 | No channel premise defaults: a channel missing `delivery`, `ordering`, `from`, `to` or `carries` refuses `malformed_wire`; `exactly-once-premise` and `unknown` are distinct admitted spellings, and another delivery spelling refuses `malformed_wire`. | Test (TC-947) |
| FR-465-AC-5 | A queue is admitted with a `literal` capacity and a `symbolic` capacity and each overflow policy (with each loss selector), carries occurrences of role `generated` and no `declaration`, and depends only on its channel; a `literal` capacity with `name`, a `symbolic` capacity with `value`, an unknown overflow spelling, a `loss` selector absent for overflow `loss`, present for `reject` or `block`, or spelled other than `head`, `tail` or `new`, refuse `malformed_wire`, and a negative capacity `invalid_package`/`invalid-value`; a `channel` target that is not a channel of the clause refuses as the reader order states. | Test (TC-947) |
| FR-465-AC-6 | A `finish` node is admitted with a closing-record binder and a Boolean constraint that reads the binders readable after the root control; a non-Boolean constraint refuses `ill_typed`/`non-boolean-root` and a read of a binder of an unjoined branch `missing_declaration`/`missing-name`; a clause's `finish` argument naming a `terminal` node refuses `invalid_model_binding`/`malformed-declaration` at the target, and a `finish` node the clause does not name refuses `invalid_semantic_graph`. | Test (TC-947) |
| FR-465-AC-7 | A `terminal` node is admitted as `when` with a condition and as `any` with none; a `when` without a condition and an `any` with one refuse `malformed_wire`; a second `terminal` node of one clause refuses `ambiguous_declaration`/`ambiguous-name`. | Test (TC-947) |
| FR-465-AC-8 | The package carries no instance, ordinal, delivery, observation or internality member: the runtime subject kinds of the "Runtime subjects" table are a function of the operators, an added `internal`/`external` member refuses `malformed_wire`, and the package compiles with no workflow instance, delivery or clock observation. | Test (TC-947) |

## Dependencies

- [FR-464](FR-464-checked-package-protocol-node-bodies.md) owns the node
  family, its keys, binders, membership and reader order.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md),
  [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) and
  [FR-170](../../functional/protocol/FR-170-evaluate-channel-queue-semantics.md)
  own the role and channel premises and the queue transition rules.
- [FR-428](../../functional/protocol/FR-428-check-replicated-roles-and-per-trigger-activation.md)
  owns replicated roles and their lifetimes, and
  [FR-429](../../functional/protocol/FR-429-report-protocol-deadlocks.md) the
  `terminal` member.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Proposed complete-V1 boundary contract for the declaration nodes of the
protocol family. Implementation and qualification remain tracked separately.
