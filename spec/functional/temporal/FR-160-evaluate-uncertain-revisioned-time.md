---
id: FR-160
title: "Evaluate uncertain, partial and revisioned temporal inputs"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-113
    type: references
---

# FR-160: Evaluate uncertain, partial and revisioned temporal inputs

## Description

When temporal inputs are partial, clock-uncertain or superseded, the evaluator SHALL propagate their exact possibility and revision sets into the temporal result.

## Inputs

Checked formula, known/missing/conflicting atom valuations, exact clock intervals,
observation revisions, progress/closure and resource limits.

## Outputs

Definite true/false, pending/indeterminate, incomplete, refused or failed result
with revision lineage.

## Behavior

Partial atoms carry possible truth values and reasons. Clock intervals use exact
unit arithmetic and establish order only when intervals require it. A revisioned
repair invalidates the affected suffix/window, recomputes under the same package
and links retained superseded and replacement results. Missing values and
midpoint guesses never become false facts.

## Possibility and revision semantics

An atom carries a nonempty possibility set over `{true, false}` plus a reason:
known values are singleton sets; missing and conflicting evidence retain both
possibilities but have distinct reasons. Boolean and temporal operators lift
pointwise over all consistent valuations. A singleton result is definite; a
two-valued result is `pending` when declared progress could resolve it and
`indeterminate` when the closed evidence remains conflicting. An empty
possibility set is an inconsistent-input refusal, not false.

An event time is an exact closed interval `[earliest, latest]` in a canonical
unit. Event `a` is definitely before `b` only when `a.latest < b.earliest`; it is
possibly before when `a.earliest < b.latest`. Overlap preserves both admissible
orders. A repair identifies the superseded revision and atom/event identities,
invalidates their temporal dependency closure, and recomputes that closure in
event-time order. Unaffected results retain their identities; replaced results
link both the prior and replacement evidence identities.

## Counter-tick intervals and endpoints

A time interval has, at each endpoint, an *open* or *closed* flag, and the
closed interval `[earliest, latest]` above is the case where both are closed. An
event stamped by a counter reading carries the interval its tick covers, which
is **half-open**: reading `k` (an unwrapped nonnegative integer, in ticks)
covers `[k, k + 1)`, closed at `k` and open at `k + 1`. A clock uncertainty of
`u` ticks widens it by Minkowski sum to `[k - u, k + 1 + u)`: the lower end
stays closed and the upper end stays open, so a stamp is never closed at the
upper end. This supersedes the "closed interval" wording that QSL ADR-026 MN-2
uses for a tick cover (see the note under Dependencies).

Over intervals of dense time with these flags, for `a` and `b`:

- `a` is *definitely before* `b` exactly when every point of `a` is below every
  point of `b`: `a.latest < b.earliest`, or `a.latest = b.earliest` and at least
  one of those two endpoints is open. Two adjacent ticks `k` and `k + 1` are
  therefore definitely ordered, and a tick is never definitely before itself.
- `a` is *possibly before* `b` exactly when `a.earliest < b.latest`.
- The difference `b - a` is the interval `[b.earliest - a.latest, b.latest -
  a.earliest]` whose endpoint is open when either operand endpoint it comes from
  is open. A bound `b - a <= d` is **true** when the difference's upper end is
  at most `d` (open or closed), **false** when its lower end exceeds `d`, or
  equals `d` and is open, and otherwise **indeterminate**. A bound `b - a < d`
  is decided the same way with the strict forms.

For a bound of 3000 ticks, an interval-uncertainty of 0 and a request at tick 0,
an acknowledgement at tick 2998 satisfies `ack - req <= 3000` (true), tick 3000
is indeterminate (the difference `(2999, 3001)` straddles the bound) and tick
3001 is false (the difference `(3000, 3002)` lies above it, its lower end open).
The vectors are `proposals/checked-package-v2/tick-cover-vectors.json`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-160-AC-1 | Known, missing and conflicting valuations produce their sound multi-valued temporal outcomes. | Test (TC-199) |
| FR-160-AC-2 | Overlapping clock intervals retain ordering uncertainty while disjoint intervals establish the required order. | Test (TC-199) |
| FR-160-AC-3 | An out-of-order revision recomputes exactly the affected result region and preserves old/new lineage. | Test (TC-199) |
| FR-160-AC-4 | Missing and conflicting atoms may share a possibility set but remain distinguishable by reason and closure behavior. | Test (TC-199) |
| FR-160-AC-5 | Interval ordering uses endpoints exactly and never substitutes a midpoint or ingestion order. | Test (TC-199) |
| FR-160-AC-6 | A counter reading `k` covers the half-open interval `[k, k + 1)`, widened by an uncertainty of `u` ticks to `[k - u, k + 1 + u)`. Two adjacent ticks are definitely ordered, a tick is not definitely before itself, and an interval open at an endpoint is definitely before one starting there. | Test (TC-199) |
| FR-160-AC-7 | With a bound of 3000 ticks, uncertainty 0 and a request at tick 0, `ack - req <= 3000` is true for an acknowledgement at tick 2998, indeterminate at tick 3000 and false at tick 3001; the same bound with uncertainty 1 decides the neighbouring ticks by the same endpoint rule (`proposals/checked-package-v2/tick-cover-vectors.json`, `TCV-01` to `TCV-12`). | Test (TC-199) |

## Dependencies

- QSL ADR-026 MN-2 (dense time) says a tick covers a "closed interval"; that
  wording is superseded by "Counter-tick intervals and endpoints" above, which
  fixes the half-open cover `[k, k + 1)` that QSL FR-251-AC-1 and RT FR-276-AC-1
  require. ADR-026 lives in the QSL repository and is amended there.

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
