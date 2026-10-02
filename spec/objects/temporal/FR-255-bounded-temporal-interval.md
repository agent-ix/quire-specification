---
id: FR-255
title: "Bounded temporal interval"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-252
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
---
# FR-255: Bounded temporal interval

## Description

When a bounded temporal operator is applied, the evaluator SHALL read its interval as the inclusive bound pair of the selected profile without shortening that interval to the supplied file.

FR-091 states the form as "inclusive integers `0 <= a <= b`" and requires the
inclusive interval and boundary rules to be applied "to every admitted bounded
future operator without shortening the interval to the supplied file". FR-092
states the same bound form for the past direction.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Lower bound `a` | FR-091 Behavior, FR-092 Inputs | Inclusive integer, `0 <= a`. |
| Upper bound `b` | FR-091 Behavior, FR-092 Inputs | Inclusive integer, `a <= b`. |
| Selected profile identity | FR-090, FR-091-AC-2 | One tick's meaning is fixed by the profile ([FR-250](FR-250-temporal-semantic-profile.md)), so the same numeric pair under two profiles is not the same interval. |
| Clock binding | FR-090, FR-094-AC-4 | Offsets are counted under one binding ([FR-252](FR-252-temporal-clock-binding.md)). |

Both endpoints participate. FR-091 requires truth tables covering "every
inclusive endpoint, zero-width intervals, nonzero lower bounds, nesting and
short traces" (FR-091-AC-1), and FR-092-AC-1 requires the same for the past
direction. The half-open reading `[a,d)` appears only inside the definitions of
`until` and `since`, over the left operand between the lower bound and the
witness, and not over the interval itself.

For the timestamped finite-window profile, eligibility is stated by clock
distance rather than by offset. FR-091: "an eligible future instant has an
admitted clock distance in `[a,b]`". FR-092: "The timestamped finite-window
profile shall quantify admitted past instants by inclusive clock distance."

## Identity and comparison

- **Identity kind** — *value*. An interval has no identity of its own and is
  compared componentwise.
- **Key** — (`a`, `b`, selected profile identity, clock binding). Equality is
  componentwise over exactly these and nothing else. Display name, source path,
  span, message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). The profile
  and binding are key components because FR-090-AC-1 holds that "The same
  predicate values and numeric interval under event-position, fixed-sample and
  timestamped-event selections retain three distinct temporal subjects and
  meanings."
- **Key order** — lexicographic over (`a`, `b`) within one equal profile and
  binding, for collection comparison only. It carries no causal meaning, no
  containment meaning and no precedence; an interval with a smaller lower bound
  is not a causal predecessor of another.
- **Causal order** — none of its own. Where the operator applying the interval
  is order-sensitive, the order used is the admitted causal/sequence order of
  FR-252 (FR-090-AC-4), never derived from key order, timestamp or ingestion
  order.
- **Absence** — an absent bound compares only to absence and is never defaulted
  to zero or to the end of the supplied data. An unbounded interval is not an
  absent bound: FR-091 states that "Where a bounded temporal profile is
  selected, the evaluator shall refuse an unbounded interval", FR-092 states the
  same for an unbounded past interval, and FR-092 refuses implicit history
  reconstruction "under every profile". Under the infinite-trace profile an
  interval operator carries this object with the infinite-trace profile
  identity in its key, and an unbounded operator carries none
  ([FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)).
- **Digest domain** — not applicable; no key component of this object is a
  digest. The profile identity it names carries one, under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Emptiness is not falsity by default

FR-091 states that "Empty existential windows are false and empty universal
windows are true only after the observation authority establishes completeness
through the upper endpoint; before then the result is pending or incomplete."
FR-092 states that "The timestamped finite-window evaluator shall require
complete history over the window before applying empty-window duals", and
FR-094 states that the evaluator "shall not apply false extension or
finite-window empty truth to an incomplete input."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-255-AC-1 | Both endpoints of the bound pair participate, including for a zero-width interval where `a` equals `b`. | Test |
| FR-255-AC-2 | The same numeric bound pair under two profile identities compares unequal. | Test |
| FR-255-AC-3 | An interval is not shortened to the extent of the supplied data. | Test |
| FR-255-AC-4 | An absent bound compares only to absence and is not defaulted to zero. | Test |
| FR-255-AC-5 | The declared key order over intervals is never cited as a causal or containment relation. | Test |
| FR-255-AC-6 | An interval `[a,b]` on an operator under `quire.temporal.infinite-trace/v1` is admitted, keyed with that profile identity, and compares unequal to the same bound pair under a bounded profile. | Test (TC-298) |

## Dependencies

- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) establish the
  inclusive bound form and the empty-window rules.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes that progress covers an inclusive boundary.
- [FR-252](FR-252-temporal-clock-binding.md) supplies the binding under which
  offsets and clock distances are counted.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Cross-family boundary rule

FR-289 preserves both conventions without identifying them: temporal
membership remains inclusive, observation carriers remain half-open, and an
explicit same-clock carrier with an end strictly greater than the inclusive
upper bound supplies observations that are then filtered by the inclusive
predicate. A discrete clock uses checked successor; a non-discrete clock
requires an explicit greater end and never invents epsilon.

## Status

Proposed cross-family reconciliation under FR-289 for
`quire-specification#40`.
