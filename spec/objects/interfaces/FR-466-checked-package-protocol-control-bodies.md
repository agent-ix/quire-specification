---
id: FR-466
title: "Checked-package protocol control and compensation bodies"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-464
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/FR-053
    type: references
  - target: ix://agent-ix/quire-specification/FR-056
    type: references
  - target: ix://agent-ix/quire-specification/FR-057
    type: references
  - target: ix://agent-ix/quire-specification/FR-058
    type: references
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/FR-427
    type: references
  - target: ix://agent-ix/quire-specification/FR-434
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-466: Checked-package protocol control and compensation bodies

## Description

When a checked package carries a protocol declaration's control tree and
compensation templates, the producer SHALL encode each control and each
template as one `protocol` node whose body is the
[FR-464](FR-464-checked-package-protocol-node-bodies.md) application with the
operator and argument layout below, and the reader SHALL refuse a node whose
checks below do not hold, with the first refusal in the order each section
lists. This requirement fixes the member set, order, binder scope and checks
of the thirteen control operators and of `compensate`.

## Control layouts

Every control node has `semantic_form: "control"`, the Boolean node as
`semantic_type`, and occurrences of role `declaration`. A control's
dependencies are the nodes its body references. **Binders-only** marks a slot
that reads binders, protocol captures and constants and never model state, `over` included (FR-464 "Binders").

| Operator | Arguments, in order |
| --- | --- |
| `sequence` | `children`: an ordered list of `protocol`/`control` nodes, in execution order; may be empty |
| `choice` | `owner`: a reference to the owning `protocol`/`role` node; `visible`: a set of the `expression` nodes whose values the owner sees at the decision; `cases`: sorted records of `label` (a text), `guard` (a Boolean term, binders-only) and `body` (a reference to a control), sorted by `label` as UTF-8 bytes, at least two |
| `parallel` | `memory`: optional text `sc`, `tso` or `ra`; `branches`: sorted records of `label` (a text) and `body` (a reference to a control), sorted by `label` as UTF-8 bytes, at least two; `join`: a Group, `policy` a text `all`, `any`, `quorum` or `predicate`, `count` an integer exactly for `quorum`, `predicate` a reference to a `function` node exactly for `predicate`; `joined`: a Group of texts, the branch labels the join awaits, in ascending label order; `outstanding`: optional text `continue` or `cancel` |
| `repeat` | `owner`: a reference to a role; `visible`: a set; `maximum`: optional integer, at least 0; `proof`: a Group, empty or the bindings `invariant` (a Boolean term) and `variant` (a term); `guard`: a Boolean term, binders-only; `body`: a reference to a control; `exhausted`: optional reference to a control, present exactly when `maximum` is present |
| `await` | `after`: a Group, `kind` a text `event` or `compensation`, `target` a reference; `clock`: a text; `within`: an interval; `event`: a reference to the awaited control; `then`: a reference to a control; `timeout`: a reference to a control |
| `check` | `condition`: a Boolean term, binders-only |
| `send` | `channel`: a reference to a `protocol`/`channel` node; `binder`: a reference to the payload binder; `related`: sorted records of `relationship` (a reference to a `relation`/`relationship` node), `from` and `to` (terms), sorted by the relationship's node-id digest, then the JCS bytes of `from`, then those of `to`; `constraint`: a Boolean term |
| `receive` | `channel`; `of`: a reference to the `send` control received; `binder`; `related`; `constraint` |
| `attempt` | `role`: a reference to a role; `operation`: a reference to a `state`/`operation_anchor` node; `ordering`: optional text `relaxed`, `acquire`, `release`, `acq_rel` or `seq_cst`; `contracts`: a set of `state`/`state_clause` nodes, may be empty; `binder`; `related`; `constraint` |
| `effect` | `of`: a reference to the `attempt` control the effect is of; `binder`; `related`; `constraint` |
| `event` | `role`: a reference to a role; `compensation`: optional reference to a `protocol`/`compensation` node; `binder`; `related`; `constraint` |
| `commit` | `role`; `binder`; `constraint` |
| `fence` | `ordering`: a text `acquire`, `release`, `acq_rel` or `seq_cst` |

