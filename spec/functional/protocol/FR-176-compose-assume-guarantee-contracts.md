---
id: FR-176
title: "Compose assume-guarantee protocol contracts"
type: FR
relationships:
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-176-AC-1 | Compatible components produce a composed graph and complete assumption-to-guarantee discharge record. | Test (TC-207) |
| FR-176-AC-2 | Type/protocol premise mismatch produces the smallest incompatible interface witness. | Test (TC-207) |
| FR-176-AC-3 | An unsupported circular proof dependency refuses without treating the cycle as self-justifying. | Test (TC-207) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
