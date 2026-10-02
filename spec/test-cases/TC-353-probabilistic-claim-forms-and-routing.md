---
id: TC-353
title: "Probabilistic claim forms, confidence parameters and evidence routing"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-407
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-408
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-353: Probabilistic claim forms, confidence parameters and evidence routing

## Description

Admit each claim form, apply the quantile reduction, check confidence-parameter identity rules, and route items by evidence kind without substitution.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Register a statistical-only backend for RT vectors unless the vector says otherwise.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Health`** and **`NoFault`** (from QSL ADR-024 §7.1). One object `n: Node { healthy: Boolean }`, initially `healthy = true`. Operation `Node::tick(fault)` has post `healthy = pre(healthy) and not fault`, with `random Node::tick(fault) ~ { true: 1, false: 9999999 };` and `workload Steady on Health { weight Node::tick = 1; }`. `NoFault` is `probabilistic NoFault using ev on Health under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0005 { probability >= 0.999 [ always[0, 1000] holds(n.healthy) ] }`. Its exact value is `(9999999/10000000)^1000`.
- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.
- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= 0.999 ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PF-01 | Each FR-407 form over a model subject with one scheduler clause | Admitted; FR-290 kind `probabilistic-satisfaction` |
| PF-02 | `under Steady` naming a workload of another model; `under Steady fair weak Server::tick` | `foreign-workload`; `fairness-under-workload` |
| PF-03 | A quantile claim over a subject where activation fails on some behaviours and one behaviour is censored | Not-activated behaviours are excluded from the conditional probability; the censored behaviour exceeds every finite threshold |
| PF-04 | `quantile 0.95 of M <= 5 ms` and `probability >= 0.95 [M <= 5 ms]` conditioned on activation over SP-01 | Same verdict; the conditional probability is `24233/25000` |
| PF-05 | `probability >= 1 [E]`; a threshold `5000 us` against a `duration` in ms | `qualitative-threshold`; exact conversion to `5 ms` |
| PF-06 | A claim with `alpha` and `beta` but no `indifference`; the same property at two confidence sets | `partial-confidence`; two distinct obligation identities |
| PF-07 | Changing the evidence kind, method or a budget of one item | Obligation identity unchanged |
| PF-08 | `Avail` with `reward cost on Server::tick = 1;` and `reward cost on Server::repair = 2;`, claim `long_run_fraction holds(v.up) weighted by cost >= 0.9` | Admitted; evaluates the weighted ratio |
| PF-09 | `Two` is one object `t: Box { done: Boolean }`, initially `done = false`, with `Box::go(d)` (pre `not done`, post `done`), `random Box::go(d) ~ { 1: 1, 3: 1 };`, `reward cost on Box::go = d;` and `workload W on Two { weight Box::go = 1; }`; `M` is `accumulate cost from holds(not t.done) until holds(t.done) within 1`, which is 1 or 3 with probability `1/2` each; claims `quantile 0.5 of M >= 3` and `quantile 0.5 of M >= 1` under `W` | Admitted; the first reduces to `Pr(M < 3 | activated) < 0.5`, which is false at `1/2`; the second reduces to `Pr(M < 1 | activated) < 0.5`, which is true at 0 |
| PF-10 | PF-09's `Two` with `M` changed to `accumulate cost from holds(t.done and not t.done) until holds(t.done) within 1`, which never activates; `quantile 0.5 of M >= 3` and `quantile 0.5 of M <= 1`, each under `W` and under every scheduler, with exact evidence; then `quantile 0.5 of M <= 1` under `W` with `alpha 0.01, beta 0.01, indifference 0.05`, statistical evidence and `max_draws` 100 | The claims hold vacuously and each exact item settles `proved`; the statistical item draws 100 non-activated samples and settles `incomplete`, `limit-reached` naming `max_draws`, value 100 |
| RT-01 | `P95` with confidence parameters naming `statistical`, then naming `exact` with no exact candidate | Routed to the statistical backend; `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| RT-02 | A claim `under every scheduler` naming `statistical`; `NoFault` with no confidence parameters naming `statistical`; an expected-reward claim naming `statistical` | `unsupported`, `every-scheduler`; `missing-confidence`; `exact-only-form`; no engine runs |
| RT-03 | An item with no evidence kind; with evidence kind `approximate` | `invalid-request`, `invalid_capability`/`absent-evidence-kind`; `unknown-evidence-kind` |
| RT-04 | `P95` with confidence parameters requested once per evidence kind, both backends registered | Two items, routed one to each backend, with separate terminal records; each item's limits that are omitted take their published defaults |
| RT-05 | `Health` with `tick`'s post loosened so that `fault = false` has two post-states, a claim with confidence parameters naming `statistical` | Negotiation settles `supported`; sampling stops at the first drawn step with two post-states and settles `unsupported`, `not-markov{state, transition, post_states}` naming both post-states |

## Expected Results

Each form, refusal and routing decision is exactly as stated; negotiation never substitutes one evidence kind for the other.
