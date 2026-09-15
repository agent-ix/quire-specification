---
id: FR-195
title: "Lower the complete language to target-neutral contract IR"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-031
    type: references
---

# FR-195: Lower the complete language to target-neutral contract IR

## Description

When lowering a checked package, the lowerer SHALL preserve every selected type,
expression, state, temporal, protocol, analysis and source-correspondence meaning
in the versioned target-neutral IR or refuse the affected item exactly.

## Inputs

Immutable checked package, lowering profile, requested items and limits.

## Outputs

Cycle-free typed contract package with per-item accounting, or explicit
unsupported/invalid/incomplete dispositions.

## Behavior

The IR carries exact definition/type/value identities, anchors, bounds,
occurrence/source maps and requested claims. Extensions are tagged versioned
nodes with declared dependencies. A missing representation emits no substitute
node and does not prevent independent supported items from lowering.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-195-AC-1 | Every complete-V1 construct has a round-trip semantic vector or an exact per-item unsupported disposition. | Test (TC-217) |
| FR-195-AC-2 | Source/type/anchor/identity/bound mutation is detected before a target artifact is emitted. | Test (TC-217) |
| FR-195-AC-3 | No lowerer approximates an unsupported construct, drops an obligation or changes sibling results. | Test (TC-217) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
