---
id: FR-328
title: "Composed execution bundle"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-010
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-180
    type: references
---

# FR-328: Composed execution bundle

## Description

When domain results are composed for a workflow request, the producer SHALL use the immutable ExecutionBundle contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.execution-bundle/v1`.
- `request` — package, workflow, selected claims, observations, domains and limits.
- `shared_prerequisites` — exact model, package, clock, population and ecosystem locks.
- `state_results` — ordered native state results.
- `temporal_results` — ordered native temporal results.
- `protocol_results` — ordered protocol results.
- `analysis_results` — ordered optional method results.
- `bundle_disposition` — completed, incomplete, refused or failed without flattening sibling axes.

## Identity and validation

The key is the canonical request plus every retained domain-result identity. Ordering follows selected item identities, not completion time.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-328-AC-1 | Every requested domain item appears exactly once even when unsupported or incomplete. | Analysis |
| FR-328-AC-2 | One sibling failure cannot erase or relabel independent completed results. | Analysis |
| FR-328-AC-3 | Shared-prerequisite contradiction refuses composition and identifies affected results. | Analysis |

## Dependencies

- [interface-010](./interface-010-execution-bundle.md) owns the operation boundary.
- FR-180 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
