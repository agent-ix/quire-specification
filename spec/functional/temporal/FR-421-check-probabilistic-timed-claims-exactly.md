---
id: FR-421
title: "Check probabilistic timed claims exactly through digital clocks"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-411
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: depends_on
---

# FR-421: Check probabilistic timed claims exactly through digital clocks

## Description

When a `probabilistic-satisfaction` item over a timed subject names `exact`
evidence, the exact provider SHALL read the subject through digital clocks as
a probabilistic timed automaton.

When the exact provider settles such an item through digital clocks, it SHALL
settle only a verdict that holds on the dense model.

When such an item's subject or claim lies outside the digital-clock reading,
the exact provider SHALL settle the item `unsupported` with the named cause of
the rule it breaks.

## Inputs

A timed subject ([FR-415](FR-415-declare-clocks-and-timed-models.md)) with
its operations' `delay` declarations, and a probabilistic claim with its scheduler
clause ([FR-420](../analysis/FR-420-sample-stochastic-timed-models.md)).

## Outputs

An [FR-411](../analysis/FR-411-settle-probabilistic-claims-exactly.md) exact
verdict over the digital process, or `unsupported` with `strict-clock-constraint`,
`delay-distribution`, `not-stochastic`, `zero-delay-cycle`, `timed-form-shape`
or `punctual-interval`.

## Behavior

### Delays

`under every scheduler`, with or without a fairness set, chooses every delay
and every scheduled identity, and random arguments are drawn by FR-406; a
model whose operations declare a `delay` declaration settles `unsupported`,
`delay-distribution{operation}`, on this form.

