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
  - target: ix://agent-ix/quire-specification/FR-306
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-481
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
requirement by an external solver selected by negotiation ("Routing a
hybrid item"). With no candidate the item settles `unsupported`. A rate written any other way, such as
`2 - 1` or `rational(2, 2)`, is not the literal `1`, so its variable is
hybrid dynamics and goes to a solver. A `hybrid-decl` whose every continuous
variable has the rate literal `1` in every mode refuses with `invalid_timed_model`/`rate-one-hybrid`, naming
the timed model form that declares it.

## Routing a hybrid item

A hybrid claim is requested as an FR-331 item of kind
`temporal-satisfaction` that carries the *hybrid-dynamics requirement*: the
item member `requirements` with the one label `hybrid-dynamics`. The
requirement, not the kind, tells a hybrid item from a temporal item over a
model without hybrid dynamics, which has the same kind.

- The request writer SHALL write `"requirements": ["hybrid-dynamics"]` on
  the item of every hybrid claim, since every hybrid model has a variable
  with hybrid dynamics ("Source form and the clock line"), and SHALL write
  no `requirements` member on any other item.
- `requirements` SHALL be a nonempty set of labels without repeats, each
  from the closed vocabulary {`hybrid-dynamics`}, carried only by an item of
  kind `temporal-satisfaction`; an item that carries `hybrid` (its solver
  budgets) SHALL carry `requirements` holding `hybrid-dynamics`. The FR-331
  schema refuses an item that breaks either rule before consumption.
- A backend descriptor that advertises `temporal-satisfaction` MAY carry
  `dynamics`, the dynamics its backend decides: a nonempty set without
  repeats drawn from `discrete`, temporal items whose subject has no hybrid
  dynamics, and `hybrid`, items that carry the hybrid-dynamics requirement.
  A descriptor with no `dynamics` decides `discrete` only, and a descriptor
  that advertises no `temporal-satisfaction` carries no `dynamics`.
- A descriptor whose `dynamics` holds `hybrid` SHALL carry, in `bounds`,
  the published defaults of both solver budgets, `max_solver_steps` and
  `max_solver_time` ("Solver budgets"), so an unset budget always has a
  value.
- Candidates (FR-290): a backend SHALL be a candidate for a
  `temporal-satisfaction` item only when it advertises the kind and its
  `dynamics` holds `hybrid` for an item carrying the hybrid-dynamics
  requirement, and `discrete` for an item carrying none. A backend whose
  descriptor has no `dynamics` is therefore never a candidate for a hybrid
  item, a backend with `["hybrid"]` is never a candidate for a temporal item
  without hybrid dynamics, and a backend with `["discrete", "hybrid"]` is a
  candidate for both. A named backend that fails this rule gives the empty
  set, and a `candidates` member listing such a backend is
  `inconsistent-candidates` (FR-290).
- A hybrid item with no candidate SHALL settle `unsupported`,
  `unsupported_projection`/`unsupported-requested-capability`, the warning
  naming `temporal-satisfaction` and the requirement `hybrid-dynamics`.
- `requirements` SHALL be part of the request identity and the cache key
  (FR-306); `dynamics` is part of the descriptor, and so of its manifest
  digest.
- A registration that advertises `temporal-satisfaction` SHALL be refused
  `invalid_capability`/`invalid-dynamics`, keyed by backend identity, when
  its `dynamics` is present and is not a list (`not-a-list`, retaining the
  member as supplied), is empty (`empty`), holds a value that is not exactly
  `discrete` or `hybrid` (`unknown-label`, retaining the element), repeats a
  label (`duplicate-label`, retaining the label), or holds `hybrid` while
  its `bounds` lack `max_solver_steps` or `max_solver_time`
  (`missing-bounds`, retaining the first absent limit name in that order).
  The registry SHALL report the first defect in that order, and within
  `unknown-label` or `duplicate-label` the first offending element in member
  order. It SHALL judge `dynamics` after every advertised pair, `domains`,
  `fairness_kinds` and the evidence kinds, so a registration with an
  earlier defect is refused with that earlier cause, and it SHALL NOT read
  the `dynamics` of a registration that advertises no
  `temporal-satisfaction`. The refusal removes only that registration. The
  FR-331 schema refuses each such `dynamics` on the wire, and a `dynamics`
  on a descriptor that advertises no `temporal-satisfaction`, before
  consumption.

A hybrid item and the descriptor of a backend that decides it, in their
FR-331 wire form (`claim`, `tool` and `domains` elided):

```json
{"request_index": 0, "kind": "temporal-satisfaction", "claim": {…},
 "extent": {"classification": "bounded", "bounds": []},
 "candidates": {"type": "set", "backends": ["dreal"]},
 "requirements": ["hybrid-dynamics"],
 "hybrid": {"limits": {"max_solver_steps": 1000}}}

{"backend": "dreal", "tool": {…},
 "advertises": [{"kind": "temporal-satisfaction", "mode": "bounded"}],
 "domains": […], "fairness_kinds": ["weak"], "dynamics": ["hybrid"],
 "bounds": {"max_solver_steps": 1000000, "max_solver_time": 60}}
```

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
  or closed as written; `[q, *]` has no upper bound. A bound of another
  dimension SHALL refuse `ill_typed`/`type-mismatch` at its span.
- The interval forms a claim admits follow its profile, as FR-370 "Placement
  and profile fit" fixes them: under `quire.temporal.timed/v1`, every
  rational bound, either end open or closed, and `[q, *]`; under an integer
  profile, such as `quire.temporal.infinite-trace/v1`, whole-second bounds
  with both ends closed, and `[a, *]` where that profile admits it. A bound
  or end the profile does not admit SHALL refuse
  `unsupported_construct`/`expression-form` at the interval's span, naming
  the profile, so a claim with rational or open bounds selects the timed
  profile.
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
| An outcome its method is sound for ("Solver methods"), whose proof covers the claim ("Proof coverage"), from a bounded method: an `unsat` from a δ-complete solver, or a flowpipe enclosure computed with outward rounding that is disjoint from the violating states | `proved`, basis `closed-scope` | `bounded-solver{method, horizon, jumps, exhaustive}`; the proof read every state the claim reads |
| An outcome its method is sound for, from an inductive method: an outward-rounded inductive enclosure disjoint from the violating states | `proved`, basis `closed-scope` | `inductive-solver{method}`; the proof holds for the claim at every time and after any number of jumps |
| An outcome its method is sound for, from a bounded method, whose proof does not cover the claim | `inconclusive` | `insufficient-horizon{horizon, jumps, exhaustive}`, the proof's own members |
| `δ-sat`, a flowpipe meeting the unsafe set, `unknown`, or an outcome its method is not sound for | `inconclusive` | `solver-inconclusive{trace}`, retaining any reported trace as unverified evidence ("Hybrid trace" below) |
| A solver stopped by its step budget | `incomplete`, execution `resource-incomplete` | `limit-reached{limit: max_solver_steps, value, setting: hybrid.max_solver_steps}` |
| A solver that exhausts `max_solver_time`, or a provider that overruns the provider deadline `deadline_ms` (FR-462) | `incomplete`, execution `resource-incomplete` | `timed-out`, the backend's own wall-clock limit |
| A run the caller's frontend ends at the caller's own deadline (a frontend-owned timer that cancels the run, QSL FR-276) | `incomplete`, execution `resource-incomplete` | `cancelled{source: deadline}`, never `timed-out` |
| No registered candidate | `unsupported` | `unsupported_projection`/`unsupported-requested-capability` |
| A trace the reader replays exactly to a violation ("Trace replay") | `refuted` | a `HybridCounterexample` in `counterexamples`; the cause `undefined-evaluation{where, cause}` when the violation is an expression with no value |

### Solver methods

A solver's *method kind* SHALL be one of this closed table. The kind, not
anything a provider says about itself, decides which outcomes are sound and
which proof basis a proof takes. A provider whose method fits no other row
reports its kind as `unclassified`.

| Method kind | Spelling | Proof basis | Outcome the kind is sound for |
| --- | --- | --- | --- |
| A δ-complete decision procedure (dReal, dReach), deciding the bounded reachability of a violating state up to its `δ` | `delta-complete` | `bounded-solver` | `unsat`: no violating state is reachable within the horizon and the jump bound |
| A flowpipe computed in outward-rounded interval or Taylor-model arithmetic (Flow*, CORA, JuliaReach, SpaceEx) | `flowpipe-outward-rounded` | `bounded-solver` | an enclosure disjoint from the violating states within the horizon and the jump bound |
| A flowpipe computed without outward rounding | `flowpipe-unrounded` | `bounded-solver` | none |
| An outward-rounded enclosure that contains every initial state and is closed under every mode's flow, for every dwell length, and under every transition's reset | `inductive-enclosure` | `inductive-solver` | an enclosure disjoint from the violating states |
| Any other method, such as simulation or sampling | `unclassified` | `bounded-solver` | none |

- An outcome SHALL be sound for a result only when its row names that
  outcome; every other pair of method kind and outcome, an `unsat` from a
  flowpipe method and an enclosure from a δ-complete solver included, SHALL
  settle `inconclusive`, `solver-inconclusive`.
- A proof SHALL carry its method kind's spelling as `method` and the proof
  basis its row names. A `bounded-solver` basis SHALL carry `delta-complete`,
  `flowpipe-outward-rounded`, `flowpipe-unrounded` or `unclassified`, and an
  `inductive-solver` basis `inductive-enclosure`; the FR-331 schema refuses
  any other `method` before consumption.
- A first-party provider SHALL write `proved` only for a row's sound
  outcome, so its `bounded-solver` proof carries `delta-complete` or
  `flowpipe-outward-rounded`. A reader SHALL refuse a first-party (`uncertified`)
  `proved` entry whose `method` is `flowpipe-unrounded` or `unclassified`
  with `invalid_result`/`invalid-value` at `/basis/method`. A plugin's
  `proved` entry is `trusted` (FR-331 "Certification") and is read with
  any `method` its basis admits.

### Proof coverage

A claim's *horizon* is the reach of its formula after its activation
position, as QSL FR-447 defines it and FR-481 derives it from the claim
node: seconds under `model-time`, positions under `model-steps`, and none
for an `on each` activation or an unbounded reach. The *claim region* is the
set of states the claim reads: under `model-time` with horizon `r`, every
state any behaviour reaches at a time `t <= r`, after any number of jumps;
under `model-steps` with horizon `n`, the start state and every state that
one of a behaviour's first `n` jumps reaches, at any time. A proof *covers*
the claim when the states it read include the claim region, and a proof
that does not cover its claim proves nothing about it.

- An `inductive-solver` proof reads every reachable state, so it SHALL
  cover every claim over its model.
- A `bounded-solver` proof reads its *proof region*: the states a
  behaviour reaches by time `horizon` with at most `jumps` jumps. Its
  `exhaustive` member SHALL list each dimension in which the solver
  established that the proof region is complete:
  - `time`: no behaviour takes more than `jumps` jumps by time `horizon`,
    so the proof region holds every state reached by time `horizon`;
  - `jumps`: no behaviour takes any of its first `jumps` jumps after time
    `horizon`, so the proof region holds every state reached by one of the
    first `jumps` jumps.

  `exhaustive` SHALL be a set without repeats drawn from `time` and
  `jumps`, empty when the solver established neither; a provider SHALL list
  a dimension only when its solver established it.
- Under `model-time`, a `bounded-solver` proof SHALL cover a claim of
  horizon `r` seconds exactly when `horizon >= r` and `exhaustive` holds
  `time`. A `jumps` 0 proof covers such a claim only when it lists `time`,
  that is, when the solver established that no behaviour jumps by `horizon`.
- Under `model-steps`, a `bounded-solver` proof SHALL cover a claim of
  horizon `n` positions exactly when `jumps >= n` and either `n` is 0 or
  `exhaustive` holds `jumps`; position `k` is the state the `k`-th jump
  reaches, a position count is never converted to seconds, and a horizon of
  0 positions reads the start state alone, which every proof reads.
- A `bounded-solver` proof SHALL NOT cover a claim with no horizon.
- A proof that does not cover its claim SHALL settle the item
  `inconclusive`, `insufficient-horizon{horizon, jumps, exhaustive}`, the
  proof's own members, never `proved`. Coverage reads the entry's
  `horizon`, `jumps` and `exhaustive` and the claim, not the method's
  soundness, so the reader that holds the claim SHALL check it for every
  `bounded-solver` proof it reads, a plugin's included, and settle a proof
  that does not cover as this cause.

Over `Braking`, which has no `initial` clause, a behaviour starts in
`Moving` with any `v0` in `[0, 20]` and `a` in `[1, 2]`, `v` falls at the
rate `a`, and the one jump, to `Stopped`, occurs at time `v0 / a`: anywhere
in `[0, 20]`, at 0 when `v0` is 0. No behaviour takes a second jump. For an `on origin` claim: `always [0, 10] holds(h.v < 30)` has
horizon 10 s, so a `delta-complete` `unsat` with `horizon` 10, `jumps` 1
and `exhaustive` `["time"]`, or with `horizon` 21/2, settles `proved`,
`bounded-solver`; with `horizon` 1 it settles `inconclusive`,
`insufficient-horizon{horizon: 1, jumps: 1, exhaustive: ["time"]}`; with
`horizon` 10, `jumps` 0 and `exhaustive` `[]` it settles
`insufficient-horizon`, since a behaviour may jump at any time in
`[0, 10]`, reaching `Stopped` states the proof did not read. `always [0, 10] eventually [0, 5] holds(h.v < 30)` has
horizon 15 s, which `horizon` 10 does not cover. Under `clock
"model-steps"`, `eventually [0, 1] holds(h.v = 0)` has horizon 1 position:
`jumps` 1 with `horizon` 20 and `exhaustive` `["jumps"]` covers it; `jumps`
1 with `horizon` 10 and `exhaustive` `[]` does not, since the jump may occur
at time 20 (`v0` 20, `a` 1), after the proof's horizon; and `jumps` 0 does not, whatever the
`horizon`. `always holds(h.v < 30)` and an `on each` claim have no horizon,
so only an `inductive-solver` proof settles them `proved`.

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
  FR-461), which every descriptor whose `dynamics` holds `hybrid` carries
  for both budgets ("Routing a hybrid item"); a value present in the
  request SHALL always apply. Each provider publishes its own defaults, and
  this requirement states no number.
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
  outcome and writes `proved` only for a method sound for that outcome
  ("Solver methods", QSL FR-250) and a proof that covers the claim ("Proof
  coverage"). The entry carries no outcome member, and a reader SHALL NOT
  decide soundness again from the entry; it checks only the closed method
  rule of "Solver methods" and the coverage of "Proof coverage".
- A proof SHALL be a `proved` entry with the basis
  `bounded-solver{method, horizon, jumps, exhaustive}`, `method` the
  spelling of a bounded method kind, `horizon` an FR-331 `Rational` number
  of seconds, `jumps` a count and `exhaustive` the set of complete
  dimensions ("Proof coverage"), or with the basis `inductive-solver{method}`, `method`
  `inductive-enclosure`. Its `certification` SHALL follow FR-331
  "Certification": `uncertified` from a first-party provider and `trusted`
  from a provider loaded as a plugin. A `method` outside the spellings its
  basis admits is refused by the schema, and a reader that holds the entry
  as a value refuses it `invalid_result`/`invalid-value` at `/basis/method`.
- A sound bounded proof that does not cover its claim SHALL be an
  `inconclusive` entry with the cause `insufficient-horizon{horizon,
  jumps, exhaustive}`, the members of the proof, with no trace. A reader
  SHALL refuse an `insufficient-horizon` cause on an item whose claim is not
  a hybrid claim with `invalid_result`/`invalid-value` at `/cause/type`, as
  it refuses a `hybrid-counterexample` there.
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
  `jumps[k - 1]`, so the trace denotes a lasso; `loop` SHALL name a state
  before the last one, and a `loop` naming the last state or beyond SHALL
  refuse with `invalid_result`/`invalid-value` at `/loop`.
- The trace's last state SHALL be the state at which the solver reports the
  possible violation; in a `loop` trace it is the state that returns to the
  state `loop` names.
- The schema checks a trace's shape only. Its correspondence to the
  declaration is checked by the reader of the result entry: the host that
  requested the item and holds its hybrid model, whichever provider, in
  process or in a process of its own, returned the trace
  ([FR-481](../../objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md)
  "Hybrid trace correspondence"). That reader SHALL refuse with
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
5. For a `loop` trace, the last state's mode is that of the state `loop`
   names and its enclosure meets that state's.

The outcome SHALL be `consistent`, or `inconsistent{at, check}` naming the
pointer of the first failing check and its number. Static replay settles no
value: the reader keeps the outcome with the trace as the `replay` member of
the item's `solver-inconclusive` cause on its stored record (FR-306):
`{"static": s}` or `{"static": s, "exact": e}`, `s` `{"outcome":
"consistent"}` or `{"outcome": "inconsistent", "at": p, "check": n}`, and `e`
`{"outcome": "not-exact"}` or `{"outcome": "not-reproduced", "at": p,
"check": n}`, `p` an RFC 6901 pointer into the trace and `n` the check's
number. A provider writes no `replay`: a reader SHALL refuse a provider's
entry that carries one with `invalid_result`/`unknown-member` at
`/cause/replay`.

**Exact replay.** A trace SHALL be replayed exactly when it is a *point
trace*, `δ` 0, every jump carrying `before`, and every `time`,
`enclosure` and `before` interval with `lower` equal to `upper`, and every
mode it dwells in has a *polynomial
flow*: after `let` substitution and expansion of non-recursive calls, every
rate is built from constants, variables, unary `-`, `+`, `-` and `*`, and the
relation "`y` occurs in `x`'s rate" has no cycle; and every invariant, guard,
reset, initial condition, activation condition, capture and claim atom the
replay evaluates calls no recursive function, so each expands to the
`HybridTerm` forms other than `Call`. The solution of a
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
4. For a `loop` trace, the last state's mode and point equal those of the
   state `loop` names and the loop's total dwell is positive, so the trace denotes one time-divergent
   periodic behaviour.
5. The claim is false on the replayed behaviour: for a `loop` trace, on that
   periodic behaviour; otherwise on the finite prefix ending at the last
   state, as a bad prefix, the claim false on every behaviour that extends
   it, decided by the pointwise meaning with every position past the
   prefix's end unknown.

The *decisive instant* of a reproduced trace SHALL be the earliest instant
of the replayed behaviour at which either the prefix up to it is a bad
prefix of the claim or an expression the replay evaluates has no value.
When checks 1 to 4 hold and that instant exists, the replay settles
`reproduced-with-evaluated-witness` (FR-364): if an expression has no value
at the decisive instant, the item settles `refuted` with the counterexample
of kind `undefined-evaluation{where, cause}`, `where` naming that instant's
state, its time stamp and the expression's locus, and the cause
`undefined-evaluation` (FR-331); otherwise the item settles `refuted` with
the trace as its `HybridCounterexample` of kind `formula`. An expression
with no value only after the decisive instant changes nothing. A `loop`
trace has no bad prefix: when checks 1 to 4 hold, it reproduces with kind
`undefined-evaluation` at the earliest instant of its periodic behaviour at
which an expression has no value, and otherwise with kind `formula` when
check 5 holds. Otherwise the exact
replay outcome is `not-reproduced{at, check}`: a `solver-inconclusive` item
stays `inconclusive` with its static outcome, and a provider's `refuted`
item settles `inconclusive`, `replay-parity` (FR-364).

A trace that is not a point trace, that dwells in a mode without a
polynomial flow, or whose replay would evaluate a recursive call, SHALL be
replayed statically only, and its item SHALL NOT
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
| FR-193-AC-6 | Each row of the verdict map settles its listed value and basis or cause; a sound solver result whose proof covers the claim settles `proved` with `bounded-solver{method, horizon, jumps, exhaustive}` or `inductive-solver{method}`, and no solver result settles `refuted`. | Test (TC-373) |
| FR-193-AC-7 | A `hybrid-decl` whose every continuous variable has the rate literal `1` in every mode refuses `invalid_timed_model`/`rate-one-hybrid`; a hybrid item with no registered candidate settles `unsupported`. A `hybrid-decl` whose only variable `t` has the rate `2 - 1` in every mode is not refused and its `t` is hybrid dynamics. | Test (TC-373) |
| FR-193-AC-8 | A solver stopped after 1,000 steps by a `max_solver_steps` of 1,000 settles `incomplete`, execution `resource-incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: hybrid.max_solver_steps}`, in steps and unconverted; a solver that exhausts `max_solver_time`, and a plugin provider that overruns `deadline_ms`, each settle `incomplete`, `resource-incomplete`, `timed-out`; a run the caller's frontend ends at the caller's own deadline settles `incomplete`, `cancelled{source: deadline}`, so no event has two causes. | Test (TC-373) |
| FR-193-AC-9 | A caller's `max_solver_time` of 1.5 s is written as the wire integer 2, 2 s as 2 and 0.2 s as 1, and a wire value of 1.5 is refused before consumption; two requests that differ only in `max_solver_time` have equal request identity and cache key bytes, two that differ only in `max_solver_steps` differ, and a `timed-out` result is stored by neither request and holds no wall-time member. | Test (TC-373) |
| FR-193-AC-10 | A `solver-inconclusive` cause carrying a `HybridTrace` with a `start` in mode `Moving` and one jump by `transition` 0 to `Stopped`, and one carrying no `trace`, are each admitted and settle `inconclusive`, `solver-inconclusive`, the trace kept on the record and absent from `counterexamples`; a trace with an empty `enclosure`, and one with a member outside `start`, `jumps`, `delta` and `loop`, are refused by the schema at that member. The reader holding the checked model refuses `invalid_result`/`invalid-value` at the member a trace whose `mode` is `Halted`, one whose `start` is in `Stopped`, one whose jump names `transition` 1 of `Moving`, which has one transition, one whose jump by `transition` 0 reaches `Moving`, one whose `enclosure` omits `v`, one whose `enclosure` adds `w`, and one with an `Interval` whose `lower` exceeds its `upper`; a reader refuses `invalid_result`/`unknown-member` at `/cause/trace` a `trace` on the `solver-inconclusive` cause of an FR-468 refinement item. | Test (TC-373, TC-431) |
| FR-193-AC-11 | `Braking` with `initial { mode Moving when { v = 20 and a >= 1 and a <= 2 and t = 0 }; mode Stopped when { v = 0 and t = 0 }; }` has two initial modes and the initial states those entries and the modes' invariants give; with no `initial` clause its initial states are `Moving`'s invariant at time 0, equal to those of `initial { mode Moving when { true }; }`; `initial { mode Halted when { true }; }` refuses `missing_declaration`/`missing-name`; a model whose initial set is empty settles `inconclusive`, `no-initial-state`. | Test (TC-373) |
| FR-193-AC-12 | In a declaration over `v`, `a` and `x`, a flow `v' = -a * v`, a guard `v * v >= 2`, a flow `x' = if v > 1 then -1 else 0`, a flow `v' = drag(v)` calling a pure function `drag(s: Rational[-100, 100; 1, 1]): Rational[-1000, 0; 1, 10]` whose body is `- s * s / 10`, so that `v = 3/2` passes and `v = 101` has no value, a `let` and a `!=` are admitted; `v' = v div 2` and a call passing `v` to an `Integer` parameter refuse `ill_typed`/`type-mismatch`; a reachable state at which `1 / v` has `v = 0` is a violating state, so no provider proves a claim over a model that reaches it. | Test (TC-373) |
| FR-193-AC-13 | A reset `{ v = pre(v) / 2 and a >= 1 and a <= 2 }` halves `v` from its pre-jump value and sets `a` to any value in `[1, 2]`, keeping `t`; `{ v = v + 1 }` adds 1 to the pre-jump `v`; `{ v = 0 and v >= 0 }` refuses `conflicting_declaration`/`duplicate-declaration`; `{ v + a = 1 }` and `{ v = 0 or v = 1 }` refuse `invalid_model_binding`/`malformed-declaration`; a transition whose reset gives only states outside the target's invariant is never enabled. | Test (TC-373) |
| FR-193-AC-14 | Over `Braking`, `always [0, 10] holds(h.v < 30)` holds on every behaviour; `always [0, 10] (holds(h.v = 0) implies once [0, 2] holds(h.v <= 1))` and `holds(h.v > 0) since [0, 3] holds(h.v = 20)` are read pointwise over instants; `eventually [0 s, 1500 ms] holds(h.v < 19)` and `always (0.5 s, 2 s] holds(h.v <= 20)` read the intervals `[0, 3/2]` and `(1/2, 2]` in seconds under `quire.temporal.timed/v1`, and the same claims under `quire.temporal.infinite-trace/v1` refuse `unsupported_construct`/`expression-form` naming that profile, while `eventually [0 s, 2 s]` is admitted there as `[0, 2]`; `[2 kg, 3 kg]` refuses `ill_typed`/`type-mismatch` and `(1 s, 1 s]` refuses `invalid_timed_model`/`invalid-interval`; under `clock "model-steps"`, `eventually [0, 1] holds(h.v = 0)` counts jumps. | Test (TC-373) |
| FR-193-AC-15 | `on each (q: Braking) when (q.v > 10)` with `capture v0: Rational = q.v;` and body `eventually [0, 10] holds(h.v < v0)` creates one obligation at every instant where `v > 10`, each reading its own `v0`; `on each (q: Other)` refuses `ill_typed`/`type-mismatch`; `fair Moving;` and `fair strong each Moving::Stopped;` resolve to `Moving`'s transitions and to those targeting `Stopped`, and `fair Moving::Halted;` refuses `missing_declaration`/`missing-name`. | Test (TC-373) |
| FR-193-AC-16 | Static replay of a trace whose `start` box meets no initial entry reports `inconsistent{at: /start, check: 1}`; one whose `before` box misses the guard by less than its `delta` 1/100 is `consistent`, and with no `delta` is `inconsistent` at check 4; neither outcome changes the `inconclusive` value. | Test (TC-373, TC-431) |
| FR-193-AC-17 | Over `Braking`, the point trace starting at `(Moving, v = 20, a = 2, t = 0)` with `before` `(time 10, v = 0, a = 2, t = 10)` and one jump by transition 0 to `(Stopped, time 10, v = 0, a = 2, t = 10)` replays exactly; against the claim `always [0, 12] holds(h.v > 0)` it reproduces and settles `refuted` with the trace as a `HybridCounterexample` of kind `formula`; the same trace with `before` `v = 1` settles `not-reproduced` at check 2, the item staying `inconclusive`, and as a provider's `refuted` entry `inconclusive`, `replay-parity`; a point trace through a mode with the flow `v' = v`, and the AC-17 trace against a claim whose atom calls a recursive function, are replayed statically only and do not refute; against `always [0, 12] holds(1 / (h.v - 5) != 0)` the AC-17 trace settles `refuted` of kind `undefined-evaluation` at time 15/2, and against `always [0, 12] holds(1 / (h.v - 5) < 100)`, false from time 1499/200 before the undefined instant, of kind `formula`; a stored record carries the static outcome as `replay`, and a provider entry carrying `replay` is refused `invalid_result`/`unknown-member` at `/cause/replay`; a `loop` naming the last state is refused `invalid_result`/`invalid-value` at `/loop`. | Test (TC-373, TC-431) |
| FR-193-AC-18 | With two registered backends advertising `temporal-satisfaction`, `qsl-model-check` with no `dynamics` and `dreal` with `dynamics` `["hybrid"]` and `bounds` `{max_solver_steps: 1000000, max_solver_time: 60}`, the item of `always [0, 10] holds(h.v < 30)` over `Braking`, carrying `"requirements": ["hybrid-dynamics"]` has the one candidate `dreal`, and a temporal item over a model without hybrid dynamics, carrying no `requirements`, has the one candidate `qsl-model-check`, neither `ambiguous-backend`; with `dreal`'s `dynamics` `["discrete", "hybrid"]` the second item has both candidates and settles `ambiguous-backend`; with only `qsl-model-check` registered the hybrid item settles `unsupported`, `unsupported-requested-capability`, its warning naming `temporal-satisfaction` and `hybrid-dynamics`; a hybrid item naming `qsl-model-check` has the empty set, and one whose `candidates` lists `qsl-model-check` settles `inconsistent-candidates`. Two requests differing only in an item's `requirements` have different request identities. | Test (TC-373) |
| FR-193-AC-19 | The schema refuses before consumption an item carrying `requirements` `[]`, `["hybrid-dynamics", "hybrid-dynamics"]` or `["stochastic-dynamics"]`, an item of kind `value-validity` carrying `requirements`, an item carrying `hybrid` without `requirements`, a temporal descriptor with `dynamics` `[]`, `["hybrid", "hybrid"]` or `["continuous"]`, a `dynamics` on a descriptor advertising only `operation-contract`, and a descriptor with `dynamics` `["hybrid"]` whose `bounds` lack `max_solver_time`; a registration not on the wire with each of these `dynamics` defects is refused `invalid_capability`/`invalid-dynamics` retaining `empty`, `duplicate-label` naming `hybrid`, `unknown-label` naming `continuous` and `missing-bounds` naming `max_solver_time`, and one with `dynamics` `"hybrid"` retaining `not-a-list`; a registration with `dynamics` `["continuous", "hybrid"]` and `bounds` lacking `max_solver_time` reports `unknown-label`, and one that also has a `fairness_kinds` defect reports `invalid-fairness-kinds`. | Test (TC-373, TC-431) |
| FR-193-AC-20 | The method table settles each pair as stated: `delta-complete` with `unsat` and `flowpipe-outward-rounded` with a disjoint enclosure, each covering, settle `proved`, `bounded-solver` carrying that spelling; `inductive-enclosure` with a disjoint enclosure settles `proved`, `inductive-solver{method: inductive-enclosure}`; `flowpipe-unrounded` and `unclassified` with any outcome, `delta-complete` with a disjoint enclosure and `flowpipe-outward-rounded` with `unsat` each settle `inconclusive`, `solver-inconclusive`. The schema refuses a `bounded-solver` with `method` `dreal` or `inductive-enclosure` and an `inductive-solver` with `method` `delta-complete`; a reader refuses an `uncertified` `bounded-solver` proof with `method` `flowpipe-unrounded` `invalid_result`/`invalid-value` at `/basis/method`, and admits the same proof as a plugin's `trusted` entry. | Test (TC-373, TC-431) |
| FR-193-AC-21 | Over `Braking`, whose one jump occurs at time `v0 / a`, anywhere in `[0, 20]`, and which takes no second jump: against `always [0, 10] holds(h.v < 30)`, a `delta-complete` `unsat` with `horizon` 10, `jumps` 1 and `exhaustive` `["time"]`, and one with `horizon` 21/2, settle `proved`, `bounded-solver`; one with `horizon` 1 settles `inconclusive`, `insufficient-horizon{horizon: 1, jumps: 1, exhaustive: ["time"]}`; one with `horizon` 10, `jumps` 0 and `exhaustive` `[]` settles `insufficient-horizon{horizon: 10, jumps: 0, exhaustive: []}`, and one with `horizon` 20, `jumps` 3 and `exhaustive` `["jumps"]` settles `insufficient-horizon`; against `always [0, 10] eventually [0, 5] holds(h.v < 30)` a `horizon` 10 proof with `exhaustive` `["time"]` settles `insufficient-horizon`; under `clock "model-steps"`, against `eventually [0, 1] holds(h.v = 0)`, `jumps` 1 with `horizon` 20 and `exhaustive` `["jumps"]` settles `proved`, `jumps` 1 with `horizon` 10 and `exhaustive` `[]` settles `insufficient-horizon{horizon: 10, jumps: 1, exhaustive: []}`, and `jumps` 0 with `horizon` 100 and `exhaustive` `["jumps"]` settles `insufficient-horizon`; against `once [0, 2] holds(h.v = 20)` under `model-steps` (horizon 0 positions) a proof with `jumps` 0 and `exhaustive` `[]` settles `proved`; against `always holds(h.v < 30)` and against FR-193-AC-15's `on each (q: Braking) when (q.v > 10)` claim, every `bounded-solver` proof settles `insufficient-horizon` and an `inductive-solver` proof settles `proved`; a plugin's `trusted` `bounded-solver` entry with `horizon` 1 against the first claim settles `insufficient-horizon` at the reader; an `insufficient-horizon` cause on an FR-468 refinement item is refused `invalid_result`/`invalid-value` at `/cause/type`. | Test (TC-373) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
