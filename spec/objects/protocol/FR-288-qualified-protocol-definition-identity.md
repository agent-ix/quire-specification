---
id: FR-288
title: "Qualified protocol-definition identity"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-030
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-202
    type: references
---
# FR-288: Qualified protocol-definition identity

## Description

When a linked protocol definition is used as an identity qualifier, the linker SHALL retain its exact admitted source, declaration ownership and semantic dependency selection rather than identifying it by a declaration name, source span or package-local table index alone.

## Properties

- **authored source document key** — the exact FR-202 document identity
  of the source owning the declaration.
- **admitted declaration handle** — the source-inventory-qualified handle of
  the protocol declaration. A wire table index is a local representation of
  this handle and is comparable only inside the same admitted package.
- **formal owner and clause** — the exact requirement package, requirement
  identity, requirement revision and clause identity retained by FR-049.
- **semantic selection closure** — the exact language edition, protocol profile
  definition identity, domain package selections (identity and version) retained by FR-030. Resource-only execution
  controls and runtime observations are excluded.

These properties form a structured key. This artifact does not prescribe a hash or permit a package artifact digest to stand in
for any member. Implementations may retain compact package-local handles, but an
exchanged handle is incomplete unless its admitted package selection supplies
every qualifier above.

## Identity and comparison

- **Identity kind** — *derived*. Linking derives this value from exact authored
  and selected inputs; a source declaration name does not author it.
- **Key** — (`authored source document key`, `admitted declaration handle`,
  `formal owner and clause`, `semantic selection closure`). Equality is
  componentwise over exactly these four structured members.
- **Key order** — lexicographic over the components in the declared sequence
  and over each finite closure in its admitted canonical order. It exists only
  for stable collection comparison.
- **Causal order** — none. The key order SHALL NOT establish source, control,
  delivery, retry or temporal order.
- **Absence** — every component and every dependency required by the selected
  closure is present. Missing, ambiguous, foreign or conflicting qualifiers
  refuse the dependent definition; none is defaulted from a current package,
  ambient profile, declaration name or table position.
- **Digest domain** — not applicable; the key carries no digest.

Display names, paths, spans, build provenance, runtime configuration values and
observations are not standalone key members and cannot substitute for one.
Changing external rendering metadata while preserving the authored source key
changes no key member; editing a display name inside the retained source does not change its document key. Changing the
authored source key, declaration handle, formal owner or semantic dependency
selection changes the definition identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-288-AC-1 | Two linked protocol definitions compare equal exactly when all four structured key components compare equal. | Test (TC-035) |
| FR-288-AC-2 | A declaration name, source span or package-local table index without its admitted package qualifiers cannot establish a protocol-definition identity. | Test (TC-035) |
| FR-288-AC-3 | Changing one source-key, owner, declaration-handle, profile, model or required correspondence component changes the identity; changing only external rendering metadata, path, build provenance, runtime observation or resource-only controls while preserving the source key does not. | Test (TC-035) |
| FR-288-AC-4 | Missing, ambiguous, foreign or conflicting key components refuse without selecting a current package, ambient profile or first matching declaration. | Test (TC-035) |
| FR-288-AC-5 | Lexicographic comparison of definition keys establishes no causal, control, delivery, retry or temporal order. | Test (TC-035) |

## Dependencies

- [FR-030](../../functional/foundation/FR-030-bind-composed-definitions.md)
  retains exact static semantic selections and excludes runtime observations.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md)
  retains the protocol declaration, formal owner and selected definitions.
- [FR-202](../foundation/FR-202-authored-source-document.md) supplies the exact
  source-document key.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) governs every
  retained digest domain.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the object
  identity discipline.

## Status

Draft. This artifact records the owner ruling for specification issue #28.
Current compiled-protocol records retain the component inputs and package-local
handles; portable componentwise comparison and downstream consumer adoption
remain planned.
