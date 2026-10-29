---
id: FR-413
title: "Probabilistic counterexample and witness scheduler"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-411
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-412
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# FR-413: Probabilistic counterexample and witness scheduler

## Description

When an exact probabilistic provider refutes a bound, it SHALL emit a
`ProbabilisticCounterexample` with exactly the members below, and replay SHALL
reproduce it with no provider present before the item settles `refuted`.

## Properties

The value has two parts: the `WitnessScheduler` and the
`ProbabilisticCounterexample` that carries it with its evidence.

### `WitnessScheduler`

`WitnessScheduler{entries}`: for each product state the evidence names, its
product state key and its choice. A choice is a scheduled identity; after a
draw with several post-states, the post-state key chosen per drawn vector;
over a timed subject under every scheduler, a unit delay or an identity; over
a timed subject under a workload with free delays
([FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md),
[FR-421](../../functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md)),
per drawn vector of the given delays, the delay of each racing identity whose
delay is free. Under every scheduler with
no fairness set the witness is memoryless and deterministic. For a claim with
a fairness set it is memoryless and randomized: each entry may give a
distribution over choices with exact rational probabilities, and it must be
fair: every bottom strongly connected component of the chain it induces on
the evidence's states meets the fair-end-component condition of FR-406. For a
long-run refutation with a fairness set, where the infimum need not be
attained, the provider mixes the actions the optimum leaves out with
probability `2^-k`, raising `k` until the induced chain's exact value is past
the threshold, and the evidence is that chain's `long-run` certificate. It is
defined on the evidence's states only.

### `ProbabilisticCounterexample`

`{initial_state, binding, bound, scheduler, evidence}`:

- `initial_state` — the index of the initial state in the subject;
- `binding` — the `over` binding;
- `bound` — the claim's bound with its threshold;
- `scheduler` — the witness scheduler: present under every scheduler, and
  under a workload over a timed subject with free delays, where it carries the
  free delays' choices; absent under a workload that leaves no choice;
- `evidence` — exactly one of a path set, a subsystem certificate or an
  undefined path.

**Path set.** A finite list of finite product paths from the initial state,
each a list of steps in FR-181 trace step content plus each step's drawn
random vector (and, over a timed subject, its delay), such that:

- on each path the event is false (for a `>= θ` bound) or true (for a `<= θ`
  bound), or, for an exact-only form, the path's last state lies in a set the
  checker's qualitative computation places on the violating side (for
  example a state of value 0, or a state of a fair end component with no
  target state);
- the paths are pairwise prefix-free, so their cylinders are disjoint;
- their exact path probabilities sum past the bound: above `1 − θ` for
  `>= θ`, above `θ` for `<= θ`.

A path's probability is the product of its step probabilities under the
workload; under a workload with free delays, the product of the workload's
step probabilities with each free delay fixed by the witness scheduler; and
under every scheduler, the product of its draw probabilities and the witness
scheduler's choice probabilities. The provider emits a path set when at most
`max_witness_paths` paths reach the bound, most probable first.

**Quantile path sets.** A quantile claim is conditioned on activation
(FR-407), so its path set reads two masses rather than one sum. Each path is
either **violating**, activated with `M > c` for `quantile q of M <= c` or with
`M < c` for `quantile q of M >= c`, or **not activated**. `V` is the exact sum
of the violating paths' probabilities and `N` that of the not-activated
paths. The paths left out have total mass `1 − V − N`, which bounds the mass
of activated, non-violating behaviours from above, so the evidence refutes
for every activation probability, including one below 1:

- `quantile q of M <= c` (claim `Pr(M <= c | activated) >= q`): the evidence
  refutes when `V > (1 − q) · (1 − N)`, strictly;
- `quantile q of M >= c` (claim `Pr(M < c | activated) < q`): the evidence
  refutes when `V > 0` and `V >= q · (1 − N)`, equality included, which is the
  negation of the strict reduced bound.

A subsystem certificate for a quantile claim carries a `lower` certificate for
`V` and one for `N`, and the same rule applies to their values.

**Subsystem certificate.** Otherwise the evidence is an FR-412 `lower`
certificate, on the chain the witness scheduler induces or under the workload,
for the violating quantity (`Pr(not E)` for `>= θ`, `Pr(E)` for `<= θ`) with
value past the bound. Long-run and expected-reward refutations use the same
shape with their own certificate kinds.

**Undefined path.** One finite product path from the initial state, in the
step content of a path-set path, of positive probability, ending at the
product state at which the claim evaluates undefined
([FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)).

