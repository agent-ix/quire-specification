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
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
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

These five outcomes are the FR-360 result labels read through the FR-379
verdict table, which fixes each label's FR-241 execution, FR-242 truth and
FR-243 settlement-basis combination; this requirement adds no label and no
combination. `refines` is `proved`, `violates` is `refuted`, and
`inconclusive`, `unsupported` and `failed` are the same-named labels. A
resource-limit stop is `failed` with FR-241 `resource-incomplete`, FR-331
result `incomplete`, never `refines` or `violates`.

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

## Relation over the model-refinement step rules

The relation is decided by the step rules of
[FR-377](../analysis/FR-377-decide-model-refinement.md) on the product of the
two explored protocol models, with one step function and one counterexample
type:

- a permitted internal implementation action is a `stutter` step: it passes
  when the corresponding specification state is unchanged;
- a visible implementation step is a step-map row whose label is its
  observation: it passes when a specification step with the same observation
  takes the corresponding state to the corresponding successor, after zero or
  more permitted specification internal steps;
- the paired initial states are FR-377's initial-state rule.

Four checks are protocol-only and run over the same product edges and
states:

- **divergence-freedom**: when the profile makes divergence observable, a loop
  of internal implementation steps at a specification state that can move is
  a violation whatever the fairness sets are, where FR-377 reads stutter tails
  only through abstract fairness;
- **refusal sets**: every implementation refusal is allowed by the matched
  specification state; a violation is the `RefusalNotPermitted` refinement
  failure of [FR-379](../analysis/FR-379-report-and-replay-refinement-counterexamples.md);
- **terminal success**: implementation terminal success matches specification
  terminal success; a violation is the `TerminalSuccessMismatch` refinement
  failure of FR-379;
- **assumption weakening**: implementation environment assumptions are no
  stronger than the specification's.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-177-AC-1 | A conforming implementation model returns a proof record naming the exact refinement relation and assumptions. | Test (TC-208) |
| FR-177-AC-2 | An extra visible trace, deadlock or assumption strengthening returns a minimal paired counterexample. | Test (TC-208) |
| FR-177-AC-3 | Missing correspondence or unsupported dynamic behavior returns inconclusive/unsupported rather than refines. | Test (TC-208) |
| FR-177-AC-4 | Internal-step stuttering preserves correspondence and cannot hide a catastrophic divergence. | Test (TC-208) |
| FR-177-AC-5 | Failure-set inclusion and assumption weakening are checked independently from visible-trace inclusion. | Test (TC-208) |
| FR-177-AC-6 | Visible-step matching, internal steps and paired initial states are decided by FR-377's step rules with internal actions as `stutter` steps and observations as step labels; an observable internal loop at a specification state that can move refutes as a divergence under every fairness set, including an empty one, while the same loop in a model refinement with empty abstract fairness holds. | Test (TC-318) |
| FR-177-AC-7 | A refusal-set violation and a terminal-success mismatch refute with the FR-379 kinds `RefusalNotPermitted` and `TerminalSuccessMismatch`, defined with their payloads, checks and replay there; a permitted refusal and matching terminal success hold. | Test (TC-318) |
| FR-177-AC-8 | `refines` settles FR-360 `proved` and `violates` settles `refuted`, each with the FR-241 and FR-242 values FR-360 names for it and the FR-243 settlement basis the FR-379 verdict table gives the outcome; `inconclusive`, `unsupported` and `failed` settle the same-named labels; a refinement run stopped by a resource limit settles `failed` with FR-241 `resource-incomplete`, never `proved` or `refuted`. | Test (TC-318) |

## Dependencies

- [FR-377](../analysis/FR-377-decide-model-refinement.md) owns the step rules,
  product and counterexample type this relation is decided on.
- [FR-379](../analysis/FR-379-report-and-replay-refinement-counterexamples.md)
  owns the verdict table and [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  the result vocabulary that this requirement's outputs are read through.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.

## References

- Design: quire-spec-language ADR-020 (MC-1, QS-8, RU-2).
- Tickets: Linear STD-133; paired QSL-367.
