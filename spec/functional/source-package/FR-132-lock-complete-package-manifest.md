---
id: FR-132
title: "Lock every complete-package identity and capability"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
  - target: ix://agent-ix/quire-specification/FR-030
    type: references
---

# FR-132: Lock every complete-package identity and capability

## Description

When publishing a checked package, the publisher SHALL emit one canonical
manifest that locks every semantic, producer, dependency and capability
selection in its own identity domain.

## Inputs

Checked package, source correspondence, model exports, semantic definitions and
declared consumer requirements.

## Outputs

A canonical package manifest and digest, or a typed identity/capability refusal.

## Behavior

The manifest separates source, semantic, producer-export, package, generated
artifact and tool identity domains. Its required-feature set contains atomic
language and consumer capabilities, not only family flags. It records original
and effective declarations, analysis plans, plugins and generated artifacts by
typed reference without including runtime observations in static identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-132-AC-1 | Canonical manifests with reordered set-valued entries have equal identities while ordered semantic entries remain order-sensitive. | Test (TC-182) |
| FR-132-AC-2 | Substituting a Filament fingerprint, package digest or generated-artifact digest across identity domains refuses. | Test (TC-182) |
| FR-132-AC-3 | Every atomic required capability is discoverable and an unknown required capability refuses rather than being dropped. | Test (TC-182) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
