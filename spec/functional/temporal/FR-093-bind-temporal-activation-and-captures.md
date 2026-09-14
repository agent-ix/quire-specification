---
id: FR-093
title: "Bind temporal activation and immutable captures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-034
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-230
    type: references
  - target: ix://agent-ix/quire-specification/FR-231
    type: references
  - target: ix://agent-ix/quire-specification/FR-232
    type: references
  - target: ix://agent-ix/quire-specification/FR-253
    type: references
---
## Description

When a temporal scope activates, the evaluator shall create a distinct
obligation instance initialized with every declared capture evaluated exactly
once at the authored activation anchor.

## Inputs

A linked temporal clause, its scope/trigger definition, one admitted trigger
event or whole-execution origin, typed predicate/capture bindings, and exact
source, model, observation and anchor identities.

## Outputs

An inactive scope, a triggered obligation with immutable typed captures, or a
located incomplete/refused activation result.

## Behavior

Each event-triggered instance shall be identified by the clause subject and
semantic trigger-event identity, not by timestamp, display name or equal payload
alone. Repeated delivery of the same semantic trigger shall not create a second
instance when its admitted payload agrees; conflicting payloads for that one
semantic trigger identity shall produce a typed contradiction/refusal. A distinct
semantic trigger shall create a distinct instance even if its captured values
compare equal.

Each capture shall retain its declared expression, type, value, source/model
binding and activation anchor. Later mutable observations shall not replace the
captured value. A capture that cannot be established shall prevent healthy
activation and shall identify the missing or inconsistent input. One instance's
captures shall not satisfy another instance's propositions.

Activation, temporal truth and participation shall remain separate dimensions.
The evaluator shall create no instance and no refusal when an admitted
activation guard is false. The evaluator shall leave an unestablished guard
incomplete or refused and shall not interpret it as false.
A trigger scope that is authoritatively closed with complete input and without
an admitted trigger shall produce inactive activation; it is not a true obligation and is
not evidence that recovery behavior was exercised. An open trigger scope shall
produce activation-unknown while retaining its independently derived FR-295
completeness state. Missing or refused trigger evidence shall produce
activation-unknown with its exact incomplete completeness or refused
assessment-execution disposition. This requirement does not define protocol
choice/conformance or the observation transport schema.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-093-AC-1 | Two distinct failed-order triggers with equal amounts create distinct obligations, and neither refund can satisfy the other's captured payment identity. | Test (TC-114) |
| FR-093-AC-2 | A duplicate receipt for one semantic trigger retains receipt provenance but creates no duplicate obligation instance. | Test (TC-114) |
| FR-093-AC-3 | Changing mutable source state after activation cannot change an already captured amount, reference or predicate valuation environment. | Test (TC-114) |
| FR-093-AC-4 | Missing, nullable, wrong-type, stale or anchor-mismatched capture input reports incomplete/refused activation before temporal evaluation. | Test (TC-114) |
| FR-093-AC-5 | A closed trigger scope with complete input and no trigger reports inactive; an open scope or missing/refused trigger evidence reports activation-unknown while completeness and execution retain their independently derived exact states, and neither case becomes temporal truth. | Test (TC-116) |
| FR-093-AC-6 | False guards create no obligation, unestablished guards remain incomplete/refused, and conflicting payloads under one semantic trigger identity refuse without replacing retained captures. | Test (TC-114) |

## Dependencies

- [FR-034](../foundation/FR-034-bind-cross-family-predicates.md) owns common predicate
  evaluation environments.
- Agent B owns protocol activation/participation result contracts; Agent F owns
  trigger observation binding and duplicate-delivery provenance.

## Status

Accepted by FS04 for activation, immutable captures and guard distinctions.
Protocol and observation consumers retain their own conformance work.
