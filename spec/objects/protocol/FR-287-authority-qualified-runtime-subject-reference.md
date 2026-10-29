---
id: FR-287
title: "Authority-qualified runtime subject reference"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-035
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
---
# FR-287: Authority-qualified runtime subject reference

## Description

When a concrete runtime subject is bound for assessment, the binder SHALL retain
its exact selected authority, subject kind and authority-supplied opaque identity
without deriving identity from its relationships or presentation.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Authority identity | FR-035 | Exact identity of the selected binding authority. |
| Subject kind | FR-035 | Exact typed discriminator; equal opaque bytes of different kinds remain unequal. For a domain object the kind is its effective declaration key (domain package identity, IR node identity) per AD-006. |
| Authority-local object identity | FR-035 | Exact non-empty bytes retained without parsing, trimming, normalization or reconstruction. |

The binding requirement identifies a typed slot and its expected authority. It
does not identify the concrete object placed in that slot. Relationships to
static definitions, workflows, roles, channels, nodes, attempts, effects or
configurations are validated separately and do not extend this value.

This value specializes FR-204's universe/type/object discipline for a runtime
binding boundary: the selected authority is the universe, the subject kind is
the type discriminator and the authority-local bytes are the object identity.

## Identity and comparison

- **Identity kind** — *value*. This object is the complete key value carried by
  an authored runtime identity; it has no identity separate from its members.
- **Key** — (`authority identity`, `subject kind`, `authority-local object
  identity`). Canonical key equality is componentwise over exactly these
  members. Semantic object equality is defined only within one authority and
  kind; crossing either boundary yields the typed foreign/wrong-kind refusal
  required by FR-204 rather than a Boolean verdict.
- **Key order** — lexicographic over the key in the declared sequence, using
  each selected authority component's own exact comparison and exact byte order
  for the authority-local identity. It ranks nothing and licenses no precedence,
  correlation or conversion.
- **Causal order** — none. Key order SHALL NOT establish source, causal,
  delivery, retry, timestamp or arrival order.
- **Absence** — every component is required. Missing or empty identity, kind or
  object bytes refuses; none compares equal to a default or empty value.

Display text, source path, span, payload, trace identifier, timestamp, arrival
position and transport status are not members. Reusing one complete key with
incompatible typed relationship facts is an identity contradiction rather than
a second subject.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-287-AC-1 | Two references have one canonical key exactly when all three members compare equal; a one-member mutation produces a distinct key, and a semantic comparison across authority or kind refuses rather than returning object equality. | Test (TC-035) |
| FR-287-AC-2 | Missing or empty key members refuse without a generated, normalized or default identity. | Test (TC-035) |
| FR-287-AC-3 | Display, locus, payload, trace, timestamp, arrival position and transport status changes alone do not change the reference. | Test (TC-035) |
| FR-287-AC-4 | Reusing one complete key with incompatible binding relationships refuses as an identity contradiction. | Test (TC-035) |
| FR-287-AC-5 | Permuting a collection preserves lexicographic key order without adding a causal, delivery or retry relation. | Test (TC-035) |

## Dependencies

- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md) is the
  behavioral authority for this value.
- [FR-204](../foundation/FR-204-declared-object-identity.md) supplies the
  universe/type/object identity discipline specialized here.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the object
  identity discipline.

## Status

Draft. This artifact records the owner ruling for specification issue #28.
Executable cross-repository binding evidence remains planned in the family test
matrices; link-time binding-requirement coordinates are not claimed as concrete
runtime identities.
