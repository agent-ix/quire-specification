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
carriers are the views ([FR-437](FR-437-explore-release-acquire-memory.md)):
each thread's `cur`, `acq` and `rel` views, each message's view, `S`, and
each view a `send` carries. Under `tso` they are each thread `b`'s
**summaries** `cur_b`, `acq_b` and `rel_b(ℓ)` per location, which play the
part of `ra`'s views of the same names, each buffered store, each memory
value, and each queue entry a `send` made inside the weak `parallel`; a
buffered store and a memory value carry the summary of the store that wrote
them. `tso` has no counterpart of `S`. A write to `ℓ` sets the write Boolean
true for its own thread's carriers and false for every other carrier; a read
by `c` does the same for `c`'s read Boolean. Synchronisation joins carriers by
Boolean or, wherever views are joined. The summary is part of the state key's
`memory` member.

**Typed form.** The `memory` member holds one
`{"type":"race-summary","location":<location>}` entry for each shared location
`ℓ` with a non-atomic access, whose value is
`{"last_write":{"thread":<thread path>,"atomic":true|false}|null,"last_reads":<map from thread path to true|false>}`.
`last_write` is `null` until a thread of the weak `parallel` writes `ℓ`;
`last_reads` has an entry for each thread that has read `ℓ`, true when that
thread's last read of `ℓ` was atomic. An RMW is both a read and a write of
`ℓ`. A carrier's Booleans are its **race map**,
`<map from location to {"write":true|false,"reads":<map from thread path to true|false>}>`,
with an entry for each shared location with a non-atomic access and, in
`reads`, an entry for each thread in that location's `last_reads`. A race map
is held where its carrier is held, so every carrier is named by its holder:

| Model | Carrier | Where its race map is held |
| --- | --- | --- |
| `ra` | every view: a thread's `cur`, `acq` and `rel` views, a message's view, `S` | the view's member `"race"` ([FR-437](FR-437-explore-release-acquire-memory.md)) |
| `ra` | the view a `send` carries | the queue entry's member `"view"`, that view ([FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md)) |
| `tso` | `cur_b`, `acq_b`, `rel_b(ℓ)` | the entry `{"type":"race-summaries","thread":<thread path>}`, value `{"cur":<race map>,"acq":<race map>,"rel":<map from location to race map>}`, one per live thread |
| `tso` | a buffered store | the store-buffer element's member `"race"` ([FR-436](FR-436-explore-total-store-order.md)) |
| `tso` | the memory value of a shared location `ℓ'` | the entry `{"type":"race-memory","location":<ℓ'>}`, value its race map, one per shared location |
| `tso` | the summary a `send` carries | the queue entry's member `"race"` |

When the subject has no shared location with a non-atomic access, none of
these members or entries exists, and every view, element and queue entry
encodes as without them. Entering an outermost weak `parallel`, each
race-summary entry has `last_write` `null` and an empty `last_reads`, and
every carrier maps each such location to `{"write":true,"reads":{}}`.

**Updates.** A step by thread `b` that accesses `ℓ` first decides `raced` from
its pre-state: the access races with `last_write` when that write is by
another thread, the write Boolean of `ℓ` in `cur_b` is false and the access or
that write is non-atomic; a write also races with each other thread `c`'s last
read when `c`'s read Boolean of `ℓ` in `cur_b` is false and the write or that
read is non-atomic. An RMW decides its read part so; when that part is
`acquire` or `seq_cst`, it then joins the race map of the message (`ra`) or
memory value (`tso`) it reads into `cur_b`, and decides its write part
against that joined `cur_b`, since what happens before the store it reads
happens before its write. Otherwise its write part is decided against the
pre-state's `cur_b`. The step then updates the summary: a write sets
`last_write` to `b` and its atomicity, and sets the write Boolean of `ℓ` true
in `cur_b` and `acq_b` and false in every other carrier; a read sets `b`'s
entry in `last_reads` to its atomicity, and sets `b`'s read Boolean of `ℓ`
true in `cur_b` and `acq_b` and false in every other carrier. `b`'s own
carriers are only `cur_b` and `acq_b`: a release view or summary, a buffered
store and a message keep what was released or stored before the access. Only
then does the step apply its model's rule, with these carrier rules:

