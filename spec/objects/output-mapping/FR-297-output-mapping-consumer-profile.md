---
id: FR-297
title: "Output-mapping consumer profile"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-297: Output-mapping consumer profile

## Description

When selecting an output consumer, the mapping coordinator SHALL identify the
target family, normative target revisions and Quire mapping profile by exact
identity, revision and digest rather than by a display name or installed tool.

## Properties

| Property | Description |
| --- | --- |
| `target_family` | Exactly one of `ocl`, `sysml-kerml` or `fretish`. |
| `target_standard_refs` | Ordered exact normative/tool references required by the selected family. |
| `mapping_profile_id` | One of `quire.output.ocl24/v1`, `quire.output.sysml2-kerml1/v1` or `quire.output.fretish31/v1`. |
| `mapping_revision` | Exact selected revision; FS06 selects `1-draft.1` for each initial profile. |
| `mapping_digest` | Raw digest of the selected mapping rule bytes under FR-201. |
| `required_capabilities` | Closed requested subset of the selected mapping profile. |
| `observer_selection` | Optional, independently versioned structural observer/tool selection; never part of semantic support. |

## Identity and comparison

- **Identity kind** — *value*. Equality is componentwise over every property
  except `observer_selection`, which is a separate downstream observation input.
- **Key** — target family, ordered target references, mapping profile identity,
  revision, digest and required capabilities. Missing members compare only to
  absence and make the profile invalid; no installed/default version is inferred.
- **Key order** — none. Canonical serialization fixes byte order but carries no
  preference, support or causal meaning.
- **Causal order** — the mapping profile is selected before target dispatch; an
  optional observer selection is downstream and cannot alter it.
- **Absence** — an absent profile or key member refuses dispatch. An absent
  observer means no structural observation is requested.
- **Digest domain** — `sha256-raw` for mapping rule bytes. Target standard
  references and tool versions are identifiers, not substitutes for the digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-297-AC-1 | Exactly one target family and matching mapping profile/revision/digest are selected before dispatch. | Test (TC-150) |
| FR-297-AC-2 | A missing, unknown, cross-family or stale profile member refuses without target output. | Test (TC-154) |
| FR-297-AC-3 | Changing a target standard reference, mapping revision, digest or requested capability changes profile equality. | Test (TC-150) |
| FR-297-AC-4 | Adding or changing a structural observer selection does not change the semantic mapping profile or support claim. | Test (TC-155) |

## Dependencies

- [FR-120](../../functional/output-mapping/FR-120-select-output-mapping-profile.md)
  defines selection behavior.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) defines the raw
  digest domain.
- [Common output-mapping contract](../../../proposals/quire-v1/output-mappings/common.md)
  fixes the subsystem boundary.

## Status

Accepted by FS06 as the exact target consumer/profile selection value.
