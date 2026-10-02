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
  observation and the protocol's captures, read through the one clause
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
`{"thread":<thread path>,"rest":<rest>,"cause":<cause>}`, with `cause` one of:

| Cause | Meaning |
| --- | --- |
| `{"type":"precondition","clause":"<qualified-name>"}` | an attempt whose first false selected precondition clause, in clause order, is the one named |
| `{"type":"no-post-state"}` | an attempt whose preconditions hold and no post-state satisfies its postconditions within the frame |
| `{"type":"constraint"}` | an event, attempt or finish whose constraint no record satisfies |
| `{"type":"empty-queue","channel":"<qualified-name>"}` | a receive with no deliverable entry |
| `{"type":"join-waiting","threads":[<thread path>, ...]}` | a join, naming the branch threads its policy still needs |
| `{"type":"check-false","node":"<node>"}` | a false `check` |
| `{"type":"gate"}` | a step disabled only by its memory model's gate |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-429-AC-1 | The `Fill` protocol with `TwoPre: self.v = 0` settles its deadlock-freedom item `refuted` with the three-position prefix to the state where `right` waits at `B` and the root at the join, and blocked threads `right`, `precondition` `TwoPre`, and root, `join-waiting` `[right]`. | Test (TC-376, TC-383) |
| FR-429-AC-2 | The repaired `Fill` protocol with `TwoPre: self.v <= 1` reaches ten states and nine transitions, both terminal states finished, and settles the item `proved`, basis `exhaustive`. | Test (TC-376) |
| FR-429-AC-3 | A protocol with `terminal when { P }` where `P` holds at its only unfinished terminal state settles the item `proved`; the same protocol with `terminal any` adds no deadlock-freedom item, and its authored claims' verdicts are unchanged. | Test (TC-383) |
| FR-429-AC-4 | An `on each` protocol whose live instance waits on a step only a new trigger could enable, with `activate` still enabled, reports that state as a deadlock; the same state with an instance-limited `activate` is a boundary state and no deadlock. | Test (TC-381, TC-383) |
| FR-429-AC-5 | Each wait cause is reported for a thread blocked by exactly that condition, and the blocked list is ordered by thread path. | Test (TC-383) |
| FR-429-AC-6 | A second `terminal` member in one protocol clause refuses `ambiguous_declaration`/`ambiguous-name` naming both. | Test (TC-383) |
| FR-429-AC-7 | A `terminal when { P }` whose `P` evaluates undefined at a reachable terminal state settles the deadlock-freedom item and every other item over the subject `refuted` with cause `undefined-evaluation`, a prefix ending at that state, and `where` naming `P`'s locus. | Test (TC-383) |

## Dependencies

- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) and
  [FR-426](FR-426-apply-protocol-steps.md) own the protocol state and
  enabledness.
- [FR-433](FR-433-replay-protocol-counterexamples.md) owns the
  counterexample wire and its replay.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `protocol-clause` and `protocol-terminal` productions.

## References

- Design: quire-spec-language ADR-027 PB-4, PD-1 to PD-5, QS-7 and QS-13;
  model-subject deadlocks in ADR-018 DL-1 to DL-7.
- Tickets: Linear STD-140; paired QSL-396.
