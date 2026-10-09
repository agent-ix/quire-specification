---
id: FR-411
title: "Settle probabilistic claims with exact evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-406
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-407
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-412
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-413
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-411: Settle probabilistic claims with exact evidence

## Description

When a `probabilistic-satisfaction` item names `exact` evidence, the exact
provider SHALL decide the bound of the claim from the exact value or a sound
interval of its quantity over the full finite subject, with a checked
certificate for each proof and a replayed counterexample for each refutation.

## Inputs

An admitted claim ([FR-407](FR-407-state-probabilistic-claims.md)) with any
scheduler clause; the request's `ExactProbLimits` (FR-331).

## Outputs

`proved` with an `exact-value` or `value-bounds` proof basis and a certificate
([FR-412](../../objects/interfaces/FR-412-probability-certificate.md));
`refuted` with a counterexample ([FR-413](../../objects/interfaces/FR-413-probabilistic-counterexample.md));
or an incomplete, inconclusive or unsupported result with its typed cause.

## Behavior

### The probabilistic product

For a form whose event is a bounded formula or a comparison of a bounded
measure, the provider forms the product of FR-406's process (or, under a
workload, its Markov chain) with the formula's deterministic finite monitor,
read at each position. A bounded measure adds to the product state its
accumulator (steps, reward or weighted sums since activation, saturating one
unit above the threshold) and the activation flag; rewards are non-negative
values from finite supports, so the accumulator takes finitely many values.
The product state key is (model state key, monitor state, accumulator); a
product edge carries the model edge's action, draw and probability. Every
product path reaches an accepting or rejecting monitor state, a **decided**
state, within `h + 1` positions; decided states are absorbing, and the product
restricted to undecided states is acyclic. An exact-only form needs no
monitor: its state predicates label model states and its reward labels edges.

### Finite-horizon forms

Probability bounds, quantiles, and means of fractions are finite-horizon
values on the product, computed by backward induction in reverse topological
order: a decided state has its terminal value; an undecided state's value is
the probability-weighted sum over its successors under a workload, or the
minimum or maximum over its actions of that sum over every scheduler. A
quantile over every scheduler decides `quantile q of M <= c` from the minimum
over schedulers of the expected terminal reward
`X = [activated] · ([M <= c] − q)`, which is at least 0 exactly when the claim
holds (a scheduler that never activates contributes 0). `quantile q of M >= c`
reduces to the strict bound `Pr(M < c | activated) < q` (FR-407), so the
provider computes the maximum over schedulers of
`Y = [activated] · ([M < c] − q)`: a maximum above 0 refutes and a maximum
below 0 proves. A maximum of exactly 0 is decided by a second pass: the
provider restricts each product state to the actions that attain the maximum
and computes the maximum activation probability over that restricted process;
the claim is refuted when it is positive, since some scheduler then activates
with `Pr(M < c | activated) = q`, and proved when it is 0. Under a workload the
conditional probability is the ratio of two finite-horizon probabilities, and
for `quantile q of M >= c` a ratio equal to `q` refutes. When the activation
probability is 0, both quantile forms settle `proved`: the claim holds
vacuously ([FR-407](FR-407-state-probabilistic-claims.md)) and no ratio is
formed. The
unweighted mean of a fraction is `1/(h + 1)` times a finite-horizon expected
count; a weighted fraction accumulates both sums and is an expected terminal
reward.

### Unbounded reachability and expected rewards

1. **Qualitative step.** The provider computes by graph algorithms the states
   of value 0 and 1: `Prob0` and `Prob1` under a workload; `Prob0A`, `Prob0E`,
   `Prob1A` and `Prob1E` over every scheduler.
2. **End components.** Over every scheduler, for a maximum, each maximal end
   component holding no target state collapses to one state keeping its
   leaving actions; the reachability operator then has a unique fixed point.
3. **Interval iteration.** The Bellman operator is iterated from 0 and from 1
   at once over dyadic intervals with outward rounding; every lower iterate
   and every upper iterate is a valid certificate. Iteration stops when the
   interval at every initial product state lies on one side of the threshold.
