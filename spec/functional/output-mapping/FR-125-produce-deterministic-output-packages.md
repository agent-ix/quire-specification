---
id: FR-125
title: "Produce deterministic output packages"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-299
    type: references
---
## Description

When every requested obligation has one valid mapping record, the package
assembler SHALL emit one immutable FR-299 package whose bytes, record order and
raw/JCS identities are deterministic for the exact request, profile and generator.

## Inputs

One admitted request, one complete ordered FR-298 record set, deterministic
target bytes and the exact Rust generator identity/version.

## Outputs

One FR-299 package or a typed failure with no partial package.

## Behavior

The assembler SHALL verify record completeness and region bounds, recompute the
source-package and target-byte digests, and derive the package identity over the
closed FS06 preimage. It SHALL exclude path, time, locale and observer messages
from output bytes and semantic identity unless a target profile explicitly
defines a stable semantic representation for them.

Structural observer results SHALL reference the immutable package after assembly
and SHALL NOT change bytes, records, classifications or identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-125-AC-1 | Equal admitted inputs produce byte-identical target output, records and package identity. | Test (TC-155) |
| FR-125-AC-2 | Any semantic input/profile/generator/record/limit/target-byte change changes or invalidates the corresponding identity. | Test (TC-154) |
| FR-125-AC-3 | All output regions are in bounds and belong to their record; non-output dispositions claim none. | Test (TC-150) |
| FR-125-AC-4 | Failure, cancellation, overflow or exhaustion exposes no partial package. | Test (TC-154) |
| FR-125-AC-5 | Observer/tool output remains separate structural evidence and cannot alter semantic mapping state. | Test (TC-155) |

## Dependencies

- [FR-121](FR-121-account-for-every-output-obligation.md).
- [FR-299](../../objects/output-mapping/FR-299-generated-output-package.md).
- FS05 strict references and [FR-201](../../objects/foundation/FR-201-canonical-identity-domain.md).

## Status

Accepted by FS06; each qcir exporter must implement this common package boundary.
