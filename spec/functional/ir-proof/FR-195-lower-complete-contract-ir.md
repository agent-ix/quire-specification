---
id: FR-195
title: "Lower the complete language to target-neutral contract IR"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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

## IR contract

The versioned IR has declarations for scalar/composite types, bounded domains,
typed expressions, functions, model identities/relations, state frames and
transitions, temporal formulas, protocol control/queues/obligations, claims and
source maps. Every node carries its schema version, semantic type and stable
node identity; references are by identity, never array position. Lowering is
per requested item and returns exactly one of `lowered`, `unsupported`,
`requires_bound`, `invalid_input` or `failed`. A lowered item includes all
reachable dependencies and a canonical semantic digest. Unknown tagged nodes
are retained for transport but refused by consumers that do not declare them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-195-AC-1 | Every complete-V1 construct has a round-trip semantic vector or an exact per-item unsupported disposition. | Test (TC-217) |
| FR-195-AC-2 | Source/type/anchor/identity/bound mutation is detected before a target artifact is emitted. | Test (TC-217) |
| FR-195-AC-3 | No lowerer approximates an unsupported construct, drops an obligation or changes sibling results. | Test (TC-217) |
| FR-195-AC-4 | Every reference resolves by stable identity to a reachable version-compatible node and every node maps to exact source loci. | Test (TC-217) |
| FR-195-AC-5 | Mixed supported and unsupported requests preserve independent per-item dispositions and emit no placeholder semantics. | Test (TC-217) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
