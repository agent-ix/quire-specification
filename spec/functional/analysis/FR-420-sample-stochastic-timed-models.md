---
id: FR-420
title: "Interpret and sample stochastic timed models"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-408
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
---

# FR-420: Interpret and sample stochastic timed models

## Description

When a probabilistic claim over a timed subject names a workload, the
provider SHALL draw each declared delay from its distribution, with each undeclared
delay left to the scheduler.

## Inputs

A `time dense` timed subject ([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md))
with its operations' `delay` members ([FR-405](FR-405-declare-probabilistic-models.md));
a workload; a probabilistic claim ([FR-407](FR-407-state-probabilistic-claims.md));
for statistical evidence, the request's grid exponent `q`, `max_sample_steps`
and `StatisticalLimits`.

## Outputs

The measure on timed behaviours under the workload, statistical results
([FR-408](FR-408-settle-probabilistic-claims-statistically.md)) whose samples
are timed behaviours, or an `unsupported` result with cause `not-stochastic` or
`time-locked-sample`.

## Behavior

### Delay distributions

An operation's `delay ~ D` member gives one of these families, every
parameter an exact rational and every delay a time quantity converted exactly
to the model's unit (FR-142):

- `uniform` — uniform over the identity's window;
- `uniform[a, b]` — uniform over the delays in `[a, b]`, conditioned on the
  window;
- `exponential(λ)` — rate `λ`, a positive exact rational per time unit,
  conditioned on the window;
- `discrete { d1: w1, …, dn: wn }` — finitely many distinct non-negative
  delays with positive exact rational weights, conditioned on the window.

Uniform over a set is uniform in length; over a set of zero length it is
uniform over its finitely many points. The delay of an operation with a
`delay` member is **given**; the delay of an operation with no `delay` member
is **free**: it is nondeterministic, and no distribution is implied for it.

### Windows

At a timed state `(s, v)`, the **window** of a scheduled identity is the set
of delays `d` for which `(s, v + d)` is reached by an admissible delay and the
identity's data precondition and guard hold there. A window is a finite union
of intervals with exact rational ends, open or closed. An identity with an
empty window, or whose given distribution has zero mass in its window, does
not race.

### The race

Under a workload, at each timed state every racing identity with a given
delay draws its delay from its distribution conditioned on its window. The
scheduler then chooses, after those draws, a delay in the window of each
racing identity with a free delay. The smallest delay wins: the model delays
by it and takes the winning identity with its random arguments drawn
(FR-406). When several identities tie at the smallest delay, the workload
orders them, drawing without replacement by operation weight and then
uniformly within an operation; the model delays once and takes them in that
order at the same instant, each only when it is still enabled at its turn.
Every identity redraws at the new state. With every racing delay given, the
timed subject under the workload is a Markov chain over timed states; free
delays make it a Markov decision process, and a bound compares the minimum
(for `>= θ`) or the maximum (for `<= θ`) over the free delays' choices.

A timed state where an identity with an unbounded window has `uniform`, or
where no identity races and delay is bounded, has no defined race and settles
`unsupported`, `not-stochastic{state, identity}`. A quiescent state where no
identity races takes the idle tail (FR-161).

### Events and measures in time

Under the timed profile, an event is a bounded-window formula decided by the
positions with time stamp up to its time horizon; an `elapsed` measure is a
time-stamp difference (FR-407), and `weighted by delay` weights each position
by the delay of the step that leaves it, so a long-run fraction is the
fraction of time a predicate holds.

### Sampling timed runs

