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

**Typed form.** The value of the `{"type":"race-summary","location":<location>}`
entry for `ℓ` is
`{"last_write":{"thread":<thread path>,"atomic":true|false}|null,"last_reads":<map from thread path to true|false>}`.
`last_write` is `null` until a thread of the weak `parallel` writes `ℓ`;
`last_reads` has an entry for each thread that has read `ℓ`, true when that
thread's last read of `ℓ` was atomic. An RMW is both a read and a write of
`ℓ`. Under `ra` each carrier's Booleans are held in its view, as the view's
member `"race":<map from location to {"write":true|false,"reads":<map from thread path to true|false>}>`
([FR-437](FR-437-explore-release-acquire-memory.md)), with an entry for each
shared location with a non-atomic access and, in `reads`, an entry for each
thread in that location's `last_reads`. A carrier is therefore named by where
its view is held: a thread's `cur`, `acq` or `rel` entry, a message's view in
the `messages` entry, `S`, or a view a `send` carries. Entering an outermost
weak `parallel`, each entry has `last_write` `null` and an empty `last_reads`,
and every view maps each such location to `{"write":true,"reads":{}}`.

**Updates under `ra`.** A step by thread `b` that accesses `ℓ` first decides
`raced` from its pre-state: the access races with `last_write` when that write
is by another thread, the write Boolean of `ℓ` in `cur_b` is false and the
access or that write is non-atomic; a write also races with each other thread
`c`'s last read when `c`'s read Boolean of `ℓ` in `cur_b` is false and the
write or that read is non-atomic. The step then updates the summary: a write
sets `last_write` to `b` and its atomicity, and sets the write Boolean of `ℓ`
true in `cur_b` and `acq_b` and false in every other view; a read sets `b`'s
entry in `last_reads` to its atomicity, and sets `b`'s read Boolean of `ℓ`
true in `cur_b` and `acq_b` and false in every other view. Only then does the
step apply FR-437's access rule. Wherever FR-437 assigns a view from another
view (a `release` or `seq_cst` store's message view, and `rel_b(ℓ)`, from
`cur_b`; every `rel_b(ℓ)` from `cur_b` at a `release` fence; a branch's views
from its parent's at entry; the view a `send` carries from `cur_b`), the
assigned view copies the other view's Booleans. Wherever FR-437 joins views,
each Boolean of the result is the or of the joined views' Booleans, and a
`relaxed` or non-atomic store's message view takes the Booleans of
`rel_b(ℓ)`, since joining the new message adds none.

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
| FR-438-AC-6 | In the message-passing protocol of FR-438-AC-1 under `ra`, after the writer's `data := 1` the `memory` member holds one race-summary entry, for `data`, whose value is `{"last_write":{"thread":<writer>,"atomic":false},"last_reads":{}}`; the writer's `cur` and `acq` views map `data` to `{"write":true,"reads":{}}`, and every other view, among them the reader's `cur` and the writer's `rel` for `flag`, maps it to `{"write":false,"reads":{}}`. After the writer's flag store, the new `flag` message's view maps `data` to `"write":true` when the store is `release` and to `"write":false` when it is `relaxed`. With the `release` store and an `acquire` load that reads that message, the reader's `cur` maps `data` to `"write":true` and its data load completes no race; with `relaxed` flags, the data load after a flag load that read 1 completes a race. A protocol whose shared accesses are all atomic has no race-summary entry and no view `race` member. | Test (TC-392) |

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
