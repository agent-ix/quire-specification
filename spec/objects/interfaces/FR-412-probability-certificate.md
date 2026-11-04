---
id: FR-412
title: "Probability certificate and its checker"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-411
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---

# FR-412: Probability certificate and its checker

## Description

When an exact probabilistic provider proves a bound, it SHALL emit a
`ProbabilityCertificate` with exactly the members below, and the checker SHALL
accept it, by re-enumerating the product and checking the stated conditions in
exact rationals with no provider present, before the item settles `proved`.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| identity | JsonObject | 1..1 |  |
| objective | JsonObject | 1..1 |  |
| kind | String | 1..1 |  |
| values | JsonObject | 0..* |  |
| ranking | JsonObject | 0..* |  |
| policy | JsonObject | 0..* |  |
| components | JsonObject | 0..* |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| identity | JSON object `{node_id, model, ordinal}`; `proposals/backend-provider-v1/schema.json` `CertificateIdentity` |
| objective | JSON object; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.objective` |
| kind | string tag lower, upper, exact or long-run |
| values | array of `{state, value}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.values` |
| ranking | array of `{state, rank}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.ranking` |
| policy | array of `{state, choice}` objects; `proposals/backend-provider-v1/schema.json` `ProbabilityCertificate.policy` |
| components | for `long-run`: per component, its value and a gain-bias pair |


`ProbabilityCertificate{identity, objective, kind, values, ranking, policy,
components}`:

| Member | Content |
| --- | --- |
| `identity` | The item's obligation identity, the structural `{node_id, model, ordinal}` of FR-418-AC-9, that binds the proof to the exact subject and claim it proves. |
| `objective` | The target or decided states, the reward, the optimisation, and the claim's fairness set when it states one. The optimisation is `minimum` or `maximum` over every scheduler; `workload` under a workload that leaves no choice; or `workload-minimum` or `workload-maximum` under a workload over a timed subject with free delays, the minimum or maximum over those delays' choices. |
| `kind` | `lower`, `upper`, `exact` or `long-run`. |
| `values` | One exact rational per product state the certificate names (a dyadic value is an exact rational), keyed by product state key (FR-411). |
| `ranking` | For `lower` and `exact`: one natural number per named state. |
| `policy` | For a lower bound on a maximum or an upper bound on a minimum, including `workload-maximum` and `workload-minimum`: one choice per named state, a memoryless deterministic scheduler. |
| `components` | For `long-run`: per component, its value `ρ_C` and a gain–bias pair. |

A certificate names its states and carries no qualitative sets; the checker
recomputes them.

Every state key in `values`, `ranking`, `policy`, each component's `bias`
and a state-located rejection SHALL preserve FR-411's complete product key:
the model state key, monitor state and accumulator, including the
activation flag. For the intermediate states of QSL
FR-196 / ADR-028 SCH-2, it SHALL also preserve the pending scheduled
identity and drawn vector. For an exact-only form, the absent monitor and
accumulator remain absent. A model state key alone SHALL NOT stand for
a bounded or intermediate product state.

QSL FR-196 / ADR-028 PR-4 now own the complete canonical octet basis:
`["quire.probability-product-key/v1", model, monitor, accumulator, pending]`
in RFC 8785 UTF-8, with every component present and actual None as null.
FR-331 transports those complete octets as `ProbabilityProductKey` in
each existing `state` row member and at `{product_key}` in a rejection.
The model is the full canonical simulation-state body;
formula monitors embed the complete parsed FR-338 array. The checker
SHALL admit every component under the actual subject/claim and compare
complete decoded canonical octets. Distinct accumulator values,
digital tail modes, pending action/draw vectors and genuine empty drawn
vectors SHALL remain distinct; None is not a fabricated zero component.

## Checker

`check_probability_certificate` recompiles the package, re-admits the subject
and re-enumerates the product through the native transition relation and the
monitor: every reachable product state for an `upper`, `exact` or `long-run`
certificate; the named states and their successors for a `lower` one. It
recomputes the decided states, the `Prob0` and `Prob1` sets, the components
and, with a fairness set, the maximal fair end components (FR-411) by its own
graph code, and checks the conditions
below in exact rationals. It solves no equation and runs no iteration.

With `F` the one-step operator of the objective (the workload's average; the
minimum or maximum over actions; for `workload-minimum` or `workload-maximum`,
the workload's average over drawn outcomes with the minimum or maximum over
the free delays' choices; or the policy's action), and values fixed at
decided, target and `Prob0` states:

- **Upper**: `F(y) <= y` at every state, so the value, the least fixed point,
  is at most `y`.
- **Lower**: `x <= F(x)` at every named state, and every non-target state with
  `x > 0` has (under the workload, under the policy, or for a minimum under
  every action) a successor with positive probability, `x > 0` and a smaller
  rank; then `x` is at most the value. An unnamed successor counts as value 0.
- **Exact**: both conditions, with equality.
- **Finite horizon**: the ranking is the remaining horizon, which the checker
  reads from the monitor state.
- **Expected reward**: the same conditions with the reward added to `F`, and
  the states whose value exceeds every finite threshold recomputed by graph.
- **Long-run**: per component, the gain–bias pair satisfies the multichain
  optimality equations for the reward `R · ([P] − ρ_C)` with gain 0, as
  equalities under the component's policy and as inequalities over every
  action, plus an `exact`, `lower` or `upper` certificate for the expected
  component value.

The checker accepts a certificate that meets its conditions and puts the
initial state's value on the claim's side of the threshold: `lower` for a
proof of `>= θ` or a refutation of `<= θ`; `upper` for a proof of `<= θ` or a
refutation of `>= θ`; `exact` for either. It refuses a certificate whose
`identity` differs from the item's with `invalid_runtime_input`/`conflicting-identity`.
A rejected certificate settles the item `inconclusive`,
`certificate-rejected{rule, at}`, `at` the complete product state key where
it failed, preserving the same components as the certificate's keys
(FR-411, FR-331): rule `inequality-fails` where an
upper, lower, exact or long-run condition above fails; `rank-missing` at a
named non-target state with `x > 0` and no rank; `rank-broken` at such a state
with no successor of positive probability, `x > 0` and a smaller rank; and
`threshold-missed` at the initial state when its value is not on the claim's
side of the threshold;
a checker stopped by a budget settles `incomplete`, `limit-reached` naming
the budget. The checker runs under the request's budgets; its cost is the
re-enumeration plus one pass over the edges.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — every member above.
- **Absence** — a `lower` or `exact` certificate without `ranking`, a lower
  bound on a maximum without `policy`, or a `long-run` certificate without
  `components` is rejected, `certificate-rejected{rule, at}` with rule
  `malformed` at `{member}`, the missing member's name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-412-AC-1 | The checker accepts the `Deliver` `exact` certificate (TC-363) (values `24/25`, `4/5`, 1 at delivered states, 0 at the two-loss state) after checking the minimum over both actions at each live state. | Test (TC-363) |
| FR-412-AC-2 | The checker accepts the `LongRun` `long-run` certificate (TC-362) (gain `1800/1801`, bias 0 at `up` and `−2000/1801` at `down`) by exact equality. | Test (TC-362) |
| FR-412-AC-3 | A certificate that violates one inequality settles `inconclusive`, `certificate-rejected` with rule `inequality-fails` at that state; a `lower` certificate with a missing rank, `rank-missing` at the unranked state; one with a rank that does not decrease, `rank-broken` at that state; one whose initial value is on the wrong side of the threshold, `threshold-missed` at the initial state; and a `lower` certificate without `ranking`, `malformed` at member `ranking`. | Test (TC-363) |
| FR-412-AC-4 | A certificate whose identity differs from the item's is refused `invalid_runtime_input`/`conflicting-identity`. | Test (TC-363) |
| FR-412-AC-5 | A `lower` certificate is checked over its named states and their successors only, with each unnamed successor at value 0. | Test (TC-363) |
| FR-412-AC-6 | The checker recomputes every qualitative set and component itself and reads no other output of the provider. | Test (TC-363) |
| FR-412-AC-7 | The checker accepts the `FairTerminates` `exact` certificate, whose objective names the fairness set, after recomputing that no fair end component exists. | Test (TC-363) |
| FR-412-AC-8 | Two named product states with the same model state but different monitor states or accumulators retain distinct keys in `values`, `ranking`, `policy`, component `bias` and state-located rejections after serialization and reading. Intermediate states that differ only in their pending action or draw also remain distinct; replacing either complete key with the model state key alone fails the comparison. | Test (TC-363) |

## Dependencies

- [FR-411](../../functional/analysis/FR-411-settle-probabilistic-claims-exactly.md)
  produces the certificate and defines the product.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines the structural
  identity of the obligation.
- [FR-331](FR-331-backend-provider-envelope.md) carries the certificate in
  the result.

## References

- QSL ADR-028 CE-1 to CE-7, FS-9.
- K. Chatterjee, T. Quatmann et al., fixed-point certificates for MDP model
  checking, TACAS 2025; F. Funke, S. Jantsch and C. Baier, TACAS 2020.
- Ticket: STD-137 (paired with QSL-371).
