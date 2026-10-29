---
id: FR-429
title: "Read protocol terminal states and report protocol deadlocks"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-428
    type: references
  - target: ix://agent-ix/quire-specification/FR-050
    type: references
  - target: ix://agent-ix/quire-specification/FR-170
    type: references
  - target: ix://agent-ix/quire-specification/FR-427
    type: references
  - target: ix://agent-ix/quire-specification/FR-434
    type: references
  - target: ix://agent-ix/quire-specification/FR-433
    type: references
---

# FR-429: Read protocol terminal states and report protocol deadlocks

## Description

When a request holds a temporal item over a protocol subject, the request
writer SHALL add one derived deadlock-freedom item for that subject unless its
protocol clause declares `terminal any`, and the analyzer SHALL report every
reachable terminal protocol state that is not intended as a deadlock, with
each waiting thread and its wait cause.

## Inputs

A protocol subject ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md))
with its clause's `terminal` member, if any; and the request's temporal items.

## Outputs

One deadlock-freedom item per distinct protocol subject, settled with a
model-check verdict; or no item for a clause that declares `terminal any`; and,
for a deadlock, the blocked-thread list of its last state.

## Behavior

### Terminal states

A protocol state with no enabled step of any thread, channel, environment or
memory model is **terminal**. It reads as a model subject's terminal state
does: closed under a bounded profile, and extended by the terminal stutter
step under infinite-trace, outside every fairness constraint.

A protocol instance is **finished** once its `finish` step has run and every
one of its threads, compensation threads included, has left `running`. Under
`on origin`, a state whose one instance is finished is terminal. Under `on
each`, a state with no live instance is terminal only when no `activate` step
is enabled.

A protocol state is terminal over protocol steps only: the protocol system
reads no clock ([FR-426](FR-426-apply-protocol-steps.md)) and has no delay
step, so no protocol state is time-locked, and a protocol subject has no
time-lock-freedom item ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md)
"Protocol subject").

A state the run does not expand because a state constraint, a memory bound or
the instance budget cuts it ([FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md))
is a boundary state: never terminal and never a deadlock.

### The `terminal` member

A protocol clause carries at most one `terminal` member, by the [shared grammar](../../../proposals/quire-v1/shared-grammar.md)'s
`protocol-terminal` production:

- `terminal when { P };` — `P` is a Boolean predicate over the model
  observation and, for a protocol activated `on origin`, the protocol's
  captures. Under `activation on each` the predicate is package-level and has no
  instance binder, so it cannot read a per-instance capture: a read of a capture
  of an `on each` protocol, or of the activation binder, refuses
  `missing_declaration`/`missing-name` at the read. `P` is read through the one clause
  evaluator at a terminal state. An undefined `P` settles `refuted` with cause
  `undefined-evaluation` by the model-side undefined rule
  ([FR-366](../temporal/FR-366-check-deadlock-freedom.md),
  [FR-426](FR-426-apply-protocol-steps.md)); it never classifies the state as
  intended or as a deadlock.
- `terminal any;` — every terminal state is intended. This is the
  per-protocol opt-out.

A terminal state is **intended** when every live instance is finished, or
when the clause declares `terminal when { P }` and `P` holds there. The
model's own `terminal` declaration is not read for a protocol subject. A
second `terminal` member refuses `ambiguous_declaration`/`ambiguous-name`,
naming both. The member is part of the checked package and of the obligation
identity.

### Deadlock

A **deadlock** of a protocol subject is a reachable terminal state that is
not intended. Under `on each`, a state is also a deadlock when at least one
instance is live, every live instance is unfinished, every enabled step is an
`activate`, and `P` does not hold there: no live instance can move until a new
trigger arrives, and none is promised.

The derived deadlock-freedom item is the reachable-state invariant `always
holds(not deadlocked)` with `deadlocked` read as above, an empty authored
fairness set, and obligation identity (subject, item kind
`deadlock-freedom`). A deadlock settles it `refuted`, basis
`decisive-counterexample`, with a finite-prefix counterexample of `kind:
deadlock` ([FR-433](FR-433-replay-protocol-counterexamples.md)). A deadlock
changes no authored claim's verdict.

### Blocked threads

A deadlock counterexample carries, for its last state, a list of every
waiting thread, in ascending JCS bytes of its path:
`{"thread":<thread path>,"rest":<rest>,"cause":<cause>}`, with `cause` one of
the following. The list of causes is closed: every waiting thread has one of
them.

**Qualified channel name.** The `<qualified-name>` of a channel is the
qualified name of the protocol that declares it (the rendering the
counterexample's `protocol` member uses,
[FR-433](FR-433-replay-protocol-counterexamples.md)), then `::`, then the
channel's declaration-local handle. The protocol's qualified name renders the
structured protocol-definition identity of
[FR-288](../../objects/protocol/FR-288-qualified-protocol-definition-identity.md)
that [FR-050](FR-050-bind-protocol-instances.md) keys a static channel by.
Two protocols that each declare a channel `Pay` give two different qualified
names, queues, identities and causes. Every `channel` member of a key,
transition identity, send record or cause is this string.

