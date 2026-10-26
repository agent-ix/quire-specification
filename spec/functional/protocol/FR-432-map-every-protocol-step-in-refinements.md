---
id: FR-432
title: "Map every concrete protocol step class by an explicit refinement row"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-177
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
---

# FR-432: Map every concrete protocol step class by an explicit refinement row

## Description

When a refinement declaration's concrete or abstract side is a protocol
subject, the checker SHALL decide every concrete protocol step class by exactly one
explicit deciding `step` row, refuse a class that no row decides or that two
rows of one kind decide, type each node row's arguments against the node's binder record, and read each step's visibility and observation
label from its deciding row alone.

## Inputs

A refinement declaration whose concrete subject, abstract subject or both are
protocol subjects ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md)),
with its step rows and the refinement's internal list (below), a
`refinement_internals` value supplied with the compilation.

## Outputs

An admitted mapping from every concrete step class to an abstract step,
`stutter` or `any`; or a located refusal naming the unmapped or doubly mapped
class.

## Behavior

### Step classes and their rows

A protocol step row is a `step` row whose left side names one concrete step
class, by the [shared grammar](../../../proposals/quire-v1/shared-grammar.md)'s `protocol-step-row` production, admitted
wherever the refinement declaration ([FR-375](../analysis/FR-375-declare-model-refinement.md))
admits a step row. The left sides read:

| Concrete step class | Row left side |
| --- | --- |
| `attempt` of one attempt node | `step <C>::<protocol>::<node>`, or the operation's row when the node has none |
| `cattempt` of one template | `step <C>::<protocol>::<template>`, or the operation's row when the template has none |
| `event`, `send`, `receive`, `fork`, `join`, `timeout`, `finish` of one node | `step <C>::<protocol>::<node>` naming the step kind: `step fork <C>::<protocol>::<parallel>`, `step join …`, and the node itself for event, send, receive, timeout and finish nodes |
| `cend` of one template | `step cend <C>::<protocol>::<template>` |
| `duplicate` or `lose` on one channel | `step duplicate <C>::<protocol>::<channel>`, `step lose …` |
| `spawn` or `retire` of one role | `step spawn <C>::<protocol>::<role>`, `step retire …` |
| `fence` of one fence node | `step <C>::<protocol>::<node>` |
| `activate` of the clause's trigger | `step activate <C>::<protocol>` |
| one memory step kind | `step memory <kind>` with `kind` a memory step name of the resolved model, such as `flush` |
| cross-model drain class | `step memory drain`, available under source defaults `sc`, `tso` and `ra`; it drains the thread's pending writes under that model's visibility rule, is a no-op under `sc`, flushes the TSO store buffer in FIFO order, and under RA has no store-buffer entries to remove and grants no additional visibility or ordering beyond FR-437 |