`under W` resolves the scheduled identity and, through FR-420's race, every
delay a `discrete` distribution gives: each race outcome at a digital timed
state (each racing identity's delay drawn from its distribution conditioned on
its window, the smallest winning, ties ordered by `W`'s weights) is one
probabilistic edge with an exact rational probability. When every racing
identity's delay is so given, the subject under `W` is a Markov chain over
digital timed states. The delay of an operation with no `delay` declaration stays
the scheduler's choice, and the provider decides the claim over every
resolution of those delays, the bound read on their minimum or maximum. A
`uniform`, `uniform[a, b]` or `exponential` distribution has no exact digital
reading and settles `unsupported`, `delay-distribution{operation}`; such claims
are checked with statistical evidence. A digital timed state with no racing
identity and a bounded delay settles `unsupported`, `not-stochastic`.

### Closed condition

Every atomic clock constraint in a guard, every time invariant and every timed
interval of the claim is non-strict; a strict one settles `unsupported`,
`strict-clock-constraint{locus}`.

### Digital clocks

Every constant of the model, the claim and the `discrete` delays is scaled to an
integer by their common denominator. A digital timed state is the discrete
state with an integer value per clock, capped at the clock's largest compared
constant plus one. A delay move adds one unit to every clock below its cap and
is admissible when the time invariants hold after it and the state is not
urgent; a discrete move is FR-181's timed step with integer clocks. For a
closed, diagonal-free probabilistic timed automaton the minimum and maximum of
a reachability probability and of an expected time to a target are equal on
the digital and the dense semantics, so the verdict holds on the dense model,
and every delay a witness takes is a valid dense delay.

### Admitted forms

Unbounded reachability and until over state predicates; deadline events
`eventually[0, D] holds(P)` and `holds(A) until[0, D] holds(B)` with `D`
closed, by one extra digital clock that no step resets, capped at `D + 1`; and
`expected elapsed until holds(B)`, each delay move carrying one unit of time
as its reward. Any other timed form settles `unsupported`,
`timed-form-shape{locus}` at the unsupported form's exact formula node and
member. The explicitly admitted deadline intervals are closed `[0,D]`,
including the punctual boundary `[0,0]`. A punctual interval in any other form
settles `unsupported`, `punctual-interval{locus}` at the exact interval operator
and interval member, consistent with FR-416's typed cause.

For an elapsed-time claim, `timed-form-shape{locus}` identifies the unsupported
elapsed operator or formula member under that claim. For a punctual-interval
claim, `punctual-interval{locus}` identifies the exact interval operator and
bound member under that claim. Before reporting either cause, the provider SHALL
resolve the probabilistic claim's exact `{declaration NodeRef, occurrence}`
through `checked_term_data.probabilistic_claims`; the locus is a member of that
resolved declaration's body and occurrence, not a lookup by display text. If two
claims have equal display names or source spans, the provider preserves their
distinct declaration identities and reports against the selected identity. A
missing, foreign, wrong-kind or mismatched claim identity is an FR-322
checked-package refusal before FR-421 evaluation, not an FR-421 unsupported
result.

### Time divergence

Every infinite digital path diverges exactly when no reachable cycle consists
of discrete moves alone, which the provider checks by a strongly connected
component decomposition of the discrete-move subgraph. A reachable zero-delay
cycle settles `unsupported`, `zero-delay-cycle{states}`. Time-locks are reported
by the time-lock-freedom item ([FR-417](FR-417-settle-time-lock-freedom.md)).

### Evidence

Under every scheduler, a witness scheduler chooses between a unit delay and
an identity at each digital state. Under a workload with free delays, the
witness scheduler chooses each free delay, in units, per drawn vector of the
given delays (FR-413). A timed path of a path set carries each step's delay and
replays by FR-181 timed replay; at its last timed state no admissible discrete
step precedes the event's time horizon, which replay checks from the guards
and time invariants. A certificate names digital product states, and the
checker re-enumerates the digital process. The digital process's size grows
with the scaled constants and is bounded only by the request's `max_states`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-421-AC-1 | On the `Retx` vector, `Deadline` at 0.99 over every scheduler settles `proved`, `exact-value{99/100}` at equality, with maximum `9999/10000`. | Test (TC-374) |
| FR-421-AC-2 | `Deadline` at 0.995 settles `refuted` by a one-path set of probability `1/100` with losses at 2 ms and 4 ms, and replay checks that the next `send` needs time past 4 ms. | Test (TC-374) |
| FR-421-AC-3 | `MeanTime` settles `proved`, `exact-value{20/9 ms}` for the maximum expected time, with minimum `10/9 ms`. | Test (TC-374) |
| FR-421-AC-4 | A strict guard settles `unsupported`, `strict-clock-constraint`; a `uniform` or `exponential` delay under a workload, or any `delay` declaration under every scheduler, settles `delay-distribution`; a zero-delay cycle settles `zero-delay-cycle`; an unadmitted timed form settles `timed-form-shape`. | Test (TC-374) |
| FR-421-AC-5 | Under a workload whose every racing delay is `discrete`, the timed subject is a Markov chain and the exact value of the all-`discrete` `RareLate` variant is `1/20`. | Test (TC-374) |
| FR-421-AC-6 | Under a workload, an operation with no `delay` declaration has its delay chosen by the scheduler, and the bound compares the minimum or maximum over those choices: on the all-`discrete` `RareLate` variant with `reply`'s delay free, the maximum is `1/2` and the minimum 0, and `probability <= 0.01` settles `refuted` with a witness scheduler that takes `reply`'s delay 3 ms. | Test (TC-374) |
| FR-421-AC-7 | For two probabilistic claims with equal display names and source spans but distinct declaration identities, an unsupported elapsed form settles `unsupported`, `timed-form-shape{locus}` at the unsupported formula/member of the selected declaration, and a non-admitted punctual interval settles `unsupported`, `punctual-interval{locus}` at that declaration's exact interval operator/member. The selected declaration and its `claim` occurrence are resolved through FR-322-AC-297; the result retains the selected NodeRef and occurrence. | Test (TC-374) |

## Dependencies

- [FR-411](../analysis/FR-411-settle-probabilistic-claims-exactly.md) supplies
  the exact methods, verdicts and evidence.
- [FR-415](FR-415-declare-clocks-and-timed-models.md) and
  [FR-420](../analysis/FR-420-sample-stochastic-timed-models.md) supply the
  timed model and delays.
- [FR-416](FR-416-evaluate-timed-temporal-claims.md) supplies the timed
  profile.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  supplies the checked probabilistic claim declaration identity and occurrence
  through `checked_term_data.probabilistic_claims` (FR-322-AC-297).

## References

- QSL ADR-028 TA-1, TA-1a, TA-2 to TA-7, RU-6; QSL ADR-026 SS-7.
- Owner ruling recorded on QSL-371: a workload that gives delays makes the
  timed subject a Markov chain, exact on the exact engine, and leftover delay
  nondeterminism is a minimum or maximum.
- M. Kwiatkowska, G. Norman, D. Parker and J. Sproston, "Performance analysis
  of probabilistic timed automata using digital clocks", 2006.
- Ticket: STD-139 (paired with QSL-373); STD-137 (paired with QSL-371).
