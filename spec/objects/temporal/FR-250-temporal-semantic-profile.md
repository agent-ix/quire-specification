---
id: FR-250
title: "Temporal semantic profile identity"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-251
    type: references
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
---
# FR-250: Temporal semantic profile identity

## Description

When an admitted native temporal clause is bound, the reader SHALL resolve its temporal meaning through exactly one of the v1 profile identities listed here rather than through a similarly spelled historical profile or an inferred default.

FR-090 states the selection rule in its Behavior: "The temporal profile shall
select exactly one of these v1 interpretations". The five members below are the
exact identities that table names.

## Values

| Value | Description |
| --- | --- |
| `quire.temporal.event-position.false-extension/v1` | One interval tick means "One admitted semantic-event position in an authoritative sequence"; the closed-boundary rule is "Atomic predicates are false after closure; constants retain their value" (FR-090). This profile "shall not claim elapsed time" (FR-090), and under FR-094 "Event-position time shall not advance during silence merely because wall time passed." |
| `quire.temporal.fixed-sample.false-extension/v1` | One interval tick means "One sample of the declared positive rational period from the declared epoch"; the closed-boundary rule is "Atomic predicates are false after closure; constants retain their value" (FR-090). The profile "shall retain epoch, period and unit identity" and "shall classify an absent runtime sample under a valid binding as incomplete" (FR-090). |
| `quire.temporal.timestamped-event.finite-window/v1` | One interval tick means "One tick in the declared timestamp unit"; the closed-boundary rule is "Quantification ranges over admitted event instants in a complete bounded window" (FR-090). The profile "shall retain clock identity, unit, admitted order and the completeness premise for every evaluated window" (FR-090). |
| `quire.temporal.infinite-trace/v1` | This profile has no closed-boundary rule: an unbounded operator ranges over the unbounded admitted continuation of the event-position sequence, an interval operator reads its closed interval `[a,b]` and a lower-bounded operator its interval `[a,*]` over that continuation, the three nest in any order (FR-367), and no evaluated window ever closes (FR-090, FR-161). It composes the event-position sequence authority named above without its false-extension closure. It "shall not claim elapsed time", matching the event-position profile it extends. FR-161 names the checked infinite-trace subject and the `proved`/`refuted`/`inconclusive`/`unsupported`/`failed` results a selected sound provider returns for it. It admits the unbounded temporal operator grammar edition FR-048 and AD-012 define. |
| `quire.temporal.timed/v1` | One interval tick is an exact rational duration of the clock binding's time unit. Positions are read in step order, each with an exact rational time stamp, over time-divergent behaviours of a timed subject or over a timed trace; no evaluated window closes. Intervals have exact rational bounds with open or closed ends and a finite upper bound (FR-255). It is selected by a `model-time` clock binding (FR-252), admits the unbounded temporal operator grammar edition, and claims elapsed time. [FR-416](../../functional/temporal/FR-416-evaluate-timed-temporal-claims.md) gives its meaning. |

These are the profiles authored and selected in native Quire source. The TL
evaluator profiles named by FR-095 are lowering targets, not members of this
vocabulary; they are recorded separately in
[FR-251](FR-251-tl-target-profile.md). `quire.temporal.infinite-trace/v1` and
`quire.temporal.timed/v1` never lower to a TL target: FR-251 records their
target as absent.

## Identity and comparison

- **Identity kind** — *value*. A profile identity label has no identity of its
  own; it is compared componentwise as an exact string. The selection it labels
  is *authored*: FR-090 requires the frontend to "bind the clause to one exact
  temporal semantic-profile definition ... before evaluation or lowering".
- **Key** — the label itself, matched exactly. Equality is over the label and
  nothing else. Display name,
  source path, span, message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). FR-090
  forbids substituting any other signal: the frontend "shall not infer a
  profile, unit, period, origin, order or boundary rule from backend
  availability, file shape, ingestion order or a similarly spelled historical
  profile."
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence, preference, capability ranking or causal meaning
  between profiles, and it is not an order over anything the profiles evaluate.
- **Causal order** — none. A profile identity is a selection, not a node in the
  authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md).
- **Absence** — a clause with no selected profile has an absent profile
  identity, and absence compares only to absence. FR-090 Outputs require "a
  located typed refusal that names the missing, unknown, inconsistent or
  unsupported dimension", and FR-090-AC-2 holds that "Unknown, multiply
  selected or internally inconsistent profile/clock definitions refuse before
  evaluation". An absent profile is never defaulted to a member.
- **Digest domain** — not applicable; the key carries no digest.

## Meaning change requires a new identity

FR-090 states: "When a temporal meaning changes, the frontend shall require a
new profile definition identity; old profile bytes and results remain historical
subjects." FR-090-AC-3 makes the components explicit: "Changing a sample period,
epoch, timestamp unit, sequence authority or closed-boundary rule changes the
binding identity and cannot reuse an earlier result."

The five members are therefore not five renderings of one meaning. FR-090-AC-1
requires that "The same predicate values and numeric interval under
event-position, fixed-sample and timestamped-event selections retain three
distinct temporal subjects and meanings"; `quire.temporal.infinite-trace/v1`
adds a fourth, since the same numeric interval under it reads an infinite
trace with no closed boundary, and
`quire.temporal.timed/v1` a fifth, since its intervals measure time between
time-stamped positions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-250-AC-1 | Each member's value equals the exact identity string in the table, and a differently spelled or historically similar identity does not match it. | Test |
| FR-250-AC-2 | The same predicate values and numeric interval under two different members compare as two distinct temporal subjects. | Test |
| FR-250-AC-3 | A clause with an absent profile selection compares only to absence and receives no default member. | Test |
| FR-250-AC-5 | No TL evaluator profile appears as a member of this vocabulary. | Test |
| FR-250-AC-6 | `quire.temporal.infinite-trace/v1` has no closed-boundary rule, admits closed-interval operators nested with unbounded ones, and admits the unbounded temporal operator grammar edition, as does `quire.temporal.timed/v1`; the event-position false-extension, fixed-sample and timestamped-event members admit no unbounded operator. | Test (TC-238, TC-298) |
| FR-250-AC-7 | `quire.temporal.timed/v1` is a member, selected only by a `model-time` clock binding, and it never lowers to a TL target. | Test (TC-368) |

## Dependencies

- [FR-090](../../functional/temporal/FR-090-select-temporal-profile-and-clock.md)
  establishes the three v1 interpretations, the selection rule and the
  identity-change rule.
- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) apply the selected
  profile to the bounded operators.
- [FR-251](FR-251-tl-target-profile.md) records the TL lowering targets held
  separate from this vocabulary.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Accepted complete-V1 temporal-profile identity contract. Implementation and
qualification remain tracked separately.
