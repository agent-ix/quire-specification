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
  ([FR-009](../../functional/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/FR-047-emit-typed-located-causes.md)). The profile
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
  absent bound: FR-091 states that "The v1 temporal profiles shall refuse
  unbounded intervals", and FR-092 refuses "unbounded past and implicit history
  reconstruction".
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
| FR-255-AC-1 | Both endpoints of the bound pair participate, including for a zero-width interval where `a` equals `b`. | Analysis |
| FR-255-AC-2 | The same numeric bound pair under two profile identities compares unequal. | Analysis |
| FR-255-AC-3 | An interval is not shortened to the extent of the supplied data. | Analysis |
| FR-255-AC-4 | An absent bound compares only to absence and is not defaulted to zero. | Analysis |
| FR-255-AC-5 | The declared key order over intervals is never cited as a causal or containment relation. | Analysis |

## Dependencies

- [FR-091](../../functional/FR-091-evaluate-bounded-future.md) and
  [FR-092](../../functional/FR-092-evaluate-bounded-past.md) establish the
  inclusive bound form and the empty-window rules.
- [FR-094](../../functional/FR-094-interpret-progress-history-and-closure.md)
  establishes that progress covers an inclusive boundary.
- [FR-252](FR-252-temporal-clock-binding.md) supplies the binding under which
  offsets and clock distances are counted.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

The temporal family and the observation family state opposite boundary
conventions for the endpoint at which an eligible member participates, and no
requirement in either family cross-references the other or states a bridging
rule.

The temporal side is inclusive. FR-091-AC-5:

> "Timestamped empty existential/universal window truth is emitted only after
> complete progress through the inclusive upper endpoint; an eligible event
> exactly at the deadline participates before settlement."

FR-094 Behavior: "Progress shall cover an inclusive boundary." FR-094-AC-1:

> "A timestamped response exactly at an inclusive deadline participates before
> progress settles the window; the same response after the deadline does not."

FR-092 Behavior: "The timestamped finite-window profile shall quantify admitted
past instants by inclusive clock distance."

The observation side is half-open.
[FR-112](../../functional/FR-112-declare-population-and-completeness.md)
Behavior:

> "For a selected FCD 1.2.0 window `[start, end)`, the assessment service SHALL
> include an exactly listed/digested member at `start` and exclude one at `end`;
> the latter can participate only in a separately selected later window."

[FR-113](../../functional/FR-113-preserve-time-progress-and-late-data.md)
Behavior:

> "For a selected FCD 1.2.0 half-open window, the observation service SHALL
> apply progress/closure only to admitted members with instants
> `start <= instant < end`."

No requirement states how a native inclusive deadline is covered by a half-open
producer window. The only statement of a bridging rule is in a proposal, not in
a requirement
([observation contract](../../../proposals/quire-v1/observation-contract.md)):

> "Timestamp coverage remains half-open: a witness at native inclusive deadline
> 30 is outside producer `[0,30)` and is admitted only by a separately selected
> range that contains 30."

This artifact records both conventions and the proposal-only bridging rule. It
resolves none of them, and it does not promote the proposal text to a
requirement.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
