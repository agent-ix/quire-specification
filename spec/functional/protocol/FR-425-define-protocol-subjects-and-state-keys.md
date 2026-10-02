---
id: FR-425
title: "Define protocol subjects and their state keys"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
  - target: ix://agent-ix/quire-specification/FR-427
    type: references
  - target: ix://agent-ix/quire-specification/FR-428
    type: references
  - target: ix://agent-ix/quire-specification/FR-434
    type: references
---

# FR-425: Define protocol subjects and their state keys

## Description

When a temporal or refinement item is checked over a protocol clause, the
analyzer SHALL build the protocol subject and its protocol states as stated
below, and SHALL key every protocol state by the FR-181 simulator state key
with the member contents stated below.

## Inputs

A checked package with one checked protocol clause; a binding of the clause's
`over` parameter to a value of its declared type; a binding of each static
role; the universe of each replicated role's population; the ordered list of
initial model states, universes and substituted proof bounds of the model
subject the clause's roles act on; and the memory model of each `parallel`
([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)).

## Outputs

The protocol subject, its initial protocol states and the state key of every
protocol state; or `requires-bound` for an unbounded root.

## Behavior

### Protocol subject

A **protocol subject** is a model subject together with one checked protocol
clause of its package, the `over` binding, the static role bindings and the
universe of each replicated role's population. The protocol's steps
([FR-426](FR-426-apply-protocol-steps.md)) are the only steps of the subject:
the model's operations run only as the protocol's attempts. The protocol
clause, the bindings, the clause's `terminal` and `scheduling` members
([FR-429](FR-429-report-protocol-deadlocks.md),
[FR-430](FR-430-apply-protocol-fairness.md)) and the resolved memory model of
every outermost `parallel` are part of the subject, and so of every
obligation identity that reads it. The obligation identity reads each
`parallel`'s resolved memory model in place of its source `memory` clause
([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)); the source
clause is package content and enters the package identity.

A clause with activation `on origin` runs one **protocol instance**, ordinal
0, from each initial model state. A clause with activation `on each` starts
one protocol instance per `activate` step
([FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md)) and
has no instance in its initial state.

Before any expansion, the analyzer SHALL classify every root of the subject:
the model's populations, parameter and result domains under the request's
universes; the record type of every binder an event, send, receive, finish,
compensation-attempt or activation step ranges over; and each replicated
role's population universe. An unbounded root returns the FR-181
`requires-bound` disposition.

### Threads and paths

- Each protocol instance's `run` control is its **root thread**, with path
  `(instance ordinal)`.
- Each `fork` of a `parallel` creates one **branch thread** per branch. Its
  path is its parent's path followed by (parallel node, branch label,
  ordinal).
- Each activated compensation registration creates one **compensation
  thread** ([FR-427](FR-427-check-protocol-compensation.md)). Its path is its
  instance's path followed by (compensation template, ordinal).
- Every ordinal, of an instance, a branch thread, a parallel instance, a
  compensation thread or a registration, is the smallest natural number that
  no live sibling of the same node and key prefix holds. So a `parallel`
  inside a `repeat` whose earlier instance still runs under `outstanding
  continue` gets a second instance with its own paths, and the ordinal is
  reused once that instance is removed.

`parallel` controls nest to any depth. Thread, instance and registration
counts have no limit of their own; the run's resource limits and the instance
budget of FR-428 bound a run.

### Rest points and dispositions

Each live thread has one **rest point**: an event node, a `parallel` waiting
to fork, the join of a `parallel` it forked, an `await`, a `check` whose value
is false, the `finish` node (root thread only), a compensation attempt or
compensation retry point (compensation threads only), or **end**. Each live
thread has a **disposition**: `running`, `completed` or `cancelled`. A thread
is live while it is `running`, or while its parallel instance's join has not
yet settled.

A **parallel instance** is keyed by (parent thread path, `parallel` node,
ordinal) and holds its branch threads' dispositions and a settled flag. It is
removed once its join has settled and every branch thread has left
`running`.

### Binders, counters, queues and roles

- An event step binds its binder to the event's record. A binder value is
  immutable. It is kept while a rest point of a live thread can still reach a
  node that reads it under the protocol's scope rules, and is removed in the
  step that makes it unreachable.
- A `repeat` with an authored maximum keeps one counter per (repeat node,
  thread path) while the thread is inside it. A `repeat` with no maximum keeps
  none. Every cycle of a repeat's body graph passes through a step node
  ([FR-052](FR-052-represent-bounded-control.md)), so every cycle of the
  protocol system's state graph through a repeat takes at least one step.
- Each channel has one queue per protocol instance: a sequence in delivery
  order per FIFO key for `fifo` ordering, and a bag for `unordered`. An entry
  is (send node, sending thread path, payload). A queue is bounded only by the
  channel's authored capacity.

### State and key

