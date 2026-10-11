---
id: TC-936
title: "ContractPackage identity hashes its closed semantic projection"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---
# TC-936: ContractPackage identity hashes its closed semantic projection

## Description

Verify the exact domain-separated canonical identity projection, including
package provenance, source correspondence and per-item refusals.

## Test Procedure

Construct a ContractPackage with a graph node, source map, selected profile,
and one supported and one refused item. Independently construct the FR-330
projection by removing only `identity`, canonicalize it as UTF-8 RFC 8785 JSON,
and compute SHA-256. Then mutate, one at a time, the package source identity,
lock, `package.lowering_profile`, graph body, source occurrence, refusal
disposition and successful caller limits that leave every result member
unchanged.
Finally reorder only object members and change JSON whitespace.

## Expected Results

The identity is exactly
`{domain: "quire.contract-ir.semantic/v1", algorithm: "sha256", digest: <64 lowercase hex>}`
and its digest equals the independent projection digest. Each included-member
mutation changes the digest. Changing only caller limits without changing any
serialized member, or changing JSON layout, does not. The digest is never the
checked package's `quire.package.semantic/v2`
digest, and the identity member is not recursively included in its own
preimage.
