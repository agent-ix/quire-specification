---
id: FR-399
title: "Settle hyperproperty and step-relation results"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-395
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-396
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-401
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
---

# FR-399: Settle hyperproperty and step-relation results

## Description

When a hyper or step-relation item over model subjects is checked, the
analyzer SHALL settle it with one FR-360 label and one FR-243 basis by the
table below, SHALL settle a refutation only after its counterexample replays,
and SHALL stop at a run limit with the limit named rather than continue past
it.

## Inputs

The item's form ([FR-396](FR-396-classify-hyperproperty-forms.md)), its
subjects, the run's outcome and evidence, and the request's model-check
limits.

## Outputs

One result per item with its label, basis, cause where inconclusive or
stopped, settlement method, and counterexample where refuted.

## The witness set of a `∀∃` clause

For an HP-3 clause and a tuple of universal paths, the witness set `X_j` at
joint position `j` holds every pair (existential tuple of model states, state
of the body's safety automaton) reachable along a matched existential prefix.
The safety automaton is the Büchi automaton of the body with every state of
empty language removed and every remaining state accepting. `X_0` holds every
(initial existential tuple, automaton state reached by reading joint position
0); `X_{j+1}` holds every (existential post-tuple, automaton successor)
reached from a member of `X_j` by an existential joint step on which `μ_E`
holds against universal step `j + 1`, the automaton reading the joint
post-position. An empty `X` stays empty.

The clause is violated by a fair universal tuple along which some `X_j` is
empty: every existential tuple matched with it has left the body's safety
closure by position `j`. If `X` stays non-empty along a fair universal
behaviour, König's lemma gives an existential tuple matched at every step on
which the body holds. `X` is a function of the universal prefix, the
existential subjects and the body, so a refutation's existential half is a
closed computation that replay repeats.

## Verdicts

| Outcome | FR-360 label | FR-243 basis | Cause or evidence |
| --- | --- | --- | --- |
| Exhaustive check completed with no violation: HP-1 every tuple true; HP-2 no fair accepting cycle of the self-composition; HP-3 no reachable empty `X` with a cycle fair for every universal variable; HP-6 no reachable bad projected prefix | `proved` | `closed-scope` | exhaustive, or reduced with its reductions |
| Inductive proof of an HP-2 safety body, when every subject has an initial state and either `μ_U` is empty or a matched fair universal tuple of the instance is established (below) | `proved` | `decisive-witness` | induction at its depth |
| Counterexample that replays | `refuted` | `decisive-counterexample` | a hyper counterexample |
| The body, a match, or a guard, the `terminal when` predicate or a state constraint of a component's subject evaluated undefined at a reachable joint step, and the `undefined` counterexample ending there replays | `refuted` | `decisive-counterexample` | cause `undefined-evaluation`, with `where` and `cause`, and that counterexample |
| Every depth up to the maximum completed with no violation | `inconclusive` | `unsettled` | `bound-reached` naming the depth |
| The match was refused or incomplete at a reachable joint step | `inconclusive` | `unsettled` | `match-undetermined` |
| No matched fair universal tuple established for a non-empty `μ_U`: a vacuous clause, where no such tuple exists ([FR-395](FR-395-analyze-hyperproperties-over-behaviours.md)), or an inductive proof of an HP-2 body for an instance with no matched fair universal tuple established, since the induction queries are also unsatisfiable when no fair tuple matches | `inconclusive` | `unsettled` | `vacuous-match` |
| An undecided successor, no initial state, an item with zero instances, an induction not closed at its depth (a step counterexample, or the solver's own undecided answer to a base, step or search query at that depth, ordered after the other stops as FR-363 "Undecided k-induction runs" states), replay disagreement or replay refusal | `inconclusive` | `unsettled` | `undecided-successor`, `no-initial-state`, `no-instance`, `induction-not-closed`, `replay-parity` or `replay-refused` |
| A run limit, meter or time budget stopped the run | `failed` with execution `resource-incomplete` | `unavailable` | FR-331 result `incomplete` with cause `limit-reached{limit, value, setting}` |
| The caller cancelled the run | `failed` with execution `resource-incomplete` | `unavailable` | FR-331 result `incomplete` with cause `cancelled{source}` |
| The backend's own wall-clock limit expired, or the backend overran the request's `deadline_ms` | `failed` with execution `resource-incomplete` | `unavailable` | FR-331 result `incomplete` with cause `timed-out` |
| HP-4, or a clause over `behaviours` under a profile other than infinite-trace | `unsupported` | `unavailable` | `unsupported_projection`/`unsupported-requested-capability` |

A **matched fair universal tuple of an instance is established** exactly when the
run carries, for that instance, a tuple of fair lasso behaviours, one per
universal variable, that `μ_U` matches at every joint step, and that tuple
replays: each lasso replays as a behaviour of its subject, every conjunct of
`μ_U` holds at every joint step of the replay, and each lasso satisfies its
own subject's fairness constraints, all checked without evaluating the body.
A tuple that fails replay, or a tuple for another instance, establishes none.
A vacuous clause has no such tuple, since none exists. For an inductive HP-2
proof the rows apply in this order: a subject with no initial state settles
`no-initial-state`, whatever tuple comes with the proof; otherwise a non-empty
`μ_U` with no established tuple for the proof's instance settles
`vacuous-match`; otherwise the proof settles `proved`. With `μ_U` empty no
tuple is needed: a finite subject with an initial state has a fair behaviour,
and every tuple of fair behaviours is matched.

An item has one instance per admitted key (FR-361) of the populations overlapping its parameter's type, a key listed by several populations being one instance, so an
item with no admitted key (FR-361) for any one of its parameters has no instance to
evaluate ([FR-395](FR-395-analyze-hyperproperties-over-behaviours.md)). The
analyzer SHALL settle it `inconclusive` with cause `no-instance` before it
explores any state, as [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md)
does for other items, after the `requires-bound` classification: a parameter
with an overlapping population that has no universe is `requires-bound`, never `no-instance`.

An HP-5 item settles by [FR-396](FR-396-classify-hyperproperty-forms.md)'s
single-existential rows.

An undefined evaluation refutes by the FR-363 rule for every model proof
engine. When the body or a match, for a universal or an existential variable,
evaluates undefined at a reachable joint step, the analyzer SHALL settle the
item `refuted` with cause `undefined-evaluation`, whose `where` names the
tuple of trace or execution variables with their aliases, the joint position,
each component's state key there and the locus of
the expression that has no value, and whose `cause` names the undefined
value's catalogued reason. Its counterexample is the `undefined` kind of
[FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md), ending at
that joint step, and [FR-401](FR-401-replay-hyper-counterexamples.md) replay
reproduces the undefined value there. The `undefined` counterexample carries
that `undefined-evaluation` value.

The same rule covers each component's subject. Each component trace runs over
its alias's subject, which evaluates a guard of each transition
enabled-tested at the component's state, its `terminal when` predicate at a
state with no enabled transition, and each state constraint that restricts
its explored states. When one of them evaluates undefined at a component
state of a reachable joint step, the analyzer SHALL settle the item `refuted`
with cause `undefined-evaluation`, whose `where` names that component's
variable and alias alone, the joint position, that component's
state key and the locus of the expression, plus,
for a guard, the FR-181 transition identity whose guard it is. An undefined
guard is never read as a disabled transition, and an undefined `terminal when`
predicate never as a non-terminal state. A subject's expression belongs to
the subject, so its undefined evaluation refutes every hyperproperty item
over that subject whose analysis reaches that state.

**Canonical order of refuting evidence.** A violation and an undefined
evaluation are both refuting evidence, ordered as FR-363 orders them. The
analyzer SHALL report the first refuting evidence in this canonical order:
for HP-1, the first tuple in the product of the subjects' FR-181 canonical
edge orders, in binding order; for HP-2, HP-3 and HP-6, the first product
state in FR-181 canonical breadth-first order over the product, whose
successors are ordered by the tuple of component transition identities in
quantifier order and then by the automaton or witness-set component. At one
tuple or product state, an undefined evaluation precedes a false body, since
the evaluation that has no value is reached first.

`bound-reached` means: for HP-2 and HP-3, no counterexample whose universal
lasso has at most the maximum depth of joint steps; for HP-1, no
counterexample whose executions all leave states at depth below it; for HP-6,
no counterexample of at most that many moves. `match-undetermined` and
`vacuous-match` join the model-subject inconclusive causes; neither adds a
label, basis or category.

## Run limits

Every limit is a caller-set member of the request's temporal `limits`
([FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md)). No
form, formula size, arity or depth is capped. Each limit is counted with
checked arithmetic as the run reaches it, and reaching one stops the run and
settles FR-331 result `incomplete` with cause `limit-reached{limit, value,
setting}`, naming the limit, its value and its setting as
[FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) names and renders it, so the caller raises it
and reruns.

| Member | Counts | Setting |
| --- | --- | --- |
| `max_witness_set` | the members one HP-3 witness set may reach | `model_check.max_witness_set` |
| `max_relation_tuples` | the tuples an HP-1 run may evaluate | `model_check.max_relation_tuples` |
| `max_solver_steps` | the solver steps of each HP-2 SMT base, step or lasso query, as the solver counts them (FR-368 "SMT solver budgets") | `smt.max_solver_steps` |
| `max_solver_time` | the whole seconds of wall-clock time of each HP-2 SMT query (FR-368) | `smt.max_solver_time` |

[FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) defines the
count the denied charge would have reached. When a run stops at
`max_witness_set` or `max_relation_tuples`, the engine's stop record SHALL
expose that count, the value plus 1, since the run stops at the first member or
tuple that would pass the limit and counts that one. The stop records of the
other model-check limits (`max_states`, `max_transitions`,
`max_automaton_states` and the evaluation meter) and the SMT solver budgets
expose no count. An HP-2 SMT run stopped by `max_solver_steps` or
`max_solver_time` settles `limit-reached` naming it, in the order of FR-363
"Undecided k-induction runs": after cancellation and the backend's own wall
clock (`timed-out`), before `induction-not-closed`. The settled
`limit-reached` cause carries `limit`, `value` and `setting` only.

The routed provider publishes each default, which applies when the request
sets none. The model-subject
limits on states, transitions and automaton states count product states, joint
steps and automaton states for hyper items.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-399-AC-1 | Each outcome row settles exactly its listed FR-360 label, FR-243 basis and cause, a stopped run `incomplete` with `limit-reached` and a cancelled run `incomplete` with `cancelled{source}`, and each label matches exactly one FR-360 member with its FR-241, FR-242 and FR-243 combination. | Test (TC-338, TC-339, TC-340, TC-342) |
| FR-399-AC-2 | An HP-3 clause is refuted exactly when a fair universal tuple reaches an empty witness set; on the leaky vault `X_1` is empty, and on the secure vault every `X` is a singleton and the clause is proved. | Test (TC-339) |
| FR-399-AC-3 | An HP-3 run whose witness set reaches `max_witness_set` stops and settles `incomplete` with cause `limit-reached` naming `max_witness_set`, its value and its setting, the engine's stop record exposing the count reached, one past the value; a request that sets none runs under the provider's published default. | Test (TC-339) |
| FR-399-AC-4 | An HP-1 run that reaches `max_relation_tuples` stops and settles `incomplete` with cause `limit-reached` naming the limit, its value and its setting, the engine's stop record exposing the count reached, one past the value; a request that sets none runs under the provider's published default. | Test (TC-343) |
| FR-399-AC-5 | A match refused at a reachable joint step settles `inconclusive` with `match-undetermined`. A clause with a non-empty `μ_U` and no matched fair universal tuple established settles `inconclusive` with `vacuous-match`, both when no such tuple exists and when an inductive HP-2 proof closes for an instance with no matched tuple established; with a matched tuple of that instance established, the same inductive proof settles `proved`, `decisive-witness`. Neither cause settles `proved`. | Test (TC-340) |
| FR-399-AC-6 | A run that completes every depth up to the maximum with no violation settles `inconclusive` with `bound-reached` naming the depth, never `proved`. | Test (TC-338) |
| FR-399-AC-7 | A refutation settles only after its counterexample replays; a counterexample that fails replay settles `inconclusive` with `replay-parity` or `replay-refused`. | Test (TC-345) |
| FR-399-AC-8 | A body that evaluates undefined at a reachable joint step settles `refuted`, basis `decisive-counterexample`, cause `undefined-evaluation` whose `where` names the variable tuple, the joint position, each component's state key and the expression's locus and whose `cause` names the catalogued reason, after its `undefined` counterexample replays to the same undefined value. | Test (TC-348) |
| FR-399-AC-9 | With a violation and an undefined evaluation both reachable, the reported refutation is the first in canonical order: the first false or undefined tuple in the product of canonical edge orders for HP-1, and the first violating or undefined product state in canonical breadth-first order for HP-2, HP-3 and HP-6. | Test (TC-348) |
| FR-399-AC-10 | A guard, the `terminal when` predicate or a state constraint of a component's subject that evaluates undefined at a component state of a reachable joint step settles the item `refuted`, `decisive-counterexample`, cause `undefined-evaluation` whose `where` names that component's variable and alias, the joint position, its state key and the expression's locus, plus the FR-181 transition identity for a guard, after its `undefined` counterexample replays to the same undefined value. | Test (TC-348) |
| FR-399-AC-11 | A hyperproperty item with no admitted key for its object parameter (no overlapping population has a key that a member record of an initial state admits) settles `inconclusive`, basis `unsettled`, cause `no-instance` before any state is explored, and never `proved` or `refuted`; a two-alias item (HB-02's `(d: S::Device, e: N::Device)`) with no admitted key for one parameter and admitted keys for the other settles the same way, so the product rule does not read only the first parameter; the same item with an overlapping population that has no universe returns `requires-bound`, not `no-instance`. | Test (TC-340) |
| FR-399-AC-12 | An HP-2 k-induction outcome at depth 1 whose step query has a counterexample, one whose solver returns `unknown` at depth 1, and one whose solver stops for a resource internal to it that no request limit names each settle `inconclusive`, `unsettled`, cause `induction-not-closed{depth: 1}`, never `proved`; a run stopped by a limit the request names settles `failed` with execution `resource-incomplete`, FR-331 `incomplete`, `limit-reached` naming that limit, one ended by the caller's own deadline settles `incomplete`, `cancelled{source: deadline}`, and one whose backend overruns the request's `deadline_ms` settles `incomplete`, `timed-out`, each never `induction-not-closed` (the order of FR-363 "Undecided k-induction runs"). | Test (TC-340) |
| FR-399-AC-13 | An HP-2 k-induction outcome at depth 1 whose solver stops at `max_solver_steps` 1000 settles `failed` with execution `resource-incomplete`, FR-331 `incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: smt.max_solver_steps}`; one stopped at `max_solver_time` 5 settles `limit-reached{limit: max_solver_time, value: 5, setting: smt.max_solver_time}`, never `timed-out`; neither is ever `induction-not-closed`. | Test (TC-340) |

## Dependencies

- [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md) and
  [FR-396](FR-396-classify-hyperproperty-forms.md) state the meaning and the
  form.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply the
  labels and bases.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the limits and the terminal record.
- [FR-401](FR-401-replay-hyper-counterexamples.md) replays counterexamples.
- [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) and [FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md)
  own the model-subject verdict map, its inconclusive causes, the undefined-evaluation rule and the temporal `limits` member.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
- QSL-366: the owner ruling that an undefined evaluation refutes.
