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
---
# FR-212: Role instance

## Description

When a protocol reference names a role instance, the protocol linker SHALL keep that role instance distinct from its static role definition, from every component identity and from every ambiguous binding, refusing rather than selecting one.

## Properties

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
- **Key** — **the prose does not fix the key components.** FR-050 requires
  binding to an "exact typed declaration and configuration identity", but it
  does not state whether the configuration identity, the role declaration, the
  workflow instance, or some combination constitutes the key. Established
  exclusions: a component identity cannot substitute (FR-050-AC-2), a name alone
  cannot substitute (FR-050), and the static role definition is not the instance
  (FR-050). Equality is componentwise over whatever the key is and nothing else;
  display name, source path, span, message text, locale and rendering never
  participate (FR-035, FR-049-AC-7).
- **Key order** — none. The key components are not fixed by the prose, so no
  total order over the key is stated and none is supplied here.
- **Causal order** — not carried by a role instance. Where a role instance
  participates in protocol control, the applicable order is the authored partial
  acyclic order of FR-052, never derived from key order, timestamp or source
  position (FR-052, FR-052-AC-2).
- **Absence** — an absent key component compares only to absence. An
  unresolved, duplicate or ambiguous role/instance reference "prevents dependent
  evaluation without removing independent valid definitions" (FR-050-AC-4), and
  a missing required role mapping refuses the dependent request rather than
  binding a default (FR-035-AC-5).
- **Digest domain** — no component named by the prose is a digest. Where a role
  instance identity is carried beside a digest in provenance, that digest's
  domain is selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-212-AC-1 | A role instance and its static role definition are not equal, and neither resolves in the other's position. | Analysis |
| FR-212-AC-2 | A component identity supplied for a role instance does not resolve. | Analysis |
| FR-212-AC-3 | An ambiguous instance binding yields no selection, and the dependent evaluation does not proceed while independent valid definitions remain. | Analysis |
| FR-212-AC-4 | This artifact declares no key components and no key order for a role instance, and cites the prose exclusions instead. | Inspection |

## Dependencies

- [FR-050](../../functional/FR-050-bind-protocol-instances.md) separates role
  instances from static definitions and refuses ambiguous bindings.
- [FR-035](../../functional/FR-035-bind-ecosystem-subjects.md) keeps role
  instances distinct subjects and governs role mappings.
- [FR-211](FR-211-protocol-role.md) is the static definition a role instance
  instantiates.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement states what constitutes a role instance's identity.

1. Whether the key includes the workflow instance is unstated. FR-050-AC-3
   establishes that two concurrent workflow instances "sharing one provider
   remain distinct assessment subjects", but it does not say that a role
   instance is scoped by a workflow instance.
2. Whether the key includes the configuration identity that FR-050 requires it
   to be bound to is unstated.
3. FR-035-AC-5 admits "an explicit compatible binding shared by two distinct
   roles". Whether that shared binding yields one role instance or two is
   unstated.
4. No key order over role instances can be established until the key components
   are.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
