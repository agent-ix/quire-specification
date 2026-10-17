---
id: FR-426
title: "Apply protocol steps atomically with canonical transition identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-053
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-170
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-171
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-228
    type: references
  - target: ix://agent-ix/quire-specification/FR-434
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
---

# FR-426: Apply protocol steps atomically with canonical transition identities

## Description

When the analyzer expands a protocol state, it SHALL enumerate exactly the
enabled steps of the kinds below, apply each step atomically, settle every
thread the step moves to its next rest point, and name each step by its
protocol transition identity.

## Inputs

A protocol subject and one of its protocol states
([FR-425](FR-425-define-protocol-subjects-and-state-keys.md)).

## Outputs

Every successor of the state, each with its protocol transition identity; or
the FR-181 `ContractUndetermined` finding for an attempt whose contract
conjunction evaluates undecided.

## Behavior

### Settling

After every step, each thread the step moved and each thread the step created
is **settled**: its structural moves are applied until it reaches a rest
point. The structural moves are: enter a `sequence` at its first child, or
leave it when empty; leave a child for its successor, and the last child for
the parent's continuation; at a `choice`, enter the one case whose guard is
true; at a `repeat` with a maximum, enter the body when the guard is true and the
counter is below the maximum, enter `exhausted` when the guard is true at the
maximum, leave when the guard is false, and advance the counter on each
re-entry; at a `repeat` with no maximum, enter the body when the guard is true
and leave when it is false; at a
`check`, continue when its value is true and rest there when it is false; at
the end of a branch body, set the disposition `completed` and rest at end; at
the end of `run`, rest at `finish`.

Choice guards, repeat guards and checks read binders and constants only, each
atom at its binder's observation anchor ([FR-053](FR-053-enforce-choice-visibility.md)).
Binder values are immutable, so settling is a function of the state, and
FR-053 makes exactly one case's guard true. A structural move is never a step
and never a position.

### Step kinds

"At" means the owning thread's rest point is that node. "Gate" is the memory
model's enabledness condition for a thread's step
([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)), true under
`sc`. Kinds `cattempt`, `cend`, `activate`, `spawn` and `retire` are stated
in [FR-427](FR-427-check-protocol-compensation.md) and
[FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md).

| Kind | Owner | Enabled when | Effect |
| --- | --- | --- | --- |
| `attempt` | thread at attempt node `n` | for one FR-181 application of `n`'s operation (receiver and argument vector from the finite domains), the effective precondition over the contract clauses `n` selects is true over the thread's observation; some post-state within the frame satisfies the selected postconditions; the binder record (receiver, arguments, result) satisfies `n`'s constraint; the gate holds | the post-state delta is written through the memory model, in place under `sc`; the binder is bound. One step per (application, post-state, result) |
| `event` | thread at an `effect`, domain `event` or `commit` node `n` | some record of `n`'s record type satisfies `n`'s constraint; for an `effect`, its attempt's binder is bound | the binder is bound; the model state is unchanged. One step per satisfying record |
| `send` | thread at send node `n` | some payload satisfies `n`'s constraint; the channel's capacity and overflow policy admit the send ([FR-170](FR-170-evaluate-channel-queue-semantics.md)); the gate holds | per FR-170: append the entry, or under `reject` at capacity leave the queue unchanged and record the rejection in the binder, or under `loss` leave the queue unchanged and record the loss in the binder; the binder is bound |
| `receive` | thread at receive node `n` | the queue holds a deliverable entry from the send node `n` names (`fifo`: the head of its key; `unordered`: any entry) whose payload satisfies `n`'s constraint; the gate holds | the entry is removed and the binder bound. One step per distinct deliverable entry |
| `duplicate`, `lose` | channel `h` | the delivery policy admits duplication (`unknown`, `at-least-once`) or loss (`unknown`, `at-most-once`) and entry `e` is queued; the channel's protocol instance is not finished; for `duplicate`, capacity admits one more entry | add a copy of `e`, or remove `e` |
| `fork` | thread at `parallel` `p` | the gate holds | creates the parallel instance and one `running` branch thread per branch, each settled from its branch start; the owner rests at `p`'s join; the memory model splits its component |
| `join` | thread at `p`'s join | the join policy holds over the dispositions, a thread counting once `completed` ([FR-171](FR-171-evaluate-dynamic-control.md)); the gate holds | the instance is settled; under `outstanding cancel` every branch thread still `running` becomes `cancelled` in this step; under `outstanding continue` they keep running; the owner settles past the join; the memory model merges its component |
| `timeout` | thread at `await` `a` | always | the owner settles into `a`'s `timeout` control |
| `finish` | root thread at `finish` | every other thread of the instance, compensation threads included, has left `running`, and some record of the finish type satisfies the finish constraint | the binder is bound and the instance is finished; the root rests at end |
| `fence` | thread at `fence` node `n` | its memory model's fence rule | its memory model's fence rule ([FR-435](../memory/FR-435-classify-memory-accesses-and-orderings.md)); none under `sc` |
| `memory` | the memory model | its model's rule | its model's rule (FR-434) |

