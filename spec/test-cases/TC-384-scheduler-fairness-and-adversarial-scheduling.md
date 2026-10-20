---
id: TC-384
title: "Scheduler fairness and adversarial scheduling"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-430
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-433
    type: verifies
---

# TC-384: Scheduler fairness and adversarial scheduling

## Description

Check the derived scheduler constraints, their `scheduler` origin and
identity, the `scheduling adversarial` opt-out, protocol fairness targets and
the replay of a starving lasso.

## Test Procedure

Model: object type `Flag` with `done: Boolean`, universe `{f}`, initial
`f.done = false`; operation `set()`, postcondition `self.done = true`, frame
`modifies self.done`. Message type `Ping` with one value.

```text
protocol Starve using p over (k: M::Flag) on origin {
  role w on M;
  channel h from w to w carries M::Ping ordering unordered
    delivery unknown capacity 2 overflow block;
  run parallel Both {
    branch noisy send S via h as (m: M::Ping) { true };
    branch quiet attempt Raise by w on M::Flag::set contracts [] as (a: M::Flag) { true };
  } join all [noisy, quiet];
  finish Done as (d: M::Flag) { true };
}
```

After `S`, the channel's `duplicate` and `lose` steps can alternate for ever
while `quiet` waits at an enabled `Raise`. Check the claim `eventually
holds(k.done)` under infinite-trace through the real local Rust boundary, with
and without `scheduling adversarial;`, and record the resolved fairness set,
the obligation identity, the verdict and the counterexample.

Parse the protocol text above at S1 under the `quire.value.complete/v1` profile and require it admissible with no diagnostic (every protocol fixture of the test cases parses).

## Vectors

| Vector | Variant | Expected result |
| --- | --- | --- |
| SF-01 | as built | Fairness set: three members, weak `whole`, origin `scheduler`, targets `noisy`, `quiet` and the root; the claim settles `proved`, since the duplicate/lose loop starves `quiet` while `Set` stays enabled and is unfair |
| SF-02 | with `scheduling adversarial;` | Fairness set empty; the claim settles `refuted` with a lasso whose loop is `duplicate`, `lose` on `h` and whose positions never set `done`; the counterexample replays |
| SF-03 | SF-02's counterexample replayed against the SF-01 subject | refuses `invalid_runtime_input`/`invalid-value`: the loop is unfair under the scheduler constraint on `quiet` |
| SF-04 | identities | SF-01 and SF-02 carry different obligation identities; the safety claim `always holds(true)` settles `proved` under both |
| SF-05 | SF-02 with the authored `fair Starve::Both::quiet;` | Fairness set holds one `authored` member; the claim settles `proved` |
| SF-06 | SF-02 with the authored `fair Starve::Both::quiet::Raise;` (attempt node) | the claim settles `proved`; the constraint is discharged by the `Raise` step |
| SF-07 | a branch `outer` holding a nested `parallel Inner` with branch `deep`, scheduling adversarial, authored `fair P::Outer::outer;` | a step of `deep`'s thread does not discharge the constraint on `outer` |
| SF-08 | an authored constraint naming `Starve::Nowhere`, and a clause with two `scheduling` members | the first refuses `missing_declaration`/`missing-name`; the second refuses `ambiguous_declaration`/`ambiguous-name` |
| SF-09 | a `parallel` inside a `repeat` with `outstanding continue` giving two live threads of branch `b`, scheduling adversarial, authored `fair each P::Par::b;` | a loop that takes steps of one `b` thread only, while the other stays enabled, is rejected; the same loop under `fair whole P::Par::b;` is admitted |

## Expected Results

Every fairness set, verdict, identity comparison and replay outcome matches
the table.
