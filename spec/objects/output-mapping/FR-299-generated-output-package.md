---
id: FR-299
title: "Generated output package"
type: FR
object: aggregate_root
relationships:
  - target: ix://agent-ix/quire-specification/FR-125
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-298
    type: references
---
# FR-299: Generated output package

## Description

When all requested obligations have valid mapping records, the package assembler SHALL
produce one immutable generated-output package binding exact target bytes
to the complete ordered record set and source/profile/generator selections.

## Members

| Property | Description |
| --- | --- |
| `source_package_ref` | Strict FS05 source-package reference: the source package identity and version the request admitted. No source digest is carried. |
| `target_profile` | Exact FR-297 value. |
| `generator` | Rust generator identity and semantic version. |
| `target_bytes` | Immutable generated target-language bytes; data, never source authority. |
| `records` | Complete FR-298 records in requested source-obligation order. |
| `limits` | Exact admitted request/output/work limits that governed generation. |

## Identity and comparison

- **Identity kind** — *aggregate root*. A package is identified by its key; there
  is no derived content hash.
- **Key** — source package reference (identity and version), target profile,
  generator identity and version, complete ordered records and limits. Package
  equality is componentwise, with `target_bytes` compared as bytes; matching
  target bytes alone is insufficient.
- **Key order** — none between packages. Record order is the requested source
  order and is part of the key.
- **Causal order** — the validated request and all records precede assembly;
  optional parser/tool observations reference the immutable package afterward.
- **Absence** — a missing property or record prevents package construction. No
  partial package or identity is exposed after refusal, cancellation or exhaustion.

## Aggregate invariants

- Every requested obligation appears exactly once and no unrequested obligation
  appears.
- Every output region is within `target_bytes`, is UTF-8 boundary-safe where the
  profile requires UTF-8, and belongs to the record that claims it.
- Unrepresented/refused records claim no output region.
- Reassembling the same admitted request with the same profile/generator yields
  byte-identical output, record identities and package identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-299-AC-1 | A package exists only after every requested obligation has exactly one valid record. | Test (TC-150) |
| FR-299-AC-2 | Mutating target bytes, source package identity or version, profile, generator identity or version, record order/content or limits makes the package compare unequal or invalid. | Test (TC-154) |
| FR-299-AC-3 | Equal admitted requests produce byte-identical packages; path, timestamp, locale and observer output cannot change them. | Test (TC-155) |
| FR-299-AC-4 | Refusal, cancellation, overflow or exhaustion exposes no partial package or plausible substitute bytes. | Test (TC-154) |
| FR-299-AC-5 | Parser/tool observations reference the package after assembly and cannot mutate it or its semantic classifications. | Test (TC-155) |

## Dependencies

- [FR-125](../../functional/output-mapping/FR-125-produce-deterministic-output-packages.md)
  defines assembly behavior.
- [FR-298](FR-298-per-obligation-output-mapping-record.md) defines records.
- FS05 defines strict source package/reference behavior.

## Status

Accepted by FS06 as the immutable output and loss-report aggregate.
