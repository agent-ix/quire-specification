---
id: FR-217
title: "Role internality vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---
# FR-217: Role internality vocabulary

## Description

When a protocol declares a role, the protocol linker SHALL apply the same identity rules to internal and external roles, without assuming that an external role is fully observable.

## Values

| Value | Description |
| --- | --- |
| `internal` | Exact lowercase spelling for a static role inside the assessed protocol. It is subject to the same identity rules as an external role (FR-050). |
| `external` | Exact lowercase spelling for a static role outside the assessed protocol. It is subject to the same identity rules as an internal role, and is not assumed to be fully observable (FR-050). |

## Identity and comparison

- **Identity kind** — *value*. An internality label has no identity of its own
  and is compared componentwise as an exact kind. It is not part of the identity
  of the role it describes; FR-050 states that "Internal and external
  participants use the same identity rules", so internality changes no identity
  rule.
- **Key** — the internality kind itself. `internal` and `external` are unequal.
  The two lowercase labels above are the closed producer/wire vocabulary.
  Display name, source path, span, message text, locale and rendering never
  participate (FR-035, FR-049-AC-7). The classification does not append a new
  component to the FR-211 role key, but it is semantic consistency content of
  the qualified static definition: changing it requires another FR-288, and
  offering different internality under one FR-288 refuses as contradiction.
- **Key order** — lexicographic over the exact label for canonical collection
  comparison only. It supplies no internality rank, precedence or observability
  relation.
- **Causal order** — not applicable. Internality contributes no causal edge;
  causal order is the authored partial acyclic order of FR-052 and is never
  derived from key order, timestamp or source position (FR-052, FR-052-AC-2).
- **Absence** — every static role requires exactly one member. Absence, an
  unknown member or another spelling refuses and is never defaulted to
  `internal`. Neither member alone establishes full observability or adequacy.
- **Digest domain** — no member of this key is a digest. Where an internality
  label is carried beside a digest in provenance, that digest's domain is
  selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-217-AC-1 | An internal role and an external role are bound under the same identity rules, and neither is given a weaker or stronger rule than the other. | Analysis |
| FR-217-AC-2 | An external role is not treated as fully observable. | Analysis |
| FR-217-AC-3 | A role with an absent internality receives no default member and is not thereby treated as fully observable. | Analysis |
| FR-217-AC-4 | `internal` and `external` are the complete exact lowercase vocabulary; an unknown or differently spelled value refuses. | Test (TC-050) |
| FR-217-AC-5 | Exactly one internality member classifies each static role, does not independently extend a role or participant key, and cannot be inferred from a model, component, provider or authority; a changed member requires another FR-288. | Test (TC-050) |

## Dependencies

- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) establishes the
  internal/external distinction, the shared identity rules and the observability
  limit.
- [FR-211](FR-211-protocol-role.md) is the role this vocabulary classifies.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) supplies the
  concrete participant key.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) qualifies the
  static role and its internality consistency content.

## Participant binding boundary

A participant is the distinct concrete principal/provider selected by a
configuration or runtime binding authority. It is neither this static role nor
the role instance that the participant fills. Its key is one FR-287 value of
kind `participant`; the static binding requirement identifies only the slot and
cannot substitute for that concrete key. Participant-to-role-instance and
participant-to-component facts are separately validated relationships rather
than participant-key members. One participant may compatibly fill multiple
role instances without merging them. Missing, empty, wrong-kind or foreign
participant input refuses, as does reusing one participant key with conflicting
binding facts.

## Status

Proposed closure for specification issue #31. Current Spec Language has no
internality field and emits no concrete participant input; its `participant`
binding-kind label is only a static slot tag. Protocol currently derives a
participant ID from such a slot. Both are partial and cannot claim executable
evidence for this rule; a versioned producer/binding contract must carry the
required values rather than defaulting old packages.
