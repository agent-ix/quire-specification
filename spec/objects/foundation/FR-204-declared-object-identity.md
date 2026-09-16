---
id: FR-204
title: "Declared object identity"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-009
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-043
    type: specifies
---
# FR-204: Declared object identity

## Description

When identity-bearing objects are compared, the evaluator SHALL compare their universe, type and object identities rather than their field values.

## Properties

- **universe** (identity component) — the declared universe the object belongs
  to. Under `quire.model.complete/v1` it is the `quire.model.object-universe/v1`
  identity of the object type's generalization component, not a population. Identity comparison across pre and post states is permitted only within
  one declared universe
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md)).
- **type** (identity component) — the declared type of the object (FR-009).
  Under the selected `quire.model.complete/v1` definition it is the effective
  declaration identity of the object's most-specific type, as supplied by the
  population member's `typeIdentity`, never a static supertype through which
  the object is viewed.
- **object_identity** (identity component) — the object's own declared identity
  within its universe and type (FR-009).

## Identity and comparison

- **Identity kind** — *value*. This artifact types the identity triple itself,
  which is compared componentwise. The object it identifies is *authored*.
- **Key** — (`universe`, `type`, `object_identity`). Equality is componentwise
  over exactly these three and nothing else. FR-009 states it directly: "Equal
  fields do not imply equal objects" and "Structural record equality never
  replaces object identity." A display label, a source path, a span, a message,
  a locale or a rendering never participates — FR-009-AC-3 makes coincident
  object *labels* from different universes a refusal, not a match.
- **Key order** — lexicographic over (`universe`, `type`, `object_identity`),
  for collection comparison only. It carries no causal meaning and expresses no
  precedence between universes or types.
- **Causal order** — none.
- **Absence** — an absent universe, type or object identity compares only to
  absence. It is never defaulted to a current or ambient universe. An unlinked
  syntax tree cannot claim a typed-expression canonical identity
  ([FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md)-AC-4).
- **Digest domain** — not applicable to `object_identity`; a declared object
  identity is authored, not digested. Under `quire.model.complete/v1` the
  universe and type components are QSL-derived identities in the
  `quire.model.object-universe/v1` and `quire.model.effective-declaration/v1`
  domains; their key bytes are the domain string followed by the digest bytes. Where a derived identity is digested instead, its domain is
  selected under [FR-201](FR-201-canonical-identity-domain.md).

## Cross-universe comparison

Comparison is defined only within one declared universe. Coincident object
labels drawn from different universes receive `foreign_reference` with cause
`foreign-universe` (FR-009-AC-3);
they do not compare equal and they do not compare unequal-but-comparable. The
same object compared across pre and post of one universe compares equal
(FR-009-AC-2).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-204-AC-1 | Two objects with equal field values and different object identities compare unequal. | Analysis |
| FR-204-AC-2 | The same object compared across pre and post of one universe compares equal. | Analysis |
| FR-204-AC-3 | Coincident labels from two universes yield `foreign_reference` rather than an equality verdict. | Analysis |
| FR-204-AC-4 | Display label, source path and span are absent from the key. | Inspection |
| FR-204-AC-5 | An object viewed through a supertype and a subtype has one triple, and two members with equal universe and object identity but different types refuse `invalid_runtime_input`/`conflicting-identity`. | Test (TC-198) |

## Dependencies

- [FR-009](../../functional/foundation/FR-009-preserve-object-identity.md) establishes the
  universe/type/object triple and the refusal.
- [FR-043](../../functional/foundation/FR-043-evaluate-finite-graph-relations.md) evaluates
  identity and reachability in explicit finite graphs.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
