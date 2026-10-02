---
id: FR-252
title: "Temporal clock binding"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
---
# FR-252: Temporal clock binding

## Description

When an admitted native temporal clause is bound, the frontend SHALL bind it to one exact clock interpretation before evaluation or lowering: over a trace subject, the binding supplied through the shared observation interface; over a model subject, the `model-steps` or `model-time` binding below.

FR-090 Inputs name the supplied components: "exact language edition and temporal
profile definition identity; typed interval bounds; and one clock binding".

## Model-subject bindings

A temporal claim over a model subject has exactly one of two bindings:

- **`model-steps`** — the model's step sequence is the event-position sequence
  authority, and an interval counts steps. It serves the event-position and
  infinite-trace profiles.
- **`model-time`** — the claim is bound to the model's time source
  ([FR-415](../../functional/temporal/FR-415-declare-clocks-and-timed-models.md)),
  and an interval counts time units. It selects `quire.temporal.timed/v1`. A
  dense source gives each position the sum of the delays before it; a digital
  source (`time tick O period p`) gives it `p` times the number of `O` steps up
  to it. Both give non-decreasing exact rational time stamps in unit `u`, with
  step order as the admitted order.

**Default.** A claim over a timed model binds `model-time` unless it writes
`clock "model-steps"`; a claim over an untimed model binds `model-steps`.
Writing the default changes nothing: the binding enters the obligation
identity, not its spelling. A `model-time` binding over an untimed model
refuses with `invalid_timed_model`/`model-time-on-untimed-model`.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Selected profile identity | FR-090 Behavior | The clock interpretation is meaningful only under one selected profile ([FR-250](FR-250-temporal-semantic-profile.md)). |
| Clock identity | FR-090 Behavior | The timestamped-event profile "shall retain clock identity, unit, admitted order and the completeness premise for every evaluated window" (FR-090). |
| Unit | FR-090 Behavior | For the timestamped-event profile, "One tick in the declared timestamp unit"; retained by the fixed-sample profile as part of "epoch, period and unit identity". |
| Epoch | FR-090 Behavior | Fixed-sample only. The profile "shall retain epoch, period and unit identity". |
| Period | FR-090 Behavior | Fixed-sample only, "the declared positive rational period" ([FR-205](../foundation/FR-205-exact-rational-value.md)). |
| Sequence authority | FR-090 Behavior, FR-090-AC-3 | Event-position only: "One admitted semantic-event position in an authoritative sequence". Changing it changes the binding identity. |
| Admitted order | FR-090 Behavior | Retained by the timestamped-event profile and required by the order-sensitive operators ([FR-254](FR-254-bounded-temporal-operator.md)). |
| Completeness premise | FR-090 Behavior | Retained "for every evaluated window" by the timestamped-event profile. |
| Time source | Model-subject bindings | `model-time` only: the model's qualified name and its time source kind (`dense` or `tick`), with the tick operation for a digital source. |
| Binding kind | Model-subject bindings | Model subject only: `model-steps` or `model-time`. |

The profiles do not carry the same component set. FR-090 states that the
event-position profile "shall not claim elapsed time", so no epoch, period or
elapsed-time component is admitted under it. A `model-time` binding carries
the timed profile identity, the time source, the unit, the period for a
digital source, and step order as sequence authority.

## Identity and comparison

- **Identity kind** — *value*. A clock binding has no identity of its own and is
  compared componentwise. The clock it names is identified; the binding is the
  componentwise selection of that clock under one profile.
- **Key** — the selected profile identity together with the clock identity,
  unit, epoch, period, sequence authority, admitted order, completeness
  premise, binding kind and time source components admitted by that profile. Equality is componentwise over
  exactly those components and nothing else. Display name, source path, span,
  message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). FR-090-AC-3
  fixes the set that changes identity: "Changing a sample period, epoch,
  timestamp unit, sequence authority or closed-boundary rule changes the binding
  identity and cannot reuse an earlier result."
- **Key order** — lexicographic over the key components, for collection
  comparison only. It carries no causal meaning and it is emphatically not an
  admitted order over instants: FR-090 states that "The temporal evaluator shall
  not use timestamp order alone to establish causality or break a tie for an
  order-sensitive operator", and FR-113 states that "Arbitrary timestamp order
  SHALL NOT establish causality."
- **Causal order** — the admitted causal/sequence order carried by the binding
  itself, never derived from the key order, from a timestamp or from ingestion
  order. FR-090 forbids inferring "a profile, unit, period, origin, order or
  boundary rule from backend availability, file shape, ingestion order or a
  similarly spelled historical profile", and FR-090-AC-4 holds that "Equal
  timestamps without an admitted causal/sequence order refuse `until`,
  `release`, `since` and `triggered`; ingestion order is not substituted."
- **Absence** — an absent component compares only to absence. FR-090 requires
  refusal of "a malformed, incompatible or missing fixed-sample clock definition
  or binding before evaluation", and FR-090-AC-2 refuses "Unknown, multiply
  selected or internally inconsistent profile/clock definitions" before
  evaluation. An absent epoch, period, unit, sequence
  authority or admitted order is never defaulted, and an absent runtime sample
  under a valid fixed-sample binding is classified "as incomplete" (FR-090), not
  as a value.
- **Digest domain** — not applicable; no key component is a digest.

## A foreign binding cannot settle

FR-094-AC-4 states that "Progress for a foreign clock, subject, source set or
binding cannot settle the obligation", and FR-113-AC-4 holds that "A mismatched
assertion definition, scope, clock, source set, or restoration basis remains
incomplete/refused and cannot settle or restart an assessment." Binding equality
is therefore a precondition of applying a progress assertion
([FR-256](FR-256-temporal-progress-assertion.md)), not an outcome of it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-252-AC-1 | Changing the sample period, epoch, timestamp unit or sequence authority yields an unequal binding. | Analysis |
| FR-252-AC-2 | An absent admitted order compares only to absence and is not substituted by ingestion order or timestamp order. | Analysis |
| FR-252-AC-3 | The declared key order over binding components is never cited as an admitted order for an order-sensitive operator. | Analysis |
| FR-252-AC-4 | A binding under the event-position profile carries no epoch, period or elapsed-time component. | Inspection |
| FR-252-AC-5 | A binding whose clock identity differs compares unequal regardless of equal unit and epoch. | Analysis |
| FR-252-AC-6 | A claim over a timed model with no `clock` member binds `model-time`, `clock "model-steps"` binds steps, a claim over an untimed model binds `model-steps`, and writing the default leaves the binding and obligation identity unchanged. | Test (TC-368) |
| FR-252-AC-7 | Changing the time source kind, unit or period of a `model-time` binding yields an unequal binding; a `model-time` binding over an untimed model refuses `invalid_timed_model`/`model-time-on-untimed-model`. | Test (TC-368) |

## Dependencies

- [FR-090](../../functional/temporal/FR-090-select-temporal-profile-and-clock.md)
  establishes the clock selection, the per-profile retained components and the
  prohibition on inferring order.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes that a progress assertion applies only to its named clock and
  binding.
- [FR-250](FR-250-temporal-semantic-profile.md) supplies the profile identity.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
