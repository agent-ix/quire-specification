---
id: FR-482
title: "Checked-package schedulability claim body and obligation identity"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-419
    type: references
  - target: ix://agent-ix/quire-specification/FR-142
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-475
    type: references
---

# FR-482: Checked-package schedulability claim body and obligation identity

## Description

When a checked package carries a `schedulable` or `response` claim over a
task set ([FR-419](../../functional/analysis/FR-419-analyze-task-set-schedulability.md)),
the producer SHALL emit it as one `quire.checked-semantic-graph/v2` node with
`node_tag: "claim"` and `semantic_form: "schedulability"`, encode its `body`
as the aggregate below, with the whole task set inside it, and compute its
obligation identity over every member that changes the claim's meaning. The
reader SHALL refuse a node whose rules below do not hold, with the first
refusal in the reader order below.

## Properties

- **body** — `{term: "aggregate", members: [binding("kind", …),
  binding("policy", …), binding("task", …), binding("bound", …),
  binding("tasks", …)]}`, in this order:
  - `kind` — a `literal` text term, `schedulable` or `response`;
  - `policy` — a `literal` text term, `fixed-priority`, `edf` or `amc`, the
    claim's `under` policy (`fixed_priority` in source is `fixed-priority`);
    a `response` claim's policy is `fixed-priority`;
  - `task` — for `response`, a `literal` text term, the name of the task
    the bound is about; for `schedulable`, the empty aggregate
    `{term: "aggregate", members: []}`;
  - `bound` — for `response`, a `quantity` literal, the bound `d`; for
    `schedulable`, the empty aggregate;
  - `tasks` — an aggregate of one task aggregate per task of the task set,
    at least one, ascending by task name in Unicode scalar value order, so
    the order in which the source lists the tasks does not enter the node.
- **task aggregate** — `{term: "aggregate", members: [binding("name", …),
  binding("arrival", …), binding("period", …), binding("deadline", …),
  binding("wcet", …), binding("wcet_hi", …), binding("criticality", …),
  binding("priority", …), binding("jitter", …), binding("blocking", …)]}`,
  in this order:
  - `name` — a `literal` text term, the task's name;
  - `arrival` — a `literal` text term, `periodic` or `sporadic`;
  - `period` — a `quantity` literal, `T` (the period, or the minimum
    inter-arrival time of a sporadic task);
  - `deadline` — a `quantity` literal, `D`;
  - `wcet` — a `quantity` literal, `C`, which is `C(LO)` for a `hi` task;
  - `wcet_hi` — for a `hi` task a `quantity` literal, `C(HI)`; for a `lo`
    task the empty aggregate;
  - `criticality` — a `literal` text term, `lo` or `hi`;
  - `priority` — an `integer` literal, the declared priority, or the empty
    aggregate for a task that declares none;
  - `jitter` and `blocking` — `quantity` literals, `J` and `B`; a task that
    declares neither has `0` for each, so a task written without `jitter`
    encodes identically to the same task written `jitter 0 ms`.
- **time values** — every `quantity` literal of the body (`bound` and each
  task's `period`, `deadline`, `wcet`, `wcet_hi`, `jitter` and `blocking`)
  is typed by one `scalar_type`/`unit` node, the task set's unit, a unit of
  the package's time dimension ([FR-142](../../functional/type-model/FR-142-evaluate-quantities-and-units.md)).
  The frontend chooses that unit and converts every source value into it
  exactly (QSL FR-246); the literal's `value` is the exact rational
  magnitude in that unit, spelled as [FR-322](FR-322-checked-package-artifact.md)
  "Scalar literal values" says.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **declaration** — none. The node has a derived node id, as a
  `claim`/`state_graph` node is ([FR-475](FR-475-checked-package-state-graph-claim-body.md)):
  it carries no `declaration` and no `owner`, two claims with equal checked
  content share one node, and each declaration is one `claim` occurrence of
  it, told apart by its occurrence ordinal. The claim's name and the task
  set's name are not in the body.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node, so a provider request names a schedulability
  item as `claim {node, occurrence}` ([FR-331](FR-331-backend-provider-envelope.md)).
- **dependencies** — `[]`. The body holds literals and aggregates only; a
  literal's `type` is a type annotation, not a dependency (FR-322
  `dependencies`).

