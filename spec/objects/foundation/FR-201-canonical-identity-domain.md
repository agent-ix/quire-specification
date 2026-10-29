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
| `ir-canonical` | Existing contract-IR canonical identity. Retains its original bare-hex syntax and algorithm; it does not adopt the JCS prefix or canonicalization (FR-018-AC-6). |
| `ir-bound` | Existing contract-IR bound identity. Held distinct from `ir-canonical` by FR-015. |
| `ir-semantic` | IR semantic digest. Includes its prescribed domain separation (FR-015-AC-2). |
| `raw-artifact-digest` | Raw artifact digest. Hashes the complete supplied bytes, including layout (FR-018). |
| `sha256-jcs` | JCS digest. SHA-256 over RFC 8785 canonical bytes. No model selection row of a lock and no declaration key carries it (FR-154, FR-321, AD-006); its remaining users are those that name it in their own requirement.  |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `quire.simulation.state-key/v1` | Simulator state-key identity. SHA-256 over the RFC 8785 canonical bytes of the typed canonical encoding of simulator state, control positions, queues, role lifetimes, observation progress and remaining bounds (FR-181). Distinct from `sha256-jcs` and from every other identity/digest domain. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |
| `quire.verification.jcs` | Verification JCS domain, `version: rfc8785-v1`, `algorithm: sha256`. Emitted as `sha256-jcs:` followed by 64 lowercase hexadecimal digits over RFC 8785 canonical UTF-8 bytes (FR-018). |
| `quire.model.effective-declaration/v2` | QSL-derived effective-declaration identity. SHA-256 over the RFC 8785 canonical bytes of `{version, owner_effective_type, original, derivation}` (FR-150). Distinct from `sha256-jcs`. |
| `quire.model.effective-view/v2` | QSL-derived effective-view identity for one normalized model. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection, rule manifest and every effective declaration sorted by identity (FR-150). Distinct from every other identity/digest domain. |
| `quire.model.object-universe/v2` | QSL-derived object-universe identity for a connected supertype component. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection and its sorted root effective-declaration identities (FR-150). Distinct from every other identity/digest domain. |

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
- **Digest domain** — not applicable to the label itself. This artifact *is* the
  vocabulary of digest domains; it does not carry one.
- **Cross-domain rule** — two identities bearing different domain labels are
  unequal. Byte equality of their digests does not make them equal, and a raw
  `sha256` digest cannot substitute for `sha256-jcs` (FR-018-AC-2).

The `sha256-jcs:` textual prefix is a domain label outside the hashed bytes. It
is not a hash preimage prefix (FR-018).

## Preimage selection is per-use, not per-domain

Three requirements independently state what is inside the preimage under the
`quire.verification.jcs` domain, and no requirement cross-references the others:

- [FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md): for B's
  automated-report profile the qualified input is the validated automated core
  alone; the external disposition and outer report wrapper are excluded.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md): the
  canonical result identity is computed over the complete closed result with
  only `resultId` omitted.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md):
  the derived result identity is taken over a distinct projection-identity
  preimage under `quire.protocol.e02-projection-identity/v1-draft.1`.

This artifact records all three. It does not reconcile them; the selection of a
preimage belongs to the requirement that selects it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-201-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-201-AC-2 | Two identities with equal digest bytes and different domain labels compare unequal. | Analysis |
| FR-201-AC-3 | A digest with an absent domain compares only to absence and receives no default member. | Analysis |
| FR-201-AC-4 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-201-AC-7 | The `quire_specification` library's `COMPLETE_VALUE_LOCK`, `COMPLETE_VALUE_SELECTION_VECTORS` and `NATIVE_DIAGNOSTICS` equal the exact bytes of `complete-value-lock.json`, `complete-value-selection-vectors.json` and `native-diagnostics.md`. | Test (TC-233) |
| FR-201-AC-8 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-201-AC-9 | Every qualification-catalog entry of the complete-value and complete-model locks names an authored artifact under a unique role and identity, and package selection is exact: each complete-value catalog role occupies one always, trigger-conditional or exactly-one slot under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`, every complete-model catalog role is always selected, and each accepted and refused selection vector is admitted or refused with exactly its recorded selection code. | Test (TC-233) |
| FR-201-AC-10 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md) establishes
  the verification JCS selection, output syntax and prefix rule.
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
