---
id: FR-335
title: "Exact ecosystem lock"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_018
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
---

# FR-335: Exact ecosystem lock

## Description

When a run or qualification scope is locked, the producer SHALL use the immutable EcosystemLock contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.ecosystem-lock/v1`.
- `repositories` — canonical remote, immutable revision and dirty-state disposition.
- `packages` — package identities, versions, source and dependency closure.
- `definitions` — edition/profile/wire/catalog identities and digests.
- `providers` — provider manifest, implementation and options pins.
- `toolchains` — Rust/compiler/solver/runtime versions and target triples.
- `builds` — feature flags, environment inputs and reproducible artifact identities.
- `deployment` — platform/container/wasm/isolation identity where applicable.

## Identity and validation

The lock identity is the canonical digest of the complete sorted lock. Missing required pins refuse; ambient defaults are not members.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-335-AC-1 | Equivalent locks serialize identically independent of discovery order. | Analysis |
| FR-335-AC-2 | Changing any meaning-affecting pin changes lock and run identity. | Analysis |
| FR-335-AC-3 | A qualification result names exactly one accepted ecosystem lock. | Analysis |

## Dependencies

- [interface_018](./interface_018-ecosystem-lock.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
