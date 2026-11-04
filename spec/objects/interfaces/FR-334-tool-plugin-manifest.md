---
id: FR-334
title: "Tool and plugin manifest"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_017
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-305
    type: references
---

# FR-334: Tool and plugin manifest

## Description

When a tool or plugin is registered, the producer SHALL use the immutable ToolPluginManifest contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| tool_id | String | 1..1 |  |
| capabilities | String | 1..1 |  |
| effects | String | 1..1 |  |
| rights | String | 1..1 |  |
| platform | String | 1..1 |  |
| limits | String | 1..1 |  |

## Field sources

- `contract_version` — exactly `quire.tool-plugin-manifest/v1`.
- `tool_id` — authority-qualified provider identity.
- `capabilities` — exact operation/profile/format tuples.
- `effects` — closed set of filesystem, network, clock, process and external-action effects.
- `rights` — required grants and scope constraints for each effect.
- `platform` — target and ABI/process-wire requirements.
- `limits` — input, output, memory, time and concurrency ceilings.

## Identity and validation

The key is every field above except transient availability. Compile-time traits and isolated executables are separate provider kinds.
A document whose `contract_version` is not `quire.tool-plugin-manifest/v1`, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-334-AC-1 | An unknown capability or effect refuses registration. | Analysis |
| FR-334-AC-2 | Pure providers declare an empty effect set and receive no ambient authority. | Analysis |
| FR-334-AC-3 | Manifest rights cannot grant language admission or semantic authority. | Analysis |

## Dependencies

- [interface_017](./interface_017-tool-plugin-manifest.md) owns the operation boundary.
- FR-305 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract.