## Replay

`replay_probabilistic_witness` recompiles the package and re-admits the
subject's initial state and universes. For a path set it re-executes each path
through the native transition relation with FR-181 replay, selects each
step's successor by its post-state key, checks that each drawn value lies
in its support and, under every scheduler or for a free delay under a
workload, that each step takes a choice the witness scheduler gives positive
probability at its product state and, with a
fairness set, that the witness is fair; it recomputes each path probability
exactly, evaluates the event, checks prefix-freeness and checks the sum
against the bound; for a quantile claim it classifies each path as violating
or not activated, recomputes `V` and `N`, and checks the quantile rule above. For a subsystem certificate it applies the witness
scheduler to rebuild the induced chain over the certificate's states and runs
the FR-412 checker. For an undefined path it re-executes the path as a path
of a path set, checks that its exact probability is positive, and evaluates
the claim at its last state: the undefined value with the locus and
catalogued reason of the item's `undefined-evaluation` cause agrees, and a
defined value disagrees.

Agreement settles `refuted`, `decisive-counterexample`. Disagreement settles
`inconclusive`, `replay-parity`. Replay refuses as FR-181 replay refuses:
`stale_dependency`/`content-mismatch` for a key with no matching
successor; `invalid_runtime_input`/`invalid-value` for a step that is not
enabled, a value outside its support, a choice that is not the scheduler's,
or a path set that is not prefix-free.

A path of a path set on which the event is false is also a counterexample for
the every-behaviour claim "the event holds on every behaviour" (FR-161).

## Identity and comparison

- **Identity kind** — *value*, compared componentwise; paths in their stated
  order.
- **Absence** — `scheduler` is absent exactly under a workload that leaves no
  free delay; a missing `scheduler` under every scheduler or under a workload
  with free delays, and every other absent member, refuses with
  `invalid_runtime_input`/`missing-member`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-413-AC-1 | The `P95` refutation at 2 ms replays from a one-path set of probability `343/5000`, each drawn value checked against its parameter's value set. | Test (TC-364) |
| FR-413-AC-2 | The `Deliver` refutation at 0.97 replays from a one-path set of probability `1/25` whose steps take the witness scheduler's choice `send_b`. | Test (TC-364) |
| FR-413-AC-3 | The `Monthly` refutation replays from a subsystem certificate through the FR-412 checker. | Test (TC-364) |
| FR-413-AC-4 | A path set that is not prefix-free, a step that departs from its scheduler, and a drawn value outside its parameter's value set each refuse `invalid_runtime_input`/`invalid-value`; a recomputed sum below the bound settles `inconclusive`, `replay-parity`. | Test (TC-364) |
| FR-413-AC-5 | A refutation with a fairness set carries a randomized witness whose path probabilities include its choice probabilities; replay checks that every bottom component of the induced chain is fair, and an unfair witness settles `inconclusive`, `replay-parity`. | Test (TC-364) |
| FR-413-AC-6 | Engines may return different valid witnesses; a vector checks that the returned evidence replays, not that it equals a stored witness. | Test (TC-364) |
| FR-413-AC-7 | An undefined path re-executes with positive probability and replays to the undefined value with the item's locus and reason, settling `refuted`; the same path ending at a state where the claim is defined settles `inconclusive`, `replay-parity`. | Test (TC-364) |
| FR-413-AC-8 | A refutation under a workload over a timed subject with a free delay carries a witness scheduler whose entries give that delay per drawn vector of the given delays; replay checks each step's free delay against it, and the same counterexample without `scheduler` refuses `invalid_runtime_input`/`missing-member`. | Test (TC-374) |
| FR-413-AC-9 | A quantile refutation replays by the two-mass rule: for `quantile q of M >= c` it refutes when `V > 0` and `V >= q · (1 − N)`, equality included, so the `Two` refutation of `quantile 0.5 of M >= 3` (`V = 1/2`, `N = 0`) replays; for `quantile q of M <= c` it refutes only when `V > (1 − q) · (1 − N)`; a path set whose activation probability is below 1 refutes only when its not-activated mass `N` is counted. | Test (TC-364, TC-360) |

## Dependencies

- [FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)
  produces the counterexample.
- [FR-412](FR-412-probability-certificate.md) checks subsystem evidence.
- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md)
  supplies step content and replay.

## References

- QSL ADR-028 WS-1 to WS-6, FS-9.
- Ticket: STD-137 (paired with QSL-371).
