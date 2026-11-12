---
id: FR-211
title: "Protocol role"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-211: Protocol role

## Description

When a protocol declaration names a role, the protocol linker SHALL bind that role to its exact typed declaration and keep it distinct from every component, endpoint and channel rather than resolving it by name.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| typed_declaration | String | 1..1 |  |
| declaration_kind | String | 1..1 |  |
| authority | String | 1..1 |  |
| choice_ownership | String | 1..1 |  |
| declaration_span | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| typed_declaration | type not stated in source; see Field sources |
| declaration_kind | type not stated in source; see Field sources |
| authority | type not stated in source; see Field sources |
| choice_ownership | type not stated in source; multiplicity not stated in source; see Field sources |
| declaration_span | type not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Typed declaration | FR-050 | "the protocol linker SHALL bind every role, role instance, workflow instance, message and channel reference to its exact typed declaration and configuration identity before conformance evaluation." |
| Declaration kind | FR-049-AC-1 | "Role, channel and relationship positions bind their required declaration kinds; substituting a payload type, endpoint or scalar identity refuses at its source location." |
| Authority | FR-050 | "The linker SHALL refuse duplicate role authorities, ambiguous instance bindings and name-only substitutions." |
| Choice ownership | FR-053 | A labeled choice requires "one owning role" and evidence the owner can distinguish every selected branch (FR-053). |
| Declaration span | FR-049-AC-11 | "Every lowered source node, including synthesized control edges and bounded occurrences, retains its declaration span and exact derivation/iteration identity." |

A static role carries no internality. Internality is a binding fact supplied
for the role by the runtime or configuration binding authority
([FR-217](FR-217-role-internality.md)); it is not a property of the role, is not
authored in the source and is not spelled by the checked package (FR-465).

A role is not a component, an endpoint or a channel: "The linker SHALL keep roles
distinct from components, endpoints and channels" (FR-050), and "A queue or
endpoint SHALL remain a channel/component reference rather than becoming a
participant role" (FR-054).

## Identity and comparison

- **Identity kind** — *derived*. The role is authored, while its identity is
  derived from the exact linked definition and admitted declaration-local
  handle.
- **Key** — (`qualified protocol-definition identity`, `declaration-local role
  handle`), exactly as FR-050 now states. The current wire represents the handle
  as a table index qualified by its admitted package and declaration; an
  unqualified index, display name or source span is not the key.
- **Key order** — lexicographic over the two components in the declared
  sequence. It exists only for stable comparison and diffing, ranks no role and
  supplies no causal order.
- **Causal order** — not carried by a role. Causal order belongs to the
  authored partial acyclic graph of FR-052, which admits no edge derived from
  source order or timestamps (FR-052, FR-052-AC-2). A role's position in source
  is not a causal fact.
- **Absence** — an absent key component compares only to absence. An
  unresolved, duplicate or ambiguous role reference "prevents dependent
  evaluation without removing independent valid definitions" (FR-050-AC-4); it
  is never defaulted to an empty name or to an implicit role.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-211-AC-1 | A role reference resolves to a typed role declaration, and an endpoint, channel or component identity supplied in a role position does not resolve. | Analysis |
| FR-211-AC-2 | Two roles bearing equal textual names are not thereby equal, and two roles claiming one authority do not both resolve. | Analysis |
| FR-211-AC-3 | Two roles compare equal exactly when their qualified definition and declaration-local role handle compare equal; display name, locus and shared participant/provider binding do not participate. | Analysis |
| FR-211-AC-4 | An absent role key component compares only to absence and receives no default. | Analysis |

## Dependencies

- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds roles to
  exact typed declarations and separates them from components, endpoints and
  channels.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) binds role
  positions to their required declaration kinds and retains declaration spans.
- [FR-053](../../functional/protocol/FR-053-enforce-choice-visibility.md) requires one
  owning role per labeled choice.
- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) forbids a queue or
  endpoint becoming a participant role.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the
  qualified linked-definition component.

## Status

Draft. Specification issue #28 fixed the key and comparison rule. Executable
cross-repository identity evidence remains planned.
