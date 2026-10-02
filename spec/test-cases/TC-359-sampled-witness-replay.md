---
id: TC-359
title: "Sampled witness wire and replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-410
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-408
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-409
    type: verifies
---

# TC-359: Sampled witness wire and replay

## Description

Record sampled witnesses, replay them as counterexamples of the every-behaviour claim, and refuse tampered witnesses.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Each statistical vector uses seed `20261001`; run it twice on two platforms and require identical sample counts, estimates and decisions.

The **Flaky** subject: object type `Job` with `done: Boolean`, population
`jobs`, universe `{j, k}`, one initial snapshot with `k.done = true`, and one
operation `step` with random parameter `ok: Boolean` of weights `9/10` for
`true` and `1/10` for `false`, which sets `k.done = ok` when `ok` is true and
removes `k` from `jobs` when `ok` is false. `rk` is a `Reference<F::Job>`
parameter valued `k`. `Lookup` is
`probability >= 0.5 [ always[0, 3] holds((lookup<F::Job>(jobs, rk) absent undefined).done) ]`
under the workload that schedules `step`, with `alpha 0.01, beta 0.01,
indifference 0.01`. On its defined values its probability is `1` and the bound
holds; a draw of `ok = false` within three steps makes the `lookup` undefined.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SW-01 | SC-04 with `max_witnesses` 2 | Two witnesses, the first in trace-index order, each a path with latency `3 ms` or more and its exact path probability (for example `343/5000` for `attempt` with `d = 3 ms`, `Ok`) |
| SW-02 | Replay of SW-01's first witness | Reproduced; it also refutes the every-behaviour claim `always (M <= 2 ms)` over the same subject |
| SW-03 | SW-01's witness with `d = 4 ms`, outside the support | `invalid_runtime_input`/`invalid-value` |
| SW-04 | SW-01's witness with its scheduled identity replaced by another enabled one | Recomputed draw disagrees: `inconclusive`, `replay-parity`; the item's `measured` decision is unchanged |
| SW-05 | `Lookup` with statistical evidence, Okamoto | The run stops at the first sample in trace-index order that draws `ok = false`, and settles `measured: rejected`, cause `undefined-evaluation`: `where` names test 0, that trace index, the position after that draw, its state-key digest and the `lookup`'s locus; `cause` is `absent-key`. That sample is the first sampled witness, carrying the undefined value |
| SW-06 | Replay of SW-05's witness | Reproduced: the recomputed draws select `ok = false` at the recorded step and the claim evaluates undefined there with the `lookup`'s locus and reason `absent-key` |

## Expected Results

Witness content, replay agreement and each refusal are exactly as stated.
