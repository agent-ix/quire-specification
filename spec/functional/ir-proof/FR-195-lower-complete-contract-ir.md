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
reachable dependencies and a canonical semantic digest. The complete-V1
lowerer consumes an admitted `quire.checked-package/v2` package through the I04
V2 reader and requires the nominal identity evidence carried by that contract.
The V2 reader refuses an unknown member or node tag before admission. A transport
relay
may retain the raw bytes of a separately selected future graph revision, but a
lowerer that does not declare that revision refuses it before lowering; raw
retention is not admission or a substitute node.

The frozen V1 reader and public Rust types remain a compatibility surface. They
are not the complete-V1 lowerer input and are never widened with V2 members. A
V1 package reaches the V2 lowerer only through I04's explicit re-link/re-check
migration, which returns a distinct V2 package plus typed package/node
correspondence; V1 bytes alone are insufficient.

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
- [I04 CheckedPackage contract](../../objects/interfaces/FR-322-checked-package-artifact.md)
  owns the V2 complete-V1 checked semantic input, the frozen V1 compatibility
  path and their strict version-specific reader/migration boundaries.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
