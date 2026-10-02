---
id: FR-406
title: "Interpret probabilistic models under a workload or every scheduler"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-407
    type: references
---

# FR-406: Interpret probabilistic models under a workload or every scheduler

## Description

When a probabilistic claim over a model subject is evaluated, the
provider SHALL read the subject as a Markov decision process on its state graph, with
every choice resolved by the scheduler clause of the claim.

## Inputs

A model subject with its initial states, `over` bindings, random parameters,
rewards and workloads ([FR-405](FR-405-declare-probabilistic-models.md)); the
claim's scheduler clause ([FR-407](FR-407-state-probabilistic-claims.md)).

## Outputs

The probability measure on behaviours for each initial state, binding and
resolved scheduler, or the typed `unsupported` cause `not-markov` naming the
state, the transition and its post-states.

## Behavior

### Actions and draws

At a model state `s`, the **actions** are the enabled scheduled identities:
those whose effective precondition holds at `s` and that have at least one
post-state for some drawn random argument vector. An action `a` has the
**draw distribution** `D(s, a)`: each vector `r` in the product of its random
parameters' supports has the product of their FR-405 probabilities. Random
arguments are drawn independently, in parameter declaration order. The action
set is decided before any draw, because preconditions read no random
parameter.

### Markov decision process

A drawn vector `r` of action `a` at `s` leads to the post-states FR-181 gives
for `(a, r)`:

- exactly one post-state `s'`: the edge `s → s'` carries `D(s, a)(r)`, summed
  over the vectors that lead to `s'`;
- several post-states: the choice among them belongs to the scheduler, which
  makes it **after** the draw, knowing `r`. The process models it as an
  intermediate state `(s, a, r)` with one action per post-state, each taken
  with probability 1. An intermediate state is not a trace position;
- no post-state: the subject is not a Markov decision process at `s`, and the
  item settles `unsupported` with cause `not-markov{state, transition,
  post_states}`.

The support graph of this process is FR-181's state graph: every random
parameter ranges over its support and every enabled action has positive
probability under every workload.

### Under a workload

Under a workload `W`, at state `s` with enabled operations `O(s)`, each
operation `o` in `O(s)` is chosen with probability `w(o) / Σ w(O(s))`, then
each of its enabled scheduled identities `E_o(s)` with probability
`1 / |E_o(s)|`, then the random arguments are drawn. `W` is a memoryless
randomized scheduler and the subject under `W` is a discrete-time Markov
chain. Under a workload, a drawn vector with several post-states settles
`unsupported`, `not-markov`. The **step probability** of a transition is the
product of the scheduler probability and the probabilities of its random
arguments. Over a timed subject, a workload that gives every racing identity's
delay makes the timed subject a discrete-time Markov chain as well
([FR-420](FR-420-sample-stochastic-timed-models.md), [FR-421](FR-421-check-probabilistic-timed-claims-exactly.md)).

### Behaviours and their measure

A behaviour is a maximal path from an initial state, extended at a terminal
state by FR-161's terminal stutter step, which carries probability 1 and zero
reward. For an initial state `s0` and a resolved scheduler `σ`, the step
probabilities define the measure `Pr[s0, σ]` on behaviours: a finite prefix
has the product of its step probabilities, extended to measurable sets of
behaviours as for Markov chains. A probabilistic claim holds for its subject
when its bound holds for every initial state and every binding of its `over`
parameter. The subject carries no distribution over its initial states.

### Every scheduler

`under every scheduler` ranges over every scheduler of the process: any
function from the finite history of states and actions to a distribution over
the enabled actions, history-dependent and randomized, including the choice
after a draw. A lower bound `>= θ` holds when the **minimum** over schedulers
of the claim's quantity is at least `θ`; an upper bound `<= θ` holds when the
**maximum** is at most `θ`. On the finite product of the process with the
claim's monitor and accumulators, the minimum and maximum of a reachability
probability, a finite-horizon expected reward, an expected reward to a target
and a long-run average reward are attained by a memoryless deterministic
scheduler.

### Fair schedulers

