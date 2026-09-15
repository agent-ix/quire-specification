---
id: FR-175
title: "Analyze protocol realizability"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-060
    type: references
  - target: ix://agent-ix/quire-specification/FR-174
    type: references
---

# FR-175: Analyze protocol realizability

## Description

When realizability is requested, the analyzer SHALL determine whether compliant
local role strategies can realize the global protocol under the selected
communication and environment premises.

## Inputs

Checked global graph, role knowledge/visibility, channel/environment assumptions,
strategy profile and bounds.

## Outputs

Realizable with strategy witnesses, unrealizable with counterexample,
inconclusive, unsupported or failed.

## Behavior

Realizability is distinct from trace conformance and projection syntax. Strategy
witnesses identify role observations and choices. Counterexamples identify the
indistinguishable local histories or environment behavior preventing a valid
strategy. Assumptions are never inferred from successful samples.

## Strategy game

The environment chooses admitted external events and channel outcomes; each role
chooses enabled local actions from only its local history and declared shared
facts. A joint strategy is winning when every maximal play consistent with the
environment premises satisfies the global safety, progress and terminal
obligations. Strategy witnesses map equivalence classes of indistinguishable
local histories to actions. Unrealizability is witnessed by two histories that
one role cannot distinguish but that require incompatible actions, or by an
admitted environment play that defeats every role choice.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-175-AC-1 | A realizable protocol returns replayable local strategy witnesses under exact premises. | Test (TC-206) |
| FR-175-AC-2 | An unrealizable hidden-choice protocol returns the conflicting local histories and violated global obligation. | Test (TC-206) |
| FR-175-AC-3 | Missing environment/channel premises or analysis limits return inconclusive rather than realizable. | Test (TC-206) |
| FR-175-AC-4 | A witness strategy selects the same action for every pair of locally indistinguishable histories. | Test (TC-206) |
| FR-175-AC-5 | Realizability quantifies over every admitted environment play rather than only sampled executions. | Test (TC-206) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
