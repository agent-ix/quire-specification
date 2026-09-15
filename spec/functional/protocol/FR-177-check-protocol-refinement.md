---
id: FR-177
title: "Check protocol implementation refinement"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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

## Refinement relation

The profile defines visible observations, internal actions, refusal sets and
whether divergence is catastrophic. A correspondence relation starts at paired
initial states. Every visible implementation step must match a specification
step with the same observation after zero or more permitted specification
internal steps; every implementation refusal must be allowed by the matched
specification state; terminal success must match terminal success. Internal
implementation steps must preserve correspondence and cannot hide infinite
divergence when divergence is observable. Environment assumptions in the
implementation must be no stronger than those of the specification.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-177-AC-1 | A conforming implementation model returns a proof record naming the exact refinement relation and assumptions. | Test (TC-208) |
| FR-177-AC-2 | An extra visible trace, deadlock or assumption strengthening returns a minimal paired counterexample. | Test (TC-208) |
| FR-177-AC-3 | Missing correspondence or unsupported dynamic behavior returns inconclusive/unsupported rather than refines. | Test (TC-208) |
| FR-177-AC-4 | Internal-step stuttering preserves correspondence and cannot hide a catastrophic divergence. | Test (TC-208) |
| FR-177-AC-5 | Failure-set inclusion and assumption weakening are checked independently from visible-trace inclusion. | Test (TC-208) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
