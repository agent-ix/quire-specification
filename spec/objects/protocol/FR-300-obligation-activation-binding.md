---
id: FR-300
title: "Protocol-obligation activation binding"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-230
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-232
    type: references
  - target: ix://agent-ix/quire-specification/FR-253
    type: references
  - target: ix://agent-ix/quire-specification/FR-260
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-300: Protocol-obligation activation binding

## Description

When an admitted event establishes a protocol obligation, the evaluator SHALL
derive one activation binding from the exact linked protocol control, the exact
admitted trigger observation, and the exact checked activation result. The
binding SHALL be the only producer-to-consumer bridge by which an event-triggered
FR-230 obligation instance is constructed.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Qualified protocol-definition identity | FR-288 | Identifies the linked finite protocol under which the control was selected. |
| Declaration-local control handle | FR-052 | Identifies the authored control occurrence; it is not a runtime subject or an obligation identity by itself. |
| Clause subject | FR-093, FR-230 | Exact subject selected by the control's checked predicate/capture binding. |
| Semantic trigger-event identity | FR-093, FR-260 | The admitted trigger observation identity selected for this activation. |
| Checked activation result | FR-093 | Retains trigger/guard establishment, activation state, immutable captures and the exact evaluation anchor. |

The qualified definition plus declaration-local control handle selects one
authored activation site. The clause subject and semantic trigger-event identity
derive the FR-230 obligation identity. A control handle, a static binding
coordinate, a display name, a source span, a timestamp, an arrival position,
or a capture value alone SHALL NOT substitute for either component of that
instance identity.

## Identity and comparison

- **Identity kind** — *value*. The binding identifies no new protocol subject;
  it carries the inputs that derive an FR-230 instance.
- **Key** — (`FR-288 qualified protocol-definition identity`, declaration-local
  control handle, clause subject, semantic trigger-event identity). Equality is
  componentwise. The FR-230 identity is the contained pair (clause subject,
  semantic trigger-event identity), not this binding's complete key. The checked
  activation state, captures and evaluation anchor are validated retained
  content, not a second invented identity; unequal checked content offered under
  one key is a typed contradiction.
- **Key order** — lexicographic over the complete key for deterministic
  population encoding only. It establishes no causal edge, temporal order or
  preference between instances.
- **Causal order** — the selected control's authored partial order under
  FR-052. Neither binding key order nor the observation's event/ingestion time
  establishes causality.
- **Absence** — an absent trigger, clause subject, checked activation result or
  required capture produces the exact inactive, activation-unknown, incomplete
  or refused outcome the owning activation contract requires. It never produces
  an empty identity, a caller-selected obligation or a successful binding.
- **Digest domain** — every carried definition, observation or activation-result
  digest retains its owner-selected domain. Equal bytes from different domains
  compare unequal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-300-AC-1 | Two bindings with the same qualified definition, control and clause subject but distinct admitted semantic trigger identities derive distinct FR-230 instances even when captures compare equal. | Test (TC-114) |
| FR-300-AC-2 | Replaying the same admitted trigger with agreeing canonical bytes produces no second binding or FR-230 instance; same trigger identity on conflicting bytes refuses. | Test (TC-114) |
| FR-300-AC-3 | Replacing any one of the definition, control handle, clause subject, trigger identity, checked activation result, capture or evaluation anchor refuses or changes the binding exactly as its owner contract requires; none is defaulted. | Test (TC-114) |
| FR-300-AC-4 | A consumer cannot construct a binding or FR-230 instance from a caller-supplied obligation label, static binding coordinate, timestamp, arrival order, display name, span or capture value. | Test (TC-114) |

## Dependencies

- [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md)
  supplies the authored control occurrence and partial order.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  supplies activation, guard, capture and semantic-trigger rules.
- [FR-230](FR-230-protocol-obligation.md) supplies the derived instance identity.
- [FR-260](../observation/FR-260-admitted-observation.md) supplies the admitted
  trigger observation identity.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked definition identity.

## Status

Accepted system contract for the producer-to-consumer activation bridge. A
runtime producer must publish this value through a versioned typed interface;
consumers must refuse its absence or mismatch rather than reconstructing it
from independently supplied static and runtime fragments.
