---
id: FR-193
title: "Analyze continuous and hybrid models soundly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-306
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-252
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
---

# FR-193: Analyze continuous and hybrid models soundly

## Description

When continuous or hybrid analysis is requested, the analyzer SHALL interpret
the declared modes, exact state/time domains, differential/algebraic constraints
and discrete transitions under a selected sound solver contract.

## Inputs

Hybrid model, initial/invariant/guard/reset definitions, a hybrid claim
with its activation, captures, fairness and intervals, time horizon or
unbounded claim, numeric enclosure profile, solver capability and limits, and
any hybrid trace a provider reports.

## Outputs

Proved enclosure, a refutation whose hybrid counterexample replays exactly,
inconclusive result (with any possible violation the solver reports, and its
replay outcome), unsupported, incomplete or failed result, with the solver's
method and numeric contract.

## Behavior

Continuous evolution is constrained by mode invariants and declared equations;
discrete transitions apply exact guards/resets. Approximate numerical methods
must return sound enclosures under declared error bounds. A point estimate or
failed convergence cannot establish satisfaction.

## Hybrid reachability contract

A hybrid state is `(mode, exact discrete state, continuous enclosure, time
interval)`. Within a mode, a provider encloses every solution of the declared
differential inclusion while maintaining the invariant. A transition may fire
only where its guard enclosure intersects the reachable set; its reset maps the
entire guarded enclosure soundly into the target mode. Numeric provenance states
the arithmetic, step policy, local/truncation error bounds and accumulated
over-approximation. Satisfaction requires the full enclosure to satisfy the
property; intersection with a violation is a possible counterexample requiring
refinement or a validated witness.

## Source form and the clock line

A hybrid model is declared with the shared grammar's `hybrid-decl`: modes,
each with its invariant, its flow equations over continuous variables, and its
transitions with guards and resets. Continuous variables whose rate is the
numeric literal `1` in every mode are clocks and belong to timed models
([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md)); every
variable with another rate, and every flow, is analyzed under this
requirement by an external solver selected by negotiation. With no candidate
the item settles `unsupported`. A rate written any other way, such as
`2 - 1` or `rational(2, 2)`, is not the literal `1`, so its variable is
hybrid dynamics and goes to a solver. A `hybrid-decl` whose every continuous
variable has the rate literal `1` in every mode refuses with `invalid_timed_model`/`rate-one-hybrid`, naming
the timed model form that declares it.

## Initial states

A `hybrid-decl` may open with an `initial` clause of one or more entries
([shared grammar](../../../proposals/quire-v1/shared-grammar.md)):

```
hybrid Braking using p {
  initial {
    mode Moving when { v = 20 and a >= 1 and a <= 2 and t = 0 };
    mode Stopped when { v = 0 and t = 0 };
  }
  mode Moving ...
}
```

- An entry `mode M when { c };` SHALL name a mode `M` of the declaration and
  SHALL give a Boolean condition `c` over the continuous variables, in the
  expression forms of "Continuous values and expressions". A mode name that
  is no mode of the declaration SHALL refuse
  `missing_declaration`/`missing-name` at its span.
- The initial states SHALL be the pairs `(M, x)` at time 0 for which some
  entry names `M`, its condition holds at `x`, and `M`'s invariant holds at
  `x`. Several entries give the union of their sets, so a declaration may
  have several initial modes, and two entries for one mode give the
  disjunction of their conditions. A mode no entry names is not initial.
- A declaration with no `initial` clause SHALL have the one entry
  `mode M when { true };`, `M` its first mode in declared order. Its initial
  states are then the states that satisfy the first mode's invariant at
  time 0, and writing that entry changes nothing.
- An *initial mode* is a mode some entry names.
- A model whose initial set is empty has no behaviour. A provider that
  establishes that the set is empty SHALL settle the item `inconclusive`,
  `no-initial-state` (FR-331), never `proved`.

## Continuous values and expressions

