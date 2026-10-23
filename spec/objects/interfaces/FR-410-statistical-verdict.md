---
id: FR-410
title: "Statistical verdict and sampled witness"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-408
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-409
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
---

# FR-410: Statistical verdict and sampled witness

## Description

When a statistical run completes its method, the provider SHALL record a
`StatisticalVerdict` with exactly the members below in the FR-331 result of a
`measured` item, and SHALL record each sampled witness so that it replays with
no provider present.

## Properties

The value has two parts: the `StatisticalVerdict` record and the sampled
witnesses it holds, each with its replay rule below.

### `StatisticalVerdict`

| Member | Content |
| --- | --- |
| `decision` | The [FR-409](../protocol/FR-409-measured-decision.md) value: `accepted`, `rejected` or `undecided`, with the undecided cause `indifference-region` or `undecided-successor`, or the rejection cause `undefined-evaluation` with its `where` and `cause`. |
| `estimate` | An exact rational: `p̂` for a probability bound; for a quantile claim, the conditional probability its decision reads, `Pr(M <= c | activated)` for `quantile q of M <= c` and `Pr(M < c | activated)` for `quantile q of M >= c` (FR-407), and the empirical nearest-rank quantile of the counted samples (the `⌈q · n⌉`-th smallest of `n`); the sample mean for a mean of a fraction; the regenerative ratio for a long-run fraction. A quantity estimate carries the threshold's unit. |
| `interval` | The confidence interval `[lower, upper]` in exact rationals for the fixed-sample, exact binomial and regenerative methods (for the exact binomial method, the outward Clopper–Pearson interval of FR-408); absent for the sequential method. |
| `basis` | `StatisticalBasis{method, confidence, samples, draws, tests, coverage, grid}`: `method` is `okamoto`, `sprt`, `clopper-pearson` or `regenerative` with its `min_cycles`; `confidence` is the claim's `α`, `β` and `ι`; `samples` is the count of counted samples; `draws` the count of drawn samples; `tests` is `m` with the per-test `α'` and `β'`; `coverage` is `finite-sample` or `asymptotic`; `grid` is the delay-draw grid exponent `q` of a timed run ([FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)) and absent otherwise. |
| `provenance` | The seed; the sampler identity `quire.simulation.sampler/v1`; and per test its index, initial state, `over` binding and trace-index range. |
| `tests` | One entry per test, in test-index order, with its own decision, estimate, interval and counts. |
| `witnesses` | Up to the request's `max_witnesses` sampled witnesses, the first in trace-index order. |

### Sampled witness

A sampled witness is a counted sample on which the event is false, or whose
measure falls on the violating side of the threshold, or a drawn sample on
which the claim evaluates undefined (FR-408). It carries:

- the test index and the trace index;
- the steps, each as the FR-181 trace step content (transition identity,
  post-state digest) plus its drawn scheduler choice, its drawn random
  argument values and, over a timed subject, its delay;
- the path probability: the exact rational product of its step probabilities;
- the event's value, or the measure's value, on the sample, or the undefined
  value with its position, expression locus and catalogued reason.

A sampled witness is a behaviour of positive probability on which the event is
false, so it is also a counterexample for the every-behaviour claim "the event
holds on every behaviour" over the same subject (FR-161) and replays as one.
A sampled witness leaves the probability bound's `measured` decision as the
run's counts set it.

### Witness replay

A sampled witness replays by FR-181 replay (re-execution of each step through
the native transition relation, successor selection by post-state digest, and
evaluation of the event, which for an undefined witness reproduces the
undefined value with its position, locus and reason), plus two checks:

1. each drawn random argument lies in its parameter's support;
2. each draw, recomputed from the seed, trace index, step and choice index
   with the sampler, selects the recorded scheduled identity, values and,
   over a timed subject, the recorded delay.

Agreement settles the witness reproduced. A value outside its support refuses
`invalid_runtime_input`/`invalid-value`; a recomputed draw that selects another
identity, value or delay settles `inconclusive`, `replay-parity`, for that
witness. The item's `measured` decision is unchanged by any witness replay.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — every member above; the per-test entries and witnesses in their
  stated orders.
- **Absence** — an absent `interval` is admitted only for the sequential
  method; every other absent member refuses the result with
  `invalid_result`/`missing-member`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-410-AC-1 | A completed statistical run records every `StatisticalVerdict` member with exact rational values, one test entry per test, and `interval` absent exactly for the sequential method. | Test (TC-358) |
| FR-410-AC-2 | A quantile run records both the conditional probability estimate and the nearest-rank empirical quantile. | Test (TC-355) |
| FR-410-AC-3 | The result keeps at most `max_witnesses` sampled witnesses, the first in trace-index order, each with its exact path probability. | Test (TC-359) |
| FR-410-AC-4 | A sampled witness replays as an FR-161 counterexample for the every-behaviour claim over the same subject. | Test (TC-359) |
| FR-410-AC-5 | Replay of a witness with a value outside its support refuses `invalid_runtime_input`/`invalid-value`; a recomputed draw selecting another identity settles `inconclusive`, `replay-parity`; neither changes the item's decision. | Test (TC-359) |

## Dependencies

- [FR-408](../../functional/analysis/FR-408-settle-probabilistic-claims-statistically.md)
  produces the verdict.
- [FR-409](../protocol/FR-409-measured-decision.md) supplies the decision.
- [FR-331](FR-331-backend-provider-envelope.md) carries it in the result.
- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md)
  supplies the trace step content and replay.

## References

- QSL ADR-024 SV-2, SV-6 to SV-8, RP-1; QSL ADR-026 SS-3, SS-6.
- Ticket: STD-137 (paired with QSL-371).
