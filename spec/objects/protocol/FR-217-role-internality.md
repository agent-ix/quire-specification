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
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
---
# FR-217: Role internality vocabulary

## Description

When a protocol declares a role, the protocol linker SHALL apply the same identity rules to internal and external roles, without assuming that an external role is fully observable.

## Values

| Value | Description |
| --- | --- |
| `internal` | Exact lowercase spelling for a bound role inside the assessed protocol. It is subject to the same identity rules as an external role (FR-050). |
| `external` | Exact lowercase spelling for a bound role outside the assessed protocol. It is subject to the same identity rules as an internal role, and is not assumed to be fully observable (FR-050). |

## Identity and comparison

- **Identity kind** — *value*. An internality label has no identity of its own
  and is compared componentwise as an exact kind. It is not part of the identity
  of the role it describes; FR-050 states that "Internal and external
  participants use the same identity rules", so internality changes no identity
  rule.
- **Key** — the internality kind itself. `internal` and `external` are unequal.
  The two lowercase labels above are the closed producer/wire vocabulary.
  Display name, source path, span, message text, locale and rendering never
  participate (FR-035, FR-049-AC-7). The classification is a binding fact: the runtime or
  configuration binding authority supplies it for the FR-211 role it binds. It
  appends no component to the FR-211 role key, is not content of the FR-288
  qualified definition and is not a member of the checked package role (FR-465),
  so changing it changes no definition identity; offering two different
  internalities for one bound role under one binding refuses as contradiction.
  One binding is one input from the binding authority for the bound roles of
  one workflow instance.
- **Key order** — lexicographic over the exact label for canonical collection
  comparison only. It supplies no internality rank, precedence or observability
  relation.
- **Causal order** — not applicable. Internality contributes no causal edge;
  causal order is the authored partial acyclic order of FR-052 and is never
  derived from key order, timestamp or source position (FR-052, FR-052-AC-2).
- **Absence** — every bound role requires exactly one member in its binding. Absence, an
  unknown member or another spelling refuses and is never defaulted to
  `internal`. Neither member alone establishes full observability or adequacy.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-217-AC-1 | An internal role and an external role are bound under the same identity rules, and neither is given a weaker or stronger rule than the other. | Analysis |
| FR-217-AC-2 | An external role is not treated as fully observable. | Analysis |
| FR-217-AC-3 | A role with an absent internality receives no default member and is not thereby treated as fully observable. | Analysis |
| FR-217-AC-4 | `internal` and `external` are the complete exact lowercase vocabulary; an unknown or differently spelled value refuses. | Test (TC-050) |
| FR-217-AC-5 | The binding (one input from the binding authority for the bound roles of one workflow instance) supplies exactly one internality for each static role it binds, as a binding fact rather than a static-definition or checked-package member; it does not extend a role or participant key or the FR-288 identity, and cannot be inferred from a model, component, provider or authority; two different internalities for one role under one binding refuse as contradiction. | Test (TC-050) |

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
  static role; internality is not part of that identity.
- [FR-465](../interfaces/FR-465-checked-package-protocol-declaration-bodies.md)
  keeps the checked package role free of internality.

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

Proposed closure for specification issue #31, with the QSL-1872 ruling:
internality is a binding fact, so a consumer reads it from the binding
authority's input and never from the Spec Language checked package or its read
view, which carry none by design (FR-465-AC-8). Current Spec Language emits no
concrete participant input; its `participant` binding-kind label is only a
static slot tag. Protocol currently derives a participant ID from such a slot.
Both are partial and cannot claim executable evidence for this rule.
