---
id: FR-190
title: "Generate and shrink model-based tests"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-190: Generate and shrink model-based tests

## Description

When model-based testing is selected, the generator SHALL derive valid inputs
and shrink candidates from the exact model domains while using native execution
as the semantic oracle.

## Inputs

Checked package, model domains, preconditions, generator/shrinker profile, seed,
case limits and native runtime selection.

## Outputs

Case results, minimal retained counterexample and complete generation/replay
provenance.

## Behavior

Generation covers declared boundaries and relationships, including empty,
minimum, maximum and just-outside refusal inputs. Shrinking preserves domain
validity and the failing verdict. A rejected precondition is not a passing case.
Every retained counterexample serializes for native replay.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-190-AC-1 | Generated cases inhabit the exact declared domains and include required boundary partitions. | Test (TC-212) |
| FR-190-AC-2 | Shrinking terminates at a domain-valid counterexample with the same clause verdict. | Test (TC-212) |
| FR-190-AC-3 | Same package/generator/seed/options reproduce cases; invalid shrink or oracle disagreement is a method failure. | Test (TC-212) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