The same bodies state what the source writes and nothing derived: a
`parallel` with no `outstanding` and a `repeat` with no `maximum` carry the
absent optional, not a default
([FR-434](../../functional/memory/FR-434-select-memory-models-for-parallel.md),
[FR-171](../../functional/protocol/FR-171-evaluate-dynamic-control.md)). An empty
`contracts` list explicitly requests no extra state clauses; it does not
bypass the operation's frame.

**Per-operator checks.** The reader checks, for one control, in this order:

1. **References.** Every reference is a declared dependency (a node of the graph absent from `dependencies` is refused `invalid_semantic_graph` at the graph stage, FR-464 "Reader order"), else
   `missing_declaration`/`missing-name` for a target that names no node, and names a node of the form its
   layout states, else `invalid_model_binding`/`malformed-declaration`; both
   located at the target. Where a layout names a control of a given operator,
   the target's operator is that operator: `receive.of` a `send`,
   `effect.of` an `attempt`, `await.after` (kind `event`) one of `send`,
   `receive`, `attempt`, `effect`, `event` and `commit`, and `await.after`
   (kind `compensation`) a `protocol`/`compensation` node.
2. **Awaited event.** The `await`'s `event` is a `receive`, an `effect` or an
   `event` control; a `send` or an `attempt` there refuses
   `unsupported_construct`/`declaration-form` at the `await`, naming the form
   and the selected protocol profile.
3. **Labels.** Two `cases` labels of one `choice`, or two `branches` labels
   of one `parallel`, are equal: `ambiguous_declaration`/`ambiguous-name` at
   the node, naming the label.
