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
| `package-fingerprint` | Package fingerprint. A lock-file digest cannot substitute for it (FR-015-AC-3). |
| `lock-file-digest` | Lock-file digest. Cannot substitute for `package-fingerprint` (FR-015-AC-3). |
| `raw-artifact-digest` | Raw artifact digest. Hashes the complete supplied bytes, including layout (FR-018). |
| `sha256-jcs` | Domain package digest. SHA-256 over the RFC 8785 canonical bytes of the semantic IR domain package lifted from spec artifacts. Every V2 model selection carries it, and every declaration key names it as its digest domain (AD-006). It is distinct from package, source and definition digests. |
| `quire.package.semantic/v2` | Complete-V1 CheckedPackage V2 semantic-package graph domain; distinct from package archive and lock-file bytes and from every other identity/digest domain. |
| `quire.source.bytes/v1` | Exact authored native source-document bytes, including layout; distinct from every definition/model artifact. |
| `quire.definition.bytes/v1` | Exact selected native profile or semantic-definition bytes, including layout; distinct from source/model bytes. |
| `quire.checked-semantic-node/v1` | Versioned checked semantic-node identity; distinct from its source occurrence and from the enclosing package identity. |
| `quire.contract-ir.semantic/v1` | Versioned generated Contract IR semantic identity; distinct from the checked source package that it lowers. |
| `quire.diagnostic-catalog.bytes/v1` | Exact diagnostic-catalog source bytes, including layout. |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `quire.tool-manifest.jcs/v1` | RFC 8785 canonical tool/plugin/provider manifest bytes. |
| `quire.simulation.state-key/v1` | Simulator state-key identity. SHA-256 over the RFC 8785 canonical bytes of the typed canonical encoding of simulator state, control positions, queues, role lifetimes, observation progress and remaining bounds (FR-181). Distinct from `sha256-jcs` and from every other identity/digest domain. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |
| `quire.verification.jcs` | Verification JCS domain, `version: rfc8785-v1`, `algorithm: sha256`. Emitted as `sha256-jcs:` followed by 64 lowercase hexadecimal digits over RFC 8785 canonical UTF-8 bytes (FR-018). |

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
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md): the enclosing
  artifact reference carries the digest over the complete canonical result
  bytes, and the digest is not a self-referential field inside those bytes.
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
| FR-201-AC-4 | A lock-file digest does not satisfy a comparison expecting a package fingerprint. | Analysis |
| FR-201-AC-6 | The complete-value definition lock closes every qualification-catalog artifact by identity, revision, `quire.definition.bytes/v1` digest domain and raw-byte digest, and its conditional and alternative package selection is exact: each catalog entry occupies one always, trigger-conditional or exactly-one slot under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`. | Test (TC-233) |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md) establishes
  the verification JCS selection, output syntax and prefix rule.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Resolved domain boundary

`package-fingerprint` is a Quire package identifier. `sha256-jcs` identifies a
domain package. They are separate domains and never compare equal, even when
digest bytes match. Neither may substitute for the other.

## Status

Accepted complete-V1 identity-domain contract. Implementation and qualification
remain tracked separately.
