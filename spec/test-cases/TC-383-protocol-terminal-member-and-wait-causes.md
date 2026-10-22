---
id: TC-383
title: "Protocol terminal member, deadlock opt-out and wait causes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-429
    type: verifies
---

# TC-383: Protocol terminal member, deadlock opt-out and wait causes

## Description

Check the protocol-level `terminal` member, the derived deadlock-freedom item
and its opt-out, the boundary-state rule, and each wait cause in the
blocked-thread list.

## Test Procedure

For each vector, build the protocol, submit a request holding one authored
claim over it, and record the request's derived items, each verdict, and the
blocked-thread list of every deadlock counterexample.

## Vectors

| Vector | Protocol | Expected result |
| --- | --- | --- |
| TD-01 | `Fill` of TC-376 with `terminal when { k.v = 1 };` | The deadlock-freedom item settles `proved`: the only unfinished terminal state has `c.v = 1` |
| TD-02 | `Fill` with `terminal when { k.v = 2 };` | `refuted` at the state with `c.v = 1` |
| TD-03 | `Fill` with `terminal any;` | No deadlock-freedom item is added; the authored claim's verdict equals its verdict without the member |
| TD-04 | `Fill` with two `terminal` members | refuses `ambiguous_declaration`/`ambiguous-name` naming both |
| TD-05 | One branch at an attempt whose second selected precondition clause is false | blocked cause `precondition` naming that clause |
| TD-06 | One branch at an attempt whose preconditions hold and whose postcondition no frame post-state satisfies | `no-post-state` |
| TD-07 | One branch at an event whose constraint is `false` | `constraint` |
| TD-08 | One branch at a receive whose channel is empty and no entry from its send node was ever lost | `empty-queue` naming the channel's qualified name |
| TD-08a | `fifo` channel, capacity 2, `exactly-once-premise`: `send P; send Q; receive RQ of Q` | `head-blocked` at `RQ` with the queue `[P, Q]`; the same on an `unordered` channel is not a deadlock |
| TD-08b | `fifo` and `unordered` channels, `exactly-once-premise`, `receive RP of P as (x) { false }` after `send P; send Q` | `constraint` at `RP` on both, since an entry from `P`'s send node is deliverable and none satisfies the constraint |
| TD-08c | Capacity 1, `block`, `exactly-once-premise`: `send P; send Q`; and the same with `send Q … as (q) { false }` | `full-queue` at the second send; `constraint` for the variant, although the queue is full |
| TD-08d | Every channel `exactly-once-premise` unless stated. Capacity 1, `loss head`: `send P; send Q; receive RP of P`; capacity 1, `loss new`: `send P; send Q; receive RQ of Q`; `at-most-once`: `send P; receive RP of P`, reached by `send` then `lose` | `displaced` at the receive in each, with the queue `[Q]`, `[P]` and empty |
| TD-08e | Capacity 1, `loss head`, `exactly-once-premise`: `repeat maximum 2 { send P }; receive R1 of P; receive R2 of P`; a receive on a channel nothing sends to | `empty-queue` at `R2` (the last removal was a `receive`); `empty-queue` |
| TD-08f | Capacity 1, `loss head`, `at-most-once`; branch `s` `repeat maximum 2 { send S of P }`, branch `r` `receive R1 of S; receive R2 of S`. Replay (FR-433) prefix A (send, send displacing the first, `R1`) and prefix B (send, `R1`, send, `lose`) | Both end in one state at `R2` with an empty queue; the blocked list is `empty-queue` for A and `displaced` for B |
| TD-08g | Two protocols of one package each declaring `channel Pay`, each blocked at a receive | the two causes name two different qualified channel names, each the protocol's qualified name, `::` and `Pay` |
| TD-08h | A compensation template whose commit node steps after its thread is created and before its first `cattempt`, with a finish record available | the compensation thread `commit-closed` naming the template and the root at `finish` `finish-waiting` naming that thread |
| TD-08i | A node `by r` for a replicated role with an empty population universe, and the same with every object retired | `no-role-instance` naming `r`; with a `spawn` enabled the thread is not so reported |
| TD-08j | A `tso` state with a non-empty buffer and a thread at a `seq_cst` fence, passed to the cause function directly | cause `gate`; the state enables `flush`, is not terminal, and appears in no deadlock report |
| TD-09 | Root at a join `all [a, b]` with `a` completed and `b` blocked | root `join-waiting` `[b]`, and `b` with its own cause |
| TD-10 | One branch at `check C using p { false };` | `check-false` naming `C` |
| TD-11 | A `tso` `parallel` whose branch holds a buffered store at its `join`, with `max_store_buffer` reached elsewhere | the join waits only on the buffer, and the state is not reported as a deadlock, since `flush` is enabled |
| TD-12 | A state cut by the instance budget (TC-381 IB-01) | not terminal and not a deadlock |
| TD-13 | `Fill` with an empty population `extras` of `Cell` and `terminal when { lookup<M::Cell>(extras, k) absent undefined .v = 1 };` | the deadlock-freedom item and the authored claim settle `refuted`, cause `undefined-evaluation`, with the prefix to that state and `where` naming `P` |

## Expected Results

Each derived-item set, verdict and blocked list matches the table, with
blocked threads ordered by the JCS bytes of their paths.
