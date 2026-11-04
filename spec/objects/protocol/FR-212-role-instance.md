---
id: FR-212
title: "Role instance"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-035
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-211
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-212: Role instance

## Description

When a protocol reference names a role instance, the protocol linker SHALL keep that role instance distinct from its static role definition, from every component identity and from every ambiguous binding, refusing rather than selecting one.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| typed_declaration_and_configuration_identity | String | 1..1 |  |
| static_definition | ProtocolRole | 1..1 |  |
| instance_binding | String | 1..1 |  |
| distinct_subject | String | 1..1 |  |
| role_mapping | String | 1..1 |  |

## Field sources

| Property | Source | Note |
| --- | --- | --- |
| Typed declaration and configuration identity | FR-050 | "the protocol linker SHALL bind every role, role instance, workflow instance, message and channel reference to its exact typed declaration and configuration identity before conformance evaluation." |
| Static definition | FR-050, [FR-211](FR-211-protocol-role.md) | "The linker SHALL keep role instances and workflow instances distinct from their static definitions." |
| Instance binding | FR-050 | "The linker SHALL refuse duplicate role authorities, ambiguous instance bindings and name-only substitutions." |
| Distinct subject | FR-035 | "Workflow instances, role instances, message deliveries, attempts and effects remain distinct subjects." |
| Role mapping | FR-035-AC-5 | "A missing, repeated or incompatible required role mapping refuses the dependent request; an explicit compatible binding shared by two distinct roles is admitted." |

A component identity supplied in a role-instance position refuses: "Substituting
an endpoint or channel identity for a role, or a component identity for a role
instance, refuses linking with the affected source location" (FR-050-AC-2).

## Identity and comparison

- **Identity kind** — *authored*. A role instance is bound through an explicit
  authored binding, not computed: FR-050 takes "explicit workflow-instance
  bindings" as an input and refuses ambiguous instance bindings, and FR-035
  requires an "explicit compatible binding" (FR-035-AC-5). It is not *derived*,
  because no requirement states a rule that computes it from its components.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `role-instance`. Its workflow instance, static role slot and
  configuration binding are separately validated relationships, not key
  components. Reusing one key with different relationship facts refuses.
- **Key order** — FR-287's lexicographic order. It is independent from protocol
  role-table order and carries no role priority or causal meaning.
- **Causal order** — not carried by a role instance. Where a role instance
  participates in protocol control, the applicable order is the authored partial
  acyclic order of FR-052, never derived from key order, timestamp or source
  position (FR-052, FR-052-AC-2).
- **Absence** — an absent key component compares only to absence. An
  unresolved, duplicate or ambiguous role/instance reference "prevents dependent
  evaluation without removing independent valid definitions" (FR-050-AC-4), and
  a missing required role mapping refuses the dependent request rather than
  binding a default (FR-035-AC-5).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-212-AC-1 | A role instance and its static role definition are not equal, and neither resolves in the other's position. | Analysis |
| FR-212-AC-2 | A component identity supplied for a role instance does not resolve. | Analysis |
| FR-212-AC-3 | An ambiguous instance binding yields no selection, and the dependent evaluation does not proceed while independent valid definitions remain. | Analysis |
| FR-212-AC-4 | Equal display or participant/provider identity does not merge role instances; equality and canonical collection order use only the FR-287 key. | Analysis |

## Dependencies

- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) separates role
  instances from static definitions and refuses ambiguous bindings.
- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md) keeps role
  instances distinct subjects and governs role mappings.
- [FR-211](FR-211-protocol-role.md) is the static definition a role instance
  instantiates.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete runtime key and its non-causal comparison order.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. Specification issue #28 fixed the key, relationship boundary and
canonical comparison order. Executable cross-repository evidence remains
planned.