- A continuous variable SHALL range over the real numbers: its values are
  those the solutions of its flows and the jumps reach. Every constant of a
  hybrid declaration and of a hybrid claim SHALL be an exact rational: a
  `uint`, a `decimal-literal`, an `exact-number` or `rational(n, d)`. No
  expression form denotes an irrational constant, and every enclosure is an
  FR-331 `Interval` of exact rationals.
- An invariant, a flow's right side, a guard, a reset, an initial condition,
  a claim's `holds(...)` expression, an activation condition and a capture
  initializer SHALL take these expression forms, each read over the reals:
  continuous variables, constants, `true` and `false`, unary `-`, `+`, `-`,
  `*` and `/`, the comparisons `=`, `!=`, `<`, `<=`, `>=` and `>`, `not`,
  `and`, `or` and `implies`, `let x = e in b`, `if c then a else b`, and a
  call of a pure function whose parameters and result are `Boolean` or
  numeric and whose body takes these forms. A product of variables gives a
  nonlinear (polynomial) term and `/` a rational one.
- Every operand SHALL be a real or Boolean value. A form whose operand or
  result has another type, such as a collection, a quantity literal, a
  string, an integer operation (`div`, `rem`, `mod`) over a real operand, or a
  call passing a real argument to an `Integer` or `Int` parameter, SHALL
  refuse `ill_typed`/`type-mismatch` at its span.
- In a call, a real argument SHALL be passed as the real it is: a numeric
  parameter type's value range is a precondition of the call, its
  precision components (denominator bounds, decimal scale, rounding) do not
  apply, and the result type's value range bounds the result in the same
  way. An argument or result outside its range has no value, as a division by
  zero has none.
- A state at which an invariant, a flow, a guard, a reset, an initial
  condition, an activation condition, a capture or a claim atom has no value
  SHALL be a violating state of every claim over the model. A proof
  (`proved`) SHALL establish that no such state is reachable; a provider that
  may reach one reports a possible violation (`inconclusive`), and a reach
  that replays exactly refutes with `undefined-evaluation` ("Trace replay").

## Resets

A transition's `reset` block relates the state before the jump to the state
after it.

- A reset SHALL be `true`, or a conjunction, possibly under `let` binders,
  of conjuncts `x ~ e`: `x` a continuous variable, `~` one of `=`, `!=`, `<`,
  `<=`, `>=` and `>`, and `e` any expression of "Continuous values and
  expressions".
- The left operand `x` of a conjunct SHALL name the value of `x` after the
  jump. Every variable read in `e`, and `pre(x)` for any continuous
  variable `x`, SHALL read the value before the jump (the pre-jump reading
  rule); `pre(x)` is admitted only in a reset.
- After the jump, a variable with an `=` conjunct SHALL take the value of
  its `e`; a variable with only other conjuncts SHALL take any real value
  that satisfies all of them, so a reset may be non-deterministic; and a
  variable no conjunct names SHALL keep its value.
- A variable with an `=` conjunct and any second conjunct SHALL refuse
  `conflicting_declaration`/`duplicate-declaration` at the second conjunct,
  naming both. A conjunct whose left operand is not a continuous variable,
  and a reset that is not a conjunction of conjuncts (a disjunction, a
  negation or a bare expression), SHALL refuse
  `invalid_model_binding`/`malformed-declaration` at its span.
- A transition SHALL be enabled at a state when its guard holds there and
  some state after the jump satisfies the reset and the target mode's
  invariant; the jump reaches one such state.

## Hybrid claims

A *hybrid claim* is a `temporal-clause` whose `over` parameter `h` is typed by
a `hybrid-decl`. In its body `h.x` reads the continuous variable `x`.

### Behaviours and positions

- A behaviour of the declaration SHALL start in an initial state at time 0
  and alternate dwells and jumps: a dwell of length `d >= 0` in mode `m`
  follows a solution of `m`'s flow equations with `m`'s invariant holding at
  every instant of `[0, d]`, and a jump takes an enabled transition of `m`
  at the dwell's end ("Resets"). A claim SHALL be read over every behaviour
  that is infinite and diverges in time, and over every finite maximal
  behaviour, the latter extended by repeating its last state at every later
  time; a behaviour that takes infinitely many jumps in finite time is not
  read.
