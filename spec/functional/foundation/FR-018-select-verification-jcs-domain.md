---
id: FR-018
title: "Preserve the verification JCS identity domain"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-018: Preserve the verification JCS identity domain

## Description

When a shared semantic reference selects the verification JCS domain, the producer shall derive its digest from the exact qualified structured value using RFC 8785.

## Inputs

A validated structured value with an explicitly selected wire/profile, and
canonical selection `domain: quire.verification.jcs`, `version: rfc8785-v1`,
`algorithm: sha256`. Its exact input derivation belongs to the selected semantic
profile; a reference alone cannot authorize an arbitrary projection.

## Outputs

`sha256-jcs:` followed by 64 lowercase hexadecimal digits of SHA-256 over RFC 8785
canonical UTF-8 bytes, or an unsupported-canonicalization refusal. The textual
prefix is outside the hashed bytes. It is a domain label, not a hash preimage prefix.

## Behavior

The [wire amendment](../../../proposals/shared-reference-2-draft/README.md) makes
this domain structurally representable while retaining both existing IR domains.
Raw artifact digests continue to hash complete supplied bytes, including layout.
Canonicalization changes neither strings nor array order. requiredFeatures is
validated as a duplicate-free decoded set and compared independently of artifact
canonicalization. No global array sort supplies these two different contracts.

For B's reviewed automated-report profile, the qualified input is the validated
automated core alone, selected before hashing. Its external disposition and
outer report wrapper are excluded. Changing a core member changes the identity;
attaching a disposition leaves it unchanged. The disposition's exact byte
ArtifactRef uses review-disposition and its core digest must match the selected
core. This rule defines correspondence only; B owns the complete report schema,
reviewer/procedure checks, completeness and outcome policy.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-018-AC-1 | The positive vector's canonical bytes and prefixed digest agree with B's existing Rust jcs_canonicalize/jcs_sha256 APIs. | Test (TC-003) |
| FR-018-AC-2 | A changed canonical domain, version, algorithm or digest refuses the selected canonical interpretation; a raw sha256 digest cannot substitute for sha256-jcs. | Test (TC-003) |
| FR-018-AC-3 | Reordered object members preserve canonical identity; UTF-16 key order and numeric serialization follow RFC 8785; reordered arrays remain distinct. | Test (TC-003) |
| FR-018-AC-4 | Reordered unique requiredFeatures compare equal; duplicate decoded features refuse before comparison and an unknown required feature refuses interpretation. | Test (TC-003) |
| FR-018-AC-5 | The automated-core vector retains its digest after attaching a matching external disposition and refuses a foreign core digest, raw-byte-digest substitution or wrong disposition role. | Test (TC-004) |
| FR-018-AC-6 | The two IR canonical domain selections retain their original bare-hex syntax and algorithms; neither adopts the JCS prefix or canonicalization. | Test (TC-003) |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md), [FR-015](FR-015-preserve-canonical-domains.md) and [FR-017](FR-017-select-verification-artifact-roles.md).
- [NFR-001](../../non-functional/NFR-001-preserve-versioned-meaning.md) preserves historical meaning; [NFR-002](../../non-functional/NFR-002-classify-resource-incompleteness.md) preserves incomplete outcomes.
- [IT-003](../../integration/IT-003-verification-reference-amendment.md) separates existing canonicalizer parity from a fully qualified shared reader.

## Status

Specification amendment. Authored vectors are conformance inputs, not executed
portfolio results or a human disposition.
