---
id: FR-407
title: "State probabilistic claims over events and measures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-092
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-408
    type: references
  - target: ix://agent-ix/quire-specification/FR-411
    type: references
---

# FR-407: State probabilistic claims over events and measures

## Description

When a package declares a probabilistic claim, the frontend SHALL admit it
only in one of the forms below, over a model subject, with a scheduler clause,
and SHALL record its workload or every-scheduler clause with its fairness set, and any confidence parameters
it states in its obligation identity.

## Inputs

A `probabilistic` clause of the shared grammar's probabilistic forms; its
profile (FR-090), model subject, scheduler clause, activation and optional
confidence parameters; the model's rewards ([FR-405](FR-405-declare-probabilistic-models.md)).

## Outputs

An admitted claim with its form, event or measure, threshold, scheduler
clause, confidence parameters and obligation identity, or a located typed
refusal.

## Behavior

### Subject, kind and scheduler clause

A probabilistic claim's subject is a model subject. Its FR-290 kind is
`probabilistic-satisfaction`. Its scheduler clause is exactly one of
`under W` for a declared workload `W` of the subject's model, or
`under every scheduler`, optionally with a fairness set of weak and strong
constraints ([FR-406](FR-406-interpret-probabilistic-models.md)). A fairness
set on a claim under a workload refuses with
`invalid_probabilistic_model`/`fairness-under-workload`.
A clause naming a workload of another model refuses with
`invalid_probabilistic_model`/`foreign-workload`.

### Events

An **event** is a Boolean function of a behaviour that a finite prefix
decides. It is one of:

- a formula under a bounded profile with activation `on origin`, whose truth
  depends on the positions up to its horizon `h` (FR-091, FR-092); interval
  operators nest in any order;
- under the timed profile, a bounded-window formula
  ([FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md)) with
  activation `on origin`, decided by the positions whose time stamp is at most
  its time horizon;
- a comparison `M <= c` or `M >= c` of a bounded measure `M` with a threshold
  `c` of `M`'s type.

### Bounded measures

A **measure** is a value computed from one behaviour over a finite window.

- **Accumulate.** `accumulate R from holds(A) until holds(B) within h` sums
  reward `R` over the steps from the first position `a <= h` where `A` holds
  to the first position `b`, `a <= b <= h`, where `B` holds, that is over the
  steps into positions `a + 1` to `b`. A behaviour with no such `a` is **not
  activated**. A behaviour with `a` and no such `b` is **censored**, and its
  value exceeds every finite threshold.
- **Steps.** `steps from holds(A) until holds(B) within h` is accumulate with
  reward 1 per step.
- **Elapsed.** Under the timed profile with a `model-time` binding,
  `elapsed from holds(A) until holds(B) within h` is the time-stamp
  difference between those two positions, and `h` is a time quantity.
- **Fraction.** `fraction holds(P) over [0, h]` is the number of positions in
  `[0, h]` where `P` holds over `h + 1`. With `weighted by R`, each position
  `i` counts with the reward of the step that leaves it, and the value is the
  weighted count over the total weight. Over a timed subject `weighted by
  delay` weights each position by the delay of the step that leaves it.

A window that reaches a terminal state continues on its stutter steps.

### Forms

| Form | Holds when, for every initial state and binding |
| --- | --- |
| `probability >= θ [E]`, `probability <= θ [E]` | The probability of `E` meets the bound. |
| `quantile q of M <= c`, `quantile q of M >= c`, with `M` an accumulate, steps or elapsed measure | `Pr(M <= c | activated) >= q`, respectively `Pr(M < c | activated) < q`, with a strict `<`. The `q`-quantile of `M` is `inf {x : Pr(M <= x) >= q}`. It is at most `c` exactly when `Pr(M <= c) >= q`, since a distribution function is right-continuous. It is at least `c` exactly when `Pr(M <= x) < q` for every `x < c`; for a measure with finitely many values that is `Pr(M < c) < q`, and equality `Pr(M < c) = q` makes the claim false. A censored behaviour exceeds every finite threshold. |
| `mean of F >= θ`, `mean of F <= θ`, with `F` a fraction | The expectation of `F` meets the bound. A mean is admitted over a fraction measure only; a latency is stated as a quantile. |
| `long_run_fraction holds(P) >= θ` or `<= θ`, optionally `weighted by R` or `weighted by delay` | The expectation of the limit, as `n → ∞`, of the fraction over `[0, n]` meets the bound. In a finite Markov chain the limit exists with probability 1 and is constant on each bottom strongly connected component. |
| `probability >= θ` or `<= θ` over `[eventually holds(P)]`, `[holds(A) until holds(B)]` or `[always holds(P)]`, `P`, `A`, `B` state predicates, under the infinite-trace profile | The reachability or until probability meets the bound; `always holds(P)` reads `1 − Pr(eventually holds(not P))`. |
| `expected accumulate R until holds(B) <= c` or `>= c`; over a timed subject also `expected elapsed until holds(B) <= c` or `>= c` | The expectation from position 0 of `R` summed over the steps up to the first position where `B` holds meets the bound; where `B` is reached with probability below 1 the expectation exceeds every finite `c`. |

