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
---

# FR-193: Analyze continuous and hybrid models soundly

## Description

When continuous or hybrid analysis is requested, the analyzer SHALL interpret
the declared modes, exact state/time domains, differential/algebraic constraints
and discrete transitions under a selected sound solver contract.

## Inputs

Hybrid model, initial/invariant/guard/reset definitions, time horizon or
unbounded claim, numeric enclosure profile, solver capability and limits.

## Outputs

Proved enclosure, inconclusive result (with any possible violation the solver
reports as unverified evidence), unsupported, incomplete or failed result,
with the solver's method and numeric contract.

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
transitions with guards and resets. Continuous variables whose rate is 1 in
every mode are clocks and belong to timed models
([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md)); every
variable with another rate, and every flow, is analyzed under this
requirement by an external solver selected by negotiation. With no candidate
the item settles `unsupported`. A `hybrid-decl` whose every continuous variable has rate 1
in every mode refuses with `invalid_timed_model`/`rate-one-hybrid`, naming
the timed model form that declares it.

## Verdict map

| Solver result | FR-331 value | Proof basis or cause |
| --- | --- | --- |
| A result the solver's method makes sound for the claim, such as an `unsat` from a δ-complete solver for bounded unreachability, or a flowpipe enclosure computed with outward rounding that satisfies the property | `proved`, basis `closed-scope` | `bounded-solver{method, horizon, jumps}`; the proof holds for the claim within that time horizon and jump bound |
| `δ-sat`, a flowpipe meeting the unsafe set, `unknown`, or a method without sound rounding | `inconclusive` | `solver-inconclusive`, retaining any reported trace as unverified evidence |
| A solver stopped by its step budget | `incomplete`, execution `resource-incomplete` | `limit-reached{limit: max_solver_steps, value, setting: hybrid.max_solver_steps}` |
| A solver that exhausts `max_solver_time`, or a provider that overruns the provider deadline `deadline_ms` (FR-462) | `incomplete`, execution `resource-incomplete` | `timed-out`, the backend's own wall-clock limit |
| A run the caller's frontend ends at the caller's own deadline (a frontend-owned timer that cancels the run, QSL FR-276) | `incomplete`, execution `resource-incomplete` | `cancelled{source: deadline}`, never `timed-out` |
| No registered candidate | `unsupported` | `unsupported_projection`/`unsupported-requested-capability` |

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

A solver result settles only `proved`, `inconclusive`, incomplete or
`unsupported`: a hybrid result does not refute, because no solver witness
replays exactly.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-193-AC-1 | A selected provider returns a sound reachable enclosure with its numeric/error contract. | Test (TC-215) |
| FR-193-AC-2 | A reachable guard/reset violation the solver reports is retained as a source-correspondent hybrid trace and settles `inconclusive`, `solver-inconclusive`. | Test (TC-215, TC-373) |
| FR-193-AC-3 | Unsupported equations, failed convergence or insufficient horizon returns inconclusive/unsupported. | Test (TC-215) |
| FR-193-AC-4 | Every flow and reset encloses all admitted continuous states and records accumulated error bounds. | Test (TC-215) |
| FR-193-AC-5 | A point estimate or enclosure intersecting both satisfying and violating regions cannot establish satisfaction. | Test (TC-215) |
| FR-193-AC-6 | Each row of the verdict map settles its listed value and basis or cause; a sound solver result settles `proved` with `bounded-solver{method, horizon, jumps}`, and no solver result settles `refuted`. | Test (TC-373) |
| FR-193-AC-7 | A `hybrid-decl` whose every continuous variable has rate 1 refuses `invalid_timed_model`/`rate-one-hybrid`; a hybrid item with no registered candidate settles `unsupported`. | Test (TC-373) |
| FR-193-AC-8 | A solver stopped after 1,000 steps by a `max_solver_steps` of 1,000 settles `incomplete`, execution `resource-incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: hybrid.max_solver_steps}`, in steps and unconverted; a solver that exhausts `max_solver_time`, and a plugin provider that overruns `deadline_ms`, each settle `incomplete`, `resource-incomplete`, `timed-out`; a run the caller's frontend ends at the caller's own deadline settles `incomplete`, `cancelled{source: deadline}`, so no event has two causes. | Test (TC-373) |
| FR-193-AC-9 | A caller's `max_solver_time` of 1.5 s is written as the wire integer 2, 2 s as 2 and 0.2 s as 1, and a wire value of 1.5 is refused before consumption; two requests that differ only in `max_solver_time` have equal request identity and cache key bytes, two that differ only in `max_solver_steps` differ, and a `timed-out` result is stored by neither request and holds no wall-time member. | Test (TC-373) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