An await's match is its event node's own step, after which the owner settles
into `then`.

**Waiting.** A thread at an attempt with no enabled application, at a receive
with no deliverable entry, at an unsatisfied join or at a false `check` has
no enabled step and waits. A false `check` never becomes true. The protocol
system reads no clock: `timeout` is enabled for as long as its `await` waits,
which admits every behaviour some deadline admits, and a compensation's
`within` window is not read.

### Undefined protocol expressions

A **protocol expression** is any expression the analyzer evaluates while it
expands or classifies a protocol state: an event, attempt, send, receive or
finish constraint; a choice or repeat guard; a `check`; a predicate join's
predicate; a capture; a compensation activation guard, retry relation or
recovery predicate ([FR-427](FR-427-check-protocol-compensation.md)); an
activation guard or `retire` predicate
([FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md)); and
a `terminal when P` predicate ([FR-429](FR-429-report-protocol-deadlocks.md)).

A protocol expression is read by the one model-side undefined rule of
[FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md), the
rule that also governs an undefined contract, guard, check or `terminal when`
predicate over a model subject
([FR-366](../temporal/FR-366-check-deadlock-freedom.md)): when a protocol
expression evaluates to the kernel's undefined value at a reachable state,
each item over the subject settles `refuted` with cause
`undefined-evaluation`. The undefined value is never read as `false`, as a
disabled step, or as an `unrecovered` status. The counterexample is a finite
prefix from an initial state to the state at which the expression was
evaluated, carrying the
[FR-364](../temporal/FR-364-replay-temporal-counterexamples.md)
`{"type":"undefined-evaluation", ...}` envelope, with `where` naming that
position, the state's `state_key` and the expression's source locus, and
`cause` the undefined value's catalogued reason. It is refuting evidence like
any other: an engine reports the first refuting evidence it finds.

### Atomicity

Every step evaluates its enabledness, applies its effect and settles its
threads with no step of another thread or of the memory model inside it. An
attempt is one step: precondition, choice of post-state and result, frame
check, binder constraint and write happen together, by the same successor
rule FR-181 applies to a model subject. A thread is never inside an attempt
between two positions.

### Protocol transition identity

Each step's transition identity is the typed canonical object
`{"type":"protocol-transition","step":"<kind>", ...}` with these members,
members not listed for a kind being absent:

| Member | Present for | Value |
| --- | --- | --- |
| `step` | every step | one of `attempt`, `event`, `send`, `receive`, `duplicate`, `lose`, `fork`, `join`, `timeout`, `finish`, `fence`, `cattempt`, `cend`, `activate`, `spawn`, `retire`, `memory` |
| `thread` | every thread-owned step | the owner's thread path (FR-425) |
| `node` | `attempt`, `event`, `send`, `receive`, `fork`, `join`, `timeout`, `finish`, `fence` | the node's static declaration path |
| `operation` | `attempt`, `cattempt` | the operation's qualified name |
| `receiver` | `attempt`, `cattempt` | the receiver reference |
| `arguments` | `attempt`, `cattempt` | the argument vector, in parameter order |
| `record` | `event`, `send`, `receive`, `finish`, `activate` | the bound record or payload |
| `actor` | a step by a replicated role | the acting instance's object reference |
| `channel` | `duplicate`, `lose` | the channel's qualified name and instance ordinal |
| `entry` | `duplicate`, `lose` | the queue entry |
| `template`, `registration` | `cattempt`, `cend` | the template's qualified name and the registration key |
| `role`, `object` | `spawn`, `retire` | the role and the object reference |
| `ordering` | `fence`, and an access with an ordering | the ordering |
| `memory_choice` | an access whose memory model chooses a message or slot | the choice its memory model names |
| `memory_step` | `memory` | the identity its memory model gives |

