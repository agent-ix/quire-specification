---
id: TC-381
title: "Per-trigger activation and the instance budget"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-428
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-429
    type: verifies
---

# TC-381: Per-trigger activation and the instance budget

## Description

Check `activate` steps, instance identity, the `max_live_instances` budget
with its default, the `instance-bound-reached` cause, the statement of the
budget in every result, and the activation-only deadlock.

## Test Procedure

Model: object type `Gate` with `v: Int[0, 1]`, universe `{g}`, initial
`g.v = 0`; operation `take()`, precondition `TakePre: self.v = 0`,
postcondition `self.v = 1`, frame `modifies self.v`. Trigger record type
`Tick` with one value.

```text
protocol Once using p over (k: M::Gate) on each (t: M::Tick) when (true) {
  role w on M;
  run attempt Take by w on M::Gate::take contracts [TakePre] as (a: M::Gate) { true };
  finish Done as (d: M::Gate) { true };
}
```

The first instance takes the gate and finishes; every later instance waits at
`Take` because `TakePre` is false. Submit a request holding the one claim
`always holds(true)` over this subject and check its derived deadlock-freedom
item through the real local Rust boundary under each budget below, recording the
verdict, cause, counterexample and terminal accounting record.

Parse the protocol text above at S1 under the `quire.value.complete/v1` profile and require it admissible with no diagnostic.

## Vectors

| Vector | `max_live_instances` | Expected result |
| --- | --- | --- |
| IB-01 | 1 | Every state with one live instance is instance-limited: (`v 0`, instance at `Take`), (`v 1`, instance at finish) and (`v 1`, instance at `Take`). No deadlock is found, since the stuck state is instance-limited. The item settles `inconclusive`, cause `instance-bound-reached`, bound 1, 3 instance-limited states, naming `max_live_instances`; the terminal record states 1, reached |
| IB-02 | 2 | A state with one live instance waiting at `Take` with `v 1` enables only `activate`, which the budget admits: it is an activation-only deadlock. The item settles `refuted` with a `kind: deadlock` prefix that starts with `activate`, takes and finishes the first instance, and activates a second instance that waits; its blocked list names that instance's root at `Take` with `precondition` `TakePre`; the counterexample replays. The terminal record states 2, reached |
| IB-03 | unset | The run uses 3 and states 3 in the terminal record |
| IB-04 | 1 and 2 | Both requests carry the same obligation identity |
| IB-05 | 2, with `TakePre: true` and `take` writing nothing | Every instance finishes; no deadlock exists; the item settles `inconclusive`, cause `instance-bound-reached`, since states with two live instances are instance-limited |
| IB-06 | 2, ordinal reuse | After the instance with ordinal 0 finishes while ordinal 1 still runs, the next `activate` creates ordinal 0 again |
| IB-07 | 2, `Tick` with one field `n: Option<Int[0, 1]>`, activation guard `when (value(t.n) = 0)`, which reads the trigger record only and is undefined for the record whose `n` is absent | every item settles `refuted`, cause `undefined-evaluation`, with the one-state prefix and `where` naming the activation guard; `activate` is not treated as disabled |

## Expected Results

Each verdict, cause, bound and count matches the table; no instance-limited
state is reported as terminal or as a deadlock; every result over this `on
each` subject states the budget used and whether it was reached.
