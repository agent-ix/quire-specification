---
id: FR-217
title: "Role internality vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
---
# FR-217: Role internality vocabulary

## Description

When a protocol declares a role, the protocol linker SHALL apply the same identity rules to internal and external roles, without assuming that an external role is fully observable.

## Values

| Value | Description |
| --- | --- |
| `internal` | A role inside the assessed protocol, such as the order, inventory and fulfillment roles of FR-050-AC-1. It is subject to the same identity rules as an external role (FR-050). |
| `external` | A role outside the assessed protocol, such as the external payment role of FR-050-AC-1. It is subject to the same identity rules as an internal role, and "an external role is not assumed to be fully observable" (FR-050). |

## Identity and comparison

- **Identity kind** — *value*. An internality label has no identity of its own
  and is compared componentwise as an exact kind. It is not part of the identity
  of the role it describes; FR-050 states that "Internal and external
  participants use the same identity rules", so internality changes no identity
  rule.
- **Key** — the internality kind itself. `internal` and `external` are unequal.
  Display name, source path, span, message text, locale and rendering never
  participate (FR-035, FR-049-AC-7). No requirement fixes a spelling for either
  kind; the labels above name the distinction FR-050 and FR-050-AC-1 draw and
  are not established wire strings.
- **Key order** — none. No requirement establishes an order, precedence or
  ordinal between an internal and an external role, so none is supplied here.
- **Causal order** — not applicable. Internality contributes no causal edge;
  causal order is the authored partial acyclic order of FR-052 and is never
  derived from key order, timestamp or source position (FR-052, FR-052-AC-2).
- **Absence** — a role whose internality is absent compares only to absence. It
  is never defaulted to `internal`, and absence never implies full
  observability, because FR-050 withholds that assumption for external roles
  without granting it anywhere else.
- **Digest domain** — no member of this key is a digest. Where an internality
  label is carried beside a digest in provenance, that digest's domain is
  selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-217-AC-1 | An internal role and an external role are bound under the same identity rules, and neither is given a weaker or stronger rule than the other. | Analysis |
| FR-217-AC-2 | An external role is not treated as fully observable. | Analysis |
| FR-217-AC-3 | A role with an absent internality receives no default member and is not thereby treated as fully observable. | Analysis |

## Dependencies

- [FR-050](../../functional/FR-050-bind-protocol-instances.md) establishes the
  internal/external distinction, the shared identity rules and the observability
  limit.
- [FR-211](FR-211-protocol-role.md) is the role this vocabulary classifies.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-050 draws this distinction over "participants" in one sentence and over
"roles" in the adjacent sentence and in FR-050-AC-1:

- FR-050 behavior: "Internal and external participants use the same identity
  rules; an external role is not assumed to be fully observable."
- FR-050-AC-1: "A protocol with internal order, inventory and fulfillment roles
  plus an external payment role retains every definition and instance identity
  independently."

FR-054 uses "participant" for a third thing: "A queue or endpoint SHALL remain a
channel/component reference rather than becoming a participant role." No
requirement states whether a participant is a role, a role instance, or a
distinct kind, so this artifact classifies roles only and leaves `participant`
untyped.