4. **Exact policy iteration.** When the interval still contains the threshold
   after `max_iterations` sweeps or at `max_precision_bits`, the provider runs
   policy iteration in exact rationals: it evaluates a memoryless
   deterministic scheduler by an exact sparse linear solve (fraction-free
   elimination), improves it action by action, and stops at a scheduler no
   action improves. A value equal to the threshold is decided. Under a
   workload this is one exact linear solve.

For an expected reward to a target, the qualitative step finds the states
that reach `B` with probability 1 under every scheduler (for a maximum) or
under some scheduler (for a minimum); elsewhere the value exceeds every
finite threshold. Interval iteration starts its upper vector from a sound
bound, zero-reward end components collapse as in step 2, and exact policy
iteration is the fallback.

### Fairness sets

With a fairness set `F` (FR-406):

- **Maximal fair end components.** The provider decomposes the product into
  maximal end components; in each, it drops the component when a weak
  constraint is enabled at every state and no action of it is in the
  component, and removes the states where a strong constraint is enabled when
  no action of it is in the component; it decomposes what remains again, and
  repeats until no component changes.
- **Bounded forms.** For probability bounds, quantiles and means of fractions
  the infimum and supremum over fair schedulers equal the minimum and maximum
  over every scheduler, since a scheduler's choices up to the horizon extend
  past it by the uniform scheduler, which is fair; the result records `F`.
- **Reachability.** The supremum of `Pr(eventually holds(B))` over fair
  schedulers equals the maximum over every scheduler. For the infimum, with
  `U` the union of the fair end components of the product restricted to states
  where `B` does not hold, the infimum is `1 − max Pr(not B until U)`, the
  maximum over every scheduler. `holds(A) until holds(B)` reads `U` over states
  where `A` holds and `B` does not, and counts reaching a state where neither
  holds as missing `B`; `always holds(P)` reads `1 − Pr(eventually holds(not P))`
  with infimum and supremum exchanged.
- **Expected rewards.** The minimum is unchanged by fairness. The maximum
  exceeds every finite threshold at a state where the fair infimum of reaching
  `B` is below 1, and at a state that reaches, among states where that
  infimum is 1, an end component holding an action of positive reward;
  elsewhere zero-reward end components collapse and the unbounded method
  above runs.
- **Long-run fractions.** A behaviour under a fair scheduler ends in a fair end
  component with probability 1. Each maximal fair end component gets the
  optimum over its actions; a fair scheduler approaches it by taking the
  actions the optimum leaves out with vanishing frequency, so the value is an
  infimum or supremum that need not be attained. The combination collapses
  every end component that holds no fair end component.

### Long-run fractions

Under a workload, the provider decomposes the chain into bottom strongly
connected components; in a component `C` the long-run fraction is
`π_C(R · [P]) / π_C(R)`, `π_C` its stationary distribution, found by an exact
linear solve (`R = 1` unweighted). Over every scheduler it decomposes the
process into maximal end components and finds each component's optimal ratio
by Dinkelbach's parametric method, each round an exact mean-payoff policy
iteration for the reward `R · ([P] − ρ)`, stopping at the `ρ` with optimal
gain 0. The claim's value is the expected
component value reached, a reachability problem of the previous section. A
component in which every cycle has total weight 0 under `R` has no defined
fraction and settles `unsupported`, `zero-weight-component{component}`.

### Arithmetic

Every number on the verdict path is an exact rational held as a pair of
arbitrary-precision integers in lowest terms, or a dyadic interval
`[m_l · 2^-p, m_u · 2^-p]` held in arbitrary-precision integers whose every
operation rounds the lower end down and the upper end up. The provider first
runs backward induction in exact rationals; when a value's bit length passes
`max_rational_bits` it reruns over dyadic intervals at `precision_bits`,
doubling `p` up to `max_precision_bits` while the interval contains the
threshold. Results are identical on every platform and are a function of the
subject, the claim and the limits alone. A value is emitted only with a
certificate the provider has first checked with the FR-412 checker.

### Verdicts