A **protocol state** is (model state, protocol instances, rest points and
dispositions, parallel instances, compensation registrations, binders,
iteration counters, queues, role instances, memory component). Its key is the
FR-181 simulator state key with these member contents:

| Member | Entry key | Entry value |
| --- | --- | --- |
| `semantic` | — | the model state's population map, as for a model subject |
| `control` | `{"type":"protocol-instance","ordinal":"<decimal>"}` | `{"type":"instance-state","finished":true\|false}` |
| `control` | a thread path | `{"type":"thread-state","rest":<rest>,"disposition":"running"\|"completed"\|"cancelled"}` |
| `control` | `{"type":"parallel-instance","parent":<thread path>,"node":"<node>","ordinal":"<decimal>"}` | `{"type":"parallel-state","settled":true\|false,"dispositions":<map from branch label text to disposition text>}` |
| `control` | `{"type":"registration","template":"<qualified-name>","thread":<thread path>,"ordinal":"<decimal>"}` | `{"type":"registration-state","status":<status>,"captures":<record>}` (FR-427) |
| `observations` | `{"type":"binder","node":"<node>","thread":<thread path>}` | the binder's record |
| `bounds` | `{"type":"repeat-counter","node":"<node>","thread":<thread path>}` | the counter, an integer |
| `bounds` | `{"type":"attempt-count","thread":<thread path>}` | a compensation thread's attempt count, an integer |
| `queues` | `{"type":"queue","instance":"<decimal>","channel":"<qualified-name>","fifo_key":<encoded>\|null}` | `sequence` (`fifo`) or `bag` (`unordered`) of `{"type":"queue-entry","send":"<node>","thread":<thread path>,"payload":<encoded>}` |
| `roles` | `{"type":"role","instance":"<decimal>","role":"<ident>"}` | a static role's binding, or a replicated role's instance map (FR-428) |
| `memory` | the members its memory model names | empty under `sc` (FR-434) |

- `<node>` is the node's static declaration path ([FR-221](../../objects/protocol/FR-221-protocol-node-occurrence.md)).
- A thread path is `{"type":"thread-path","instance":"<decimal>","steps":[<step>, ...]}`,
  each step `{"node":"<node>","branch":"<label>","ordinal":"<decimal>"}` for a
  branch thread or `{"template":"<qualified-name>","ordinal":"<decimal>"}` for
  a compensation thread, outermost first.
- `<rest>` is `{"type":"rest","kind":<kind>,"node":"<node>"|null}` with
  `kind` one of `node`, `fork`, `join`, `retry-point` and `end`; `node` is
  null only for `end`.

Each map is sorted by its keys' JCS bytes, as FR-181 states. Two protocol
states with equal keys are one state. The ordinal rule, the removal of parallel
instances, binders and finished instances make the key a function of what the
protocol can still do, not of how it got there.

### Initial states

For each initial model state, an `on origin` subject's initial protocol state
holds that model state, instance 0 with its root thread settled from the
start of `run` ([FR-426](FR-426-apply-protocol-steps.md)), empty queues, no
binders and the static role bindings. An `on each` subject's initial protocol
state holds the model state and no instance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-425-AC-1 | Two behaviours that reach the same model state, rest points, dispositions, binders, counters, queues and role instances by different interleavings produce byte-identical state keys and are coalesced into one state; states that differ in any one of those members have distinct keys. | Test (TC-375) |
| FR-425-AC-2 | A `parallel` nested three deep inside a `repeat` with `outstanding continue` gives a second live parallel instance ordinal 1 while ordinal 0 still runs, and reuses ordinal 0 after the first instance is removed, so the loop's reachable state set is finite. | Test (TC-375) |
| FR-425-AC-3 | A binder no reachable node reads is absent from `observations` in the post-state of the step that binds it; a branch binder read after an `all` join stays present until the reading node's step. | Test (TC-375) |
| FR-425-AC-4 | The protocol clause outside its `memory` clauses, its bindings, its `terminal` and `scheduling` members and the resolved memory models each change the obligation identity when changed, a source `memory` clause changes it only through the resolved model, and the `on origin` subject has exactly one instance per initial model state. | Test (TC-375) |
| FR-425-AC-5 | A subject whose replicated role population or binder record type is unbounded returns `requires-bound` before any expansion, naming the root. | Test (TC-375) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the state-key
  envelope, the typed canonical value form and the map ordering.
- [FR-052](FR-052-represent-bounded-control.md) owns the causal control graph
  the threads walk.
- [FR-426](FR-426-apply-protocol-steps.md) owns the steps; FR-427 and FR-428
  own registrations, compensation threads, instances and replicated roles.
- [FR-434](../memory/FR-434-select-memory-models-for-parallel.md) owns the
  memory model and the `memory` member's content.

## References

- Design: quire-spec-language ADR-027 PS-1 to PS-8, FO-3, AE-2, QS-1 and QS-3.
- Tickets: Linear STD-140; paired QSL-396.