- Under the `model-time` binding (FR-252), the positions of a behaviour
  SHALL be its instants: the pairs `(k, t)` with `t` in the time span of its
  `k`-th dwell, ordered by `k` and then by `t`, each with the time stamp
  `t`, a non-negative real number of seconds. A hybrid model is a timed
  subject in unit second, so a claim with no `clock` member binds
  `model-time`.
- Under `clock "model-steps"`, the positions SHALL be the start state and
  the state each jump reaches, in order, and an interval counts positions.
- A claim SHALL be read pointwise over these positions with the meaning of
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) "Pointwise
  meaning": the future operators `eventually`, `always`, `until` and
  `release` and the past operators `once`, `historically`, `since` and
  `triggered`, each with its interval where it writes one, and with no
  interval where the profile admits an unbounded operator (FR-367). A
  `holds(e)` atom reads `e` at the position's state.

### Intervals

- Under `model-time`, an `interval`'s `uint` bounds SHALL count seconds,
  and a `timed-interval`'s bounds SHALL be quantity literals of FR-142's
  time dimension, converted exactly to exact rational seconds, each end open
  or closed as written; `[q, *]` has no upper bound and is admitted where the
  profile admits `[a, *]`. A bound of another dimension SHALL refuse
  `ill_typed`/`type-mismatch` at its span.
- Under `model-steps`, an interval's bounds SHALL count positions, and a
  `timed-interval` SHALL refuse `ill_typed`/`type-mismatch` at its span.
- An interval whose lower bound exceeds its upper bound, or whose bounds are
  equal with an open end, SHALL refuse `invalid_timed_model`/`invalid-interval`
  at its span (FR-255).

### Activations and captures

- `on origin` SHALL create one obligation, at the first position.
- `on each (q: D)`, `D` the `over` parameter's declaration, SHALL create one
  obligation at every position, and `on each (q: D) when (g)` one at every
  position where `g` holds. In the body and in `g`, `q.x` SHALL read `x` at
  the activation position. An `on each` parameter typed by another
  declaration SHALL refuse `ill_typed`/`type-mismatch` at its type.
- A capture `capture n: T = e;`, `T` `Boolean` or numeric, SHALL evaluate
  `e` once at its obligation's activation position, and `n` SHALL read that
  value throughout the obligation (FR-093). A value outside `T`'s range has
  no value ("Continuous values and expressions").
- The claim SHALL hold on a behaviour when every obligation's body holds at
  its activation position.

### Fairness

- A fairness constraint `fair [weak | strong] [whole | each] N;` (FR-362)
  SHALL name hybrid transitions: `N` a mode `S` names every transition of
  `S`, and `S::T` the transitions of `S` whose target is `T`. A name that
  resolves to no transition SHALL refuse `missing_declaration`/`missing-name`
  at its span.
- A `whole` constraint SHALL be one constraint over the named transitions;
  an `each` constraint one constraint per named transition.
- A constraint SHALL be enabled at an instant when one of its transitions is
  enabled there ("Resets"), and taken when the behaviour jumps by one of
  them. A behaviour SHALL satisfy a weak constraint when, if the constraint
  is enabled at every instant from some instant on, it is taken infinitely
  often, and a strong constraint by FR-416 "Strong fairness over time". A
  claim SHALL be read only over the behaviours that satisfy its fairness
  set, and FR-362 admits a constraint only on a clause whose profile admits
  fairness.

## Verdict map

