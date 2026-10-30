---
id: FR-201
title: "Canonical identity domain vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-015
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
| `ir-canonical` | Existing contract-IR canonical identity. Retains its original bare-hex syntax and algorithm; it does not adopt the JCS prefix or canonicalization. |
| `ir-bound` | Existing contract-IR bound identity. Held distinct from `ir-canonical` by FR-015. |
| `raw-artifact-digest` | Raw artifact digest. Hashes the complete supplied bytes, including layout. |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |

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
  unequal. Byte equality of their digests does not make them equal.

## Structural identities

A structural identity is a value of named components compared componentwise,
never a digest and never derived from a hash. A node id is `{package, path,
ordinal}` (FR-322 "Structural node identity"); an effective declaration
identity is `{owner_effective_type, original}` (FR-150); the identity members
of a universe, unit, variant and member (`UniverseId`, `UnitId`, `VariantId`,
`MemberId`) are a declared name path and an ordinal in the owner, while an
object's identity stays the authored object identity within its universe and
type, the FR-204 triple (universe, type, object identity); and an obligation identity is (the claim or subject `node_id`, the
identity of the model that owns it, an ordinal) (FR-418-AC-9). Two structural
identities are equal exactly when every component is equal. Components compare
in the order stated, strings by their UTF-8 bytes, arrays element by element
with a proper prefix first and integers numerically. No other member of the
record that carries an identity contributes to it.

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
| FR-201-AC-10 | Two obligations of one clause, application, function, parity claim, function-value parity claim or per-step record whose records differ only in a member outside the triple (`source_span`, a resource ceiling, an embedded oracle, a symbol, the subject path, the results, the solver, `unwind` or an option) have equal obligation identities; changing the subject `node_id`, the owning model identity or the ordinal gives a different identity. | Test (TC-952) |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the effective declaration identity.
- QSL ADR-013 O-09 and [AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
  arrow 5 define the obligation identity;
  [FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md) and
  [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  generate and replay the obligations it identifies.

## Status

Accepted complete-V1 identity-domain contract.