The last two rows are the **exact-only forms**.

### Thresholds and units

A probability threshold, a quantile level and a fraction threshold `θ` are
exact rationals with `0 < θ < 1`; the frontend refuses `0` or `1` with
`invalid_probabilistic_model`/`qualitative-threshold`, naming the corresponding
every-behaviour or possible-property form. A reward or time threshold has the
reward's or time's dimension and converts exactly by FR-142. A bound with
equality meets a non-strict bound; the reduced bound of `quantile q of M >= c`
is strict, so equality fails it. Every estimate and value in a result is an
exact rational, in the threshold's unit where it has one.

### Confidence parameters

A claim may state three exact rationals: `alpha` (`α`), the greatest
probability of rejecting the claim when its bound holds; `beta` (`β`), the
greatest probability of accepting it when its bound fails; and `indifference`
(`ι`), with `0 < α < 1`, `0 < β < 1`, `0 < ι`, `θ − ι > 0` and `θ + ι < 1`.
A claim states each of the three exactly once, or none; a set that omits or
repeats one refuses with `invalid_probabilistic_model`/`partial-confidence`. Statistical evidence requires them
([FR-408](FR-408-settle-probabilistic-claims-statistically.md)); exact evidence
reads none of them ([FR-411](FR-411-settle-probabilistic-claims-exactly.md)). A
claim that states none is checked with exact evidence only.

### Obligation identity

The obligation identity of a probabilistic claim includes its subject, form,
event or measure, threshold, scheduler clause (the workload's identity, or every scheduler with
its fairness set) and, when stated, its confidence parameters. One property
stated under two workloads, two fairness sets or two confidences is two
obligations. The evidence kind, the statistical method and every budget are
request settings and are outside the obligation identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-407-AC-1 | Each form in the forms table is admitted over a model subject with one scheduler clause and requires the FR-290 kind `probabilistic-satisfaction`; a workload of another model refuses `invalid_probabilistic_model`/`foreign-workload`. | Test (TC-353) |
| FR-407-AC-2 | On a vector with activation at a known position, a not-activated behaviour is excluded from a quantile's conditional probability and a censored behaviour exceeds every finite threshold. | Test (TC-353) |
| FR-407-AC-3 | `quantile q of M <= c` and `probability >= q [M <= c]` conditioned on activation give the same verdict on every vector; the `P95` distribution of TC-351 gives `Pr(M <= 5 ms) = 24233/25000`. | Test (TC-353) |
| FR-407-AC-4 | A threshold of 0 or 1 refuses `invalid_probabilistic_model`/`qualitative-threshold`; a reward threshold in another unit of the same dimension converts exactly. | Test (TC-353) |
| FR-407-AC-5 | A claim with all three confidence parameters is admitted with them in its obligation identity; a partial set refuses `invalid_probabilistic_model`/`partial-confidence`; two confidences of one property are two obligation identities. | Test (TC-353) |
| FR-407-AC-6 | Changing the workload or the fairness set changes the obligation identity, and a fairness set under a workload refuses `invalid_probabilistic_model`/`fairness-under-workload`; changing the evidence kind, method or a budget does not. | Test (TC-353) |
| FR-407-AC-7 | A weighted long-run fraction and a `weighted by delay` fraction are admitted and evaluate as the weighted ratio. | Test (TC-353, TC-374) |
| FR-407-AC-8 | `quantile q of M >= c` reduces to the strict bound `Pr(M < c | activated) < q`: with `M` equal to 1 or 3 with probability `1/2` each, `quantile 0.5 of M >= 3` is false (the quantile is 1, and `Pr(M < 3) = 1/2` is not below `1/2`), and `quantile 0.5 of M >= 1` is true. | Test (TC-353, TC-360) |

## Dependencies

- [FR-406](FR-406-interpret-probabilistic-models.md) supplies the measure each
  form is read under.
- [FR-090](../temporal/FR-090-select-temporal-profile-and-clock.md),
  [FR-091](../temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) supply bounded events.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  units and conversion.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) carries the
  kind `probabilistic-satisfaction`.

## References

- QSL ADR-024 PF-1 to PF-10, PM-8; QSL ADR-028 SP-4, XF-1 to XF-8; QSL ADR-026
  SS-1.
- Owner rulings recorded on QSL-371: confidence parameters appear only for
  statistical evidence, and a claim without them is exact-only; weighted
  long-run claims are kept; mean latency is dropped in favour of quantiles;
  expected reward and time are exact-only.
- Ticket: STD-137 (paired with QSL-371).
