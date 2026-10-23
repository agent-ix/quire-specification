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
| `package-fingerprint` | Package fingerprint. A lock-file digest cannot substitute for it (FR-015-AC-3). |
| `lock-file-digest` | Lock-file digest. Cannot substitute for `package-fingerprint` (FR-015-AC-3). |
| `raw-artifact-digest` | Raw artifact digest. Hashes the complete supplied bytes, including layout (FR-018). |
| `sha256-jcs` | Domain package digest. SHA-256 over the RFC 8785 canonical bytes of the semantic IR domain package lifted from spec artifacts. Every V2 model selection carries it, and every declaration key names it as its digest domain (AD-006). It is distinct from package and source digests. |
| `quire.package.semantic/v2` | Complete-V1 CheckedPackage V2 semantic-package graph domain; distinct from package archive and lock-file bytes and from every other identity/digest domain. |
| `quire.source.bytes/v1` | Exact authored native source-document bytes, including layout; distinct from every definition/model artifact. |
| `quire.checked-semantic-node/v1` | Checked semantic-node identity; distinct from its source occurrence and from the enclosing package identity. |
| `quire.contract-ir.semantic/v1` | Generated Contract IR semantic identity; distinct from the checked source package that it lowers. |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `quire.tool-manifest.jcs/v1` | RFC 8785 canonical tool/plugin/provider manifest bytes; a provider's FR-331 `ManifestDigest` is SHA-256 over its complete descriptor's bytes, `tool` included. |
| `quire.build-inputs/v1` | Build-input identity of the provider code that implements a backend: SHA-256 over the RFC 8785 bytes of its FR-331 build-input record `{sources, dependencies, compiler, target, profile, options}`, computed by the build that compiles it (FR-331 "Backend tool"). Distinct from every other identity/digest domain. |
| `quire.cache-key/v1` | Cache-entry key. SHA-256 over the RFC 8785 canonical bytes of the closed object `{operation, package_id, request, backend, manifest, target}` (FR-306): the cached operation, the subject's `quire.package.semantic/v2` `package_id`, the canonical request, the provider's `BackendId`, its FR-331 `ManifestDigest` as a `quire.tool-manifest.jcs/v1` digest record and, for `generate` only, the generation target and its options. Distinct from every other identity/digest domain. |
| `quire.simulation.state-key/v1` | Simulator state-key identity. SHA-256 over the RFC 8785 canonical bytes of the typed canonical encoding of simulator state, control positions, queues, role lifetimes, observation progress and remaining bounds (FR-181). Distinct from `sha256-jcs` and from every other identity/digest domain. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |
| `quire.verification.jcs` | Verification JCS domain, `version: rfc8785-v1`, `algorithm: sha256`. Emitted as `sha256-jcs:` followed by 64 lowercase hexadecimal digits over RFC 8785 canonical UTF-8 bytes (FR-018). |
| `quire.model.effective-declaration/v2` | QSL-derived effective-declaration identity. SHA-256 over the RFC 8785 canonical bytes of `{version, owner_effective_type, original, derivation}` (FR-150). Distinct from `sha256-jcs` and from the `quire.checked-semantic-node/v1` domain. |
| `quire.model.effective-view/v2` | QSL-derived effective-view identity for one normalized model. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection, rule manifest and every effective declaration sorted by identity (FR-150). Distinct from every other identity/digest domain. |
| `quire.model.object-universe/v2` | QSL-derived object-universe identity for a connected supertype component. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection and its sorted root effective-declaration identities (FR-150). Distinct from every other identity/digest domain. |
| `quire.ir-proof.obligation-identity/v1` | CG obligation identity. SHA-256 over the RFC 8785 canonical bytes, through QSL ADR-013 §2's one encoder, of an object with exactly four members: the subject node id, the subject's occurrence key (`node_id`, `role`, `ordinal`), the obligation kind and `arguments`, except for a parity claim, whose whole preimage is the QSL ADR-013 O-09 parity preimage (members `arguments`, `node`, `obligation_kind` and `occurrence_key` = `{ordinal, role}`). The subject is a clause, an operation application, a function (occurrence role `declaration`) or the application node of a parity claim (occurrence role `expression`). For a clause, an application or a function, `arguments` holds one entry per parameter, each a parameter node id with its declared per-argument domain; for a parity claim it holds one entry per operand position, each `{operand, position, domain}`, as that preimage encodes it. A per-step protocol record is the one other form: its object has exactly the members `subject_kind` (the string `protocol-refinement-step`) and `record` (the canonical `StepSimulationRecord`), and no occurrence key. Every other member of the CG obligation record (resource ceilings, embedded oracles, module, harness and contract symbols, subject path, results, solver, unwind and options), `source_span` and every source region are outside the preimage, so changing any of them leaves the identity unchanged. CG mints the digest for every form; QSL recomputes the parity form from the same preimage. Distinct from `quire.checked-semantic-node/v1`, `quire.contract-ir.semantic/v1` and every other identity/digest domain. |

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

