---
id: TC-378
title: "Channels under each ordering, delivery and overflow policy"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-170
    type: verifies
---

# TC-378: Channels under each ordering, delivery and overflow policy

## Description

Check the `send`, `receive`, `duplicate` and `lose` steps of FR-426 for a
`fifo` and an `unordered` channel under every delivery policy, the three
overflow policies at capacity, and the untimed `timeout` of an `await`.

## Test Procedure

Build a protocol whose `sender` branch sends two messages `m1` then `m2` on
channel `h` and whose `receiver` branch receives twice from that send node.
Vary the channel's ordering, delivery policy, capacity and overflow policy as
each vector states, explore exhaustively, and record at each state the queue
contents and the enabled channel steps.

## Vectors

| Vector | Channel | Expected result |
| --- | --- | --- |
| CH-01 | `fifo` by a constant key, `exactly-once-premise`, capacity 2 | With both messages queued, only `m1` is deliverable; no `duplicate` or `lose` step exists; the receiver binds `m1` then `m2` on every behaviour |
| CH-02 | `unordered`, `exactly-once-premise`, capacity 2 | With both queued, two `receive` steps are enabled, one per entry; both receive orders are behaviours |
| CH-03 | `fifo`, `at-least-once`, capacity 3 | `duplicate` is enabled for each queued entry while the queue holds fewer than 3; `lose` never is |
| CH-04 | `fifo`, `at-most-once`, capacity 2 | `lose` is enabled for each queued entry; `duplicate` never is; a behaviour that loses `m1` leaves the receiver waiting with `displaced` |
| CH-05 | `unordered`, `unknown`, capacity 2 | both `duplicate` (below capacity) and `lose` are enabled |
| CH-06 | `fifo`, `exactly-once-premise`, capacity 1, overflow `block` | With `m1` queued, the second `send` is not enabled until the receiver takes `m1` |
| CH-07 | as CH-06 with overflow `reject` | With `m1` queued, the second `send` is enabled, leaves the queue unchanged and records a send record with outcome `rejected` |
| CH-08 | as CH-06 with overflow `loss new` | With `m1` queued, the second `send` is enabled, drops the arriving message, leaves `m1` queued and records a send record with outcome `lost` |
| CH-08h | as CH-06 with overflow `loss head` | With `m1` queued, the second `send` drops `m1`, queues the arriving message and records a send record with outcome `accepted` and `displaced` the entry of `m1` |
| CH-08t | as CH-06 with overflow `loss tail` | With `m1` queued (capacity 1, so `m1` is also the newest), the second `send` drops `m1`, queues the arriving message and records outcome `accepted` with `displaced` the entry of `m1`; with capacity 2 and `m1`, `m2` queued the oldest stays and `m2` is dropped, the send record holding outcome `accepted` and `displaced` the entry of `m2` |
| CH-10 | `fifo`, capacity 2, each overflow policy, with `m1`, `m2` queued and a third send `m3` | `reject`: outcome `rejected`, queue `[m1, m2]`; `loss new`: outcome `lost`, queue `[m1, m2]`; `loss head`: outcome `accepted`, `displaced` the entry of `m1`, queue `[m2, m3]`; `loss tail`: outcome `accepted`, `displaced` the entry of `m2`, queue `[m1, m3]`; on an empty queue the outcome is `accepted` and `displaced` none. Every record holds the channel's qualified name, and the send's binder holds the payload |
| CH-11 | Two protocols of one package each declaring `channel Pay` | The two channels have different queue keys, transition identities and wait causes; no key of one names the other's channel |
| CH-12 | Capacity 1, `block`, `exactly-once-premise`: `sequence { send P via ch; send Q via ch }` with no receiver | The sender at `Q` has no enabled step, waits, and the deadlock reports cause `full-queue` naming the channel |
| CH-09 | `await W after S within [0, 5] match receive R … then … timeout …` with no sender | `timeout(W)` is enabled at every state where `W` waits; no clock value appears in any state key |

## Expected Results

Every enabled-step set and queue content matches the table, and the queue of
each channel is keyed per protocol instance and FIFO key as FR-425 states.