Several successors with one identity are told apart by their post-state's
`quire.simulation.state-key/v1` digest. Successors are visited in ascending
JCS bytes of the identity, as FR-181 states for model transition identities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-426-AC-1 | The two-branch `Fill` protocol (TC-376) reaches exactly seven states and six transitions from `c.v = 0`: one `fork`, both orders of the two attempts where enabled, one `join` and one `finish`, with the listed enabled steps at each state. | Test (TC-376) |
| FR-426-AC-2 | Each join policy `all`, `any`, `quorum(2)` of three and a predicate join is enabled exactly at its threshold disposition and not before, under `outstanding continue` the remaining branches keep taking steps after the join, and under `outstanding cancel` they become `cancelled` in the join step and take no further step. | Test (TC-377) |
| FR-426-AC-3 | A `fifo` channel delivers only the head of each key and an `unordered` channel any entry; `duplicate` and `lose` are enabled exactly under the delivery policies that admit them; a `block` send at capacity waits and a `reject` send at capacity records the rejection. | Test (TC-378) |
| FR-426-AC-4 | Choice, repeat and check moves produce no transition identity and no position; a repeat at its maximum with a true guard enters `exhausted`; a false `check` leaves its thread waiting with no enabled step. | Test (TC-376) |
| FR-426-AC-5 | An attempt never interleaves with another step: its precondition is read and its post-state written at one position, and its successors are exactly the FR-181 successors of that application with the binder constraint applied. | Test (TC-376) |
| FR-426-AC-6 | Every step carries a protocol transition identity with exactly the members its kind lists; successors are visited in ascending JCS bytes of the identity, and two successors with one identity are distinguished by post-state digest. | Test (TC-376, TC-387) |
| FR-426-AC-7 | An `await` whose match is never enabled has its `timeout` step enabled at every state where it waits, with no clock read. | Test (TC-378) |
| FR-426-AC-8 | A repeat with no maximum keeps no counter in `bounds`, re-enters its body while its guard is true and leaves it when the guard is false; a loop whose guard stays true closes a cycle through the state key, so its reachable state set is finite when its body's states are. | Test (TC-376) |
| FR-426-AC-9 | `finish` is not enabled while a branch thread under `outstanding continue` or a compensation thread of the instance is `running`, even when a record satisfies the finish constraint; it is enabled once every such thread has left `running`. | Test (TC-377, TC-380) |
| FR-426-AC-10 | An attempt constraint that reads `lookup<T>(p, r) absent undefined` at a key the population does not hold, at a reachable state, settles every item over the subject `refuted` with cause `undefined-evaluation`, a prefix ending at that state, and `where` naming the constraint's locus; the step is not treated as disabled. | Test (TC-376) |

## Dependencies

- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) owns the
  protocol state, threads and paths.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  application successor rule, digest and canonical order.
- [FR-170](FR-170-evaluate-channel-queue-semantics.md) and
  [FR-171](FR-171-evaluate-dynamic-control.md) own channel and join policy
  meaning.
- [FR-434](../memory/FR-434-select-memory-models-for-parallel.md) owns the
  gate, split, merge, write and memory steps.

## References

- Design: quire-spec-language ADR-027 FO-1, FO-2, ST-1 to ST-10, AT-1 to AT-3,
  TS-2, TS-3, QS-2 and QS-4.
- Tickets: Linear STD-140; paired QSL-396.
