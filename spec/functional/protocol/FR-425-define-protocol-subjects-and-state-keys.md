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
  - target: ix://agent-ix/quire-specification/FR-050
    type: references
  - target: ix://agent-ix/quire-specification/FR-466
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
  reused once that instance is removed and no thread below it is live.
- A fork **holds** its ordinal while its parallel instance is live and, after
  the instance is removed, while any live thread's path extends one of the
  instance's branch paths, as a thread forked by an inner `parallel` under an
  `outstanding continue` join can. A **sibling** of a fork is another holder of
  an ordinal of the same `parallel` node under the same parent thread.
- A compensation thread holds its ordinal while it is live. A sibling of a
  compensation thread is a live compensation thread of the same template in the
  same instance, and the thread an activation creates takes the smallest
  ordinal no such sibling holds.

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
  immutable. A `send` binds its binder to the send record
  ([FR-426](FR-426-apply-protocol-steps.md)).
- Each live thread's control state holds a **binder environment**: a map from
  each binder node the thread may read to the bound value. A thread reads a
  node's binder as its own environment's value.
  - An event step adds its binder to its thread's environment, replacing an
    earlier entry for the node.
  - A `fork` copies the parent's environment into each new branch thread.
  - A `join` with policy `all` adds to the joining thread's environment the
    entries that each joined branch thread, or a thread below it, bound after
    the fork, so a branch binder stays readable after an `all` join
    ([FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md)'s
    `Out` of an `all`-joined `parallel`).
  - A compensation thread starts with an empty environment
    ([FR-427](FR-427-check-protocol-compensation.md)).
  - Each thread holds its own copy of a value, so a rebinding, a re-fork or an
    ordinal's reuse never changes what another thread reads.
- A thread keeps an environment entry while it **reaches** a reader of the
  node, and drops the entry in the step after which it reaches none. A thread
  reaches the tree-order interval from its rest point to the end of its scope
  (the nearest enclosing `branch`, or the whole `run` and `finish` for a root
  thread), nested branch bodies included. The interval is empty for a rest at
  or before the node's binding node, and a `repeat`'s back edge widens it to the
  whole body of the enclosing `repeat`. A branch thread of an `all` join also
  reaches on into everything its joiner reaches, for the entries it bound or
  that threads below it bound, so an entry of a branch of an inner `all`-joined
  `parallel` is kept through the enclosing branch and up to the outermost
  joiner; and a joining thread keeps an entry through a later fork whose
  branches read it, because that fork copies it. So a branch of a later
  `parallel` reads the binder of an earlier `all`-joined `parallel` through the
  environment its forking thread copied to it.
- A `repeat` with an authored maximum keeps one counter per (repeat node,
  thread path) while the thread is inside it. A `repeat` with no maximum keeps
  none. Every cycle of a repeat's body graph passes through a step node
  ([FR-052](FR-052-represent-bounded-control.md)), so every cycle of the
  protocol system's state graph through a repeat takes at least one step.
- Each channel has one queue per protocol instance: a sequence in delivery
  order per FIFO key for `fifo` ordering, and a bag for `unordered`. An entry
  is (send node, sending thread path, payload). A queue is bounded only by the
  channel's authored capacity. A channel is named by its **qualified name**,
  the qualified protocol-definition identity
  ([FR-050](FR-050-bind-protocol-instances.md)) followed by the channel's
  declaration-local handle, so two protocols that each declare a channel `Pay`
  have different channels, queues, transition identities and wait causes.

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
| `observations` | `{"type":"binder","node":"<node>","thread":<thread path>}` | the binder's record, one entry for each (binder node, thread whose environment holds it), each environment pruned by the reach rule |
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
| FR-425-AC-3 | A binder no reachable node reads is absent from `observations` in the post-state of the step that binds it; a branch binder read after an `all` join stays present, under the branch's path until the join and under the joiner's after it, until the reading node's step. | Test (TC-375) |
| FR-425-AC-4 | The protocol clause outside its `memory` clauses, its bindings, its `terminal` and `scheduling` members and the resolved memory models each change the obligation identity when changed, a source `memory` clause changes it only through the resolved model, and the `on origin` subject has exactly one instance per initial model state. | Test (TC-375) |
| FR-425-AC-5 | A subject whose replicated role population or binder record type is unbounded returns `requires-bound` before any expansion, naming the root. | Test (TC-375) |
| FR-425-AC-6 | With `maximum 3` and instances 0 and 1 of a `parallel` both live under `outstanding continue`, when instance 0 ends and is removed while instance 1 still runs, the third fork takes ordinal 0, not 2. When the first instance's branch has forked an inner `parallel` whose branch is still `running` after the first instance's join settled, the second fork takes ordinal 1, because that thread's path extends the first instance's branch path, and takes ordinal 0 once that thread completes. | Test (TC-375) |
| FR-425-AC-7 | A template with two registrations, whose first activates while its compensation thread is live: the second compensation thread takes ordinal 1; when `cend` has removed the first thread before the second activates, it takes ordinal 0. | Test (TC-375) |
| FR-425-AC-8 | In a `repeat` of maximum 2, `parallel P` with an `all` join binds `a` in a branch, then `parallel Q` with branches `u` (`attempt U1`, then a `check` reading `a`) and `v`, `join any … outstanding continue`: after `join(P)` the forking thread's environment holds `a`, and `fork(Q)` copies it into `u` and `v`. In iteration 2, with `Q`'s first instance still live, `fork(Q)` takes ordinal 1 and gives its `u` an environment holding iteration 2's `a`; iteration 1's `u` still evaluates its `check` over iteration 1's `a`, no state refuses, and when iteration 1's `u` completes its entry is dropped from `observations`. | Test (TC-375) |
| FR-425-AC-9 | An entry bound in a branch of an inner `all`-joined `parallel` and read after the outermost join is held by the inner branch, then by the enclosing branch, then by the outermost joiner, and is absent from `observations` in the step after the reading node's step. A joining thread whose entry is read only by branches of a later `parallel` keeps it through that fork, which copies it into each branch; a thread resting at or before the node's binding node holds no entry for it. | Test (TC-375) |
| FR-425-AC-10 | Two protocol definitions of one package that each declare `channel Pay` give subjects whose `queues` entries and `duplicate` and `lose` identities hold channel names in each protocol's own qualified name; no key or identity of one names the other's channel. | Test (TC-375) |

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