| Outcome | FR-331 value | FR-243 basis | Proof basis or cause |
| --- | --- | --- | --- |
| Exact value on the claim's side of the threshold, equality meeting a non-strict bound and failing the strict reduced bound of `quantile q of M >= c` | `proved` | `closed-scope` | `exact-value{value, reductions}` with an `exact` certificate |
| Sound interval on the claim's side | `proved` | `closed-scope` | `value-bounds{lower, upper, method, reductions}`, `method` one of `backward-induction{precision_bits}` and `interval-iteration{precision_bits, sweeps}`, with a `lower` or `upper` certificate |
| Bound fails (every scheduler or a workload) | `refuted` | `decisive-counterexample` | an FR-413 counterexample that replays |
| The claim evaluates undefined at a product state reached with positive probability, under some scheduler or the workload | `refuted` | `decisive-counterexample` | cause `undefined-evaluation`, with `where` and `cause`, and an FR-413 undefined path that replays |
| Interval straddles the threshold at the budget | `incomplete`, execution `resource-incomplete` | `unavailable` | `precision-budget{lower, upper}` |
| A run limit reached first | `incomplete`, execution `resource-incomplete` | `unavailable` | `limit-reached` naming the `ExactProbLimits` member, its value and the request setting that raises it |
| Certificate rejected; witness replay disagrees | `inconclusive` | `unsettled` | `certificate-rejected{rule, at}` naming the failed check and its state; `replay-parity` |
| `not-markov`, `zero-weight-component`, and the timed causes of [FR-421](../temporal/FR-421-check-probabilistic-timed-claims-exactly.md) | `unsupported` | `unavailable` | that cause |

`reductions` lists the applied state-space reductions, empty when none
applies. The proof basis states the least favourable (initial state, binding)
pair; the result keeps one entry per pair with its value or interval and,
under every scheduler, whether it is a minimum or a maximum. Both members
count as proof evidence. An exact result carries FR-242 truth and an FR-243
basis and no FR-409 `measured` value.

### Undefined evaluation

An undefined evaluation refutes by the FR-363 rule for every model engine.
When the claim evaluates undefined at a product state that some scheduler, or
the workload, reaches with positive probability, the provider SHALL settle the
item `refuted` with cause `undefined-evaluation`, whose `where` names the
(initial state, binding) pair, the product state key and the locus of the
expression that has no value, and whose `cause` names the undefined value's
catalogued reason, whatever the bound and the claim's value elsewhere. Its
evidence is the FR-413 undefined path to the first such product state in
FR-181 canonical breadth-first order. An undefined state found before a limit
stops exploration settles the item.

### Evidence on a partial product

A lower certificate and a path set read only the states they name and those
states' successors, and an unexplored successor counts as value 0. Evidence of
that kind found before a limit stops exploration therefore settles the item:
a proof of a `>= θ` bound or a refutation of a `<= θ` bound. Every other
verdict needs the complete product.

### Symmetry

Under an admitted symmetry declaration whose group leaves every random
parameter's distribution and, under a workload, the workload invariant, and
the claim's event invariant under the binding's stabiliser, the provider
computes on the quotient, the checker re-enumerates the same quotient, and
the proof basis records the reduction.

### Limits

