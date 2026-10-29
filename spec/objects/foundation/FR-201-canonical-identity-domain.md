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
| `quire.package.semantic/v2` | Complete-V1 CheckedPackage V2 semantic-package graph domain; distinct from every other identity/digest domain. |
| `quire.source.bytes/v1` | Exact authored native source-document bytes, including layout; distinct from every definition/model artifact. |
| `quire.checked-semantic-node/v1` | Checked semantic-node identity; distinct from its source occurrence and from the enclosing package identity. |
| `quire.generated-artifact.bytes/v1` | Exact generated artifact bytes under its selected artifact kind/profile. |
| `typed-expression-identity` | Source-free typed-expression identity. Requires a typed derivation; an unlinked syntax tree cannot claim it, and current native syntax has no qualified canonical semantic identity (FR-015, FR-015-AC-4). |
| `quire.model.effective-declaration/v2` | (BLOCKED-BY STD-604, owner: planner titan:QSL:plan; effective-declaration hash identity) QSL-derived effective-declaration identity. SHA-256 over the RFC 8785 canonical bytes of `{version, owner_effective_type, original, derivation}` (FR-150). Distinct from `sha256-jcs` and from the `quire.checked-semantic-node/v1` domain. |
| `quire.model.effective-view/v2` | (BLOCKED-BY STD-604, owner: planner titan:QSL:plan; effective-view hash identity) QSL-derived effective-view identity for one normalized model. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection, rule manifest and every effective declaration sorted by identity (FR-150). Distinct from every other identity/digest domain. |
| `quire.model.object-universe/v2` | (BLOCKED-BY STD-604, owner: planner titan:QSL:plan; object-universe hash identity) QSL-derived object-universe identity for a connected supertype component. SHA-256 over the RFC 8785 canonical bytes of the ModelSelection and its sorted root effective-declaration identities (FR-150). Distinct from every other identity/digest domain. |
| `quire.ir-proof.obligation-identity/v1` | (BLOCKED-BY STD-583, owner: planner titan:QSL:plan; obligation_identity hash) CG obligation identity. SHA-256 over the RFC 8785 canonical bytes, through QSL ADR-013 §2's one encoder, of an object with exactly four members: the subject node id, the subject's occurrence key (`node_id`, `role`, `ordinal`), the obligation kind and `arguments`, except for a parity claim, whose whole preimage is the QSL ADR-013 O-09 parity preimage (members `arguments`, `node`, `obligation_kind` and `occurrence_key` = `{ordinal, role}`). The subject is a clause, an operation application, a function (occurrence role `declaration`) or the application node of a parity claim (occurrence role `expression`). For a clause, an application or a function, `arguments` holds one entry per parameter, each a parameter node id with its declared per-argument domain; for a parity claim it holds one entry per operand position, each `{operand, position, domain}`, as that preimage encodes it. A per-step protocol record is one other form: its object has exactly the members `subject_kind` (the string `protocol-refinement-step`) and `record` (the canonical `StepSimulationRecord`), and no occurrence key. A function-value parity claim (QSL FR-357, function level) is the other: its object has exactly the members `subject_kind` (the string `function-value-parity`), `node` (the function node id), `occurrence_key` (`{ordinal: 0, role: "declaration"}`) and `arguments` (one `{parameter, position, type}` entry per declared parameter in declared order, `type` the parameter node's `semantic_type`), with no obligation kind, as QSL FR-484 spells it. Every other member of the CG obligation record (resource ceilings, embedded oracles, module, harness and contract symbols, subject path, results, solver, unwind and options), `source_span` and every source region are outside the preimage, so changing any of them leaves the identity unchanged. CG mints the digest for the clause, application, function, parity and per-step forms; QSL recomputes the parity form from the same preimage, and QSL alone mints the function-value parity form (at `qsl_replay::call_site`) and recomputes it at replay. Distinct from `quire.checked-semantic-node/v1` and every other identity/digest domain. |

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
  unequal. Byte equality of their digests does not make them equal, and a raw
  `sha256` digest cannot substitute for another domain's digest.

## Removed domains

The `sha256-jcs` and `quire.verification.jcs` domains and the `ir-semantic` row are removed
with no replacement. The files that still cite them are edited by the open STD-583
stack and carry that PR as the blocker: BLOCKED-BY QSpec #517 (FR-322, IT-070,
TC-952, AD-006, `proposals/checked-package-v2/README.md`) and BLOCKED-BY QSpec #518
(`proposals/backend-provider-v1`, `proposals/checked-package-v2` data,
`proposals/conformance-corpus-v1` and the tests that read them).

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
| FR-201-AC-10 | Two obligations of one clause, application, function, parity claim, function-value parity claim or per-step record whose records differ only in a member outside the preimage (`source_span`, a resource ceiling, an embedded oracle, a symbol, the subject path, the results, the solver, `unwind` or an option) have equal `quire.ir-proof.obligation-identity/v1` digests; changing the subject node id, the occurrence key's `role` or `ordinal`, the obligation kind, one argument's parameter node id or declared domain (for a parity claim, an operand, its position or its domain, an inline literal's value included; for a function-value parity claim, a parameter node id, its position or its declared type node) or a per-step record member gives a different digest; the per-step digest is unchanged by its occurrence key; and the digest compares unequal to a digest with equal bytes in any other domain. | Test (TC-952) |

## Dependencies

- [FR-015](../../functional/foundation/FR-015-preserve-canonical-domains.md) establishes the
  distinct domains and the substitution prohibitions.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
- [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  defines the `quire.model.effective-declaration/v2`, `quire.model.effective-view/v2`
  and `quire.model.object-universe/v2` preimages.
- QSL ADR-013 O-09 and [AD-016](../../assurance/AD-016-semantic-family-extension-path.md)
  arrow 5 define the `quire.ir-proof.obligation-identity/v1` preimage;
  [FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md) and
  [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  generate and replay the obligations it identifies.

## Status

Accepted complete-V1 identity-domain contract.
