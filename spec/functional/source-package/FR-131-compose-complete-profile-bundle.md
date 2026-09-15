---
id: FR-131
title: "Compose the complete V1 profile bundle"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
---

# FR-131: Compose the complete V1 profile bundle

## Description

When a source package selects complete V1, the linker SHALL resolve a
dependency-closed profile bundle covering every subsystem required by the
176-row capability inventory.

## Inputs

Exact edition, semantic definitions, imported packages and required features.

## Outputs

An immutable linked package with one resolved feature graph, or a located
missing/conflicting-definition refusal.

## Behavior

The bundle includes language/package, value/model/expression, temporal,
observation, protocol, runtime, method-plan, IR/backend, tooling/mapping and
evidence contracts. A partial bundle remains valid only under its own identity
and cannot claim complete V1. Runtime and backend availability do not alter the
linked semantic feature graph.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-131-AC-1 | A dependency-closed selection containing all inventory-owned facets links as complete V1. | Test (TC-180) |
| FR-131-AC-2 | Removing one required facet identifies that dependency and refuses the complete-V1 claim. | Test (TC-180) |
| FR-131-AC-3 | Installing a backend with broader or narrower support does not change the linked package identity or admission. | Test (TC-180) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