| Cause | Meaning |
| --- | --- |
| `{"type":"precondition","clause":"<qualified-name>"}` | an attempt whose first false selected precondition clause, in clause order, is the one named |
| `{"type":"no-post-state"}` | an attempt whose preconditions hold and no post-state satisfies its postconditions within the frame |
| `{"type":"constraint"}` | an event, attempt or finish whose constraint no record satisfies; a receive that has a deliverable entry from the send node it names and no such entry's payload satisfies the receive's constraint; a send whose constraint no payload satisfies |
| `{"type":"empty-queue","channel":"<qualified-name>"}` | a receive with no deliverable entry, no entry from the send node it names on the channel, and no loss to account for the absence (the `displaced` rule below) |
| `{"type":"head-blocked","channel":"<qualified-name>"}` | a receive whose queue holds an entry from the send node it names, none of which is deliverable because the head of each FIFO key holding one is an entry from another send node |
| `{"type":"full-queue","channel":"<qualified-name>"}` | a send under overflow `block` whose channel queue is at capacity |
| `{"type":"displaced","channel":"<qualified-name>"}` | a receive for which no entry from the send node it names is queued on the channel, because a loss removed it |
| `{"type":"join-waiting","threads":[<thread path>, ...]}` | a join, naming the branch threads its policy still needs |
| `{"type":"finish-waiting","threads":[<thread path>, ...]}` | the root thread at `finish` while another thread of its instance, a compensation thread included, is still `running`, naming those threads |
| `{"type":"commit-closed","template":"<qualified-name>"}` | a compensation thread of the named template at its first compensation attempt after a step of the template's commit node has occurred in its instance: no further `cattempt` is enabled ([FR-427](FR-427-check-protocol-compensation.md)) and `cend` is enabled only at a retry point |
| `{"type":"no-role-instance","role":"<ident>"}` | a thread at an event, attempt, send, receive or compensation-attempt node owned by replicated role `r` when `r` has no live instance and no `spawn` step for it is enabled ([FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md)) |
| `{"type":"check-false","node":"<node>"}` | a false `check` |
| `{"type":"gate"}` | a step disabled only by its memory model's gate, which for a `fence` is the memory model's fence rule ([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)) |

**One cause per thread.** A waiting thread has exactly one cause: `gate` when
its memory model's gate alone disables its step, and otherwise the first that
applies in this order for its rest point:

- an `attempt`, `event`, `send` or `receive` node, or a compensation attempt,
  owned by a replicated role (`by r`): `no-role-instance` first;
- an `attempt`: `precondition`, `no-post-state`, `constraint`;
- a compensation attempt (`cattempt`): `commit-closed`, `precondition`,
  `no-post-state`, `constraint`;
- a `receive`: `constraint`, `head-blocked`, `displaced`, `empty-queue`;
- a `send`: `constraint` when no payload satisfies its constraint, else
  `full-queue` when `block` finds the queue at capacity;
- `finish`: `finish-waiting`, else `constraint`.

A `receive` is `constraint` only when an entry from its send node is
deliverable (the head of its FIFO key, or any entry of an `unordered`
channel) and none satisfies its constraint. It is `head-blocked` only on a
`fifo` channel: an `unordered` channel has no head and is never
`head-blocked`. It is `displaced` when no entry from its send node is queued
on the channel and the last event on the reported prefix that removed or
dropped an entry from that send node on that channel, in any FIFO-key queue,
was a loss (a `lose` step, an overflow loss selector displacing the entry, or
a send that was `lost`) and not a `receive`; otherwise it is `empty-queue`.