A claim `under every scheduler` may state a **fairness set** `F` of weak and
strong fairness constraints, `whole` or `each`, over the model's operations,
with FR-161's granularity. The set is part of the claim and of its obligation
identity. A constraint is enabled at a model state when one of its scheduled
identities is an action there, and taken by a step whose scheduled identity
belongs to it. Intermediate states, the terminal stutter step and, over a
timed subject, delay moves belong to no constraint and enable none.

A scheduler `σ` is **fair** for `F` from `s0` when the behaviours that satisfy
every constraint of `F` have probability 1 under `Pr[s0, σ]` (Baier and
Kwiatkowska). With a fairness set, the minimum and maximum above read as the
infimum and supremum over fair schedulers, for every initial state and
binding. A fair scheduler always exists: the uniform scheduler over the
enabled actions is fair with probability 1.

An end component `(C, A)` of the process, or of its product with the claim's
monitor, a strongly connected set of states
`C` with actions `A` that never leave it, is **fair** for `F` when, for each
strong constraint enabled at some state of `C`, `A` holds an action of it, and
for each weak constraint enabled at every state of `C`, `A` holds an action of
it. A scheduler that stays in a fair end component and takes every action of
`A` with positive probability is fair with probability 1, and under a fair
scheduler the states and actions a behaviour takes infinitely often form a
fair end component with probability 1. A claim under a workload states no
fairness set.

### Almost-sure fairness under a workload

Every enabled transition has positive probability under a workload, so a
behaviour reaches a bottom strongly connected component with probability 1
and takes every transition of it infinitely often. Every fairness constraint
of FR-161, weak or strong, therefore holds with probability 1 under every
workload, and a claim under a workload carries no fairness set.

### Every-behaviour claims over the same subject

A non-probabilistic temporal claim over a probabilistic model reads the model
as nondeterministic, over the same state graph, and ignores weights. A claim
FR-161 proves holds on every behaviour of positive probability under every
scheduler.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-406-AC-1 | Under a workload, each step probability equals the operation weight share times the uniform identity share times the drawn arguments' probabilities, in exact rationals. | Test (TC-351) |
| FR-406-AC-2 | A drawn vector with no post-state settles `unsupported`, `not-markov{state, transition, post_states}` under every scheduler clause; several post-states settle `not-markov` under a workload and are a scheduler choice made after the draw under every scheduler. | Test (TC-351, TC-352) |
| FR-406-AC-3 | A claim holds only when its bound holds for every initial state and every `over` binding, each under its own measure. | Test (TC-351) |
| FR-406-AC-4 | `under every scheduler` compares the minimum for a lower bound and the maximum for an upper bound, and its value on the `Deliver` vector of TC-352 is the stated minimum and maximum. | Test (TC-352) |
| FR-406-AC-5 | A claim `under every scheduler` with a fairness set compares the infimum or supremum over the schedulers fair for that set; on the `Coin` vector (TC-352, TC-361) the minimum termination probability is 0 with no fairness set and the fair infimum is 1 with `fair strong Proc::flip` or `fair weak Proc::flip`; the fairness set enters the obligation identity. | Test (TC-352) |
| FR-406-AC-6 | Every FR-161 fairness constraint holds with probability 1 under a workload, and a claim FR-161 proves holds on every behaviour of positive probability. | Test (TC-351) |

## Dependencies

- [FR-405](FR-405-declare-probabilistic-models.md) declares the random
  parameters, rewards and workloads interpreted here.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) supplies the
  successor relation and the state graph.
- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) supplies
  behaviours, the terminal stutter step and fairness constraints.

## References

- QSL ADR-024 PM-2 to PM-8; QSL ADR-028 PR-1 to PR-3, SCH-1 to SCH-5.
- QSL ADR-028 FS-1 to FS-3.
- Owner rulings recorded on QSL-371: fair schedulers follow Baier and
  Kwiatkowska; the scheduler picks after the draw.
- C. Baier and M. Kwiatkowska, "Model checking for a probabilistic branching
  time logic with fairness", Distributed Computing, 1998.
- Ticket: STD-137 (paired with QSL-371).
