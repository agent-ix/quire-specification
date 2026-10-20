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
the following. The list of causes is closed. Every `channel` is the channel's
**qualified name**: the qualified protocol-definition identity followed by the
channel's declaration-local handle
([FR-050](FR-050-bind-protocol-instances.md)), so two protocols that each
declare a channel `Pay` give two different channels, queues and causes.

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
| `{"type":"check-false","node":"<node>"}` | a false `check` |
| `{"type":"gate"}` | a step disabled only by its memory model's gate |

**One cause per thread.** A waiting thread has exactly one cause: `gate` when
its memory model's gate alone disables its step, and otherwise the first that
applies in this order for its rest point:

- an `attempt`: `precondition`, `no-post-state`, `constraint`;
- a `receive`: `constraint`, `head-blocked`, `displaced`, `empty-queue`;
- a `send`: `constraint` when no payload satisfies its constraint, else
  `full-queue` when `block` finds the queue at capacity.

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
| FR-429-AC-10 | Each source of loss gives `displaced` at a receive whose entry is gone: with capacity 1 and `overflow loss head`, `send P; send Q; receive` of `P` reports `displaced` with the queue `[Q]`; with `overflow loss new`, `send P; send Q; receive` of `Q` reports `displaced` with the queue `[P]`; with delivery `at-most-once`, `send P; lose; receive` of `P` reports `displaced`. | Test (TC-383) |
| FR-429-AC-11 | `displaced` follows the reported prefix: with capacity 1, `overflow loss head`, `repeat maximum 2 { send P }; receive R1 of P; receive R2 of P`, the receive `R2` at an empty queue is reported `empty-queue`, because the last removal of an entry from that send node was a `receive`; a receive on a channel nothing has sent to is reported `empty-queue`. Two counterexamples ending in one state through a loss and through a `receive` carry `displaced` and `empty-queue` respectively. | Test (TC-383) |
| FR-429-AC-12 | Two protocols of one package that each declare a channel `Pay` give blocked-thread causes whose `channel` members are the two different qualified names. | Test (TC-383) |

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
