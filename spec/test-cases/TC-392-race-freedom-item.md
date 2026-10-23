---
id: TC-392
title: "Race-freedom item and race counterexamples"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-438
    type: verifies
---

# TC-392: Race-freedom item and race counterexamples

## Description

Check when the race-freedom item is derived, how races are decided under
synchronisation, the race counterexample and its independent replay, and that
races change no authored claim.

## Test Procedure

Use the guarded message-passing protocol: the writer branch runs `data := 1`
then `flag := 1`; the reader branch runs the flag load `LF` (`r1 := flag`) and
then `choice G by c2 visible(f)`, where `f` is `LF`'s binder, whose case
`read when { f.result = 1 }` runs the data load `LD` (`r2 := data`) and whose
case `skip when { f.result != 1 }` runs an
empty sequence. `data` is written and read non-atomically, and the data load
happens only after the reader has seen the flag set. Apply the variants
below. Submit a request holding the authored
claim `always holds(not (r1 = 1 and r2 = 0))` and record the derived items,
their verdicts and the race counterexamples.

## Vectors

| Vector | Variant | Expected result |
| --- | --- | --- |
| DR-01 | `ra`, `release` flag store, `acquire` flag load | one race-freedom item, `proved` |
| DR-02 | `ra`, `relaxed` flag accesses | race-freedom item `refuted`; `kind: race`, `race.location` `(data, v)`, `earlier` the writer's non-atomic store position, `later` the reader's non-atomic load position; replays to `reproduced-with-evaluated-witness` |
| DR-03 | DR-01 with a second authored claim over the same subject | still exactly one race-freedom item; its obligation identity is (subject, resolved models, `race-freedom`) |
| DR-04 | every shared access atomic | no race-freedom item |
| DR-05 | `sc` resolved model, `data` non-atomic | no race-freedom item |
| DR-06 | `tso`, non-atomic store in one branch and atomic load in the other, no synchronisation | `refuted` with a race counterexample |
| DR-07 | DR-06 with the store before the `parallel`'s fork and the load inside a branch | `proved`: the fork orders them |
| DR-08 | DR-06 with the store's thread sending on a channel after the store and the load's thread receiving before the load | `proved` |
| DR-09 | DR-02's counterexample with `earlier` changed to a position whose access happens before `later` | replay settles `inconclusive`, cause `replay-parity` |
| DR-10 | the authored claim in DR-02 and in DR-02 with `data` accesses made `relaxed` | the same verdict |
| DR-11 | under `ra`: DR-01, DR-02, and the message-passing shape with the flag passed by a `send` after `data := 1` and a `receive` before `LD`; read the state key after the writer's `data := 1`, after its flag store or `send`, and after the reader's flag load or `receive` | as FR-438-AC-6 states: after `data := 1`, `last_write` the writer, non-atomic, and `write` true only in the writer's `cur` and `acq`; the new `flag` message's view `write` true under DR-01 and false under DR-02; under DR-01 the reader's `cur` `write` true after a flag load that reads the new message, and the data load completes no race; under DR-02 the data load completes a race; in the channel variant the queue entry's `view` is the writer's `cur` with `write` true, the reader's `cur` has `write` true after the `receive`, and the item is `proved`; DR-04 has no entry and no `race` member |
| DR-12 | under `tso`: the message-passing shape with a `release` flag store and `acquire` flag load, the same with `relaxed` flag accesses, and the message-passing shape with the flag passed by a `send` after `data := 1` and a `receive` before `LD`; read the state key after the writer's `data := 1`, after its flag store or `send`, after `flush(writer)`, and after the reader's flag load or `receive` | as FR-438-AC-7 states: the `data` entry and the writer's `race-summaries` after `data := 1`; the buffered `flag` element's and then `race-memory`'s `write` true for the `release` store and false for the `relaxed` one; the queue entry's `race` `write` true; the item `proved` for the `release`/`acquire` and channel variants and `refuted` on `data` with `relaxed` flags |
| DR-13 | thread `a` runs `r := x` (non-atomic) then `x := 1` (`release`); thread `b` runs `fetch_add(1)` on `x`; under `ra` and under `tso`, with the RMW `acquire` and then `relaxed` | `acquire`: `proved` under both models; `relaxed`: `refuted` with `race.location` `x`, `earlier` `a`'s load, `later` the RMW (FR-438-AC-8) |

## Expected Results

Every derived-item set, verdict and replay outcome matches the table; replay
decides happens-before with vector clocks independent of the race summary.
