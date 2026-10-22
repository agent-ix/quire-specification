---
id: TC-380
title: "Compensation in a protocol subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-427
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-431
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-239
    type: verifies
---

# TC-380: Compensation in a protocol subject

## Description

Explore the `Pay` protocol with a refund compensation and check its states,
registration, activation, `cattempt` and `cend` steps, commit cutoff, and two
counted-step interval verdicts.

## Test Procedure

Model: object type `Acct` with `bal: Int[0, 1]`, universe `{x}`, initial
`x.bal = 0`. Operation `charge()`, precondition `ChargePre: self.bal = 0`,
postcondition `ChargePost: self.bal = 1`; operation `refund()`, precondition
`self.bal = 1`, postcondition `self.bal = 0`; both with frame `modifies
self.bal`. Record type `Failure` with field `failed: Boolean`. The effect record
type has one value, and the finish binder `d: M::Acct` ranges over the one
`Acct` object of the single-object universe.

```text
protocol Pay using p over (k: M::Acct) on origin {
  role w on M;
  compensate Refund for Main::Paid as (fwd: M::ChargeEffect)
      by w on M::Acct::refund using p clock "c" {
    activate first (f: M::Failure) when { f.failed } { }
    within [0, 10];
    attempts 1 of M::RefundAttempt;
    retry (e: M::RefundAttempt, l: M::RefundAttempt) { false };
    commit never;
    recover (r: M::RecoveryView) { r.bal = 0 };
  }
  run sequence Main {
    attempt C by w on M::Acct::charge contracts [ChargePre, ChargePost] as (c: M::Acct) { true };
    effect Paid of Main::C as (pd: M::ChargeEffect) { true };
    event Fail by w as (f: M::Failure) { true };
  }
  finish Done as (d: M::Acct) { true };
}
```

Explore exhaustively under `sc` through the real local Rust boundary and
record every state, registration status and enabled step.

Parse the protocol text above at S1 under the `quire.value.complete/v1` profile and require it admissible with no diagnostic.

## Vectors

| Vector | Check | Expected result |
| --- | --- | --- |
| CP-01 | Reachable states | Nine states, eight transitions: t0 (`bal 0`, at `C`) `attempt(C)`; t1 (`bal 1`, at `Paid`) `event(Paid)`; t2 (at `Fail`, `registered`) `event(Fail)` with `failed` false to t3 and true to t4; t3 (at finish, `registered`) `finish` to t5; t4 (at finish, `activated`, compensation thread at its attempt, count 0) `cattempt(Refund)` to t6; t5 finished, `closed`; t6 (`bal 0`, at retry point, count 1) `cend(Refund)` to t7; t7 (`recovered`) `finish` to t8; t8 finished |
| CP-02 | Finish gating | `finish` is not enabled at t4 or t6, where the compensation thread is running |
| CP-03 | Only `cend` at the limit | at t6 the count equals `attempts 1` and the retry relation is false, so `cend` is the thread's only step; the recovery predicate holds and the status is `recovered` |
| CP-04 | Deadlock-freedom item | `proved`, basis `exhaustive`; both terminal states are finished |
| CP-05 | `eventually[0,2] holds(k.bal = 0)` on the failure path t0, t1, t2, t4, t6, t7, t8 | holds at position 2 (distance 2 to t6, counting `event(Fail)` and `cattempt`) and fails at position 1 (distance 3); `cend` and `finish` are uncounted |
| CP-06 | same claim on the success path t0, t1, t2, t3, t5 | fails at every position from 1 |
| CP-07 | variant with a `commit Settled by w as (s: M::Settle) { true };` node between `Paid` and `Fail`, and the template's `commit Main::Settled;` | the `commit` step closes the `registered` registration (status `closed`); `event(Fail)` with `failed` true then activates nothing, no compensation thread exists and no `cattempt` is enabled; a template forward effect after the commit step registers nothing |
| CP-08 | variant with two sibling branches each binding a `Failure` record with `failed` true | two behaviours, each activating the registration once at the first trigger taken; the second trigger changes no registration |
| CP-09 | Undefined recovery predicate | A variant whose `recover` predicate reads `lookup<T>(p, r) absent undefined` at a key the population does not hold settles every item `refuted`, cause `undefined-evaluation`, with the prefix to t6 and `where` naming the recovery predicate; the registration is never recorded `unrecovered` |
| CP-10 | Undefined activation guard | A variant whose `activate first` guard reads `lookup<T>(p, r) absent undefined` at a key the population does not hold settles every item `refuted`, cause `undefined-evaluation`, with the prefix to t2, at the `event(Fail)` expansion |
| CP-11 | Attempt type position | The model gains a scalar type `Total` and a variant type `Mode`. Variants of the protocol text above with `attempts 1 of Set<M::Total>[0, 3]`, `attempts 1 of Integer` and `attempts 1 of M::Mode` each parse at S1 with no diagnostic; checking each refuses once `invalid_model_binding`/`malformed-declaration` at the type, retaining the compensation `Refund`, member `attempts` and the type as written, with no refusal at the `retry` binders; the unchanged `attempts 1 of M::RefundAttempt` checks (FR-239-AC-8) |

## Expected Results

Every count, status and enabled-step set matches the table; the interval
verdicts match CP-05 and CP-06.
