---
id: FR-408
title: "Settle probabilistic claims with statistical evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-407
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-409
    type: references
  - target: ix://agent-ix/quire-specification/FR-410
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-408: Settle probabilistic claims with statistical evidence

## Description

When a `probabilistic-satisfaction` item names `statistical` evidence, the
provider SHALL sample behaviours of the subject under the claim's workload
with the `quire.simulation.sampler/v1` sampler, decide the claim's event or
measure on each sample, and decide the claim from the counts with the method
the request names, at the claim's own confidence parameters.

## Inputs

An admitted claim ([FR-407](FR-407-state-probabilistic-claims.md)) under a
workload, with its confidence parameters; the request's method, seed,
`min_cycles`, `max_witnesses` and `StatisticalLimits` (FR-331).

## Outputs

A `measured` result ([FR-409](../../objects/protocol/FR-409-measured-decision.md),
[FR-410](../../objects/interfaces/FR-410-statistical-verdict.md)), or an
incomplete or unsupported result with its typed cause.

## Behavior

### Admission

At negotiation, before any sample is drawn, the provider settles the item
`unsupported` with:

- `missing-confidence` when the claim states no confidence parameters;
- `every-scheduler` when the claim's scheduler clause is `under every
  scheduler`, with or without a fairness set;
- `exact-only-form` when the claim's form is an exact-only form (FR-407).

At negotiation the provider also settles `invalid-request` with
`invalid_capability`/`not-statistical` a request that names the sequential
method for a mean of a fraction (see "Sequential method"). No engine runs for
that item, and the disposition is its one terminal accounting record.

While sampling, the provider stops the run and settles the item `unsupported`
with:

- `not-markov` when a drawn step breaks FR-406's Markov condition, at the
  first such step;
- `not-stochastic` over a timed subject when a racing identity's operation
  declares no `delay` member, at the first state where it races
  ([FR-420](FR-420-sample-stochastic-timed-models.md));
- `time-locked-sample{state}` over a timed subject when a sample reaches a
  time-locked state before its horizon (FR-420).

### Tests and Bonferroni

The claim holds for every initial state and every `over` binding, so the
provider runs one statistical test per (initial state, binding) pair, `m`
tests in all, each at `α' = α / m` and `β' = β / m`. The family-wise errors
then stay within the claim's `α` and `β` whatever the dependence between
tests. Under an admitted symmetry declaration whose group leaves the workload
and every random-parameter distribution invariant, the provider runs one test
per orbit of pairs, the least pair in key order, and `m` counts orbits.

### Sampling and trace indices

Sample `i` of test `j` starts from test `j`'s initial state and uses trace
index `i · m + j` of the sampler; no draw selects the start. At each step the
sampler draws once for the scheduler (operation by weight, then scheduled
identity uniformly), then once per random parameter in declaration order,
each by exact integer weights with its own choice index (FR-181). Distinct
trace indices give independent draws.

### Fixed-sample method (Okamoto)

For an event with true probability `p`, the provider draws
`N = ⌈ln(1 / min(α', β')) / (2 ι²)⌉` samples and reports the interval
`[p̂ − ι, p̂ + ι]` around the exact estimate `p̂`. For a `>= θ` bound the test
**accepts** when the interval lies at or above `θ`, **rejects** when it lies
below `θ`, and is **undecided** otherwise; a `<= θ` bound is symmetric. For
the strict reduced bound of `quantile q of M >= c` (FR-407) the test accepts
only when the interval lies strictly below `q`; an interval whose upper end
equals `q` is undecided. The
same `N` decides a mean of a fraction, whose values lie in `[0, 1]`.

### Sequential method (SPRT)

For `>= θ` the test is Wald's sequential probability ratio test of
`H0: p >= p0` against `H1: p <= p1`, `p0 = θ + ι`, `p1 = θ − ι`. Each counted
sample adds `ln(p1 / p0)` when the event holds and
`ln((1 − p1) / (1 − p0))` when it fails. The test **rejects** when the sum
reaches `ln(1 / α')` and **accepts** when it reaches `ln(β')`. A `<= θ` bound
swaps the roles of the hypotheses. The run stops at the first decision or at
the `max_samples` budget. The sequential method decides a probability bound or
a quantile; a request naming it for a mean of a fraction, whose samples are
not Bernoulli, settles `invalid-request` with
`invalid_capability`/`not-statistical` at negotiation, and the fixed-sample
method decides that form.

### Activation

A quantile claim is conditioned on activation. A sample that is not activated
is drawn and not counted: it advances the trace index and counts toward
`max_draws`, never toward `N` or the SPRT sum. Sampling cannot observe that
activation has probability 0, the case in which the claim holds vacuously
(FR-407): such a run draws only non-activated samples until `max_draws` and
settles `incomplete`, `limit-reached` naming `max_draws`. That is the expected
statistical outcome for the case, not a contradiction of the exact verdict.

### Regenerative method

For a long-run fraction, the provider samples one behaviour from `s0` and cuts
it into regeneration cycles at each return to `s0` (equal state key; over a
timed subject, the initial discrete state with every clock 0). Cycle `k`
contributes its weighted count `Y_k` of positions where `P` holds and its
total weight `L_k`. The estimate is `Â = Σ Y_k / Σ L_k`, and the interval
half-width is the Chow–Robbins half-width

`z · √(S² + 1/n) / (L̄ · √n)`

for `n` complete cycles, `S²` the sample variance of `Y_k − Â · L_k`, `L̄` the
mean cycle weight and `z` the one-sided `1 − min(α', β')` standard normal
quantile. The `1/n` term keeps the half-width positive when `S²` is 0, as it
is while every complete cycle has the same ratio `Y_k / L_k`, for example
before any cycle has left the regeneration state, so a degenerate sample
never yields a zero-width interval. The run stops once the half-width is at
most `ι` and at least `min_cycles` cycles are complete, and decides as the
fixed-sample method. Its coverage is `asymptotic`; the fixed-sample and
sequential methods have coverage `finite-sample`. A cycle that does not return
to `s0` within `max_cycle_steps` steps stops the run with `incomplete`,
`limit-reached{limit, value, setting}` naming `max_cycle_steps`, its value and
the request setting that raises it, as every other budget does.

### Exact rational bounds

Counts, estimates, interval ends and thresholds are exact rationals. Each
logarithm, square root and normal quantile above is replaced by a lower and an
upper rational bound, each within `2^-64` of the true value, and every rounded
quantity is rounded outward, toward "undecided", so that no rounding can make
a decision that the true values would not make:

- **Okamoto.** `N` uses the upper bound of `ln(1 / min(α', β'))` and rounds
  up.
- **SPRT.** The provider keeps two sums. The lower sum adds each sample's
  increment at its lower bound, and the upper sum adds it at its upper bound,
  so the true sum lies between them. The rejection threshold `ln(1/α')` takes
  its upper bound and the acceptance threshold `ln(β')` its lower bound, both
  away from zero. The test rejects only when the lower sum reaches the
  rejection threshold, and accepts only when the upper sum reaches the
  acceptance threshold.
- **Regenerative half-width.** The half-width takes the upper bound of `z`,
  the upper square-root bound of the exact rational `S² + 1/n`, the exact
  rational `L̄` and the lower bound of `√n`, so the computed half-width is at
  least the true one. The run stops only when this upper bound is at most
  `ι`, and the interval uses it.

The bounds are computed as follows, all in integer arithmetic:

- `ln x` for a rational `x > 0`: write `x = 2^k · y` with `1 <= y < 2`; then
  `ln x = k · ln 2 + 2 · atanh((y − 1) / (y + 1))`, `ln 2 = 2 · atanh(1/3)`,
  and `atanh(u) = Σ u^(2n+1) / (2n + 1)` summed until the geometric tail bound
  `|u|^(2n+3) / ((2n + 3)(1 − u²))` is below `2^-70`; the partial sum and the
  tail bound give a lower and an upper rational bound.
- `√x`: the integer square root of `⌊x · 2^140⌋` and its successor, scaled by
  `2^-70`, give the lower and upper bounds.
- the standard normal quantile `z_{1−δ}`: bisection over rationals on
  `Φ(z) = 1/2 + (1/√(2π)) Σ (−1)^n z^(2n+1) / (2^n n! (2n + 1))`, each `Φ`
  evaluation bounded by the alternating series' next term and `π` bounded by
  Machin's formula with the same tail rule, until the bracket is narrower than
  `2^-64`; the upper end is used.

The decision is a function of the counts alone and is identical on every
platform.

### Undefined evaluation

A sampled undefined evaluation is a rejection, by the FR-363 rule for every
model engine. When the claim's event, measure or activation evaluates
undefined at a position of a drawn sample, counted or not, the provider SHALL
stop the run and settle `measured: rejected` with cause
`undefined-evaluation`, whose `where` names the test index, the trace index,
the position, the state-key digest there and the locus of the expression that
has no value, and whose `cause` names the undefined value's catalogued
reason. The sample is recorded as the run's first sampled witness
([FR-410](../../objects/interfaces/FR-410-statistical-verdict.md)), and its
replay reproduces the undefined value at that position. One undefined sample
decides the rejection, whatever the counts, so it carries no error bound.

### Stops, budgets and identity

A sample whose expansion stops (FR-181) stops the run with that cause. A
contract evaluated undecided during a step settles undecided with cause
`undecided-successor`. The budgets are `StatisticalLimits{max_samples,
max_draws, max_cycle_steps}` plus time and the clause meter; each is set by
the request, with the published default FR-461 gives it. Reaching any of them
stops the run with `incomplete`, `limit-reached{limit, value, setting}`,
naming the budget, its value and the request setting that raises it. A caller
cancellation stops the run with `incomplete`, `cancelled{source}`, whose
`source` is the cancellation identity the caller supplied, retained exactly. Neither records a `measured` value. The method, `min_cycles`, the seed and the
budgets are recorded in the result and are outside the obligation identity.

### Reproducibility

Re-running with the same subject, claim, method, `min_cycles`, seed and
sampler reproduces every sample, count and decision. A rerun with a new seed
is a new, independent measurement of the same obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-408-AC-1 | A claim with no confidence parameters, an every-scheduler clause, or an exact-only form settles `unsupported` at negotiation with `missing-confidence`, `every-scheduler` or `exact-only-form` respectively, and no engine runs; a step that breaks the Markov condition stops the run at that step while sampling and settles `unsupported`, `not-markov`. | Test (TC-353) |
| FR-408-AC-2 | Okamoto's `N` for `α = β = 1/100`, `m = 1` is 9,210,341 at `ι = 1/2000` and 23,026 at `ι = 1/100`, and the decision follows the interval's position against `θ`. | Test (TC-355) |
| FR-408-AC-3 | SPRT accepts or rejects at the thresholds `ln(β')` and `ln(1/α')` from the per-sample increments, and on the `P95` vector (TC-355) at `<= 2 ms` it settles rejected. | Test (TC-355) |
| FR-408-AC-4 | A not-activated sample advances the trace index and counts toward `max_draws` only. | Test (TC-355) |
| FR-408-AC-5 | The regenerative method on the `LongRun` vector (TC-356) settles accepted with coverage `asymptotic`; a run whose cycle does not return to `s0` within `max_cycle_steps` settles `incomplete`, `limit-reached{limit, value, setting}` naming `max_cycle_steps`, its value and the request setting that raises it, with no `measured` value. | Test (TC-356) |
| FR-408-AC-6 | With two initial states each test runs at `α/2` and `β/2`; a symmetric pair whose workload and distributions are invariant runs one test. | Test (TC-357) |
| FR-408-AC-7 | Every logarithm, square root and normal quantile is a rational bound within `2^-64`; `N`, the SPRT sums and thresholds and the regenerative half-width round outward toward undecided, so a sum or half-width within the rounding error of a threshold gives undecided; two runs on different platforms with the same inputs give identical counts and decisions. | Test (TC-355) |
| FR-408-AC-8 | Reaching `max_samples`, `max_draws`, `max_cycle_steps`, time or the clause meter settles `incomplete`, `limit-reached` naming the budget, its value and the request setting that raises it; a caller cancellation settles `incomplete`, `cancelled` naming the cancellation source, never `limit-reached`; neither records a `measured` value. | Test (TC-356) |
| FR-408-AC-9 | Re-running with the same seed reproduces every sample and the decision; the method, seed and budgets are outside the obligation identity. | Test (TC-355) |
| FR-408-AC-10 | A request naming the sequential method for a mean of a fraction settles `invalid-request` at negotiation with `invalid_capability`/`not-statistical`, and no sample is drawn. | Test (TC-355) |
| FR-408-AC-11 | A run in which the claim evaluates undefined on a drawn sample stops at that sample and settles `measured: rejected`, cause `undefined-evaluation` naming the test index, trace index, position, state-key digest, expression locus and catalogued reason, with that sample as its first sampled witness, which replays to the same undefined value. | Test (TC-359) |
| FR-408-AC-12 | The regenerative half-width is the Chow–Robbins half-width `z · √(S² + 1/n) / (L̄ · √n)`, rounded outward, and is positive when the sample variance is 0: on a fixture whose every cycle is identical (length 1, `P` holding), the half-width is `z / n`, the run does not stop at `min_cycles` 1000, and with `z = z_{0.99} ≈ 2.3263478740` and `ι = 1/5000` it stops at exactly `n = 11632`, the least `n` with `z / n <= ι`. | Test (TC-356) |

## Dependencies

- [FR-406](FR-406-interpret-probabilistic-models.md) and
  [FR-407](FR-407-state-probabilistic-claims.md) supply the measure and forms.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) and the
  [sampler definition](../../../proposals/quire-v1/definitions/simulation-sampler.md)
  supply weighted, indexed draws.
- [FR-409](../../objects/protocol/FR-409-measured-decision.md) and
  [FR-410](../../objects/interfaces/FR-410-statistical-verdict.md) carry the
  result.

## References

- QSL ADR-024 ST-1 to ST-10, RX-1, RP-1 to RP-3; QSL ADR-026 SS-5.
- A. Wald, *Sequential Analysis*, 1947; M. Okamoto, 1959; H. L. S. Younes and
  R. G. Simmons, CAV 2002; P. W. Glynn and D. L. Iglehart, 1988.
- Ticket: STD-137 (paired with QSL-371).
