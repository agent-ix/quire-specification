---
id: FR-015
title: "Preserve established canonical identity domains"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-015: Preserve established canonical identity domains

## Description

When a canonical identity is derived, the deriving tool shall use the exact selected domain algorithm.

## Inputs

Qualified canonicalizer input, object kind where required, domain/version and expected identity.

## Outputs

Canonical bytes/digest or an unsupported-canonicalization refusal.

## Behavior

Existing contract-IR canonical and bound identities, `sha256-jcs` verification digests, package fingerprints and raw artifact digests remain distinct. Source-free typed-expression identity requires a typed derivation. JCS cannot silently replace exact-integer/scalar-order IR canonicalization. Current native syntax has no qualified canonical semantic identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-015-AC-1 | Withdrawn: recorded file digests of golden bytes are removed. | Withdrawn |
| FR-015-AC-2 | An IR semantic digest includes its prescribed domain separation. | Test |
| FR-015-AC-3 | Withdrawn: the lock-file digest and package fingerprint are removed. | Withdrawn |
| FR-015-AC-4 | An unlinked syntax tree cannot claim a typed-expression canonical identity. | Test |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md) supplies the user need.
- [FR-201](../../objects/foundation/FR-201-canonical-identity-domain.md) lists the identity domain vocabulary.

## Status

Accepted by FS05 as standard behavior. Historical bytes and domain algorithms
remain unchanged; consumer-specific execution and matrix status stay separate.
