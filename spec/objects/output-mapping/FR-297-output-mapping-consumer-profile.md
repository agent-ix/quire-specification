---
id: FR-297
title: "Output-mapping consumer profile"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: specifies
---
# FR-297: Output-mapping consumer profile

## Description

When selecting an output consumer, the mapping coordinator SHALL identify the
target family, normative target references and Quire mapping profile by exact
identity rather than by a display name or installed tool.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| target_family | String | 1..1 |  |
| target_standard_refs | String | 0..* |  |
| mapping_profile_id | String | 1..1 |  |
| required_capabilities | String | 1..1 |  |
| observer_selection | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| target_family | string tag, exactly one of `ocl`, `sysml-kerml`, `fretish` |
| target_standard_refs | type not stated in source; see Field sources |
| mapping_profile_id | string identity, one of three `quire.output.*/v1` names; absence refuses dispatch (Absence) |
| required_capabilities | type not stated in source; see Field sources |
| observer_selection | type not stated in source; see Field sources |


| Property | Description |
| --- | --- |
| `target_family` | Exactly one of `ocl`, `sysml-kerml` or `fretish`. |
| `target_standard_refs` | Ordered exact normative/tool references required by the selected family. |
| `mapping_profile_id` | One of `quire.output.ocl24/v1`, `quire.output.sysml2-kerml1/v1` or `quire.output.fretish31/v1`. |
| `required_capabilities` | Closed requested subset of the selected mapping profile. |
| `observer_selection` | Optional, independent structural observer/tool selection; never part of semantic support. |

## Identity and comparison

- **Identity kind** — *value*. Equality is componentwise over every property
  except `observer_selection`, which is a separate downstream observation input.
- **Key** — target family, ordered target references, mapping profile identity
  (the name and version spelled by `mapping_profile_id`) and required capabilities.
  No content digest of the profile or of a target standard is carried or compared;
  the name and version are the identity. Missing members compare only to
  absence and make the profile invalid; no installed/default version is inferred.
- **Key order** — none. Canonical serialization fixes byte order but carries no
  preference, support or causal meaning.
- **Causal order** — the mapping profile is selected before target dispatch; an
  optional observer selection is downstream and cannot alter it.
- **Absence** — an absent profile or key member refuses dispatch. An absent
  observer means no structural observation is requested.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-297-AC-1 | Exactly one target family and matching mapping profile identity are selected before dispatch. | Test (TC-150) |
| FR-297-AC-2 | A missing, unknown or cross-family profile member refuses without target output. | Test (TC-154) |
| FR-297-AC-3 | Changing a target standard reference, the mapping profile name or version, or a requested capability changes profile equality; no digest member exists to be compared. | Test (TC-150) |
| FR-297-AC-4 | Adding or changing a structural observer selection does not change the semantic mapping profile or support claim. | Test (TC-155) |

## Dependencies

- [FR-120](../../functional/output-mapping/FR-120-select-output-mapping-profile.md)
  defines selection behavior.
- [Common output-mapping contract](../../../proposals/quire-v1/output-mappings/common.md)
  fixes the subsystem boundary.

## Status

Accepted by FS06 as the exact target consumer/profile selection value.
