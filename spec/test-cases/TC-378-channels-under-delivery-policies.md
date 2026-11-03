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
`fifo` and an `unordered` channel under every delivery policy and a delivery
count interval, the three
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
| CH-13 | `fifo`, `[2,2]`, capacity 2, overflow `block`; the sender sends only `m1` and the receiver receives three times from that send node | The `send` has one step, `count` `"2"`, queuing two entries of `m1` with send record `entries` two `accepted` and `pending` `"0"`; the first two receives complete on every behaviour and the third waits `empty-queue`; no `duplicate` or `lose` step exists |
| CH-14 | as CH-13 with `[0,1]` | The `send` has two steps, `count` `"0"` (no entry, `entries` `[]`) and `count` `"1"`; after `count` `"0"` the first receive waits `empty-queue`, after `count` `"1"` it completes; `d = 0` is the only behaviour that loses `m1` |
| CH-15 | as CH-13 with `[1,3]` and no receiver | The `send` has three steps: `count` `"1"` and `"2"` queue one and two entries and settle past the send; `count` `"3"` appends two entries, records `pending` `"1"`, leaves the thread at the send with a `send-pending` count 1 in the state key, and the thread waits with cause `full-queue` naming the channel |
| CH-15r | CH-15's `count` `"3"` behaviour with the receiver receiving once | After the receive, the pending send has one step, `count` `"1"` with record `m1`, appending the last entry with `pending` `"0"`; the thread settles past the send |
| CH-16 | as CH-13 with `[1,1]`, and separately with `exactly-once-premise` | `[1,1]` queues exactly one entry per send and enables no `duplicate` or `lose` step; the two channels have different channel premises, and the policy-word channel's send record holds `outcome` and `displaced` where the interval's holds `count`, `entries` and `pending` |
| CH-17 | `fifo`, `[2,2]`, capacity 2, overflow `reject`, with `m0` already queued | The `send` processes two entries: `entries` `accepted` then `rejected`, `pending` `"0"`, queue `[m0, m1]` |
| CH-18 | `fifo`, capacity 1, overflow `block`, `[2,2]`; branch `a` sends `m1` and branch `b` sends `m2` on two send nodes of the channel; a receiver takes one entry | The behaviour where `a` sends first leaves `a` pending `{1, m1}` and the one where `b` sends first leaves `b` pending `{1, m2}`, two states with different keys; after the receive, the pending send's last step has `count` `"1"`, `record` its own payload, appends that payload and binds its binder to it |
| CH-19 | `fifo`, capacity 1, overflow `block`, `[0,3]`, with `m0` already queued | The `count` `"0"` step is enabled, queues nothing and settles past the send; the steps with `count` `"1"` to `"3"` wait `full-queue` |
| CH-20 | `[0,0]`, capacity 0, overflow `block` | The send's only step, `count` `"0"`, is enabled and completes the send; no deadlock is reported at it |
| CH-21 | `[0,1000]`, `reject`, capacity 2, with a work limit below the charges of the send's 1001 steps; then with a work limit that fits | The first run settles `incomplete` naming the work limit, its frontier holding the state at the send, and no count is dropped or merged; the second has 1001 `send` steps from that state, `count` `"0"` to `"1000"`, generated in ascending `count` |
| CH-09 | `await W after S within [0, 5] match receive R … then … timeout …` with no sender | `timeout(W)` is enabled at every state where `W` waits; no clock value appears in any state key |

## Expected Results

Every enabled-step set and queue content matches the table, and the queue of
each channel is keyed per protocol instance and FIFO key as FR-425 states.
