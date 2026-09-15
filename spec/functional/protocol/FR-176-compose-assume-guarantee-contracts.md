---
id: FR-176
title: "Compose assume-guarantee protocol contracts"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
---

# FR-176: Compose assume-guarantee protocol contracts

## Description

When composing protocol contracts, the analyzer SHALL prove compatibility and
discharge each component guarantee only from declared environment assumptions
and independently established sibling guarantees.

## Inputs

Component protocols, typed interfaces, assumptions, guarantees, dependency graph
and selected proof profile.

## Outputs

Compatible composition with discharged obligations, incompatibility witness,
circularity refusal or inconclusive result.

## Behavior

Composition aligns exact role/message/channel/value identities. Every guarantee
records the assumptions used. A dependency cycle is accepted only with an
explicit sound fixed-point rule supported by the selected profile; otherwise it
refuses as circular reasoning.

## Discharge fixed point

The analyzer forms a dependency graph whose nodes are qualified guarantees and
whose edges name the assumptions used in their proofs. Acyclic nodes are
discharged in topological order. A cyclic strongly connected component is
accepted only when the selected profile declares a monotone complete lattice,
initial element and convergence rule; iteration continues to its least fixed
point or returns inconclusive at the resource bound. No guarantee in a cycle is
available as an assumption before that rule establishes the fixed point.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-176-AC-1 | Compatible components produce a composed graph and complete assumption-to-guarantee discharge record. | Test (TC-207) |
| FR-176-AC-2 | Type/protocol premise mismatch produces the smallest incompatible interface witness. | Test (TC-207) |
| FR-176-AC-3 | An unsupported circular proof dependency refuses without treating the cycle as self-justifying. | Test (TC-207) |
| FR-176-AC-4 | A supported cyclic composition records every fixed-point iteration and its monotone convergence premise. | Test (TC-207) |
| FR-176-AC-5 | Changing a component identity, interface type or environment assumption invalidates every dependent discharge. | Test (TC-207) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
