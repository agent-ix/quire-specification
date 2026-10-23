---
id: FR-478
title: "Protocol-obligation activation binding"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-464
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-093
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-230
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/FR-213
    type: references
  - target: ix://agent-ix/quire-specification/FR-232
    type: references
  - target: ix://agent-ix/quire-specification/FR-260
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-478: Protocol-obligation activation binding

## Description

When an admitted activation of a protocol establishes an obligation of a
temporal clause the protocol requires, the evaluator SHALL derive one
activation binding from the exact linked protocol definition, the exact row of
the protocol's activation relation
([FR-464](../interfaces/FR-464-checked-package-protocol-node-bodies.md)
"Activation relation"), the workflow instance, the clause subject, the exact
origin of the activation and the exact checked activation result
([FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)).
The binding SHALL be the only producer-to-consumer bridge by which an
[FR-230](FR-230-protocol-obligation.md) obligation instance is constructed.
A consumer SHALL NOT construct a binding or an instance from a control or
temporal clause position, a name, a capture value, a clock, a timestamp, a
static binding coordinate or an observation arrival order, and SHALL NOT take
the relation row from any carrier other than the admitted package.

## Properties

- **qualified protocol-definition identity** — the linked finite protocol
  whose activation established the obligation (FR-288).
- **activation relation row** — the `protocol_clause` node key, the
  activation (`origin`, or `each` with its trigger node key) and the required
  `temporal_clause` node key (FR-464 "Activation relation"). It selects one
  authored activation site and the temporal clause whose obligation is
  established.
- **workflow instance** — the workflow execution the activation belongs to
  (FR-213).
- **clause subject** — the exact subject selected by the activation's checked
  predicate and capture binding (FR-093, FR-230).
- **origin** — `whole_execution` with the workflow execution identity for an
  `origin` row; `trigger_event` with the admitted semantic trigger-event
  identity for an `each` row (FR-093, FR-230, FR-260).
- **checked activation result** — the trigger and guard establishment, the
  activation state, the immutable captures and the exact evaluation anchor
  (FR-093, FR-232).

The FR-230 instance key is derived from the binding: `package` and
`protocol definition` from the qualified definition, `workflow instance` from
the workflow instance, `control occurrence` from the activation relation row
(its clause key and temporal clause key), `clause subject` from the clause
subject, and `origin kind` and `origin identity` from the origin. No
component of that key is taken from anywhere else.

## Identity and comparison

- **Identity kind** — *value*. The binding identifies no new protocol subject;
  it carries the inputs from which an FR-230 instance is derived.
- **Key** — (qualified protocol-definition identity, activation relation row,
  workflow instance, clause subject, origin kind, origin identity). Equality is
  componentwise. The checked activation result is validated retained content,
  not a key component.
- **Key order** — lexicographic canonical byte order over the complete key,
  for deterministic population encoding only. It establishes no causal edge,
  temporal order or preference between bindings.
- **Causal order** — the authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md).
  Neither the key order nor an observation's event or ingestion time
  establishes causality.
- **Absence** — an `each` row with an absent trigger, an absent clause subject,
  an absent checked activation result or an absent required capture produces
  the inactive, activation-unknown, incomplete or refused outcome FR-093
  requires, never an empty identity, a caller-selected obligation or a
  successful binding. An `origin` row carries the workflow execution identity
  as its origin identity, never an absent trigger placeholder (FR-230-AC-4).
- **Digest domain** — every carried definition, observation or
  activation-result digest retains its owner-selected domain
  ([FR-201](../foundation/FR-201-canonical-identity-domain.md)). Equal bytes
  from different domains compare unequal.

## Refusals

| Defect | Refusal |
| --- | --- |
| The activation relation row is not a row of the admitted package's relation for the qualified definition: a row of another definition or package, a clause key or temporal clause key the relation does not hold, or a row assembled by the consumer | `missing_declaration`/`missing-name`, naming the row's clause key and temporal clause key |
| The origin kind does not match the row's activation: `trigger_event` on an `origin` row, `whole_execution` on an `each` row | `invalid_runtime_input`/`conflicting-identity`, naming the row and the origin kind |
| A required component (definition, row, workflow instance, clause subject, origin identity, checked activation result) is absent | `invalid_runtime_input`/`missing-member`, naming the component, with no binding and no instance |
| The same key offered with unequal checked activation content, or the same semantic trigger identity with conflicting admitted bytes | `invalid_runtime_input`/`conflicting-identity`, naming the key, with neither offer selected |
| A carried digest whose domain differs from its owner-selected domain | `stale_dependency`/`digest-domain-mismatch` |

A refused binding constructs no FR-230 instance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-478-AC-1 | Two bindings with the same definition, row, workflow instance and clause subject but distinct admitted semantic trigger identities derive distinct FR-230 instances even when their captures compare equal. | Test (TC-114) |
| FR-478-AC-2 | Replaying the same admitted trigger with agreeing canonical bytes produces no second binding and no second FR-230 instance; the same trigger identity on conflicting bytes refuses `invalid_runtime_input`/`conflicting-identity`. | Test (TC-114) |
| FR-478-AC-3 | Replacing any one of the definition, row, workflow instance, clause subject, origin, checked activation result, capture or evaluation anchor changes the binding's key or refuses exactly as the Refusals table states; none is defaulted. | Test (TC-114) |
| FR-478-AC-4 | A consumer cannot construct a binding or an FR-230 instance from a caller-supplied obligation label, a static binding coordinate, a timestamp, an arrival order, a display name, a span or a capture value: a binding whose row the admitted package's activation relation does not hold refuses `missing_declaration`/`missing-name`. | Test (TC-114) |
| FR-478-AC-5 | A protocol activation consumes a required temporal clause only through its FR-464 activation relation row; omitting the row, cross-wiring it to another clause's or another temporal clause's row, or inferring it from a shared clock or name refuses rather than selecting an obligation; an `origin` row's binding carries the workflow execution identity and a `trigger_event` origin on it refuses `invalid_runtime_input`/`conflicting-identity`. | Test (TC-114) |

## Dependencies

- [FR-464](../interfaces/FR-464-checked-package-protocol-node-bodies.md)
  supplies the activation relation and its rows.
- [FR-093](../../functional/temporal/FR-093-bind-temporal-activation-and-captures.md)
  supplies activation, guard, capture and semantic-trigger rules.
- [FR-230](FR-230-protocol-obligation.md) supplies the derived instance key.
- [FR-260](../observation/FR-260-admitted-observation.md) supplies the admitted
  trigger observation identity.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked definition identity, and
  [FR-213](FR-213-workflow-instance.md) the workflow instance.