**Deciding row.** Each class has exactly one deciding row. For an `attempt`
class, the deciding row is the node row naming that attempt node when one
exists, and otherwise the row of its operation; an operation row therefore
decides every attempt node of that operation that has no node row, and a
node row and an operation row for the same node are not a conflict. A
`cattempt` class is decided the same way, by its template row or else its
operation row. Every other class is decided by its own row. Each right side is written out, by `protocol-step-target`: an abstract operation application; an abstract protocol step,
`node <A>::<protocol>::<node>(…)`, whose arguments are the node's binder
record as stated in "The binder record of an abstract node"; `stutter`; or
`any`. A right side naming an abstract protocol node resolves to an `attempt`,
`effect`, `event`, `commit`, `send`, `receive` or `finish` node of the selected abstract protocol;
a `timeout`, `fence`, `parallel` or other node, or a name that is no node of that
protocol, refuses `missing_declaration`/`missing-name` at the node. No right
side can therefore name an abstract `timeout`, `fence` or `parallel` node: those
abstract classes are either listed as internal (the refinement's internal list)
or visible and matched by no concrete row. Nothing defaults: a concrete
step class with no row refuses at check time
`missing_declaration`/`missing-name`, naming the node, template, channel,
role, trigger or memory step kind; two node rows for one node, two template
rows for one template, two operation rows for one operation, or two rows for
any other class refuse `invalid_model_binding`/`conflicting-binding`, naming
both.

### The binder record of an abstract node

The arguments of `node <A>::<protocol>::<node>(…)` are one expression per
field of the node's **binder record**, in the order below, `_` leaving a field
open. A row with no argument list leaves every field open. The grammar admits no empty list `()`. The fields by node
kind are:

| Node kind | Fields, in order | Field types |
| --- | --- | --- |
| `attempt` | `receiver`; one field per parameter of the node's operation, in declaration order; `result`, present only when the operation declares a result | the reference type of the type the attempt's operation name names (`on M::Cell::inc` names `M::Cell`, even when a supertype declares the operation); each parameter's declared type; the declared result type |
| `effect`, `event`, `commit`, `finish` | when the node's binder type `T` is a record type, one field per field of `T`, in declaration order; otherwise exactly one field | each field's declared type; otherwise `T` |
| `send`, `receive` | as `event`, over the channel's `carries` type | as `event` |

So a binder of a scalar, enumeration, variant or object type is one field of
that type, and no node kind is refused as a right side for its binder's type.
Each argument expression is typed against its field's type after reference
lifting, over the scope FR-375 gives the arguments of an abstract-operation
target: the concrete receiver as `self`, the concrete operation's arguments by
parameter name and the concrete pre-state. An argument list
whose length differs from the field count refuses
`ill_typed`/`operator-ineligible` at the node, retaining `expected`, the field count, and
`actual`, the written argument count, as an operation application with the wrong number of
arguments does. An argument whose type is not its field's type refuses
`ill_typed`/`type-mismatch` at the argument, retaining `expected`, the field's type, and `actual`, the argument's type.
The observation label of the visible step holds the field values the row gives
and leaves an open field unconstrained.

### A protocol on the abstract side

A declaration selects an abstract protocol subject in its header, by
`abstract <A>::<protocol>`, the `abstract-subject` production of
[FR-375](../analysis/FR-375-declare-model-refinement.md): the model alias and
the protocol clause's name, joined by `::`. A header with the alias alone
selects the model subject, and this section does not apply. The parser
distinguishes the two by the `::` and resolves neither; selection resolves
the protocol and refuses an unknown one as FR-375 states. Under a model
subject header, a step row whose right side names an abstract protocol node
refuses `invalid_model_binding`/`malformed-declaration` at the node: the
node exists, but the declaration did not select its protocol.

An abstract protocol subject's state is (model state, protocol state). The
refinement mapping defines its visible model fields; its rest points,
binders, queues and instances are hidden fields, tracked by the set of
consistent abstract states.

- A row to an abstract node or operation makes the concrete step **visible**,
  with the **observation label** of that abstract step: its kind, its node or
  operation, and the binder record its row's arguments give.
- A `stutter` row makes the step **internal**. An `any` row admits either.
- A visible concrete step passes when an abstract step with its label is
  enabled from a state of the set after zero or more abstract internal steps,
  and the set after it is closed under abstract internal steps. An internal
  concrete step passes when the mapped visible fields are unchanged, and
  leaves the set unchanged, closed the same way.
- The abstract internal steps are the refinement's internal list; an
  abstract step the list does not name is visible.

### The refinement's internal list

A profile selection carries an identity and nothing else, and the refinement
grammar has no `internal` row, so the list is not source. It is the
compilation's optional `refinement_internals` input, supplied with the source
as a supplied library is, and FR-375 states how it enters the node, the lock
and replay. The list belongs to a refinement declared in the compiled source
(the unit, or one supplied library; a library's list is its own).

**Shape.** A `refinement_internals` value is a JSON array of objects
`{"refinement": "<name>", "internal": ["<class>", ...]}`, closed to those two
keys. `refinement` is the name of a refinement declared in the compiled
source; `internal` is an array of class spellings.

**Class spellings.** A class spelling is the left side of the class's row in
the table above, without `step` and with the refinement's abstract alias
`<A>` in place of `<C>`. It has one space after a leading keyword, no other
whitespace and no other characters, and is its own canonical spelling.

| Abstract step class | Spelling |
| --- | --- |
| `attempt`, `event`, `send`, `receive`, `timeout`, `finish` or `fence` of one node; `cattempt` of one template | `<A>::<protocol>::<node>`, or `<A>::<protocol>::<template>` |
| `fork`, `join` of one `parallel` node | `fork <A>::<protocol>::<node>`, `join <A>::<protocol>::<node>` |
| `cend` of one template | `cend <A>::<protocol>::<template>` |
| `duplicate`, `lose` on one channel | `duplicate <A>::<protocol>::<channel>`, `lose <A>::<protocol>::<channel>` |
| `spawn`, `retire` of one role | `spawn <A>::<protocol>::<role>`, `retire <A>::<protocol>::<role>` |
| `activate` of the clause's trigger | `activate <A>::<protocol>` |
| one memory step kind | `memory <kind>` |
| cross-model drain class | `memory drain` |

`<A>::<protocol>` must be the refinement's abstract alias and selected
abstract protocol. A bare node path names the one class of an attempt, event,
send, receive, timeout, finish or fence node or of a template's `cattempt`; a
`parallel` node has the two classes `fork` and `join`, so its bare path is not
a class.

**Resolution.** The resolved list of a refinement is the set of classes its
entries name, written in ascending order of the UTF-8 bytes of their
alias-free spellings: a canonical spelling with `<A>::` removed (`Full::fin`, `lose Full::ch`, `activate Full`, `memory flush`), the spelling FR-375 carries as `abstract_internal`. The order therefore does not depend on the local abstract alias, and renaming the alias leaves the resolved list unchanged. The order of objects in the array and of entries within
one `internal` array is free and does not change the result.

**Absent is empty.** A refinement with no object in the value, and an absent
value, has an empty list: every abstract step class is visible. Absence is
not a refusal.

**One class, one decision.** A class that is in the list and is also the
abstract node a step row's right side names (`node <A>::<protocol>::<node>(…)`)
refuses `invalid_model_binding`/`conflicting-binding`, naming the row and the
list entry; the row does not win and the list does not win. The conflict
exists only for a class the right side can name.

**Defects.** Each refusal carries as its locus the RFC 6901 JSON Pointer of the
offending value inside the `refinement_internals` value (such as
`/0/internal/1`), since a JSON value has no source span; FR-375 states how a
refusal is located in its input. A refusal naming two values carries both pointers.

| Defect | Refusal |
| --- | --- |
| the value is not an array of objects; an object lacks `refinement` or `internal`, has another key, a non-string `refinement`, a non-array `internal` or a non-string entry | `invalid_model_binding`/`malformed-declaration` |
| an entry that is not one of the spellings above (an empty string, `frob x`, `Spec::Twice`, extra or leading whitespace, a non-identifier part) | `invalid_model_binding`/`malformed-declaration` |
| `refinement` names no refinement of the compiled source; for a refinement with an abstract protocol subject, an entry spelled correctly whose alias or protocol is not the refinement's abstract alias and selected protocol, or whose node, template, channel, role or memory kind does not exist, or whose bare path names a `parallel` node | `missing_declaration`/`missing-name` |
| two objects naming one refinement; one canonical spelling twice in one `internal` array | `invalid_model_binding`/`conflicting-binding` |
| a non-empty list for a refinement whose abstract subject is a model | `invalid_model_binding`/`malformed-declaration` at the object's pointer |

For a refinement whose abstract subject is a model, no entry is resolved against a protocol, because a model subject selects none: the name-resolution part of the third row does not apply to its entries, a spelled entry is neither `missing-name` nor admitted, and only the last row (and the not-yet-built refusal below) reports a non-empty list.

A defect of a value attributable to no refinement (not an array of objects,
`{}`, a non-string or unresolved `refinement`) is reported by this table
alone. Each refusal yields no admitted refinement node. While a producer has
not built `refinement-internal-list` (FR-375), every defect above is still
reported except the last: a non-empty list on a model-subject refinement is
then refused only as not yet built, at the name after `abstract`, and the
`invalid_model_binding`/`malformed-declaration` of that row applies once the
feature is built.

Each concrete initial protocol state pairs with an abstract initial model
state, with the abstract root thread settled from its `run`. Divergence,
refusal sets, terminal success and assumption weakening are checked by
FR-177 over the same product.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-432-AC-1 | A protocol refinement whose concrete protocol has a `fork`, a `join`, a `send` and a `finish` with no rows refuses `missing_declaration`/`missing-name` once per class, naming each node; with one row per class written out as `stutter`, `any` or an abstract step it is admitted. | Test (TC-386) |
| FR-432-AC-2 | A node row and an operation row for the same attempt node admit, with the node row deciding that node's steps and the operation row deciding a second attempt node of the same operation that has no node row; two node rows for one node, or two operation rows for one operation, refuse `invalid_model_binding`/`conflicting-binding` naming both. | Test (TC-386) |
| FR-432-AC-3 | A concrete protocol under `tso` with no `step memory flush` row refuses naming the `flush` kind; with the row it is admitted. A `step memory drain` row is a valid class under each source default `sc`, `tso` and `ra`; it drains pending writes according to the selected model, with no-op behavior under `sc` and no stronger visibility promise under `ra`. | Test (TC-386, TC-310) |
| FR-432-AC-4 | A concrete step mapped by `stutter` that changes a mapped visible field refutes the refinement; one mapped to an abstract send whose label no enabled abstract step carries, after abstract internal steps, refutes it; with the matching abstract step enabled it passes. | Test (TC-386) |
| FR-432-AC-5 | An abstract step absent from the refinement's internal list is visible: a concrete internal step cannot match it. | Test (TC-386) |
| FR-432-AC-6 | A declaration headed `abstract Spec::Req` over the abstract model alias `Spec` selects its protocol clause `Req`, and its step rows may name that protocol's nodes; the same rows under the header `abstract Spec` select a model subject and a row naming `node Spec::Req::Ask` refuses `invalid_model_binding`/`malformed-declaration` at that node. | Test (TC-386) |
| FR-432-AC-7 | The value `[{"refinement": "R", "internal": ["Spec::Full::fin", "lose Spec::Full::ch"]}]`, its permutations and the same entries in two arrays swapped resolve `R` to the same list of classes, written alias-free `Full::fin` then `lose Full::ch` in UTF-8 byte order; the value absent, `[]` and an object for another refinement each resolve `R` to the empty list with no refusal. | Test (TC-386) |
| FR-432-AC-8 | Each abstract step class spells and resolves: a node path for an attempt, event, send, receive, timeout, finish and fence node and a template path for a `cattempt`, and `fork`, `join`, `cend`, `duplicate`, `lose`, `spawn`, `retire`, `activate` and `memory` entries as the table states, each making exactly its class internal; the bare path of the `parallel` node refuses `missing_declaration`/`missing-name`. | Test (TC-386) |
| FR-432-AC-9 | Each defect row of the list's table refuses with its code and cause at the pointer of the offending value, with no admitted node: a non-array value, `{}`, an object with a missing, extra or mistyped key (`"refinement": 7` and `"internal": "x"` included) and a non-string entry, an unspelled entry, an unresolved refinement, alias, protocol, node, channel, role, kind or `parallel` bare path, a repeated object and a repeated spelling (both pointers), and a non-empty list on a refinement headed `abstract Spec`. | Test (TC-386) |
| FR-432-AC-10 | A list naming `Spec::Full::a1` beside a row `step … -> node Spec::Full::a1(self)` refuses `invalid_model_binding`/`conflicting-binding` naming the row and the entry; the same list with the row's right side `-> node Spec::Full::a2(self)` is admitted; a list naming `lose Spec::Full::ch` beside any rows is admitted. | Test (TC-386) |
| FR-432-AC-11 | Renaming the abstract alias `Spec` to `spec` with the list's entries renamed to match gives the same resolved list in the same order and the same node id: the alias-free order puts `Full::fin` before `lose Full::ch`, while the alias-including spellings `spec::Full::fin` and `lose spec::Full::ch` order the other way, so the rename discriminates the two orders. | Test (TC-386) |
| FR-432-AC-12 | A refinement headed `abstract Spec` with a non-empty list reports only the last defect row (or only the not-yet-built refusal), never `missing_declaration`/`missing-name` for an entry naming a protocol. | Test (TC-386) |
| FR-432-AC-13 | Over the abstract protocol `Full` of TC-386 IL-01, a step row whose right side is `node Spec::Full::to` (a `timeout`), `node Spec::Full::fe` (a `fence`), `node Spec::Full::par` (a `parallel`) or `node Spec::Full::Ghost` (no node) refuses `missing_declaration`/`missing-name` at the node; the same row naming an attempt, event, send, receive or finish node is admitted. | Test (TC-386) |
| FR-432-AC-14 | Over the abstract protocol `A` of the abstract model `Spec`, headed `abstract Spec::A`, with attempt `a1` on `twice(x: Int[0, 9])` (no result), attempt `a2` on `quad(x: Int[0, 9])` with result `Int[0, 99]`, event `ev as (e: Spec::Evt)` with `Evt{p: Boolean, q: Int[0, 9]}` and event `eb as (b: Boolean)`, the binder record of `a1` is (`receiver`, `x`), of `a2` is (`receiver`, `x`, `result`), of `ev` is (`p`, `q`) and of `eb` is the one field `b`; the rows `node Spec::A::a1(self, 1)`, `node Spec::A::a2(self, 1, _)`, `node Spec::A::ev(true, 3)`, `node Spec::A::eb(true)` and `node Spec::A::a1` with no argument list are admitted. | Test (TC-386) |
| FR-432-AC-15 | In the protocol of FR-432-AC-14, each refusal is `ill_typed`/`operator-ineligible` at the node retaining `expected` (the field count) and `actual` (the argument count): `node Spec::A::a1(self, 1, 2)` with 2 and 3, `node Spec::A::a1(self)` with 2 and 1, `node Spec::A::ev(true)` with 2 and 1, and `node Spec::A::eb(true, false)` with 1 and 2; `node Spec::A::a1(self, true)` refuses `ill_typed`/`type-mismatch` at the argument `true` with `expected` `Int[0, 9]` and `actual` `Boolean`, and `node Spec::A::ev(true, true)` does the same at its second argument with `expected` `Int[0, 9]` and `actual` `Boolean`. | Test (TC-386) |

## Dependencies

- [FR-177](FR-177-check-protocol-refinement.md) owns protocol refinement and
  its divergence, refusal, terminal-success and assumption checks.
- [FR-426](FR-426-apply-protocol-steps.md) owns the step classes.
- [FR-375](../analysis/FR-375-declare-model-refinement.md) owns where the
  internal list is hosted, its request pointers, its effect on the node and
  the unbuilt-feature refusal; [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  and [FR-323](../../objects/interfaces/FR-323-native-runtime-envelope.md)
  own its lock and replay.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `protocol-step-row` production.

## References

- Design: quire-spec-language ADR-027 PR-1 to PR-3 and RU-5; the refinement
  step rows of ADR-020 RU-1.
- Tickets: Linear STD-140; paired QSL-396 (with STD-133 and QSL-367 for model
  refinement); Linear STD-182 for the abstract protocol subject header; Linear STD-199 for
the internal list.