| Solver result | FR-331 value | Proof basis or cause |
| --- | --- | --- |
| A result the solver's method makes sound for the claim, such as an `unsat` from a δ-complete solver for bounded unreachability, or a flowpipe enclosure computed with outward rounding that satisfies the property | `proved`, basis `closed-scope` | `bounded-solver{method, horizon, jumps}`; the proof holds for the claim within that time horizon and jump bound |
| `δ-sat`, a flowpipe meeting the unsafe set, `unknown`, or a method without sound rounding | `inconclusive` | `solver-inconclusive{trace}`, retaining any reported trace as unverified evidence ("Hybrid trace" below) |
| A solver stopped by its step budget | `incomplete`, execution `resource-incomplete` | `limit-reached{limit: max_solver_steps, value, setting: hybrid.max_solver_steps}` |
| A solver that exhausts `max_solver_time`, or a provider that overruns the provider deadline `deadline_ms` (FR-462) | `incomplete`, execution `resource-incomplete` | `timed-out`, the backend's own wall-clock limit |
| A run the caller's frontend ends at the caller's own deadline (a frontend-owned timer that cancels the run, QSL FR-276) | `incomplete`, execution `resource-incomplete` | `cancelled{source: deadline}`, never `timed-out` |
| No registered candidate | `unsupported` | `unsupported_projection`/`unsupported-requested-capability` |
| A trace the reader replays exactly to a violation ("Trace replay") | `refuted` | a `HybridCounterexample` in `counterexamples`; the cause `undefined-evaluation{where, cause}` when the violation is an expression with no value |

### Solver budgets

A hybrid item carries its two budgets as `hybrid.limits.max_solver_steps` and
`hybrid.limits.max_solver_time`, beside `temporal.limits` and
`probabilistic.limits` (FR-331), so FR-461's rules for naming, raising and
defaulting a limit apply to both. Their setting names are
`hybrid.max_solver_steps` and `hybrid.max_solver_time`.

- `max_solver_steps` SHALL be a whole, positive count of solver steps as
  the provider counts its own steps. A report of it SHALL state the count as
  the provider counts it and SHALL NOT convert it to time. It is
  deterministic, so it SHALL be part of the request identity and the cache
  key (FR-306).