## Obligation identity

The obligation identity of a schedulability item binds the node's whole
body: the claim's `kind` and `policy`, the `response` task and bound, and
every task with its arrival, `T`, `D`, `C`, `C(HI)`, criticality, priority,
`J` and `B` in the task set's unit. These are the claim's stated premises
(FR-419 "Premises"), so changing any one gives a different node id and a
different obligation identity, and a verdict holds for exactly those
values. Two claims that differ only in policy, such as `under fixed_priority`
and `under edf` over one task set, have different node ids and different
obligation identities. Two claims that differ only in source spelling with
the same checked meaning (whitespace and comments, the order the tasks are
listed, an omitted `jitter` or `blocking` against an explicit zero, the
claim's or the task set's name) share one node id and one obligation
identity. A task set has no other subject: no model, profile, clock or
initial state enters the identity.

## Reader order

The schedulability step is the last of the body steps, after the protocol
step, in the one order [FR-322](FR-322-checked-package-artifact.md) "Body
steps" fixes. It visits every `claim`/`schedulability` node in ascending
ascending node-id order. For one node, the checks run in this order and the
first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the five bindings in
   order, `tasks` is an aggregate of at least one member, and each member of
   `tasks` is an aggregate of exactly the ten task bindings in order, else
   `invalid_semantic_graph` at the first aggregate, in body order, that
   breaks the rule.
2. **Literals.** `kind`, `policy`, each task's `arrival` and each task's
   `criticality` are text literals with one of their listed values, each
   task's `name` is a non-empty text literal, and each `integer` `priority`
   is not negative, else `invalid_package`/`invalid-value` at the first such
   literal in body order.
3. **Form fit**, in body order: for `schedulable`, `task` and `bound` are
   the empty aggregate; for `response`, `policy` is `fixed-priority`, `task`
   is a text literal and `bound` is a `quantity` literal; for each task,
   `wcet_hi` is a `quantity` literal when `criticality` is `hi` and the empty
   aggregate when it is `lo`, `priority` is an `integer` literal or the empty
   aggregate, and every other time value is a `quantity` literal. Else
   `invalid_semantic_graph` at the binding that breaks the rule.
4. **Task order.** Task names are strictly ascending in Unicode scalar value
   order, else `invalid_semantic_graph` at the `tasks` binding; and a
   `response` claim's `task` names one of them, else `invalid_semantic_graph`
   at the `task` binding.
5. **Unit.** Every time value is typed by the type of the first time value
   in body order, and that type is a `scalar_type`/`unit` node of the
   package's time dimension (FR-142), else `ill_typed`/`type-mismatch` at
   the first time value that breaks the rule.
6. **Task set.** The task set meets every condition of FR-419 "Task sets"
   for the claim's policy: positive `T`, `D` and `C`, `J` and `B` not
   negative and `C(LO) <= C(HI)` for a `hi` task; under `fixed-priority`
   and `amc`, a `priority` on every task and no two tasks with equal
   priorities; under `edf`, `J` and `B` zero; and under `amc`, `D <= T`. Else
   `invalid_timed_model`/`invalid-task-set` at the first task aggregate in
   body order that breaks a condition, or, for two equal priorities, at the
   second of the two.

