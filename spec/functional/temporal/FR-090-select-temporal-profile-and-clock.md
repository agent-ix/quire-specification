---
id: FR-090
title: "Select an exact temporal profile and clock"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-030
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-specification/FR-252
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-416
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
---
## Description

When admitting a native temporal clause, the frontend shall bind the clause to
one exact temporal semantic-profile definition and one exact clock
interpretation before evaluation or lowering.

## Inputs

An admitted native clause; exact language edition and temporal profile
definition identity; typed interval bounds; and one clock binding, supplied
through the shared observation interface for a trace subject or by the model's
time source for a model subject (FR-252).

## Outputs

A temporal subject carrying the selected profile, clock and interval meaning,
or a located typed refusal that names the missing, unknown, inconsistent or
unsupported dimension.

## Behavior

The temporal profile shall select exactly one of these v1 interpretations:

| Profile identity | One interval tick means | Closed-boundary rule |
| --- | --- | --- |
| `quire.temporal.event-position.false-extension/v1` | One admitted semantic-event position in an authoritative sequence | Atomic predicates are false after closure; constants retain their value |
| `quire.temporal.fixed-sample.false-extension/v1` | One sample of the declared positive rational period from the declared epoch | Atomic predicates are false after closure; constants retain their value |
| `quire.temporal.timestamped-event.finite-window/v1` | One tick in the declared timestamp unit | Quantification ranges over admitted event instants in a complete bounded window |
| `quire.temporal.infinite-trace/v1` | One admitted semantic-event position in an authoritative sequence | Not applicable; the evaluated sequence never closes |
| `quire.temporal.timed/v1` | One exact rational duration of the binding's time unit, measured between time-stamped positions in step order | Not applicable; time-divergent behaviours never close |

The event-position profile shall not claim elapsed time. The infinite-trace
profile composes the event-position sequence authority without its
false-extension closure and likewise shall not claim elapsed time.

The infinite-trace profile shall admit unbounded temporal operators, interval
operators with a closed interval `[a,b]` and lower-bounded operators with an
interval `[a,*]`, nested in any order
([FR-367](FR-367-evaluate-interval-operators-under-infinite-trace.md)). FR-161
defines the checked infinite-trace subject and its sound-provider result
vocabulary; FR-048 and AD-012 define the unbounded temporal operator grammar
it admits. The event-position false-extension, fixed-sample and
timestamped-event profiles admit no unbounded operator.

When the infinite-trace profile governs a refinement declaration
([FR-375](../analysis/FR-375-declare-model-refinement.md)), it requires
`quire.value.complete/v1` and `quire.model.complete/v1` for the refinement's
rows ([FR-250](../../objects/temporal/FR-250-temporal-semantic-profile.md)); the
rows are admitted under exactly those layers and their `requires` closure.

Over a model subject ([FR-361](FR-361-read-model-behaviours-as-temporal-traces.md)),
the authoritative sequence is the subject's step sequence: one tick is one
step of a behaviour, and the clause's clock role binds to that step sequence.
A model subject admits the event-position false-extension and infinite-trace
profiles only.

The timed profile reads positions in step order, each with an exact rational
time stamp, over time-divergent behaviours of a timed subject (FR-415) or over
a timed trace (a finite prefix or lasso of time-stamped positions). Each of
its intervals has exact rational bounds `a <= b` in the binding's unit, each
end closed or open, with `a < b` when an end is open, and a finite upper bound
(FR-255). It admits the unbounded operator grammar edition as well as timed
intervals, and FR-416 gives it its meaning. It is selected by a `model-time`
clock binding over a timed subject (FR-252).

The fixed-sample profile shall retain epoch, period and unit identity.

The fixed-sample profile shall classify an absent runtime sample under a valid
binding as incomplete. The frontend shall refuse a malformed, incompatible or
missing fixed-sample clock definition or binding before evaluation.

The timestamped-event profile shall retain clock identity, unit, admitted order
and the completeness premise for every evaluated window.

The temporal evaluator shall not use timestamp order alone to establish
causality or break a tie for an order-sensitive operator.

The frontend shall not infer a profile, unit, period, origin, order or boundary
rule from backend availability, file shape, ingestion order or a similarly
spelled historical profile.

When a temporal meaning changes, the frontend shall require a new profile
definition identity; old profile bytes and results remain historical subjects.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-090-AC-1 | The same predicate values and numeric interval under event-position, fixed-sample and timestamped-event selections retain three distinct temporal subjects and meanings. | Test (TC-110) |
| FR-090-AC-2 | Unknown, multiply selected or internally inconsistent profile/clock definitions refuse before evaluation and name the affected clause. | Test (TC-110) |
| FR-090-AC-3 | Changing a sample period, epoch, timestamp unit, sequence authority or closed-boundary rule changes the binding identity and cannot reuse an earlier result. | Test (TC-113) |
| FR-090-AC-4 | Equal timestamps without an admitted causal/sequence order refuse `until`, `release`, `since` and `triggered`; ingestion order is not substituted. | Test (TC-113) |
| FR-090-AC-5 | Historical profile definitions remain unchanged and a backend capability request cannot silently select a different executable profile. | Test (TC-117) |
| FR-090-AC-7 | Selecting `quire.temporal.infinite-trace/v1` or `quire.temporal.timed/v1` admits the unbounded temporal operator grammar edition, and `quire.temporal.infinite-trace/v1` also admits interval operators nested with unbounded ones; selecting the event-position false-extension, fixed-sample or timestamped-event profile admits no unbounded operator. | Test (TC-238, TC-368) |
| FR-090-AC-8 | Over a model subject, a clause under the event-position false-extension or infinite-trace profile binds its clock role to the subject's step sequence with one tick per step, and a clause under the fixed-sample or timestamped-event profile settles `unsupported`. | Test (TC-290) |
| FR-090-AC-9 | Selecting `quire.temporal.timed/v1` reads intervals as exact rational time bounds with open or closed ends between time-stamped positions; the same numeric interval under the timed profile and under a step-counting profile retains two distinct meanings. | Test (TC-368) |
| FR-090-AC-10 | A refinement governed by `quire.temporal.infinite-trace/v1` admits `mod` and `Reference` comparison in its rows under the layers the profile requires, whatever the unit's other declarations select, and a row using `protocol Spec::Twice` refuses `unsupported_construct`/`expression-form` at the `protocol` token naming `quire.model.complete/v1`; a plain temporal clause under the profile, outside any refinement, does not newly require `quire.value.complete/v1` and is admitted in a unit that selects no value layer. | Test (TC-310) |

## Dependencies

- [FR-030](../foundation/FR-030-bind-composed-definitions.md) admits the exact language
  edition and temporal definition closure consumed here. FR-020 reports
  composed release capability and is not an admission prerequisite.
- Agent F's observation contract supplies clock bindings and completeness
  assertions; this requirement defines their temporal interpretation, not
  their transport or authority schema.
- [FR-415](FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](FR-416-evaluate-timed-temporal-claims.md) declare timed subjects
  and give the timed profile its meaning.

## Status

Accepted by FS04 for exact temporal profile, clock and boundary selection.
Executable profile coverage remains separately tracked.
