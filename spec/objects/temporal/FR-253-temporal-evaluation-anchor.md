---
id: FR-253
title: "Temporal evaluation anchor"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-252
    type: references
  - target: ix://agent-ix/quire-specification/FR-244
    type: references
---
# FR-253: Temporal evaluation anchor

## Description

When a bounded temporal operator is evaluated, the evaluator SHALL apply its interval offsets relative to one admitted evaluation anchor rather than to a position recovered from file order or wall time.

FR-091 Behavior fixes the future form: "At anchor `i`, for inclusive integers
`0 <= a <= b`". FR-092 fixes the past form over the same anchor, reading
positions `i-d`. FR-093 names the authored activation anchor at which every
declared capture is evaluated exactly once.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| anchor_identity | String | 1..1 |  |
| clock_binding | String | 1..1 |  |
| position_sample_index_or_instant | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| anchor_identity | type not stated in source; see Field sources |
| clock_binding | type not stated in source; see Field sources |
| position_sample_index_or_instant | type not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Anchor identity | FR-090 Outputs, FR-093 Inputs | FR-093 Inputs require "exact source, model, observation and anchor identities". |
| Clock binding | FR-090, FR-094 Inputs | The anchor position is expressed under one clock binding ([FR-252](FR-252-temporal-clock-binding.md)). |
| Position, sample index or instant | FR-090 Behavior | Its meaning is the selected profile's tick: an admitted semantic-event position, a declared sample, or a tick in the declared timestamp unit. |

FR-095 requires a supported lowering to preserve the "evaluation anchor" as one
of its named correspondence axes, and FR-095-AC-4 holds that mutating that axis
"breaks correspondence and names that axis".

## Identity and comparison

- **Identity kind** — *value*. An anchor has no identity of its own and is
  compared componentwise; the identities it carries are those of the clause,
  binding and clock that produced it.
- **Key** — the anchor identity together with its clock binding and its
  position, sample index or instant under that binding. Equality is
  componentwise over exactly those components and nothing else. Display name,
  source path, span, message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). FR-093 is
  explicit that a timestamp is not an identity component of what the anchor
  activates: "Each event-triggered instance shall be identified by the clause
  subject and semantic trigger-event identity, not by timestamp, display name or
  equal payload alone."
- **Key order** — for two anchors under one equal clock binding, the order over
  the position, sample index or instant component, used only to compare and diff
  collections of anchors. It carries no causal meaning. FR-090 states that "The
  temporal evaluator shall not use timestamp order alone to establish causality
  or break a tie for an order-sensitive operator", so this order is never
  presented as an admitted order and never breaks a tie.
- **Causal order** — the admitted causal/sequence order carried by the clock
  binding (FR-252), and for a protocol subject the authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). It is never
  derived from the key order, from a timestamp or from ingestion order.
  FR-090-AC-4 holds that "Equal timestamps without an admitted causal/sequence
  order refuse `until`, `release`, `since` and `triggered`; ingestion order is
  not substituted."
- **Absence** — an absent anchor component compares only to absence. FR-093-AC-4
  holds that "Missing, nullable, wrong-type, stale or anchor-mismatched capture
  input reports incomplete/refused activation before temporal evaluation", and
  FR-055-AC-4 refuses a binding under a different evaluation anchor. An absent
  anchor is never defaulted to position zero, to the first record, or to the
  earliest timestamp.

## The anchor is not the history boundary

FR-092 holds the anchor and the origin apart: past offsets are read at `i-d`
from the anchor, while whether those positions exist is decided by the history
boundary ([FR-257](FR-257-history-boundary.md)). FR-092 states that "A supplied
history cutoff is not an execution origin."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-253-AC-1 | Two anchors under different clock bindings compare unequal regardless of equal position values. | Test |
| FR-253-AC-2 | The declared key order over anchors is never cited to break a tie for an order-sensitive operator. | Test |
| FR-253-AC-3 | An absent anchor compares only to absence and is not represented as position zero. | Test |
| FR-253-AC-4 | A timestamp is absent from the declared key of the instance an anchor activates. | Test |

## Dependencies

- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) establish the
  anchor-relative offset readings.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  establishes the authored activation anchor and the anchor identity inputs.
- [FR-252](FR-252-temporal-clock-binding.md) supplies the clock binding and the
  admitted causal order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Activation vocabulary

FR-093, FR-055 and FR-061 now use the same source state `triggered`. FR-062's
`activated` spelling remains a target projection label and is never admitted as
the source activation state attached to this anchor.

## Status

Proposed activation-spelling reconciliation for `quire-specification#40`.