- **`ra`.** Wherever FR-437 assigns a view from another view (a `release` or
  `seq_cst` store's message view, and `rel_b(ℓ)`, from `cur_b`; every
  `rel_b(ℓ)` from `cur_b` at a `release` fence; a branch's views from its
  parent's at entry; the view a `send` carries from `cur_b`), the assigned
  view copies the other view's race map. Wherever FR-437 joins views, each
  Boolean of the result is the or of the joined views' Booleans, and a
  `relaxed` or non-atomic store's message view takes the race map of
  `rel_b(ℓ)`, since joining the new message adds none.
- **`tso`.** A store by `b` to `ℓ` gives its buffered element, or for a locked
  access the memory value it writes, the race map of `cur_b` when the store is
  `release` or `seq_cst`, and of `rel_b(ℓ)` when it is `relaxed` or
  non-atomic; a `release` or `seq_cst` store also sets `rel_b(ℓ)` to `cur_b`'s.
  A load by `b` that reads a buffered element or a memory value joins its race
  map into `acq_b`, and, when the load is `acquire` or `seq_cst`, into
  `cur_b`. An RMW reads as such a load with the read part of its ordering and
  writes memory as such a store with the write part, its written value's race
  map also joining the read one's, so a release sequence continues through
  it. `flush(b)` gives the memory value of the flushed location the flushed
  element's race map. An `acquire` fence joins `acq_b` into `cur_b`; a
  `release` fence sets every `rel_b(ℓ)` to `cur_b`'s; an `acq_rel` or
  `seq_cst` fence does both. Each branch starts with its parent's summaries,
  and the `join` sets the parent's summaries to the join of its branches'. A
  `send` records `cur_b`'s race map in its queue entry, and its `receive`
  joins that map into the receiver's `cur` and `acq`.

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
| FR-438-AC-6 | In the message-passing protocol of FR-438-AC-1 under `ra`, after the writer's `data := 1` the `memory` member holds one race-summary entry, for `data`, whose value is `{"last_write":{"thread":<writer>,"atomic":false},"last_reads":{}}`; the writer's `cur` and `acq` views map `data` to `{"write":true,"reads":{}}`, and every other view, among them the reader's `cur` and the writer's `rel` for `flag`, maps it to `{"write":false,"reads":{}}`. After the writer's flag store, the new `flag` message's view maps `data` to `"write":true` when the store is `release` and to `"write":false` when it is `relaxed`. With the `release` store and an `acquire` load that reads that message, the reader's `cur` maps `data` to `"write":true` and its data load completes no race; with `relaxed` flags, the data load after a flag load that read 1 completes a race. With the flag passed instead by a `send` after `data := 1` and a `receive` before the data load, the queue entry's `view` member is the writer's `cur` view, whose `race` member maps `data` to `"write":true`; after the `receive` the reader's `cur` maps `data` to `"write":true`, and the item settles `proved`. A protocol whose shared accesses are all atomic has no race-summary entry and no view `race` member. | Test (TC-392) |
| FR-438-AC-7 | In the message-passing protocol of FR-438-AC-1 under `tso`, after the writer's `data := 1` the race-summary entry for `data` is `{"last_write":{"thread":<writer>,"atomic":false},"last_reads":{}}`; the writer's `race-summaries` entry maps `data` to `{"write":true,"reads":{}}` in `cur` and `acq` and to `{"write":false,"reads":{}}` in `rel` for `flag`, the reader's maps it to `"write":false` in `cur`, and the buffered `data` element's `race` member maps it to `"write":false`. After the writer's flag store, the buffered `flag` element maps `data` to `"write":true` when the store is `release` and `"write":false` when it is `relaxed`, and after `flush(writer)` of that element the `race-memory` entry for `flag` holds the same map. With the `release` store and an `acquire` load of `flag` that reads 1 from memory, the reader's `cur` maps `data` to `"write":true` and its data load completes no race, so the race-freedom item settles `proved`; with `relaxed` flags the data load after a flag load that read 1 completes a race and the item settles `refuted` on `data`. With the flag passed by a `send` after `data := 1` and a `receive` before the data load, the queue entry's `race` member maps `data` to `"write":true` and the item settles `proved`. | Test (TC-392) |
| FR-438-AC-8 | A protocol whose thread `a` loads `x` non-atomically and then stores `x := 1` with `release`, and whose thread `b` runs a `fetch_add(1)` on `x` with `ordering acquire`, settles its race-freedom item `proved` under `ra` and under `tso`: when the RMW reads `a`'s store, it joins that store's race map, in which `a`'s read Boolean of `x` is true, before deciding its write part. With the RMW `ordering relaxed`, the item settles `refuted` with a race counterexample on `x` whose `earlier` is `a`'s load and whose `later` is the RMW. | Test (TC-392) |

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
