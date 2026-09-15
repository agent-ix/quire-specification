---
id: FR-175
title: "Analyze protocol realizability"
type: FR
relationships:
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-175-AC-1 | A realizable protocol returns replayable local strategy witnesses under exact premises. | Test (TC-206) |
| FR-175-AC-2 | An unrealizable hidden-choice protocol returns the conflicting local histories and violated global obligation. | Test (TC-206) |
| FR-175-AC-3 | Missing environment/channel premises or analysis limits return inconclusive rather than realizable. | Test (TC-206) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