4. **Join.** Every `joined` text is a `branches` label, else
   `missing_declaration`/`missing-name` at the text; the texts appear once and
   in ascending label order, else `invalid_semantic_graph`; `all` names every branch,
   else `invalid_package`/`invalid-value` at `joined`; `any` and `predicate`
   name at least one; `quorum` has `1 <= count <= ` the number of joined
   branches, else `invalid_package`/`invalid-value` at `count`. `count` and
   `predicate` are present exactly for their policy, else the layout is violated
   (`malformed_wire`).
   The `predicate` function has one parameter per `joined` text, the `k`-th
   parameter receiving the disposition of the `k`-th text in that ascending
   order, and each parameter type and the result type is the `boolean` type
   node: a parameter is `true` when its branch thread is `completed` and
   `false` when it is `running`, the only two dispositions a join step reads
   ([FR-426](../../functional/protocol/FR-426-apply-protocol-steps.md)). A
   parameter count that differs from the number of `joined` texts, a
   parameter of another type, or a result of another type refuses
   `ill_typed`/`type-mismatch` at `predicate`, retaining `expected` (the
   required parameter and result types) and `actual` (the function's).
5. **Types.** The binder of a `send` and of a `receive` has the `carries`
   node of its channel as `semantic_type`, else `ill_typed`/`type-mismatch`
   at the control, retaining `expected` and `actual`; a `receive` names the
   channel of the `send` it receives, else `ill_typed`/`operator-ineligible`
   at the control. Every Boolean term is Boolean, else `ill_typed`/
   `non-boolean-root` at the slot. Each `contracts` target is a
   `state_clause` node of clause kind `precondition` or `postcondition` whose
   anchor argument
   ([FR-341](FR-341-checked-package-state-clause-body.md)) is the `operation`
   target, else `invalid_model_binding`/`malformed-declaration` at the
   target. Each `related` end term conforms to the endpoint type of its
   relationship ([FR-322](FR-322-checked-package-artifact.md) "Reference
   conformance"), else `ill_typed`/`type-mismatch` at the end, retaining
   `relationship`, `end`, `expected` and `actual`; its `relationship` is a
   `relation`/`relationship` node, else `invalid_model_binding`/
   `wrong-export` at the target, and one the clause's `relationships`
   argument lists, else `missing_declaration`/`missing-name`.
6. **Bounds.** A `maximum` or an interval bound that is negative refuses
   `invalid_package`/`invalid-value` at the value, at strict wire validation, as
   [FR-370](FR-370-checked-package-temporal-clause-body.md) refuses a negative
   bound. An interval with
   `lower > upper` refuses `ill_typed`/`type-mismatch` at the interval,
   retaining `member` (`within`), `expected` (a closed interval with
   `lower <= upper`) and `actual` (the interval as written).

Binder scope, the model-state refusals and the decision facts are slot checks
of the clause, run at FR-464 step 6 in the order FR-464 states; they are not
part of a control's local checks.

The choice owner is one role. Exhaustiveness and exclusiveness of the
guards ([FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md))
are a proof obligation the checker discharged before it emitted the package;
the reader does not re-prove them. The visibility of each distinguishing
fact to the owner is a structural fact the reader does check, as "Decision
facts" states.

## Decision facts

A `choice` and a `repeat` have one owning role and a `visible` list of the
facts the owner observes at the decision. A **decision fact** is an
`expression` node that reads exactly one field of one record binder. A
binder is *eligible* for the owner only as this table gives, and a binder of
another kind never is:

| Binder, introduced by | Eligible when |
| --- | --- |
| a `receive` | the owner is the `to` role of the receive's channel |
| an `attempt` | the attempt's `role` is the owner |
| an `event` | the event's `role` is the owner, with or without a `compensation` |
| a `send`, `effect` or `commit`, and the protocol activation binder | never |

Each `visible` entry reads eligible binders, protocol captures and constants. The protocol input `over` is model state (FR-464): a `visible` entry or a guard that reads it refuses `ill_typed`/`operator-ineligible` at the read, naming the slot, before binder scope and before this table is consulted; it is never a `wrong_snapshot`, and "the protocol input binder" is not a row of the table. The finish binder and the binders of a compensation template are not in the table either: neither is readable in a `run` slot (FR-464 "Readable binders" lists the finish binder for the `finish` stage only and a template's binders for the template's stages only), so a `visible` entry or a guard that reads one refuses `missing_declaration`/`missing-name` as an out-of-scope read (`read` `stage` for the finish binder, `foreign` for a template binder), which precedes this check, and never `wrong_snapshot`. A **bare binder read**, a reference to a record binder (the protocol activation binder included) that projects no field, reads no field and so is no decision fact and has no `read(binder, field)` to retain: in a `choice` or `repeat` `visible` entry or guard it refuses `ill_typed`/`operator-ineligible` at the read, naming the slot, whatever the binder's kind, and never `wrong_snapshot`; the `wrong_snapshot` detail therefore always carries a field, and no field-less or empty-field variant exists. It follows the out-of-scope, `over` and model-state refusals of the slot and precedes the table. A protocol capture is a frozen value every role holds: a guard or a `visible` entry may read one, and it needs no listing. A guard (a `case`
`guard`, a `repeat` `guard`) reaches a binder only through a `visible`
entry node, its closure stopping at the listed entries; a composite entry
makes its whole value visible and none of its operands. A read of a fact
that is not an eligible, individually listed fact of the owner refuses
`wrong_snapshot`/`wrong-observation` at the read, located at the node and
retaining `node`, `owner`, `read` (the binder and field) and `reason`:
`other-role` when the binder is eligible only for another role,
`ineligible-kind` when it is in the table's last row, and `unlisted` when it
is eligible but no individual `visible` entry lists the fact. This is the
catalogued refusal for an unobservable choice
([FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md)): the
catalog pairing `wrong_snapshot`/`wrong-observation` is confirmed for it
and no further cause is added. A binder that is eligible but not readable
at the node, such as a parallel sibling's binder before the join, an optional
branch's binder, an `await` match read outside its `then` path or a binder
bound inside the `repeat` body, refuses as an out-of-scope read ("Binder
scope"), which precedes this check. The decision-fact check runs after the
`over`, model-state and scope refusals of the same slot, for the `visible`
entries first, then the guards in case order. An empty `visible` set is
admitted when the guards read constants and captures only.


## Binder scope

A control reads the binders readable at its entry, `In(n)`, and a step
control also its own binder. `Out(n)` is the set readable after the control
completes on every path. A construct that introduces binders takes its
entry count (`size(In(n))` for a control) as the level of its first binder and
the following levels, in layout order, for its others (FR-464 "Binders").

| Control | Reads and flow |
| --- | --- |
| root | `In(root)` is `C0` |
| `sequence` | `In(child 1)` is `In(sequence)`; `In(child i+1)` is `Out(child i)`; `Out(sequence)` is `Out(last child)`, or `In(sequence)` for no child |
| `choice` | the guards and `visible` read `In(choice)`; `In(case body)` is `In(choice)`; `Out(choice)` is `In(choice)`: a case's binders do not escape it |
| `parallel` | `In(branch body)` is `In(parallel)`; sibling branches read nothing of each other; `Out(parallel)` is `In(parallel)` and the `Out` of every branch for policy `all`, and `In(parallel)` for any other policy |
| `repeat` | the guard and `visible` read `In(repeat)`; `In(body)` and `In(exhausted)` are `In(repeat)`; `Out(repeat)` is `In(repeat)`: iteration binders do not escape |
| `await` | `In(event)` is `In(await)`; `In(then)` is `Out(event)`; `In(timeout)` is `In(await)`; `Out(await)` is `In(await)`: the matched event is readable on `then` only |
| `send`, `receive`, `attempt`, `effect`, `event`, `commit` | `constraint` and `related` read `In(n)` and the node's `binder`; `Out(n)` is `In(n)` and the `binder` |
| `check`, `fence` | `check` reads `In(check)`; `Out` is `In` |

The `Out(root)` set is what `finish` reads
([FR-465](FR-465-checked-package-protocol-declaration-bodies.md)). A read of a
binder outside the readable set, including a sibling branch's binder before
the join and a choice case's binder after the choice, refuses
`missing_declaration`/`missing-name`; two binders of one name readable at one
place refuse `ambiguous_declaration`/`ambiguous-name` (FR-464).

## Compensation

A `protocol`/`compensation` node (operator `compensate`) is a template that
registers one obligation per concrete forward effect. It has semantic_type the
Boolean node, `declaration` `P`, `compensate`, the template's name, and
occurrences of role `declaration`. Its thirteen arguments, in order:

| # | Binding | Value |
| --- | --- | --- |
| 1 | `forward` | a reference to the `effect` control the template is paired with |
| 2 | `forward_binder` | a reference to the binder for the forward effect's record |
| 3 | `role` | a reference to the role that owes the compensation |
| 4 | `operation` | a reference to the `state`/`operation_anchor` node of the compensating operation |
| 5 | `clock` | a text: the clock role |
| 6 | `registration_captures` | ordered records of `binder` and `initializer`, evaluated at registration |
| 7 | `trigger` | a Group: `binder`, the trigger record's binder; `guard`, a Boolean term |
| 8 | `activation_captures` | ordered records of `binder` and `initializer`, evaluated once at activation |
| 9 | `within` | an interval: the activation-relative window |
| 10 | `attempts` | a Group: `maximum`, an integer, at least 1, counting the initial attempt; `record_type`, a reference to the attempt record's type node |
| 11 | `retry` | a Group: `earlier` and `later`, binders of the attempt record type; `relation`, a Boolean term |
| 12 | `commit` | optional: a reference to the `commit` control that forbids later registration and attempts; absent for `commit never` |
| 13 | `recovery` | a Group: `binder`, the recovery view's binder; `predicate`, a Boolean term |

`commit never` is the absent optional; it names no commit and is not an
empty commit. A template does not invoke the compensating operation and
registers nothing at check time; it states the registration, activation,
retry, commit and recovery rules the runtime later applies
([FR-056](../../functional/protocol/FR-056-register-compensation.md),
[FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md),
[FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md),
[FR-427](../../functional/protocol/FR-427-check-protocol-compensation.md)).

**Compensation scope.** A template reads these binders by stage, and no
others; `Caps` is the protocol captures.

| Stage | Where | Binders and captures it may read |
| --- | --- | --- |
| `registration-capture` | registration capture `i` | `forward_binder`, `over`, the activation binder, `Caps` and the registration captures before `i` |
| `trigger-guard` | `trigger.guard` | the trigger `binder`, `Caps` and the registration captures |
| `activation-capture` | activation capture `j` | the trigger `binder`, `over`, `Caps`, the registration captures and the activation captures before `j` |
| `retry` | `retry.relation` | `earlier`, `later`, `Caps`, the registration and the activation captures |
| `recovery` | `recovery.predicate` | the recovery `binder`, `Caps`, the registration and the activation captures |

The forward record is readable in a registration capture and, once captured,
through that capture. The trigger guard, the retry relation and the recovery
predicate bar model state: a read of `over` or of a model-state operation
refuses `ill_typed`/`operator-ineligible` at the read, naming the stage,
before the table is consulted (FR-464); registration and activation captures
may read `over`. A read outside the stage's list refuses as FR-464 "Readable
binders" states, with `read` `forward` for a capture of the same stage written
later than the read or the capture itself, `foreign` for a binder another
template declares, and `stage` otherwise.

The template introduces its binders in layout order: `forward_binder`, the
registration captures, the trigger binder, the activation captures,
`earlier`, `later`, the recovery binder, from the entry count `size(C0)`.

The reader checks, for one template, in this order (the scope checks are slot
checks at FR-464 step 6):

1. **References.** Every reference is a declared dependency and names a node of
   the stated form (as "Per-operator checks" 1): `forward` an `effect`
   control, `role` a role of the clause, `operation` a `state`/
   `operation_anchor` node, `record_type` a `model` node of form
   `record_value_type` or `event_type`, `commit` a `commit` control; the
   target of an `await.after` of kind `compensation` and of
   `event.compensation` is a template of the same clause.
2. **Types.** `forward_binder` has the `semantic_type` of the forward
   effect's own binder, and `earlier` and `later` have the `record_type`
   target as `semantic_type`, else `ill_typed`/`type-mismatch` at the template
   retaining `expected` and `actual`; each Boolean term is Boolean, else
   `ill_typed`/`non-boolean-root` at the slot.
3. **Bounds.** `attempts.maximum` is at least 1, else `ill_typed`/
   `type-mismatch` at `maximum`, retaining `member` (`attempts`), `expected` (a
   count of at least one) and `actual`; `within` is an interval with
   `lower <= upper`, else `ill_typed`/`type-mismatch` at the interval
   retaining `member` (`within`), `expected` and `actual`.
4. **Commit position.** A present `commit` control that *dominates* the
   forward effect refuses `unsupported_construct`/`declaration-form` at
   `commit`, retaining the selected protocol profile, `member` (`commit`), the
   commit, the effect and `reason` `commit-precedes-effect`: every execution
   would take it before the effect registers the obligation. A commit that
   follows the effect, one in a sibling branch of the effect (unordered against
   it), and one in a single case of a `choice` or a single branch of a
   non-`all` join before the effect, are admitted.

   *Dominance.* The set `taken(n)` of commits every execution of control `n`
   takes is: for a leaf, itself if it is a `commit`, else empty; for a
   `sequence`, the union over its children; for a `parallel` with policy `all`,
   the union over every branch (every branch executes, so a commit in any one of
   them is taken); for a `parallel` with any other policy and for a `choice`,
   the intersection over its branches or cases; for an `await`, the intersection
   over its `then` and `timeout`; for a `repeat`, empty (the body may not run).
   A commit `C` dominates the forward effect `E` when some ancestor `sequence`
   of `E` holds, before the child that holds `E`, a child `X` with `C` in
   `taken(X)`. A commit in one branch and `E` in a sibling branch has no such
   sequence, so it is unordered.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-466-AC-1 | Each of the thirteen control operators is admitted with exactly its layout; the arguments reordered, a missing or surplus binding, an unknown operator spelling and an enumerated value outside its set (`memory`, `outstanding`, `policy`, `ordering`, `kind`) refuse `malformed_wire` or refuse `operation-member-mismatch`. | Test (TC-948) |
| FR-466-AC-2 | A `sequence` holds its children in execution order and may hold none; reordering two children changes the node key; a child that is not a `control` node refuses `invalid_model_binding`/`malformed-declaration` at the target, a child that is a node absent from `dependencies` `invalid_semantic_graph` at the `sequence` node's `/dependencies` (graph stage, FR-322-AC-96), and a child that names no node `missing_declaration`/`missing-name`. | Test (TC-948) |
| FR-466-AC-3 | A `choice` is admitted with an owning role, a `visible` set and at least two labelled cases sorted by label; one case, equal labels, cases out of label order, a guard that reads model state and a guard that reads `over` refuse (`malformed_wire`, `ambiguous_declaration`/`ambiguous-name`, `invalid_semantic_graph`, `ill_typed`/`operator-ineligible` at the read); a non-Boolean guard refuses `ill_typed`/`non-boolean-root`; the guards' exhaustiveness is not re-proved. | Test (TC-948) |
| FR-466-AC-4 | A `parallel` is admitted for each join policy; `all` naming every branch, `any` and `predicate` naming at least one and `quorum` with `1 <= count <=` the joined branches; a join naming a missing branch label refuses `missing_declaration`/`missing-name` at the label, `all` omitting a branch and `quorum` outside its range `invalid_package`/`invalid-value`, and a repeated or out-of-order joined label `invalid_semantic_graph`; branches are sorted ascending by label, and a branch list out of that order is `invalid_semantic_graph`. | Test (TC-948) |
| FR-466-AC-5 | A `repeat` is admitted with a maximum and an `exhausted` control, with a `proof`, with both, and with neither; `exhausted` without `maximum` and `maximum` without `exhausted` refuse `malformed_wire`, a body that does not necessarily step refuses `unsupported_construct`/`declaration-form` (FR-464), and `maximum: 0` is admitted. | Test (TC-948) |
| FR-466-AC-6 | An `await` is admitted after an event node and after a compensation, awaiting a `receive`, an `effect` or an `event`; awaiting a `send` or an `attempt` refuses `unsupported_construct`/`declaration-form`; an anchor that is not a step control, or an `await` anchored at a control that follows it, refuses (`invalid_model_binding`/`malformed-declaration`, `invalid_package`/`definition-cycle`). | Test (TC-948) |
| FR-466-AC-7 | A `send` and a `receive` are admitted over one channel with payload binders typed as the channel's `carries`; a binder of another type, a `receive` of a `send` on another channel and a `receive` whose `of` is not a `send` refuse as the reader order states; the pair in the wrong causal order refuses `invalid_package`/`definition-cycle`. | Test (TC-948) |
| FR-466-AC-8 | An `attempt` is admitted with a role, an operation anchor, each memory ordering or none, and `contracts` that are the pre- and postconditions of that anchor; a `contracts` target anchored at another operation, an `invariant` clause and a non-clause node refuse `invalid_model_binding`/`malformed-declaration`; an empty `contracts` list is admitted. An `effect` is admitted of an `attempt` and refuses `invalid_model_binding`/`malformed-declaration` of any other control. | Test (TC-948) |
| FR-466-AC-9 | An `event` is admitted with and without a compensation; a `commit` and a `fence` are admitted; `related` entries name a relationship the clause lists with end terms of the endpoint types, and an unlisted relationship refuses `missing_declaration`/`missing-name` and an ill-typed end `ill_typed`/`type-mismatch`. | Test (TC-948) |
| FR-466-AC-10 | Binder scope: a successor reads a predecessor's binder, an `await`'s `then` reads the matched event, a join of every branch exposes the branches' binders, and `finish` reads the root's `Out` set; a read of a sibling branch's binder before the join, of a choice case's or a repeat iteration's binder after it, of the matched event on `timeout` and of a branch binder after a join whose policy is not `all` refuses `missing_declaration`/`missing-name`. | Test (TC-948) |
| FR-466-AC-11 | A `compensate` template is admitted with `commit` present and absent (`never`), each of the thirteen arguments in order, and with a commit that follows or is unordered against the forward effect; `attempts.maximum: 0` and an inverted `within` refuse `ill_typed`/`type-mismatch` (member, expected, actual), and a commit that dominates the forward effect refuses `unsupported_construct`/`declaration-form` retaining the profile and reason `commit-precedes-effect`, while a commit in one case of a `choice` before the effect is admitted; a `forward` that is not an `effect` control and a `commit` that is not a `commit` control refuse `invalid_model_binding`/`malformed-declaration`; `earlier`/`later` binders of another type refuse `ill_typed`/`type-mismatch`. | Test (TC-948) |
| FR-466-AC-12 | Compensation scope, by the five stages: the trigger guard reads the trigger binder, the protocol captures and the registration captures and refuses the forward binder and the protocol activation binder (`missing_declaration`/`missing-name`) and `over` (`ill_typed`/`operator-ineligible`); an activation capture reads the trigger binder, `over`, the captures and earlier activation captures; the retry relation reads `earlier`, `later` and the captures and refuses the trigger binder (`missing-name`) and `over` (`operator-ineligible`); the recovery predicate reads its binder and the captures and refuses `earlier` (`missing-name`) and `over` (`operator-ineligible`); a registration capture reads the forward binder, `over` and the activation binder; each `missing-name` refusal is at the template retaining name, stage, owner and `read`. | Test (TC-948) |
| FR-466-AC-13 | The worked protocol (roles, a channel and queue, a send and a receive, an attempt and an effect, a parallel of two branches joined `all`, a choice, a repeat, an await, a commit, a compensation and a finish) is admitted; its node keys are unchanged by reformatting and by a second reference, and change with any retargeted reference, renamed control, reordered child or changed literal. | Test (TC-948) |
| FR-466-AC-14 | Decision facts: a `choice` owned by the `to` role of a channel decides on its `receive` record's Boolean field listed individually in `visible`; the other role's receive record, a `send`, `effect` or `commit` binder, a field of the protocol activation binder, an unlisted fact and a fact reachable only inside a composite entry each refuse `wrong_snapshot`/`wrong-observation` with `reason` `other-role`, `ineligible-kind` or `unlisted`; an attempt or event binder is eligible for the role it is `by`; a repeat's `visible` and guard follow the same rule; a binder not readable at the node refuses as an out-of-scope read first. | Test (TC-948) |
| FR-466-AC-15 | A `predicate` join over `joined` `[east, west]` whose `function` node has two `boolean` parameters and a `boolean` result is admitted; the same join with a one-parameter function, a three-parameter function, a second parameter of an integer type, or an integer result refuses `ill_typed`/`type-mismatch` at `predicate`, retaining expected and actual. | Test (TC-948) |
| FR-466-AC-16 | Refusal precedence in one `visible` entry or guard, which fixes FR-466-AC-14 and agrees with FR-464-AC-8 and FR-464-AC-12: a direct read of the protocol input `over` refuses `ill_typed`/`operator-ineligible` at the read, naming the slot, in a `choice` `visible` entry, a `choice` guard, a `repeat` `visible` entry and a `repeat` guard, and is never `wrong_snapshot`/`wrong-observation`; a field of the protocol activation binder, readable at the node, refuses `wrong_snapshot`/`wrong-observation` with `reason` `ineligible-kind`, while a bare read of that binder (or of any record binder) with no field projection refuses `ill_typed`/`operator-ineligible` at the read, naming the slot, in each of the four slots, and is never `wrong_snapshot` (a slot holding both a bare binder read and an in-scope ineligible field read refuses the bare read, `ill_typed`/`operator-ineligible`, whatever their order); a read of the finish binder, or of a binder a compensation template declares, from a `visible` entry or a guard refuses `missing_declaration`/`missing-name` (`read` `stage`, respectively `foreign`) and never `wrong_snapshot`, since neither is readable in a `run` slot; a slot that holds both a read of `over` and a binder read refuses the `over` read, and a slot that holds both an out-of-scope binder read and an in-scope ineligible one refuses the out-of-scope read. Reader evidence is owned by IR (IR-874, FR-038-AC-291/292); TC-948 DF6b and DF7d..DF7h are the published oracle. | Analysis |

## Dependencies

- [FR-464](FR-464-checked-package-protocol-node-bodies.md) owns the node
  family, keys, binders, membership, derived causal relation and reader
  order; [FR-465](FR-465-checked-package-protocol-declaration-bodies.md) the
  declarations these controls name.
- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) and
  [FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md) own the
  control constructs and choice visibility;
  [FR-426](../../functional/protocol/FR-426-apply-protocol-steps.md) the steps the
  nodes denote.
- [FR-056](../../functional/protocol/FR-056-register-compensation.md),
  [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md),
  [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  and [FR-427](../../functional/protocol/FR-427-check-protocol-compensation.md)
  own compensation registration, commit, retry and recovery.
- [FR-342](FR-342-checked-package-operation-anchor-body.md) owns the
  `state`/`operation_anchor` node an `attempt` and a template name, and
  [FR-341](FR-341-checked-package-state-clause-body.md) the state clauses an
  `attempt` lists.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Proposed complete-V1 boundary contract for the control and compensation nodes
of the protocol family. Implementation and qualification remain tracked
separately.
