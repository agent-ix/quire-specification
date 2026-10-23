---
id: SR-16215
title: "spec-review of the race-summary typed form and update rule (quire-specification PR 396)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a7a49309; spec/functional/memory/FR-436-explore-total-store-order.md; spec/functional/memory/FR-437-explore-release-acquire-memory.md; spec/functional/memory/FR-438-report-data-races.md; spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md; spec/test-cases/TC-392-race-freedom-item.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-438
    type: references
  - target: ix://agent-ix/quire-specification/FR-437
    type: references
  - target: ix://agent-ix/quire-specification/FR-436
    type: references
  - target: ix://agent-ix/quire-specification/FR-425
    type: references
---

# Spec review of the race-summary typed form and update rule (quire-specification PR 396)

## Summary

Ticket: STD-360 (spec gap behind QSL-932), QSpec half; the QSL half is
quire-spec-language #1328. One spec-review pass over
`git diff origin/main...a7a49309` (5 files, +98/-10). origin/main is an
ancestor of the head. Oracles: QSL ADR-025 RA-1 to RA-7, TSO-1 to TSO-8,
MS-1, MS-2, MK-1, DR-2 to DR-6 at #1328's head (72edb781), and RC11/C11
happens-before.

Gates run by the reviewer at the head: `quire validate --scope . 'spec/**/*.md'`
gives 86 failure lines (the brief's baseline); the five touched files report
no failure. `make check-spec-validation` of QSL #1328, #1315 and #1319 with
`QSPEC_DIR` at this head exits 0. The cargo tests were not re-run: no
test-parsed fixture or vector changed.

Examined, clean:

- Typed form. The race-summary entry value, the race map and the holder table
  name every carrier: `ra` views (`cur`, `acq`, `rel`, message views, `S`)
  hold a `race` member, the `ra` in-flight `send` view lives in the queue
  entry's `view` member, and `tso` uses `race-summaries` per thread, an element
  `race` member, `race-memory` per location and a queue-entry `race` member.
  Absent when no shared location has a non-atomic access, so existing state
  keys keep their bytes for race-free subjects.
- Update order for plain loads and stores, against C11 happens-before: a
  write sets its Boolean true only in `cur_b` and `acq_b` and false in every
  other carrier, so `rel_b`, earlier buffered stores and published messages
  keep what was released before it (a later `data := 2` after a release
  correctly races with a reader that acquired the earlier release). Copy on
  assignment, `or` on join, and a `relaxed` or non-atomic store taking
  `rel_b(ℓ)`'s map follow RA-2 to RA-6; a fence-release followed by a relaxed
  store synchronises, as in C11. The `tso` rules mirror them over the store a
  load reads, and the RMW written map joins the read one's, so release
  sequences continue.
- Consistent with ADR-025 MS-2: the race maps are memory-component data read
  by `raced`, replay and rendering; no `holds` atom reads them.
- DR-11 and FR-438-AC-6 hand-checked under `ra` (release and relaxed flag):
  after `data := 1` only the writer's `cur`/`acq` map `data` to
  `"write":true`, the new `data` message's view takes `rel_W(data)` (false);
  the release `flag` message copies `cur_W` (true), the relaxed one takes
  `rel_W(flag)` (false); the acquire load joins it into the reader's `cur`
  and the data load completes no race; with relaxed flags it races. All
  stated values hold.
- DR-12 and FR-438-AC-7 hand-checked under `tso`: the buffered `data` element
  takes `rel_W(data)` (false), the release `flag` element takes `cur_W`
  (true), `flush` copies it to `race-memory` for `flag`, the acquire load from
  memory joins it into the reader's `cur`, and FIFO buffers put `data` in
  memory before `flag`; the `send` records `cur_W` (true). All stated values
  hold.
- FR-425: the queue-entry bullet merges cleanly with open PR #389's FR-425
  edits (different sections; `git merge-tree` clean); #390 does not touch
  FR-425.
- Ids: FR-438-AC-6, FR-438-AC-7, TC-392 DR-11 and DR-12, and the entry types
  `race-summaries` and `race-memory` are unused on main and in every open
  QSpec PR.
- Behaviour-only wording; lead defaults D1 to D7 are labelled in the PR body.
  No follow-up, deferral or narrowing.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | [soundness] An RMW decides the race of its write part from the pre-state, before its own acquire synchronisation, so it reports a race that C11 rules out. Thread `a` reads `x` non-atomically, then stores `x` with `release`; thread `b`'s `acquire` (or `acq_rel`) RMW of `x` reads that store. The read happens before the RMW (program order, then the store synchronises with the RMW), so there is no race. But "Updates" decides `raced` before the step joins the read message's race map into `cur_b`, and `b`'s pre-state `cur_b` has `a`'s read Boolean false, so "a write also races with each other thread `c`'s last read" fires. The item goes `refuted`, and replay's vector clocks then settle `inconclusive`, `replay-parity`, where the right answer is `proved`. This applies under `ra` and under `tso` (locked RMW). Fix: decide an RMW's write-part races after joining the race map it reads when the read part is `acquire` or `seq_cst` (or check them against `cur_b` joined with that map), and add a vector with this shape. | spec/functional/memory/FR-438-report-data-races.md:102-114 | wrong-requirement |
| FND-002 | medium | [coverage] Nothing tests the `ra` half of the channel carriers. FR-437 and FR-425 add a queue-entry `view` member for every `send` inside an `ra` weak `parallel`, race or no race (D7: without it two states that differ only in an in-flight view share a key). FR-438 says the `send` view copies `cur_b`'s race map and `receive` joins it. FR-438-AC-6 and DR-11 have no `send`/`receive` variant, and no AC checks the `view` member or that two states differing only in an in-flight view get different keys. FR-438-AC-7 covers only `tso`. Fix: add to FR-438-AC-6 and DR-11 the variant with the flag passed by `send` and `receive` under `ra`, giving the queue entry's `view.race` for `data` and the verdict `proved`. Also add an FR-425 or FR-437 AC with two states that differ only in an in-flight `ra` view. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:76-80; spec/functional/memory/FR-438-report-data-races.md:186; spec/test-cases/TC-392-race-freedom-item.md:45 | correct-requirement-no-evidence |
| FND-003 | low | [test-intent] DR-12 does not use the protocol that FR-438-AC-7 states. AC-7 states its channel variant over the message-passing protocol: the flag is passed by `send` after `data := 1`, then a `receive` before the non-atomic data load. DR-12 uses DR-08 for it instead, which is DR-06's shape: a non-atomic store and an atomic load, with no flag and no choice. DR-12 also reuses DR-01 and DR-02, which the table defines as `ra` variants, "under `tso`" without naming what changes. Fix: in DR-12, name the `tso` message-passing variants and the channel variant explicitly, or change AC-7 to DR-08's shape. | spec/test-cases/TC-392-race-freedom-item.md:46; spec/functional/memory/FR-438-report-data-races.md:187 | wrong-requirement |

## Verdict

Not mergeable yet: two medium findings. The typed form, the holder table, the
plain-access update order and both hand-checked vectors are sound against C11
happens-before and ADR-025 MS-2. Before merging, FND-001 needs a one-sentence
ordering rule for RMWs plus a vector, and FND-002 needs an `ra` channel
variant. The same RMW ordering appears in QSL ADR-025 DR-3 and FR-224
(#1328), so fix both halves together.

## Dispositions

Round 1. `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines, the same as main. `cargo test --test checked_package_v2 --test backend_provider_v1` passes.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-438 "Updates" now says that an RMW whose read part is `acquire` or `seq_cst` joins the race map it reads into `cur_b` before it decides its write part. Otherwise the write part is decided against the pre-state. This covers both models. New FR-438-AC-8 and TC-392 DR-13: `a` loads `x` non-atomically and then stores it `release`, and `b` runs `fetch_add` on `x`. The `acquire` RMW is `proved` under both models; the `relaxed` RMW is `refuted` with `earlier` `a`'s load. |
| FND-002 | fixed | FR-438-AC-6 and DR-11 gain the `ra` channel variant: the queue entry's `view` is the writer's `cur` with `data` `write` true, the reader's `cur` has `write` true after the `receive`, and the item is `proved`. New FR-437-AC-15 and TC-393 MB-13 give two states that differ only in the in-flight view's `at` and therefore get different keys. |
| FND-003 | fixed | DR-12 now names its three `tso` variants explicitly, all over the message-passing shape: `release`/`acquire`, `relaxed`, and the flag passed by `send`/`receive`. This matches FR-438-AC-7; DR-08's shape is no longer used. |
