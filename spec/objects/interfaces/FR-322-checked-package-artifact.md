---
id: FR-322
title: "Immutable checked-package artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-004
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-132
    type: references
---

# FR-322: Immutable checked-package artifact

## Description

When a native package completes linking and checking, the producer SHALL use the immutable CheckedPackageArtifact contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.checked-package/v1`.
- `package_id` — domain-separated digest of the checked semantic graph and package lock.
- `lock` — source, edition, ordered profiles, dependencies, model selection and feature identities.
- `semantic_graph` — typed declarations, values, expressions, temporal artifacts and protocol artifacts.
- `source_map` — total semantic-occurrence to original-source-region mapping.
- `capability_report` — one disposition for every requested feature.
- `diagnostics` — catalog-qualified located refusals and non-results.

## Identity and validation

The key is (`contract_version`, `package_id`); runtime inputs, backend availability and presentation ordering are excluded.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-322-AC-1 | Every admitted semantic occurrence maps to at least one original source region. | Analysis |
| FR-322-AC-2 | Changing backend capability availability cannot change package admission or package identity. | Analysis |
| FR-322-AC-3 | Changing a meaning-affecting lock member changes the package identity. | Analysis |

## Dependencies

- [interface-004](./interface-004-checked-package.md) owns the operation boundary.
- FR-132 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
