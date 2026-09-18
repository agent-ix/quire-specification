---
id: FR-120
title: "Select one exact output-mapping profile"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/FR-116
    type: references
  - target: ix://agent-ix/quire-specification/FR-297
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-032
    type: references
---
## Description

When admitting an output-mapping request, the coordinator SHALL select exactly
one FR-297 target family, normative target revision, Quire mapping profile
identity/revision/digest and requested capability set before dispatch.

## Inputs

One strict FS05 source-package reference; exact native/model/semantic profile
selections; an ordered nonempty requested-obligation set; one FR-297 profile;
and explicit common/target resource limits.

## Outputs

One admitted immutable request or a typed refusal naming every mismatched,
unknown, duplicate, absent or exhausted input axis. Refusal exposes no generated
target bytes.

## Behavior

The coordinator SHALL validate the complete source package and selections before
target dispatch. It SHALL NOT infer a target, standard revision, mapping revision,
tool version, capability, model binding or observer from installed software,
filename, extension, display text or a previous request.

One request selects one target profile. Producing another target requires a
separate request and package, so a mapper cannot silently fall back across
families. Unknown required features or cross-family profile members refuse.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-120-AC-1 | Each admitted request selects exactly one matching target family/profile/revision/digest and ordered obligation set. | Test (TC-150) |
| FR-120-AC-2 | Missing, duplicate, unknown, stale or cross-family selection members refuse before dispatch with no target bytes. | Test (TC-154) |
| FR-120-AC-3 | Installed tools, paths, extensions, display text and prior requests cannot supply an omitted selection. | Test (TC-154) |
| FR-120-AC-4 | Requests for two target families produce separate immutable requests and cannot share a semantic result by fallback. | Test (TC-150) |

## Dependencies

- FS01 and FS05 source/profile/interchange authority.
- [FR-116](../observation/FR-116-map-observation-results-to-consumers.md).
- [FR-297](../../objects/output-mapping/FR-297-output-mapping-consumer-profile.md).

## Status

Accepted by FS06. Implementation is qcir #55/#56/#57 behind the common boundary.
