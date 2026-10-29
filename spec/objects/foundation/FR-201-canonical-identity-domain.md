---
id: FR-201
title: "Canonical identity domain vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-015
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-018
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-150
    type: specifies
---
# FR-201: Canonical identity domain vocabulary

## Description

When a digest is compared to another digest, the reader SHALL compare the
selected identity domain before the bytes, so that an identity derived under one
domain is unequal to an identity derived under another regardless of byte
equality.

## Values

| Value | Description |
| --- | --- |
| `ir-canonical` | Existing contract-IR canonical identity. Retains its original bare-hex syntax and algorithm; it does not adopt the JCS prefix or canonicalization (FR-018-AC-6). BLOCKED-BY QSpec #512: FR-018 is withdrawn there. |
| `ir-bound` | Existing contract-IR bound identity. Held distinct from `ir-canonical` by FR-015. |
| `raw-artifact-digest` | Raw artifact digest. Hashes the complete supplied bytes, including layout (FR-018). |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |
| `quire.model.effective-declaration/v2` | QSL-derived effective-declaration identity. SHA-256 over the RFC 8785 canonical bytes of `{version, owner_effective_type, original, derivation}` (FR-150). Distinct from every other identity/digest domain. BLOCKED-BY STD-604 (owner: planner titan:QSL:plan): effective-declaration hash and its `{version, owner_effective_type, original, derivation}` preimage (FR-150, model-complete.md). |
| `quire.model.effective-view/v2` | QSL-derived effective-view identity for one normalized model. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection, rule manifest and every effective declaration sorted by identity (FR-150). Distinct from every other identity/digest domain. BLOCKED-BY STD-604 (owner: planner titan:QSL:plan): effective-view digest over effective-declaration identities. |
| `quire.model.object-universe/v2` | QSL-derived object-universe identity for a connected supertype component. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection and its sorted root effective-declaration identities (FR-150). Distinct from every other identity/digest domain. BLOCKED-BY STD-604 (owner: planner titan:QSL:plan): object-universe digest over effective-declaration identities. |

## Identity and comparison

- **Identity kind** — *value*. A domain label has no identity of its own; it is
  compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Spellings are not normalized
  aliases, and a hyphenated and an underscored spelling are different labels.
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence, preference or causal meaning between domains.
- **Absence** — a digest supplied without a selected domain has an absent
  domain. An absent domain compares only to absence; it is never defaulted to
  `raw-artifact-digest` or to any other member.
- **Causal order** — not applicable. A domain label participates in no causal
  structure.
- **Cross-domain rule** — two identities bearing different domain labels are
  unequal. Byte equality of their digests does not make them equal. BLOCKED-BY QSpec #512: FR-018-AC-2 is withdrawn there.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-201-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-201-AC-2 | Two identities with equal digest bytes and different domain labels compare unequal. | Analysis |
| FR-201-AC-3 | A digest with an absent domain compares only to absence and receives no default member. | Analysis |
| FR-201-AC-4 | Withdrawn: the lock-file digest and package fingerprint domains are removed. | Withdrawn |
| FR-201-AC-7 | Withdrawn: the byte-equality test of the exposed constants against their files is removed. | Withdrawn |
| FR-201-AC-8 | Withdrawn: it existed only for the cache key, which is removed with the result cache. | Withdrawn |
| FR-201-AC-9 | Package selection is exact under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`, and each accepted and refused selection vector is admitted or refused with exactly its recorded selection code. | Test (TC-233) |
| FR-201-AC-10 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md) is withdrawn in QSpec #512 (BLOCKED-BY QSpec #512).
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the `quire.model.effective-declaration/v2`, `quire.model.effective-view/v2`
  and `quire.model.object-universe/v2` preimages.

## Structural identities

Package, node and obligation identities are not digests and carry no domain:
they are structural values compared componentwise (FR-322, FR-479).

Runtime identities follow the same rule. The identity of a runtime value, unit,
compound unit, node or key (the exact-arithmetic and runtime-accounting
identities that QSL FR-060 and FR-089 mint and FR-142 consumes) is a
`{package, path, ordinal}` structural id or, for a value with no declaration,
the ordered list of its components, each a name, a path segment or an ordinal,
compared componentwise. No runtime identity is a hash, digest or checksum, and
no hash replaces one: where two runtime identities must be told apart, they
differ in a component.

## Status

Accepted complete-V1 identity-domain contract.