## Definition bytes by reference

The `quire_specification` library of this repository's Rust package exposes
`COMPLETE_VALUE_LOCK`, `COMPLETE_VALUE_SELECTION_VECTORS` and
`NATIVE_DIAGNOSTICS`: the exact bytes of
`proposals/quire-v1/definitions/complete-value-lock.json`, its
package-selection vectors `complete-value-selection-vectors.json` and
`proposals/quire-v1/definitions/native-diagnostics.md`.
A consumer reads the complete-value lock's catalog and selection vectors from
them instead of holding a copy.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-201-AC-1 | Each member's value equals the exact label string in the table, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-201-AC-2 | Two identities with equal digest bytes and different domain labels compare unequal. | Analysis |
| FR-201-AC-3 | A digest with an absent domain compares only to absence and receives no default member. | Analysis |
| FR-201-AC-4 | A lock-file digest does not satisfy a comparison expecting a package fingerprint. | Analysis |
| FR-201-AC-7 | The `quire_specification` library's `COMPLETE_VALUE_LOCK`, `COMPLETE_VALUE_SELECTION_VECTORS` and `NATIVE_DIAGNOSTICS` equal the exact bytes of `complete-value-lock.json`, `complete-value-selection-vectors.json` and `native-diagnostics.md`. | Test (TC-233) |
| FR-201-AC-8 | A `quire.cache-key/v1` key computed over the same cached operation, `package_id`, canonical request, `BackendId`, `ManifestDigest` and generation target equals one computed by a second conformer that derives the `ManifestDigest` from the same descriptor independently, and compares unequal to a digest with equal bytes in any other domain. | Test (TC-226) |
| FR-201-AC-9 | Every qualification-catalog entry of the complete-value and complete-model locks names an authored artifact under a unique role and identity, and package selection is exact: each complete-value catalog role occupies one always, trigger-conditional or exactly-one slot under the closed triggers `text_bearing`, `ieee_operation` and `integer_div_rem`, every complete-model catalog role is always selected, and each accepted and refused selection vector is admitted or refused with exactly its recorded selection code. | Test (TC-233) |
| FR-201-AC-10 | Two obligations of one clause, application, function, parity claim or per-step record whose records differ only in a member outside the preimage (`source_span`, a resource ceiling, an embedded oracle, a symbol, the subject path, the results, the solver, `unwind` or an option) have equal `quire.ir-proof.obligation-identity/v1` digests; changing the subject node id, the occurrence key's `role` or `ordinal`, the obligation kind, one argument's parameter node id or declared domain (for a parity claim, an operand, its position or its domain, an inline literal's value included) or a per-step record member gives a different digest; the per-step digest is unchanged by its occurrence key; and the digest compares unequal to a digest with equal bytes in any other domain. | Test (TC-952) |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md) establishes
  the verification JCS selection, output syntax and prefix rule.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the `quire.model.effective-declaration/v2`, `quire.model.effective-view/v2`
  and `quire.model.object-universe/v2` preimages.
- QSL ADR-013 O-09 and [AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
  arrow 5 define the `quire.ir-proof.obligation-identity/v1` preimage;
  [FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md) and
  [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  generate and replay the obligations it identifies.

## Resolved domain boundary

`package-fingerprint` is a Quire package identifier. `sha256-jcs` identifies a
domain package. They are separate domains and never compare equal, even when
digest bytes match. Neither may substitute for the other.

## Status

Accepted complete-V1 identity-domain contract. Implementation and qualification
remain tracked separately.
