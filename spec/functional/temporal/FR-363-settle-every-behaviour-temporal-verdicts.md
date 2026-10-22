---
id: FR-363
title: "Settle a temporal claim over every behaviour of a model"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-091
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
---

# FR-363: Settle a temporal claim over every behaviour of a model

## Description

When a temporal claim over a model subject is analyzed, the analyzer SHALL
return exactly one verdict from the verdict table below, whose label, basis,
proof basis and cause state how strongly the claim was settled.

## Inputs

A checked temporal clause, its model subject
([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md)), its resolved
fairness set ([FR-362](FR-362-apply-fairness-constraints.md)) and the
request's search horizon and caller-configurable resource limits.

## Outputs

One verdict per item: an [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
label with its FR-241, FR-242 and [FR-243](../../objects/protocol/FR-243-settlement-basis.md)
combination, the matching [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
result value, and the proof basis, depth or cause the table names.

## Behavior

### Meaning of a model verdict

The truth of a formula on one trace is the value the temporal trace evaluator
returns: the bounded profile's own evaluation over a finite trace; exact
evaluation over a lasso under infinite-trace; three-valued evaluation over a
finite prefix under infinite-trace. Replay, monitors and every analysis engine
use this one definition.

A formula **holds for a subject** exactly when the trace evaluator returns true
on every admitted behaviour: every behaviour under a bounded profile, and every
behaviour that satisfies the fairness set under infinite-trace. A
**refutation** is one admitted behaviour on which the evaluator returns false,
or on which an expression the analysis evaluates is undefined ("Undefined
evaluation" below).

### Property forms

The checker SHALL classify each claim into one form and record it with the
claim:

| Form | Profile | Shape |
| --- | --- | --- |
| reachable-state invariant | infinite-trace | `always holds(I)`, `I` a state predicate |
| bounded | event-position false-extension | any clause under that profile |
| safety | infinite-trace | in negation normal form, a formula with no unbounded `eventually` and no unbounded `until`; interval operators and past operators are admitted |
| liveness | infinite-trace | every other admitted infinite-trace formula |

The forms are tried in table order and the first that matches wins, so
`always holds(I)` with `I` a state predicate is a reachable-state invariant,
never safety.

A bounded claim activated `on origin` has a truth on a behaviour that depends
only on its first `h + 1` positions, where `h` is the formula's horizon: 0 for
an atom or constant; the largest operand horizon for a Boolean connective; the
upper bound `b` plus the largest operand horizon for a future interval
operator; and the largest operand horizon for a past operator. At a terminal
state within the horizon, the remaining positions read by the profile's
closed-boundary rule ([FR-091](FR-091-evaluate-bounded-future.md)). A bounded
claim activated `on each` holds when its formula holds at every activating
position of every behaviour; it is an unbounded safety property.

### Verdict table

| Verdict | Meaning | FR-360 label | FR-243 basis | FR-331 result | Proof basis or cause |
| --- | --- | --- | --- | --- | --- |
| Exhaustive | every admitted behaviour of the subject was examined | `proved` | `closed-scope` | `proved` | proof basis `exhaustive` |
| Bounded-complete to `k` | a bounded `on origin` claim unrolled to depth `k >= h`, so every behaviour is decided | `proved` | `closed-scope` | `proved` | proof basis `bounded-complete`, depth `k` |
| Inductive at `k` | the property holds in the first `k` steps of every behaviour, and any `k` consecutive states of the declared types that satisfy it have only satisfying successors | `proved` | `decisive-witness` | `proved` | proof basis `inductive`, depth `k` |
| Refuted | a counterexample that reproduced by replay ([FR-364](FR-364-replay-temporal-counterexamples.md)) | `refuted` | `decisive-counterexample` | `refuted` | none |
| Refuted, undefined evaluation | the claim, a guard, a `terminal when` predicate or a state constraint evaluates undefined at a position of an admitted behaviour, and a counterexample ending at that position reproduced by replay ([FR-364](FR-364-replay-temporal-counterexamples.md)) | `refuted` | `decisive-counterexample` | `refuted` | cause `undefined-evaluation`, with `where` and `cause` |
| Bound reached at `k` | the analysis completed every depth up to `k` and found no counterexample of length at most `k` | `inconclusive` | `unsettled` | `inconclusive` | cause `bound-reached`, depth `k` |
| Inconclusive, other | `induction-not-closed` with depth `k`: the base case holds to `k` and the step case has a counterexample to induction; `undecided-successor`: a contract conjunction of the successor relation is undecided (defined below), so the successor relation is not exactly known; `no-initial-state`: the subject has no initial state; `no-instance`: the item has zero instances, so no state was explored and the item is never read as holding; `replay-parity`: replay disagreed with the engine (FR-364) | `inconclusive` | `unsettled` | `inconclusive` | that cause |
| Stopped | a run limit stopped the analysis before it completed its method: state, transition or automaton-state limit, work meter, time, cancellation | `failed`, execution `resource-incomplete` | `unavailable` | `incomplete` | the limit that stopped it, its value and the request member that raises it |
| Unsupported | no candidate decides the item, or the subject does not admit the clause's profile | `unsupported` | `unavailable` | `unsupported` | the FR-368 or FR-361 cause |

No other label, basis or result value is added for model analysis. A proof
basis is carried only on a `proved` result.

### Undefined evaluation

An expression **evaluates undefined** when it returns the kernel's undefined
value: an evaluation with no value and a catalogued reason, such as
`lookup<T>(p, r) absent undefined` at a key the population does not hold
(reason `absent-key`). This one rule covers every expression an analysis of a
model subject evaluates at a state:

- the claim, read at a position through a `holds` atom or a state predicate;
- an operation's guard, its effective precondition, evaluated for a candidate
  transition at its pre-state;
- the `terminal when` predicate of the model's `terminal` declaration
  ([FR-366](FR-366-check-deadlock-freedom.md)), evaluated at a state with no
  enabled transition;
- a state constraint that restricts the subject's explored states.

When any of them evaluates undefined at a state of an admitted behaviour, the
analyzer SHALL settle the item `refuted`, basis `decisive-counterexample`,
FR-331 result `refuted`, with the FR-331 result cause `undefined-evaluation`
carrying:

- `where`: the position, the `quire.simulation.state-key/v1` digest of the
  state at that position (`state_key`), the source locus of the expression
  that has no value, and, for a guard, the FR-181 transition identity whose
  guard it is;
- `cause`: the undefined value's catalogued reason.

An undefined guard is never read as a disabled transition, and an undefined
`terminal when` predicate never as a non-terminal state. A guard, `terminal
when` predicate or state constraint belongs to the subject, so its undefined
evaluation refutes every item over the subject whose analysis reaches that
state. The counterexample is
the [FR-364](FR-364-replay-temporal-counterexamples.md) undefined-evaluation
counterexample ending at that state; its replay re-evaluates the claim, guard,
`terminal when` predicate or state constraint at the reported state.

This rule binds every engine that settles a claim over a model subject. Each
engine's `where` names its own evaluation point with the FR-364 key names. A
false evaluation and an undefined evaluation are both refuting evidence: an
engine reports the first refuting evidence it finds, in either form, and its
counterexample ends at that evidence. At one state, an undefined evaluation
precedes a false one. Statistical analysis reports the same evidence in its
own result form: a sampled behaviour on which an expression evaluates
undefined is a rejection, `measured: rejected`, cause `undefined-evaluation`.
An undefined evaluation adds no label, basis or result value.

### Undecided successor

A contract conjunction of the successor relation (an operation's effective
precondition, or its postcondition over a candidate post-state) is
**undecided** when its evaluation completes with neither `true` nor `false`:
the clause evaluator refuses it or returns an incomplete outcome, for example
because an observation the contract reads is unavailable or incomplete. The
successor relation is then not exactly known, and the item settles
`inconclusive`, cause `undecided-successor`. An undecided contract is not an
undefined evaluation: a guard or claim expression that returns the kernel's
undefined value refutes by the rule above.

### Length and depth

The **length** of a counterexample is its number of transitions, prefix and
loop together. "No counterexample of length at most `k`" is the one meaning of
bound reached at `k`, for every engine.

The depth `k` is the bounded-search horizon: a method parameter that the
request supplies as `max_depth` and the result states. It is not a resource
limit. A run that completes every depth up to `k` settles bound reached at
`k`, stating `k`. A run that a resource limit stops before it completes `k`
settles stopped, naming that limit, its value and how to raise it.

### Scope

A verdict holds for exactly its subject: the checked package, the initial
states, the universes and any substituted proof bounds. A verdict over one
universe is joined only to the request that names that universe. Over an
`over` parameter it holds for the item's instances only: a universe key of a population overlapping the parameter's type that
no member record of an initial state admits, including one only an operation
could create, is an excluded key of the item
([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md)), and the verdict
states nothing about the object an operation creates under it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-363-AC-1 | Each verdict returns exactly the FR-360 label, FR-243 basis, FR-331 result value and proof basis or cause of its row, and a proof basis appears only on a `proved` result. | Test (TC-293, TC-294) |
| FR-363-AC-2 | `always holds(I)` classifies as a reachable-state invariant, never safety, by table order, `always (holds(req) implies eventually[0,5] holds(ack))` as safety, `always eventually holds(p)` as liveness, and any event-position false-extension clause as bounded. | Test (TC-298) |
| FR-363-AC-3 | A run whose search horizon is `k` and that completes every depth up to `k` with no counterexample settles `inconclusive`, cause `bound-reached`, depth `k`, never `proved`; a counterexample of length `k` is found by the same run. | Test (TC-294) |
| FR-363-AC-4 | A run stopped by its state, transition or automaton-state limit, its meter, its time budget or cancellation before completing its depth settles FR-331 `incomplete` with FR-360 `failed`, execution `resource-incomplete`, naming the limit, its value and how to raise it. | Test (TC-294, TC-298) |
| FR-363-AC-5 | A subject whose expansion meets an operation precondition that the clause evaluator refuses or leaves incomplete, because the observation it reads is unavailable, settles `inconclusive`, cause `undecided-successor`, and never `refuted`. | Test (TC-294) |
| FR-363-AC-6 | A bounded `on origin` claim with horizon `h` unrolled to `k >= h` with no counterexample settles `proved`, proof basis `bounded-complete`, depth `k`; a bounded claim refuted over a model returns a finite prefix covering the horizon from the refuted activation or ending at a terminal state. | Test (TC-293) |
| FR-363-AC-7 | A verdict carries its subject; a verdict over one universe never settles an item whose request names a different universe. | Test (TC-302) |
| FR-363-AC-8 | A claim that evaluates undefined at a reachable position settles `refuted`, basis `decisive-counterexample`, FR-331 result `refuted`, cause `undefined-evaluation` whose `where` names the position, its state-key digest and the expression's locus and whose `cause` names the undefined value's catalogued reason; the same claim with the read defined at every reachable position settles by its truth values. | Test (TC-305) |
| FR-363-AC-9 | A model whose operation guard evaluates undefined for a candidate transition at a reachable state settles every item over the subject `refuted`, cause `undefined-evaluation` whose `where` names that position, its state-key digest, the guard's locus and the transition identity, and never reads the transition as disabled; a model whose `terminal when` predicate evaluates undefined at a reachable state with no enabled transition settles `refuted` the same way, never reading the state as non-terminal; a state constraint that evaluates undefined at a reachable state settles `refuted` the same way, naming the constraint's locus; at a state where the claim is both false and undefined, the reported cause is `undefined-evaluation`. | Test (TC-305) |

## Dependencies

- [FR-161](FR-161-analyze-infinite-trace-properties.md) owns the infinite-trace
  semantics and forbids proof from finite prefixes.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply every
  label and basis this table uses.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the result value and the terminal record.

## References

- quire-spec-language ADR-018 §1 (TP-1 to TP-4, V-1 to V-8), SM-1, SM-5 and
  QS-6, QS-7; Linear STD-131 (paired with QSL-366).
