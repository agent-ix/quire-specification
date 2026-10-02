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
| TD-08 | One branch at a receive whose channel is empty and whose sender has finished | `empty-queue` naming the channel |
| TD-09 | Root at a join `all [a, b]` with `a` completed and `b` blocked | root `join-waiting` `[b]`, and `b` with its own cause |
| TD-10 | One branch at `check C using p { false };` | `check-false` naming `C` |
| TD-11 | A `tso` `parallel` whose branch holds a buffered store at its `join`, with `max_store_buffer` reached elsewhere | the join waits only on the buffer, and the state is not reported as a deadlock, since `flush` is enabled |
| TD-12 | A state cut by the instance budget (TC-381 IB-01) | not terminal and not a deadlock |
| TD-13 | `Fill` with an empty population `extras` of `Cell` and `terminal when { lookup<M::Cell>(extras, k) absent undefined .v = 1 };` | the deadlock-freedom item and the authored claim settle `refuted`, cause `undefined-evaluation`, with the prefix to that state and `where` naming `P` |

## Expected Results

Each derived-item set, verdict and blocked list matches the table, with
blocked threads ordered by the JCS bytes of their paths.
