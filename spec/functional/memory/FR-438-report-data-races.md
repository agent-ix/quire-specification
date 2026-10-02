---
id: FR-438
title: "Report data races on non-atomic locations through a derived item"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-434
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-435
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-436
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-437
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-439
    type: references
---

# FR-438: Report data races on non-atomic locations through a derived item

## Description

When a request holds an item over a weak-memory protocol subject with a
non-atomic shared access, the request writer SHALL add one derived
race-freedom item for that subject, which reports each data race with a
replayable race counterexample.

## Inputs

A protocol subject with a weak `parallel` and its access classes
([FR-435](FR-435-classify-memory-accesses-and-orderings.md)); the request's
items.

## Outputs

One race-freedom item per distinct such subject, settled with a model-check
verdict; on a race, a `kind: race` counterexample.

## Behavior

### Data race

Under a weak model, a non-atomic access explores as a `relaxed` access under
`ra` and as `MOV` under `tso`. A **data race** is two accesses to one shared
location by different threads, at least one a store or RMW and at least one
non-atomic, where neither happens before the other. **Happens-before** is
program order together with synchronisation: an `acquire` or `seq_cst` load,
RMW or later `acquire` fence reading from a `release` or `seq_cst` store or
its release sequence; `fork` and `join`; and `send` with its `receive`. Under
`tso` the same synchronisation is read over the store a load reads from,
buffered or in memory.

### Race summary

For each shared location `ℓ` with a non-atomic access, the memory component
records the thread of `ℓ`'s last write and whether it was atomic, and for each
thread whether its last read of `ℓ` was atomic. Each happens-before carrier
holds Booleans: whether `ℓ`'s last write happens before it, and, for each
thread `c`, whether `c`'s last read of `ℓ` happens before it. Under `ra` the
carriers are the views ([FR-437](FR-437-explore-release-acquire-memory.md));
under `tso` they are each thread's current and acquire summaries and each
buffered store and memory value, carrying its storer's summary. A write to
`ℓ` sets the write Boolean true for its own thread's carriers and false for
every other carrier; a read by `c` does the same for `c`'s read Boolean.
Synchronisation joins carriers by Boolean or, wherever views are joined. The
summary is part of the state key's `memory` member as
`{"type":"race-summary","location":<location>}` entries.

### The derived race-freedom item

For each distinct subject among a request's items whose resolved model is
weak and that has a non-atomic shared access, the request writer SHALL add one
**race-freedom item**: the reachable-state invariant `always holds(not
raced)`, where `raced` is derived, not authored, and is true at the post-state
of a step that completes a data race. Its obligation identity is (subject,
resolved models, item kind `race-freedom`). It has no opt-out. Under `sc` the
item is not derived.

It settles as a reachable-state invariant: `proved` with no race; `refuted`
with a race counterexample; and `inconclusive` (including
`memory-bound-reached`) or `failed` as for any such item. A race changes no
authored claim's verdict.

### Race counterexample

A race counterexample is a finite prefix from an initial state to the step
that completes the race, with `kind: race` and the member
`race: {"location":<location>,"earlier":"<position>","later":"<position>"}`
naming the position of the earlier conflicting access and of the completing
step. The canonical race counterexample is the canonical path to the first
such step in canonical breadth-first order, with `earlier` the last
conflicting access the summary names.

### Race replay

Replay re-executes the prefix ([FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md)),
then computes happens-before over the replayed prefix from scratch, with
vector clocks over the synchronisation above and independent of the race
summary, and checks that the accesses at `earlier` and `later` are by
different threads, touch `location`, include a write and a non-atomic access,
and are unordered by happens-before. Agreement settles
`reproduced-with-evaluated-witness` and the item `refuted`; disagreement
settles `inconclusive`, cause `replay-parity`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-438-AC-1 | The message-passing protocol with non-atomic `data`, whose reader loads `data` only on the branch of a choice where its flag load read 1, and with a `release` store and `acquire` load of the flag, settles its race-freedom item `proved` under `ra`; with a `relaxed` flag it settles `refuted` with a race counterexample on `data` that replays. | Test (TC-392) |
| FR-438-AC-2 | A request with two items over one weak subject with a non-atomic access receives exactly one race-freedom item; a subject whose shared accesses are all atomic, or whose resolved model is `sc`, receives none. | Test (TC-392) |
| FR-438-AC-3 | A race between a non-atomic store and an atomic load in different threads with no synchronisation between them is reported; the same accesses ordered by a `fork`, a `join` or a `send` and its `receive` are not. | Test (TC-392) |
| FR-438-AC-4 | A race counterexample whose `earlier` and `later` accesses are ordered by replay's vector clocks settles `inconclusive`, cause `replay-parity`. | Test (TC-392) |
| FR-438-AC-5 | An authored claim over a racy subject receives the same verdict as over the same subject with the racing accesses made `relaxed`. | Test (TC-392) |

## Dependencies

- [FR-436](FR-436-explore-total-store-order.md) and
  [FR-437](FR-437-explore-release-acquire-memory.md) own the synchronisation
  the carriers follow.
- [FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md) owns prefix
  replay and the memory bound.

## References

- Design: quire-spec-language ADR-025 DR-1 to DR-8 and RU-4; Flanagan and
  Freund, FastTrack (PLDI 2009).
- Tickets: Linear STD-138; paired QSL-372.