- `max_solver_time` SHALL be a wall-clock budget for the solver's run, in
  whole seconds, independent of the provider deadline `deadline_ms`. When the
  request writer writes a request for a caller that supplied a fractional
  duration, it SHALL round the duration up to the next whole second, never
  down, and write the whole number; the wire carries a canonical whole
  positive integer (FR-331's `PositiveCount`), and a provider SHALL refuse a
  fractional or otherwise non-canonical value before consumption, as FR-331
  refuses any out-of-domain value.
- When the caller sets no value for a budget, the request writer SHALL omit the
  member and the provider SHALL apply its descriptor `bounds` (FR-331,
  FR-461); a value present in the request SHALL always apply. QSL's 60
  seconds and 1,000,000 steps (QSL FR-250) are the descriptor bounds of its
  own built-in hybrid solver, owned by QSL; this requirement states no
  number.
- `max_solver_time` is a wall-clock setting. The request identity and the
  cache key SHALL exclude it, no member of a stored record SHALL record wall
  time, and a result it limits (`timed-out`) SHALL NOT be cached or stored
  (FR-306). Two requests that differ only in `max_solver_time` SHALL have
  equal request identity.

A hybrid item settles `refuted` only through exact replay ("Trace
replay"): a trace that the reader cannot replay exactly refutes nothing.

## Solver result on the wire

A hybrid provider reports each item's solver result as one FR-331 result
entry, and that entry is the wire form of the solver result.

- The producer of a result entry decides, before it writes the entry,
  whether the result is a proof: a first-party provider, one the producer
  runs as its own engine, applies the verdict map to its solver's method and
  outcome and writes `proved` only for a method sound for that outcome (QSL
  FR-250). The entry carries no outcome member, and a reader SHALL NOT
  decide soundness again from the entry.
- A proof SHALL be a `proved` entry with the basis
  `bounded-solver{method, horizon, jumps}`, `method` a non-empty spelling of
  the solver's method kind, `horizon` an FR-331 `Rational` number of seconds
  and `jumps` a count. Its `certification` SHALL follow FR-331
  "Certification": `uncertified` from a first-party provider and `trusted`
  from a provider loaded as a plugin. This requirement names no closed
  method vocabulary, and a reader refuses a `method` outside its own types
  as FR-331 refuses any proof basis outside the reader's types.
- A `δ-sat`, a flowpipe meeting the unsafe set, an `unknown`, and a result
  from a method not sound for its outcome SHALL each be an `inconclusive`
  entry with the cause `solver-inconclusive`. When the solver reports a path
  to the possible violation, the cause SHALL carry that path as its `trace`
  member, a `HybridTrace`; when it reports none, the cause SHALL carry no
  `trace`.
- A provider MAY report a refutation as a `refuted` entry whose
  `counterexample` names a `counterexamples` entry holding a
  `HybridCounterexample`: `{"type": "hybrid-counterexample", "kind": k,
  "trace": t}`, `k` `formula` or an `undefined-evaluation{where, cause}`
  envelope, and `t` a `HybridTrace`. The reader settles the item only after
  exact replay ("Trace replay").
- Every other row of the verdict map keeps its own value and cause: a budget
  stop, a `max_solver_time` exhaustion and a `deadline_ms` overrun settle
  `incomplete`, a run the caller's frontend cancels at its own deadline
  `incomplete` `cancelled{source: deadline}`, and a missing candidate
  `unsupported`, each with no trace; a missing solver or tool and a backend
  fault take the `unsupported` and `failed` results FR-331 records for them.

### Hybrid trace

A `HybridTrace` is the path to a possible violation, stated in the names of
the source `hybrid-decl`, so each state corresponds to a source mode and
each jump to a source transition:

```json
{
  "start": {"mode": "Moving", "time": {"lower": …, "upper": …},
            "enclosure": {"a": {"lower": …, "upper": …}, "v": {"lower": …, "upper": …}}},
  "jumps": [
    {"transition": 0,
     "state": {"mode": "Stopped", "time": {…}, "enclosure": {…}}}
  ]
}
```

- `start` SHALL be the trace's first state, in an initial mode ("Initial
  states").
- Each state SHALL carry `mode`, the name of a mode of the declaration;
  `time`, an FR-331 `Interval` of exact rational seconds enclosing the time
  at which the state is reached; and `enclosure`, one FR-331 `Interval` for
  each continuous variable of the declaration, keyed by the variable's name.
- Each `jumps` entry SHALL carry `transition`, the zero-based position of
  the transition taken among the transitions of the previous state's mode in
  source order, and `state`, the state the jump reaches, whose `mode` is that
  transition's target. `jumps` SHALL be present, and empty for a trace with
  no jump. A `jumps` entry MAY carry `before`, the state just before the
  jump, with `time` and `enclosure` and no `mode`, its mode being the
  previous state's.
- A trace MAY carry `delta`, a non-negative FR-331 `Rational`, the tolerance
  under which the solver's method decided (the `δ` of a δ-complete solver);
  an absent `delta` is 0, and a negative `delta` SHALL refuse
  `invalid_result`/`invalid-value` at `/delta`. A trace MAY carry `loop`, a count naming the state
  that its last state returns to, 0 for `start` and `k` for the state of
  `jumps[k - 1]`, so the trace denotes a lasso; a `loop` beyond the trace's
  last state SHALL refuse with `invalid_result`/`invalid-value` at `/loop`.
- The trace's last state SHALL be the state at which the solver reports the
  possible violation; in a `loop` trace it is the state that returns to the
  state `loop` names.
- The schema checks a trace's shape only. Its correspondence to the
  declaration is checked by the reader that holds the item's checked hybrid
  model: the producer, which serves a hybrid item in process, because a
  checked-package v2 document has no hybrid node and a package that holds a
  hybrid model has no v2 bytes (QSL FR-447). That reader SHALL refuse with
  `invalid_result`/`invalid-value`, at the member's pointer, a trace whose
  `mode` names no mode of the declaration, whose `start` is not in an
  initial mode, whose `transition` names no transition of the previous state's mode,
  whose jump reaches a mode other than that transition's target, whose
  `enclosure` keys differ from the declaration's continuous variables, or
  whose `Interval` has `lower` greater than `upper`.
- `solver-inconclusive` is also the cause of a non-hybrid solver's
  `unknown` (FR-468). A reader SHALL refuse a `trace` on the cause of an
  item whose claim is not a hybrid claim with `invalid_result`/
  `unknown-member` at `/cause/trace`.
- A `solver-inconclusive` trace is unverified evidence until replay: a
  reader SHALL keep it on the item's `inconclusive` record with its replay
  outcome, and SHALL settle a result from it only by exact replay ("Trace
  replay").

### Trace replay

The reader that holds the item's checked hybrid model SHALL replay every
`HybridTrace` it reads, the `trace` of a `solver-inconclusive` cause and the
trace of a `HybridCounterexample`, against that model and the item's claim.
Replay has two parts.

**Static replay.** Every trace SHALL be replayed statically, in exact
rational interval arithmetic over the trace's `Interval`s, each comparison
widened by the trace's `delta` `δ` (0 when absent): `a < b` reads
`a < b + δ`, `a <= b` reads `a <= b + δ`, `a = b` reads `|a - b| <= δ`,
`>` and `>=` likewise, and `a != b` holds whenever `δ > 0`. A check *fails*
when its widened interval value is definitely false over the enclosures it
reads. The checks, in trace order:

1. `start` meets an initial entry of its mode: the entry's condition and the
   mode's invariant over `start.enclosure`, and `start.time` contains 0.
2. Every state meets its mode's invariant over its enclosure.
3. Each state's `time.upper` is at least the previous state's `time.lower`;
   a jump's `before.time.upper` is at least the previous state's
   `time.lower`, and `before.time` meets the reached state's `time`, since a
   jump takes no time.
4. For a jump with `before`: the source mode's invariant and the guard over
   `before.enclosure`; for each `=` conjunct `x = e`, the interval of `e`
   over `before.enclosure` meets the reached state's interval of `x`; each
   other conjunct over both; and for a variable no conjunct names,
   `before`'s interval meets the reached state's.
5. For a `loop` trace, the last state's intervals meet those of the state
   `loop` names.

The outcome SHALL be `consistent`, or `inconsistent{at, check}` naming the
pointer of the first failing check and its number. Static replay settles no
value: the reader keeps the outcome with the trace.

**Exact replay.** A trace SHALL be replayed exactly when it is a *point
trace*, `δ` 0, every jump carrying `before`, and every `time`,
`enclosure` and `before` interval with `lower` equal to `upper`, and every
mode it dwells in has a *polynomial
flow*: after `let` substitution and expansion of non-recursive calls, every
rate is built from constants, variables, unary `-`, `+`, `-` and `*`, and the
relation "`y` occurs in `x`'s rate" has no cycle. The solution of a
polynomial flow from a point is then a vector of polynomials in time with
exact rational coefficients, computed by integrating the rates in that
relation's order, and exact replay SHALL decide each check over the reals
exactly, every atom along a dwell being a univariate rational function of
time whose sign on an interval is decidable:

1. `start` satisfies an initial entry.
2. For each dwell from a state to the next jump's `before`, of length
   `d = before.time - state.time >= 0`: the solution at `d` equals
   `before`, and the mode's invariant holds at every instant of `[0, d]`.
3. The guard holds at `before`, the reached state's `time` equals
   `before.time`, the reached state satisfies the reset against `before`,
   and it satisfies the target mode's invariant.
4. For a `loop` trace, the last state equals the state `loop` names and the
   loop's total dwell is positive, so the trace denotes one time-divergent
   periodic behaviour.
5. The claim is false on the replayed behaviour: for a `loop` trace, on that
   periodic behaviour; otherwise on the finite prefix ending at the last
   state, as a bad prefix, the claim false on every behaviour that extends
   it, decided by the pointwise meaning with every position past the
   prefix's end unknown.

When every check holds, the replay settles `reproduced-with-evaluated-witness`
(FR-364) and the item `refuted` with the trace as its `HybridCounterexample`
of kind `formula`. When an expression has no value at an instant of the
replayed prefix, the item settles `refuted` with the counterexample of kind
`undefined-evaluation{where, cause}`, `where` naming the state and its time
stamp, and the cause `undefined-evaluation` (FR-331). Otherwise the exact
replay outcome is `not-reproduced{at, check}`: a `solver-inconclusive` item
stays `inconclusive` with its static outcome, and a provider's `refuted`
item settles `inconclusive`, `replay-parity` (FR-364).

A trace that is not a point trace, or that dwells in a mode without a
polynomial flow, SHALL be replayed statically only, and its item SHALL NOT
refute. The reader SHALL record a trace in the item's `counterexamples` only
when exact replay reproduces it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-193-AC-1 | A selected provider returns a sound reachable enclosure with its numeric/error contract. | Test (TC-215) |
| FR-193-AC-2 | A reachable guard/reset violation the solver reports is retained as a source-correspondent hybrid trace and settles `inconclusive`, `solver-inconclusive`. | Test (TC-215, TC-373) |
| FR-193-AC-3 | Unsupported equations, failed convergence or insufficient horizon returns inconclusive/unsupported. | Test (TC-215) |
| FR-193-AC-4 | Every flow and reset encloses all admitted continuous states and records accumulated error bounds. | Test (TC-215) |
| FR-193-AC-5 | A point estimate or enclosure intersecting both satisfying and violating regions cannot establish satisfaction. | Test (TC-215) |
| FR-193-AC-6 | Each row of the verdict map settles its listed value and basis or cause; a sound solver result settles `proved` with `bounded-solver{method, horizon, jumps}`, and no solver result settles `refuted`. | Test (TC-373) |
| FR-193-AC-7 | A `hybrid-decl` whose every continuous variable has the rate literal `1` in every mode refuses `invalid_timed_model`/`rate-one-hybrid`; a hybrid item with no registered candidate settles `unsupported`. A `hybrid-decl` whose only variable `t` has the rate `2 - 1` in every mode is not refused and its `t` is hybrid dynamics. | Test (TC-373) |
| FR-193-AC-8 | A solver stopped after 1,000 steps by a `max_solver_steps` of 1,000 settles `incomplete`, execution `resource-incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: hybrid.max_solver_steps}`, in steps and unconverted; a solver that exhausts `max_solver_time`, and a plugin provider that overruns `deadline_ms`, each settle `incomplete`, `resource-incomplete`, `timed-out`; a run the caller's frontend ends at the caller's own deadline settles `incomplete`, `cancelled{source: deadline}`, so no event has two causes. | Test (TC-373) |
| FR-193-AC-9 | A caller's `max_solver_time` of 1.5 s is written as the wire integer 2, 2 s as 2 and 0.2 s as 1, and a wire value of 1.5 is refused before consumption; two requests that differ only in `max_solver_time` have equal request identity and cache key bytes, two that differ only in `max_solver_steps` differ, and a `timed-out` result is stored by neither request and holds no wall-time member. | Test (TC-373) |
| FR-193-AC-10 | A `solver-inconclusive` cause carrying a `HybridTrace` with a `start` in mode `Moving` and one jump by `transition` 0 to `Stopped`, and one carrying no `trace`, are each admitted and settle `inconclusive`, `solver-inconclusive`, the trace kept on the record and absent from `counterexamples`; a trace with an empty `enclosure`, and one with a member outside `start`, `jumps`, `delta` and `loop`, are refused by the schema at that member. The reader holding the checked model refuses `invalid_result`/`invalid-value` at the member a trace whose `mode` is `Halted`, one whose `start` is in `Stopped`, one whose jump names `transition` 1 of `Moving`, which has one transition, one whose jump by `transition` 0 reaches `Moving`, one whose `enclosure` omits `v`, one whose `enclosure` adds `w`, and one with an `Interval` whose `lower` exceeds its `upper`; a reader refuses `invalid_result`/`unknown-member` at `/cause/trace` a `trace` on the `solver-inconclusive` cause of an FR-468 refinement item. | Test (TC-373, TC-431) |
| FR-193-AC-11 | `Braking` with `initial { mode Moving when { v = 20 and a >= 1 and a <= 2 and t = 0 }; mode Stopped when { v = 0 and t = 0 }; }` has two initial modes and the initial states those entries and the modes' invariants give; with no `initial` clause its initial states are `Moving`'s invariant at time 0, equal to those of `initial { mode Moving when { true }; }`; `initial { mode Halted when { true }; }` refuses `missing_declaration`/`missing-name`; a model whose initial set is empty settles `inconclusive`, `no-initial-state`. | Test (TC-373) |
| FR-193-AC-12 | In a declaration over `v`, `a` and `x`, a flow `v' = -a * v`, a guard `v * v >= 2`, a flow `x' = if v > 1 then -1 else 0`, a flow `v' = drag(v)` calling a pure function `drag(s: Rational[-100, 100; 1, 1]): Rational[-1000, 0; 1, 10]` whose body is `- s * s / 10`, so that `v = 3/2` passes and `v = 101` has no value, a `let` and a `!=` are admitted; `v' = v div 2` and a call passing `v` to an `Integer` parameter refuse `ill_typed`/`type-mismatch`; a reachable state at which `1 / v` has `v = 0` is a violating state, so no provider proves a claim over a model that reaches it. | Test (TC-373) |
| FR-193-AC-13 | A reset `{ v = pre(v) / 2 and a >= 1 and a <= 2 }` halves `v` from its pre-jump value and sets `a` to any value in `[1, 2]`, keeping `t`; `{ v = v + 1 }` adds 1 to the pre-jump `v`; `{ v = 0 and v >= 0 }` refuses `conflicting_declaration`/`duplicate-declaration`; `{ v + a = 1 }` and `{ v = 0 or v = 1 }` refuse `invalid_model_binding`/`malformed-declaration`; a transition whose reset gives only states outside the target's invariant is never enabled. | Test (TC-373) |
| FR-193-AC-14 | Over `Braking`, `always [0, 10] holds(h.v < 30)` holds on every behaviour; `always [0, 10] (holds(h.v = 0) implies once [0, 2] holds(h.v <= 1))` and `holds(h.v > 0) since [0, 3] holds(h.v = 20)` are read pointwise over instants; `eventually [0 s, 1500 ms] holds(h.v < 19)` and `always (0.5 s, 2 s] holds(h.v <= 20)` read the intervals `[0, 3/2]` and `(1/2, 2]` in seconds; `[2 kg, 3 kg]` refuses `ill_typed`/`type-mismatch` and `(1 s, 1 s]` refuses `invalid_timed_model`/`invalid-interval`; under `clock "model-steps"`, `eventually [0, 1] holds(h.v = 0)` counts jumps. | Test (TC-373) |
| FR-193-AC-15 | `on each (q: Braking) when (q.v > 10)` with `capture v0: Rational = q.v;` and body `eventually [0, 10] holds(h.v < v0)` creates one obligation at every instant where `v > 10`, each reading its own `v0`; `on each (q: Other)` refuses `ill_typed`/`type-mismatch`; `fair Moving;` and `fair strong each Moving::Stopped;` resolve to `Moving`'s transitions and to those targeting `Stopped`, and `fair Moving::Halted;` refuses `missing_declaration`/`missing-name`. | Test (TC-373) |
| FR-193-AC-16 | Static replay of a trace whose `start` box meets no initial entry reports `inconsistent{at: /start, check: 1}`; one whose `before` box misses the guard by less than its `delta` 1/100 is `consistent`, and with no `delta` is `inconsistent` at check 4; neither outcome changes the `inconclusive` value. | Test (TC-373, TC-431) |
| FR-193-AC-17 | Over `Braking`, the point trace starting at `(Moving, v = 20, a = 2, t = 0)` with `before` `(time 10, v = 0, a = 2, t = 10)` and one jump by transition 0 to `(Stopped, time 10, v = 0, a = 2, t = 10)` replays exactly; against the claim `always [0, 12] holds(h.v > 0)` it reproduces and settles `refuted` with the trace as a `HybridCounterexample` of kind `formula`; the same trace with `before` `v = 1` settles `not-reproduced` at check 2, the item staying `inconclusive`, and as a provider's `refuted` entry `inconclusive`, `replay-parity`; a point trace through a mode with the flow `v' = v` is replayed statically only and does not refute. | Test (TC-373, TC-431) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