Statistical evidence requires every racing identity's delay to be given; a
free delay that races settles the item `unsupported`, `not-stochastic`, at the
first state where it races. A sample repeats the race from the initial state.
Each delay draw and each tie-break is one more choice of FR-408's step, with
its own choice index in the sampler preimage. A `discrete` draw selects by
exact integer weights. A `uniform`, `uniform[a, b]` or `exponential` draw
takes a uniform value `u` on the grid of `2^-q` from the generator and returns
the conditioned inverse distribution function at `u`, computed with FR-408's
rational bounds and rounded up, so every sampled delay is an exact rational.
A sampled delay is the declared distribution quantised to that grid, which
changes each draw's distribution function by at most `2^-q`; the `measured`
result is a measurement of the quantised model, and its basis records `q`. A
sample ends when its next position's time stamp passes the event's or
measure's time horizon, or at an idle tail. A sample that reaches a time-locked
state before its horizon, where no positive delay is admissible and no
transition identity is enabled, settles the item `unsupported`,
`time-locked-sample{state}`, naming that state, and keeps no witness from it:
the measure is over time-divergent behaviours only, and the time-lock-freedom
item ([FR-417](../temporal/FR-417-settle-time-lock-freedom.md)) reports the
time-lock. A sample that takes more steps
than `max_sample_steps` before passing its horizon stops the run with
`incomplete`, `limit-reached` naming `max_sample_steps`, its value and the
request setting that raises it. `q`, `max_sample_steps` and the
`StatisticalLimits` members are request settings with published defaults;
the default of `max_sample_steps` is `2^24`.

### Regeneration and witnesses

The regenerative method cuts a timed run at each return to the initial
discrete state with every clock 0. A sampled witness over a timed subject
carries each step's delay and replays by FR-181 timed replay plus the draw
recomputation of FR-410, which recomputes each delay from the seed, trace
index, step, choice index and `q`.

### Every-behaviour claims

Non-probabilistic claims over a stochastic timed model read it as a timed
model and ignore distributions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-420-AC-1 | On the `RareLate` vector, `reply` with `delay ~ discrete { 1 ms: 90, 3 ms: 10 }` ties with `timeout` at 3 ms with probability `1/10`, the workload orders the tie evenly, and the event's exact probability is `1/20`. | Test (TC-374) |
| FR-420-AC-2 | Every sampled delay is an exact rational from the `2^-q` grid's conditioned inverse distribution function, `q` is recorded in the basis, and each sample replays with its delays. | Test (TC-374) |
| FR-420-AC-3 | A racing identity of an operation with no `delay` member settles a statistical item `unsupported`, `not-stochastic`; under exact evidence its delay is a scheduler choice and the bound compares the minimum or maximum. | Test (TC-374) |
| FR-420-AC-4 | An identity with an unbounded window and `uniform`, or a state with bounded delay and no racing identity, settles `unsupported`, `not-stochastic{state, identity}`. | Test (TC-374) |
| FR-420-AC-5 | A sample exceeding `max_sample_steps` stops the run with `incomplete`, `limit-reached` naming `max_sample_steps`, its value and how to raise it; an omitted `max_sample_steps` is `2^24`. | Test (TC-374) |
| FR-420-AC-6 | With `reply` under `delay ~ uniform` the event's probability is 0, the statistical item settles `measured: accepted`, and the every-behaviour claim `NoLateReply` settles `refuted`. | Test (TC-374) |
| FR-420-AC-7 | A sample that reaches a state with no admissible positive delay and no enabled transition identity before its horizon settles the item `unsupported`, `time-locked-sample{state}` naming that state, with no witness kept. | Test (TC-374) |

## Dependencies

- [FR-405](FR-405-declare-probabilistic-models.md) declares `delay` members;
  [FR-406](FR-406-interpret-probabilistic-models.md) supplies the scheduler
  resolution; [FR-408](FR-408-settle-probabilistic-claims-statistically.md)
  supplies the statistical methods.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) supply timed
  moves and replay.

## References

- QSL ADR-026 SD-1 to SD-5, SS-1 to SS-6, OV-8; QSL ADR-028 RU-6, TA-1a.
- Owner rulings: stochastic delays with statistical checking (recorded on
  QSL-373); a workload that gives delays resolves delay choice and leftover
  delay nondeterminism is a minimum or maximum (recorded on QSL-371).
- A. David et al., "Statistical model checking for networks of priced timed
  automata", FORMATS 2011.
- Ticket: STD-139 (paired with QSL-373); STD-137 (paired with QSL-371).