`ExactProbLimits` holds `max_states`, `max_transitions`,
`max_automaton_states`, `max_iterations`, `precision_bits`,
`max_precision_bits`, `max_rational_bits`, `max_policy_iterations` and
`max_witness_paths`, plus time and the clause meter. Each is set by the
request, with the published default FR-461 gives it, and each is outside the
obligation identity. A caller cancellation stops the run with `incomplete`,
`cancelled{source}`, whose `source` is the cancellation cause, `requested` when the caller cancels and `deadline` when the caller's deadline ends the run, and records no value. The evidence kind is a request member and is outside the claim's
obligation identity; the confidence parameters a claim states are inside it
and are not read.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-411-AC-1 | On the `NoFault` vector (TC-360) the exact value is `(9999999/10000000)^1000`, settled `proved`, `exact-value`, when `max_rational_bits` admits it, and `proved`, `value-bounds{…, backward-induction{64}}` at 64 bits otherwise. | Test (TC-360) |
| FR-411-AC-2 | On the `P95` vector (TC-360) the item settles `proved`, `exact-value{24233/25000}` at 5 ms and `refuted` at 2 ms with value `4491/5000`. | Test (TC-360) |
| FR-411-AC-3 | On the `Deliver` vector (TC-361) the minimum is `24/25`, the maximum `99/100` and the workload value `391/400`; the claim at 0.97 over every scheduler settles `refuted` and at 0.95 settles `proved`, `exact-value{24/25}`. | Test (TC-361) |
| FR-411-AC-4 | A value equal to its threshold settles `proved` for a non-strict bound through exact policy iteration or exact backward induction. | Test (TC-361, TC-374) |
| FR-411-AC-5 | Unbounded reachability and expected reward to a target settle by the qualitative step, end-component collapse and interval iteration; a target reached with probability below 1 makes the expected reward exceed every finite threshold. | Test (TC-361) |
| FR-411-AC-6 | On the `LongRun` vector (TC-362) the long-run value is `1800/1801`, settled `proved`; a zero-weight component settles `unsupported`, `zero-weight-component`. | Test (TC-362) |
| FR-411-AC-7 | An interval that straddles the threshold at `max_precision_bits` settles `incomplete`, `precision-budget{lower, upper}`; another limit settles `incomplete`, `limit-reached` naming the member, its value and how to raise it. | Test (TC-365) |
| FR-411-AC-8 | A lower certificate or path set found before a limit stops exploration settles a proof of `>= θ` or a refutation of `<= θ`. | Test (TC-365) |
| FR-411-AC-9 | An exact result carries truth and an FR-243 basis and no `measured` value, and the claim's confidence parameters are not read. | Test (TC-360) |
| FR-411-AC-10 | On the `Coin` vector (TC-352, TC-361), `Terminates` settles `refuted` with minimum 0 and `FairTerminates` (`fair strong Proc::flip`) settles `proved`, `exact-value{1}`, because no fair end component remains; `fair weak Proc::flip` gives the same value. | Test (TC-361) |
| FR-411-AC-11 | With a fairness set, a bounded form's value equals its unrestricted minimum or maximum and the result records the fairness set. | Test (TC-361) |
| FR-411-AC-12 | A claim that evaluates undefined at a product state reached with positive probability settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming the pair, the product state key, the expression locus and the catalogued reason, with an undefined path that replays, also when the bound holds on the claim's defined values. | Test (TC-364) |
| FR-411-AC-13 | `quantile 0.5 of M >= 3` with `M` equal to 1 or 3 with probability `1/2` each settles `refuted`, the exact value `Pr(M < 3) = 1/2` equal to `q` failing the strict bound, both under a workload and, through the zero-maximum second pass, over every scheduler; `quantile 0.5 of M >= 1` settles `proved`. | Test (TC-360) |
| FR-411-AC-14 | A quantile claim whose activation probability is 0 settles `proved` in both forms, under a workload and over every scheduler. | Test (TC-360) |
| FR-411-AC-15 | A caller cancellation of an exact run stops it with `incomplete`, `cancelled{source}` with `source` `requested`, not `limit-reached`, and records no value. | Test (TC-365) |

## Dependencies

- [FR-406](FR-406-interpret-probabilistic-models.md) and
  [FR-407](FR-407-state-probabilistic-claims.md) supply the semantics and forms.
- [FR-412](../../objects/interfaces/FR-412-probability-certificate.md) and
  [FR-413](../../objects/interfaces/FR-413-probabilistic-counterexample.md)
  carry the evidence.
- [FR-243](../../objects/protocol/FR-243-settlement-basis.md) and
  [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) carry
  the result.

## References

- QSL ADR-028 SP-1 to SP-6, PR-4 to PR-6, FS-4 to FS-8, XF-1 to XF-8, AR-1
  to AR-4, FH-1 to FH-4, UR-1 to UR-7, LR-1 to LR-5, XV-1 to XV-7, LM-1 to LM-3, RX-1.
- Owner ruling recorded on QSL-371: exact probabilistic checking is a native
  engine with certified results; fair schedulers follow Baier and Kwiatkowska.
- S. Haddad and B. Monmege, 2018; C. Baier et al., CAV 2017; T. Quatmann and
  J.-P. Katoen, CAV 2018; W. Dinkelbach, 1967; M. L. Puterman, 1994.
- Ticket: STD-137 (paired with QSL-371).
