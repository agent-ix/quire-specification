---
id: FR-177
title: "Check protocol implementation refinement"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
---

# FR-177: Check protocol implementation refinement

## Description

When refinement is requested, the analyzer SHALL compare an implementation
model with its specification under the selected trace, failure and observation
refinement relation.

## Inputs

Specification graph, implementation graph/model, correspondence, environment
assumptions, refinement profile and bounds.

## Outputs

Refines with proof record, violates with counterexample, inconclusive,
unsupported or failed.

## Behavior

The relation declares permitted internal steps, observations, failures,
divergence and dynamic identities. Every implementation behavior must map to a
permitted specification behavior under the exact correspondence. Extra traces,
deadlock, divergence or stronger environment assumptions produce a witness.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-177-AC-1 | A conforming implementation model returns a proof record naming the exact refinement relation and assumptions. | Test (TC-208) |
| FR-177-AC-2 | An extra visible trace, deadlock or assumption strengthening returns a minimal paired counterexample. | Test (TC-208) |
| FR-177-AC-3 | Missing correspondence or unsupported dynamic behavior returns inconclusive/unsupported rather than refines. | Test (TC-208) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
