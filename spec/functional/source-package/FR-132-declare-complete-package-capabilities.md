---
id: FR-132
title: "Declare every complete-package capability and identity domain"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
  - target: ix://agent-ix/quire-specification/FR-030
    type: references
---

# FR-132: Declare every complete-package capability and identity domain

## Description

When publishing a checked package, the publisher SHALL declare every atomic
required capability of the package and keep each identity it carries in its
own identity domain.

## Inputs

Checked package, source correspondence, selected domain packages, semantic definitions and
declared consumer requirements.

## Outputs

A checked package carrying its required-feature set, or a typed
identity/capability refusal.

## Behavior

The package separates source, semantic, domain package, package and generated
artifact identity domains. Its required-feature set contains atomic language
and consumer capabilities, not only family flags. It records original and
effective declarations, analysis plans, plugins and generated artifacts by
typed reference without including runtime observations in static identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-132-AC-2 | Substituting a lock-recorded domain package `sha256-jcs` digest, package digest or generated-artifact digest across identity domains refuses. | Test (TC-182) |
| FR-132-AC-3 | Every atomic required capability is discoverable and an unknown required capability refuses rather than being dropped. | Test (TC-182) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
