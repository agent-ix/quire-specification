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
  - target: ix://agent-ix/quire-specification/FR-435
    type: references
  - target: ix://agent-ix/quire-specification/FR-466
    type: references
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-436
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
re-entry, the counter being 0 on each entry from outside the `repeat`; at a `repeat` with no maximum, enter the body when the guard is true
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
| `event` | thread at an `effect`, domain `event` or `commit` node `n` | some record of `n`'s record type satisfies `n`'s constraint; for an `effect`, its attempt's binder is bound in the thread's binder environment (after the join of an `all`-joined `parallel` whose branch holds the attempt, the binder condition holds, because the join added the binder) | the binder is bound; the model state is unchanged. One step per satisfying record |
| `send` | thread at send node `n` | some payload satisfies `n`'s constraint; the channel's capacity and overflow policy admit the send ([FR-170](FR-170-evaluate-channel-queue-semantics.md): under `block`, not while the queue is at capacity); the gate holds | per FR-170: append the entry, or under `reject` at capacity leave the queue unchanged, or under `loss` apply the loss selector (`head` removes the oldest queued entry and appends the arriving one, `tail` removes the newest and appends the arriving one, `new` drops the arriving one and leaves the queue unchanged); the binder is bound to the payload and the **send record** described below is recorded |
| `receive` | thread at receive node `n` | the queue holds a deliverable entry from the send node `n` names (`fifo`: the head of its key; `unordered`: any entry) whose payload satisfies `n`'s constraint; the gate holds | the entry is removed and the binder bound. One step per distinct deliverable entry |
| `duplicate`, `lose` | channel `h` | the delivery policy admits duplication (`unknown`, `at-least-once`) or loss (`unknown`, `at-most-once`); a delivery count interval ([FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md)) admits neither; and entry `e` is queued; the channel's protocol instance is not finished; for `duplicate`, capacity admits one more entry | add a copy of `e`, or remove `e` |
| `fork` | thread at `parallel` `p` | the gate holds | creates the parallel instance and one `running` branch thread per branch, each settled from its branch start; the owner rests at `p`'s join; the memory model splits its component |
| `join` | thread at `p`'s join | the join policy holds over the dispositions, a thread counting once `completed` ([FR-171](FR-171-evaluate-dynamic-control.md)); a `predicate` policy holds when its function, applied to one Boolean per joined label in ascending label order (`true` for `completed`, `false` for `running`), returns `true` ([FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md)); the gate holds | the instance is settled; under `outstanding cancel` every branch thread still `running` becomes `cancelled` in this step, together with every thread at or below it and every parallel instance those threads forked, which are settled and removed in the same step, and none of them takes a further step; under `outstanding continue` they keep running; the owner settles past the join; the memory model merges its component |
| `timeout` | thread at `await` `a` | always | the owner settles into `a`'s `timeout` control |
| `finish` | root thread at `finish` | every other thread of the instance, compensation threads included, has left `running`, and some record of the finish type satisfies the finish constraint | the binder is bound and the instance is finished; the root rests at end |
| `fence` | thread at `fence` node `n` | its memory model's fence rule alone ([FR-435](../memory/FR-435-classify-memory-accesses-and-orderings.md)); the rule is true under `sc`, and under `tso` a `seq_cst` fence is enabled only while its thread's store buffer is empty and an `acquire`, `release` or `acq_rel` fence always ([FR-436](../memory/FR-436-explore-total-store-order.md)); the step needs no precondition or constraint and waits on nothing else | the thread settles past the node; the model state is unchanged; its memory model's fence effect on the memory component, none under `sc` |
| `memory` | the memory model | its model's rule | its model's rule (FR-434) |

An await's match is its event node's own step, after which the owner settles
into `then`.

The binder of a `send` holds the sent payload, of the channel's `carries`
type, as the checked package types it
([FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md)).
The step also records a **send record**, its result: `{"type":"send-record",
"channel":"<qualified-name>","payload":<encoded>,"outcome":"accepted"|
"rejected"|"lost","displaced":<queue-entry>|null}`. `channel` is the channel's
qualified name ([FR-429](FR-429-report-protocol-deadlocks.md)); `payload` is the
sent payload; `outcome` is `accepted` when the entry was appended, `rejected`
when `reject` refused the send at capacity, and `lost` when a `loss new`
selector dropped the arriving payload; `displaced` is the queue entry (the
`queue-entry` form of FR-425) that a `loss head` or `loss tail` selector
removed to admit the arriving one, and `null` otherwise. A `rejected` or `lost`
send leaves no entry. A send that displaces an entry also appends its own. The
send record is carried by its step as the `send_record` member of a
counterexample step ([FR-433](FR-433-replay-protocol-counterexamples.md)),
which replay recomputes and compares; it is in neither the state key nor the
transition identity. The state key holds the binder's payload and the queue the
outcome left.

