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

When two identities are compared, the reader SHALL compare them componentwise
by their named components ("Structural identities" below), so that no identity
is derived from a hash of bytes and no identity is compared by anything other
than its components.

## Identity and comparison

- **Identity kind** — *value*, compared componentwise.
- **Key** — the named components of the identity, matched exactly. Spellings are
  not normalized aliases.
- **Key order** — the component order stated below, for collection comparison
  only. It carries no precedence, preference or causal meaning.
- **Absence** — an absent component compares only to absence and is never
  defaulted.
- **Causal order** — not applicable. An identity participates in no causal
  structure.

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
| FR-201-AC-1 | Withdrawn: the identity-domain label vocabulary is removed with the digest domains it labelled. | Withdrawn |
| FR-201-AC-2 | Withdrawn: it compared digests across domain labels, and no digest domain remains. | Withdrawn |
| FR-201-AC-3 | Withdrawn: it concerned a digest with an absent domain, and no digest remains. | Withdrawn |
| FR-201-AC-4 | Withdrawn: the lock-file digest and package fingerprint domains are removed. | Withdrawn |
| FR-201-AC-7 | Withdrawn: the byte-equality test of the exposed constants against their files is removed. | Withdrawn |
| FR-201-AC-8 | Withdrawn: it existed only for the cache key, which is removed with the result cache. | Withdrawn |
| FR-201-AC-9 | Package selection is exact under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`, and each accepted and refused selection vector is admitted or refused with exactly its recorded selection code. | Test (TC-233) |
| FR-201-AC-10 | Two obligations of one clause, application, function, parity claim, function-value parity claim or per-step record whose records differ only in a member outside the triple (`source_span`, a resource ceiling, an embedded oracle, a symbol, the subject path, the results, the solver, `unwind` or an option) have equal obligation identities; changing the subject `node_id`, the owning model identity or the ordinal gives a different identity. | Test (TC-952) |

## Dependencies

- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the effective declaration identity.
- QSL ADR-013 O-09 and [AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
  arrow 5 define the obligation identity;
  [FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md) and
  [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  generate and replay the obligations it identifies.

## Status

Accepted complete-V1 structural identity contract.