A `dependencies` list other than `[]` is FR-322's graph-shape refusal,
`invalid_semantic_graph` at the node's `/dependencies`, which precedes this
step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-482-AC-1 | Over TC-372's `Ctl` in a unit that declares the unit `ms` of the dimension `Time`, `schedulable CtlFp on Ctl under fixed_priority;` produces one `claim`/`schedulability` node whose body has exactly the bindings `kind`, `policy`, `task`, `bound` and `tasks` in that order, with `kind` `schedulable`, `policy` `fixed-priority`, `task` and `bound` the empty aggregate, and `tasks` the three task aggregates `t1`, `t2`, `t3`, each with its ten bindings: `arrival` `periodic`, `period` `4`, `6`, `12`, `deadline` equal to the period, `wcet` `1`, `2`, `5`, `wcet_hi` the empty aggregate, `criticality` `lo`, `priority` `1`, `2`, `3`, and `jitter` and `blocking` `0`, every time value a `quantity` literal typed by the `ms` unit node. The node has `semantic_type` the `scalar_type`/`boolean` node, no `declaration`, no `owner`, one occurrence of role `claim` and `dependencies` `[]`. `response T3 on Ctl.t3 <= 12 ms under fixed_priority;` produces a node with `kind` `response`, `policy` `fixed-priority`, `task` the text `t3` and `bound` the `quantity` literal `12`. | Test (TC-967) |
| FR-482-AC-2 | `CtlFp` and `schedulable CtlEdf on Ctl under edf;` have different node ids and different obligation identities; `CtlFp` over `Ctl` with `t3`'s `wcet` 6 ms has a different node id and obligation identity; `CtlFp` over `Ctl` with its tasks listed `t3`, `t1`, `t2`, with `jitter 0 ms` added to `t1`, with other whitespace and comments, or with the claim named `Other` has `CtlFp`'s node id and obligation identity; one unit declaring `CtlFp`'s claim twice, as `CtlFp` and `CtlFp2`, has one node with two `claim` occurrences, ordinals 0 and 1. | Test (TC-967) |
| FR-482-AC-3 | Fed to the reader, each edit of `CtlFp`'s node refuses with the code at the locus stated: `tasks` with no member, `invalid_semantic_graph` at `tasks`; `policy` and `kind` swapped in order, `invalid_semantic_graph` at the body; `policy` `rate-monotonic`, `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/1`; `kind` `response` with `task` still the empty aggregate, `invalid_semantic_graph` at the `task` binding; `t1`'s `wcet_hi` a `quantity` literal with `criticality` `lo`, `invalid_semantic_graph` at `t1`'s `wcet_hi` binding; tasks in the order `t2`, `t1`, `t3`, `invalid_semantic_graph` at `tasks`; `T3`'s `task` the text `t9`, `invalid_semantic_graph` at the `task` binding; `t2`'s `period` typed by a unit of length, `ill_typed`/`type-mismatch` at that literal; `t1`'s `period` `0`, `invalid_timed_model`/`invalid-task-set` at `t1`'s aggregate; `t2`'s `jitter` `-1`, `invalid_timed_model`/`invalid-task-set` at `t2`'s aggregate; `t3`'s `priority` `1`, `invalid_timed_model`/`invalid-task-set` at `t3`'s aggregate; `dependencies` naming the `ms` unit node, `invalid_semantic_graph` at `/dependencies`. The same equal-priority edit of `CtlEdf`'s node is admitted. | Test (TC-967) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `schedulability` among the
`claim` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the checked-package node
  form, node keys, scalar literal spellings, `dependencies` and the reader's
  refusal order.
- [FR-419](../../functional/analysis/FR-419-analyze-task-set-schedulability.md)
  states the task set, the claims and their conditions;
  [FR-142](../../functional/type-model/FR-142-evaluate-quantities-and-units.md) the time
  dimension and exact unit conversion.
- [FR-475](FR-475-checked-package-state-graph-claim-body.md) is the sibling
  body of the `claim`/`state_graph` node.

## References

- STD-375 (this requirement); quire-spec-language FR-246 (S3 check, the
  task set's unit) and FR-454 (S4 emission and the I2 reader).