**Waiting.** A thread at an attempt with no enabled application, at an event,
send or finish no record or payload of which satisfies its constraint, at a
receive with no deliverable entry, at a `block` send whose channel is at
capacity, at a `fence` whose memory model's fence rule is false, at an
unsatisfied join, at `finish` while another thread of its instance is
`running`, at a node owned by a replicated role with no live instance, at a
compensation attempt that a commit step has closed, or at a false `check` has
no enabled step and waits. A `cancelled` thread takes
no step. A false `check` never becomes true. The protocol
system reads no clock and has no delay step. An `attempt` of an operation whose
guard reads a model clock evaluates that guard at the composed state's current
clock valuation, which no protocol step advances or stops, so protocol steps
neither advance nor stop model clocks. A protocol may select a timed model, and
its derived items are then the deadlock-freedom item and, with a weak
`parallel`, the race-freedom item
([FR-425](FR-425-define-protocol-subjects-and-state-keys.md) "Protocol
subject"). `timeout` is enabled for as long as its `await` waits,
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
| `record` | `event`, `send`, `receive`, `finish`, `activate`, `cattempt` | the bound record or payload; for `cattempt`, its attempt record ([FR-427](FR-427-check-protocol-compensation.md)) |
| `actor` | a step taken by an instance of a replicated role, `send` and `receive` it owns included | the acting instance's object reference |
| `channel` | `duplicate`, `lose` | the channel's qualified name and instance ordinal |
| `entry` | `duplicate`, `lose` | the queue entry |
| `template`, `registration` | `cattempt`, `cend` | the template's qualified name and the registration key |
| `role`, `object` | `spawn`, `retire` | the role and the object reference |
| `instance` | `spawn`, `retire` | the ordinal of the protocol instance the step acts in, because a role instance belongs to one protocol instance |
| `ordering` | `fence`, and an access with an ordering | the ordering |
| `memory_choice` | an access whose memory model chooses a message or slot | the choice its memory model names |
| `memory_step` | `memory` | the identity its memory model gives |

Several successors with one identity are told apart by their post-state's
state key. Successors are visited in ascending
JCS bytes of the identity, as FR-181 states for model transition identities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-426-AC-1 | The two-branch `Fill` protocol (TC-376) reaches exactly seven states and six transitions from `c.v = 0`: one `fork`, both orders of the two attempts where enabled, one `join` and one `finish`, with the listed enabled steps at each state. | Test (TC-376) |
| FR-426-AC-2 | Each join policy `all`, `any`, `quorum(2)` of three and a predicate join is enabled exactly at its threshold disposition and not before, under `outstanding continue` the remaining branches keep taking steps after the join, and under `outstanding cancel` they become `cancelled` in the join step and take no further step. | Test (TC-377) |
| FR-426-AC-3 | A `fifo` channel delivers only the head of each key and an `unordered` channel any entry; `duplicate` and `lose` are enabled exactly under the delivery policies that admit them; a `block` send at capacity waits and a `reject` send at capacity records the rejection. | Test (TC-378) |
| FR-426-AC-4 | Choice, repeat and check moves produce no transition identity and no position; a repeat at its maximum with a true guard enters `exhausted`; a false `check` leaves its thread waiting with no enabled step. | Test (TC-376) |
| FR-426-AC-5 | An attempt never interleaves with another step: its precondition is read and its post-state written at one position, and its successors are exactly the FR-181 successors of that application with the binder constraint applied. | Test (TC-376) |
| FR-426-AC-6 | Every step carries a protocol transition identity with exactly the members its kind lists; successors are visited in ascending JCS bytes of the identity, and two successors with one identity are distinguished by post-state key. | Test (TC-376, TC-387) |
| FR-426-AC-7 | An `await` whose match is never enabled has its `timeout` step enabled at every state where it waits, with no clock read. | Test (TC-378) |
| FR-426-AC-8 | A repeat with no maximum keeps no counter in `bounds`, re-enters its body while its guard is true and leaves it when the guard is false; a loop whose guard stays true closes a cycle through the state key, so its reachable state set is finite when its body's states are. | Test (TC-376) |
| FR-426-AC-9 | `finish` is not enabled while a branch thread under `outstanding continue` or a compensation thread of the instance is `running`, even when a record satisfies the finish constraint; it is enabled once every such thread has left `running`. | Test (TC-377, TC-380) |
| FR-426-AC-10 | An attempt constraint that reads `lookup<T>(p, r) absent undefined` at a key the population does not hold, at a reachable state, settles every item over the subject `refuted` with cause `undefined-evaluation`, a prefix ending at that state, and `where` naming the constraint's locus; the step is not treated as disabled. | Test (TC-376) |
| FR-426-AC-11 | Over a `fifo` channel of capacity 2, each `send` binds its binder to the payload and gives its step a `send_record` `{channel, payload, outcome, displaced}` with `channel` in the qualified name. On an empty queue `outcome` is `accepted` and `displaced` is none. With the queue holding `P1` then `P2`, `overflow reject` gives `rejected` and `overflow loss new` gives `lost`, each leaving the queue `[P1, P2]`; `overflow loss head` gives `accepted` with `displaced` `P1` and the queue `[P2, new]`; `overflow loss tail` gives `accepted` with `displaced` `P2` and the queue `[P1, new]`. | Test (TC-378) |
| FR-426-AC-12 | A thread at `fence F ordering seq_cst;` has one `fence` step, which changes no model state and moves the thread to its next sibling. Under `sc` it is enabled in every state. Under `tso` the `seq_cst` fence is enabled exactly when its thread's buffer is empty and waits otherwise, and an `acquire` fence is enabled in every state. Two branches that each hold a fence reach no deadlock under `sc` that the same protocol without the fences does not reach. | Test (TC-376) |
| FR-426-AC-13 | `parallel Outer { branch a attempt Fast; branch b sequence { parallel Inner { branch x attempt X1; branch y attempt Y1 } join any [x, y] outstanding continue; attempt After } } join any [a] outstanding cancel`, every attempt always enabled: in a state where `a` has completed, `b` rests at `Inner`'s join and `x` and `y` are `running`, the `join(Outer)` step makes `b`, `x` and `y` `cancelled`, settles and removes `Inner`'s instance, and none of them takes a step in any later state; `Y1` and `After` are enabled in no state after that join. | Test (TC-377) |
| FR-426-AC-14 | A `block` send at capacity has no enabled step and waits, and a deadlock at it reports `full-queue` ([FR-429](FR-429-report-protocol-deadlocks.md)); a thread at a `fence` whose memory model's rule is false (a `seq_cst` fence under `tso` with a non-empty buffer) has no enabled step while its `flush` step is enabled; a `cancelled` thread never takes a step. | Test (TC-378, TC-376, TC-377) |
| FR-426-AC-15 | In a protocol with `activation on each` and a replicated role `r`, `spawn(r, o)` of one object `o` in protocol instance 0 and in protocol instance 1 have different transition identities that differ in the `instance` member, and so do `retire(r, o)` in the two instances; a `spawn` or `retire` identity holds the members `step`, `role`, `object` and `instance`, and no `thread`. | Test (TC-382) |
| FR-426-AC-16 | `run sequence Main { parallel P { branch left attempt A as (a); branch right attempt B } join all; effect E of Main::P::left::A as (e) { true } }`, with `A` and `B` always enabled: at the state after `join(P)`, `event(E)` is enabled and the root thread's environment holds `A`'s binder, added by the join. | Test (TC-376) |
| FR-426-AC-17 | A `parallel J` of branches `a`, `b` and `c`, each one always-enabled `event`, joined `predicate(g)` over `[a, b, c]` with `outstanding cancel`, where `g(p, q, r)` is `true` exactly when `q` is `true` and `r` is `false`: `join(J)` is enabled at exactly the states whose completed branch set is `{b}` or `{a, b}`, and at no state whose completed set is `{}`, `{a}`, `{c}`, `{a, c}`, `{b, c}` or `{a, b, c}`; a join at `{b}` makes `a` and `c` `cancelled` in that step. | Test (TC-377) |
| FR-426-AC-18 | An `attempt` whose operation guard reads a model clock is evaluated at the current clock valuation, and no protocol step changes that valuation. | Test (TC-370) |

## Dependencies

- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) owns the
  protocol state, threads and paths.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  application successor rule, state key and canonical order.
- [FR-170](FR-170-evaluate-channel-queue-semantics.md) and
  [FR-171](FR-171-evaluate-dynamic-control.md) own channel and join policy
  meaning.
- [FR-434](../memory/FR-434-select-memory-models-for-parallel.md) owns the
  gate, split, merge, write and memory steps.

## References

- Design: quire-spec-language ADR-027 FO-1, FO-2, ST-1 to ST-10, AT-1 to AT-3,
  TS-2, TS-3, QS-2 and QS-4.
- Tickets: Linear STD-140; paired QSL-396.