`displaced` is a property of the reported replay prefix, not of the state: two
counterexamples that end in one state can carry different causes. Replay
([FR-433](FR-433-replay-protocol-counterexamples.md)) recomputes the blocked
list from the prefix. The list is data for the reader and does not enter the
state key.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-429-AC-1 | The `Fill` protocol with `TwoPre: self.v = 0` settles its deadlock-freedom item `refuted` with the three-position prefix to the state where `right` waits at `B` and the root at the join, and blocked threads `right`, `precondition` `TwoPre`, and root, `join-waiting` `[right]`. | Test (TC-376, TC-383) |
| FR-429-AC-2 | The repaired `Fill` protocol with `TwoPre: self.v <= 1` reaches ten states and nine transitions, both terminal states finished, and settles the item `proved`, basis `exhaustive`. | Test (TC-376) |
| FR-429-AC-3 | A protocol with `terminal when { P }` where `P` holds at its only unfinished terminal state settles the item `proved`; the same protocol with `terminal any` adds no deadlock-freedom item, and its authored claims' verdicts are unchanged. | Test (TC-383) |
| FR-429-AC-4 | An `on each` protocol whose live instance waits on a step only a new trigger could enable, with `activate` still enabled, reports that state as a deadlock; the same state with an instance-limited `activate` is a boundary state and no deadlock. | Test (TC-381, TC-383) |
| FR-429-AC-5 | Each wait cause is reported for a thread blocked by exactly that condition, each thread carries exactly one cause, and the blocked list is ordered by thread path. | Test (TC-383) |
| FR-429-AC-6 | A second `terminal` member in one protocol clause refuses `ambiguous_declaration`/`ambiguous-name` naming both. | Test (TC-383) |
| FR-429-AC-7 | A `terminal when { P }` whose `P` evaluates undefined at a reachable terminal state settles the deadlock-freedom item and every other item over the subject `refuted` with cause `undefined-evaluation`, a prefix ending at that state, and `where` naming `P`'s locus. | Test (TC-383) |
| FR-429-AC-8 | On a `fifo` channel of capacity 2 with delivery `exactly-once-premise`, a sender that sends `P` then `Q` and a receive of `Q` reached while the queue is `[P, Q]` is reported with cause `head-blocked` naming the channel's qualified name. The same receive on an `unordered` channel is not blocked, and a receive of `P` with a constraint no payload of `P` satisfies is reported `constraint`, on the `fifo` channel and on the `unordered` one. | Test (TC-383) |
| FR-429-AC-9 | On a channel of capacity 1 with overflow `block` and delivery `exactly-once-premise`, a sender at `send Q` while the queue holds `P` is reported `full-queue` naming the channel. The same send with a constraint no payload satisfies is reported `constraint` although the queue is full. | Test (TC-383) |
| FR-429-AC-10 | Every channel below has delivery `exactly-once-premise` unless stated, so neither `lose` nor `duplicate` is enabled. With capacity 1 and `overflow loss head`, `send P; send Q; receive` of `P` deadlocks with the queue `[Q]` and cause `displaced`; with `overflow loss new`, `send P; send Q; receive` of `Q` deadlocks with the queue `[P]` and cause `displaced`; with delivery `at-most-once`, `send P; receive` of `P` has a deadlock reached by the `send` and `lose` steps, with cause `displaced`. | Test (TC-383) |
| FR-429-AC-11 | With capacity 1, `overflow loss head` and delivery `exactly-once-premise`, `repeat maximum 2 { send P }; receive R1 of P; receive R2 of P` deadlocks at `R2` with an empty queue and cause `empty-queue`, because the last removal of an entry from that send node was a `receive`; a receive on a channel nothing has sent to is reported `empty-queue`. With capacity 1, `overflow loss head`, delivery `at-most-once`, branch `s` `repeat maximum 2 { send S of P }` and branch `r` `receive R1 of S; receive R2 of S`, the prefix send, send (displacing), `R1` and the prefix send, `R1`, send, `lose` both end in one state at `R2`; replaying each ([FR-433](FR-433-replay-protocol-counterexamples.md)) gives `empty-queue` for the first and `displaced` for the second. | Test (TC-383) |
| FR-429-AC-12 | Two protocols of one package that each declare a channel `Pay` give blocked-thread causes whose `channel` members are the two different qualified names, each the declaring protocol's qualified name, `::` and `Pay`. | Test (TC-383) |
| FR-429-AC-13 | A protocol whose compensation template has a step of its commit node occur after the compensation thread was created, with the thread still at its first compensation attempt and a finish record available, deadlocks with the compensation thread `commit-closed` naming the template and the root at `finish` `finish-waiting` naming that thread. | Test (TC-383) |
| FR-429-AC-14 | A node owned by a replicated role `r` whose population universe is empty, or whose every object is retired, so that `r` has no live instance and no `spawn` is enabled, deadlocks with the thread `no-role-instance` naming `r`; when a `spawn` is enabled the thread is not waiting on the role. | Test (TC-383) |
| FR-429-AC-15 | A thread at a `fence` whose memory model's fence rule is false (a `seq_cst` fence under `tso` with a non-empty buffer) has cause `gate` and no other cause when the cause function is evaluated at that state. A deadlock report never shows it under the built-in models, because `flush` is enabled while the buffer is non-empty, so the state is not terminal. | Test (TC-383) |

## Dependencies

- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) and
  [FR-426](FR-426-apply-protocol-steps.md) own the protocol state and
  enabledness.
- [FR-050](FR-050-bind-protocol-instances.md) owns the qualified channel
  identity, and [FR-170](FR-170-evaluate-channel-queue-semantics.md) the
  queue, capacity and overflow meaning the causes read.
- [FR-433](FR-433-replay-protocol-counterexamples.md) owns the
  counterexample wire and its replay.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `protocol-clause` and `protocol-terminal` productions.

## References

- Design: quire-spec-language ADR-027 PB-4, PD-1 to PD-5, QS-7 and QS-13;
  model-subject deadlocks in ADR-018 DL-1 to DL-7.
- Tickets: Linear STD-140; paired QSL-396.
