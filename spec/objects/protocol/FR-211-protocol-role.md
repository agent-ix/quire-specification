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
---
# FR-211: Protocol role

## Description

When a protocol declaration names a role, the protocol linker SHALL bind that role to its exact typed declaration and keep it distinct from every component, endpoint and channel rather than resolving it by name.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Typed declaration | FR-050 | "the protocol linker SHALL bind every role, role instance, workflow instance, message and channel reference to its exact typed declaration and configuration identity before conformance evaluation." |
| Declaration kind | FR-049-AC-1 | "Role, channel and relationship positions bind their required declaration kinds; substituting a payload type, endpoint or scalar identity refuses at its source location." |
| Authority | FR-050 | "The linker SHALL refuse duplicate role authorities, ambiguous instance bindings and name-only substitutions." |
| Internality | FR-050, [FR-217](FR-217-role-internality.md) | Internal or external; "an external role is not assumed to be fully observable" (FR-050). |
| Choice ownership | FR-053 | A labeled choice requires "one owning role" and evidence the owner can distinguish every selected branch (FR-053). |
| Declaration span | FR-049-AC-11 | "Every lowered source node, including synthesized control edges and bounded occurrences, retains its declaration span and exact derivation/iteration identity." |

A role is not a component, an endpoint or a channel: "The linker SHALL keep roles
distinct from components, endpoints and channels" (FR-050), and "A queue or
endpoint SHALL remain a channel/component reference rather than becoming a
participant role" (FR-054).

## Identity and comparison

- **Identity kind** — *authored*. A role is written in native source and bound
  to its exact typed declaration (FR-050); every lowered source node retains its
  declaration span (FR-049-AC-11). It is not computed from its own components
  and it is not a value.
- **Key** — **the prose does not fix the key components.** FR-050 establishes
  only exclusions: a name alone is not the key, because the linker "SHALL refuse
  ... name-only substitutions" (FR-050); an endpoint or channel identity
  substituted for a role refuses (FR-050-AC-2); and two roles may not share one
  authority, because duplicate role authorities refuse (FR-050). Whatever the
  key is, equality is componentwise over it and nothing else: display name,
  source path, span, message text, locale and rendering never participate
  (FR-035, FR-049-AC-7). This artifact supplies no key components of its own.
- **Key order** — none. Because the prose does not fix the key components, no
  total order over the key can be stated without inventing one, and none is
  stated here. A collection of roles therefore has no declared comparison order.
- **Causal order** — not carried by a role. Causal order belongs to the
  authored partial acyclic graph of FR-052, which admits no edge derived from
  source order or timestamps (FR-052, FR-052-AC-2). A role's position in source
  is not a causal fact.
- **Absence** — an absent key component compares only to absence. An
  unresolved, duplicate or ambiguous role reference "prevents dependent
  evaluation without removing independent valid definitions" (FR-050-AC-4); it
  is never defaulted to an empty name or to an implicit role.
- **Digest domain** — no component named by the prose is a digest. Where a role
  identity is carried beside a digest in provenance, that digest's domain is
  selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-211-AC-1 | A role reference resolves to a typed role declaration, and an endpoint, channel or component identity supplied in a role position does not resolve. | Analysis |
| FR-211-AC-2 | Two roles bearing equal textual names are not thereby equal, and two roles claiming one authority do not both resolve. | Analysis |
| FR-211-AC-3 | This artifact declares no key components and no key order for a role, and cites the prose exclusions instead. | Inspection |
| FR-211-AC-4 | An absent role key component compares only to absence and receives no default. | Analysis |

## Dependencies

- [FR-050](../../functional/FR-050-bind-protocol-instances.md) binds roles to
  exact typed declarations and separates them from components, endpoints and
  channels.
- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) binds role
  positions to their required declaration kinds and retains declaration spans.
- [FR-053](../../functional/FR-053-enforce-choice-visibility.md) requires one
  owning role per labeled choice.
- [FR-054](../../functional/FR-054-bind-channel-premises.md) forbids a queue or
  endpoint becoming a participant role.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

No requirement states what constitutes a role's identity. The prose establishes
that a name alone does not (FR-050), that an endpoint, channel or component
identity cannot substitute (FR-050-AC-2), and that a duplicate authority refuses
(FR-050) — but it never names the components of the key. Consequently:

1. Whether a role's key includes its declaring package, protocol definition,
   language edition or profile identity is unstated.
2. Whether a role's key includes its authority is unstated; only that duplicate
   authorities refuse.
3. No key order over roles can be established until the key components are.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
