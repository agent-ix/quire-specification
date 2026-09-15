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

## Generation and shrinking contract

Generators are derived from checked finite domains and relationship
constraints, not from implementation types. Partition construction includes
empty, singleton, each inclusive endpoint, adjacent just-outside inputs and
cross-field boundary combinations. Generation is rejection-free after domain
construction; an unsatisfiable domain is a model refusal. Shrinking follows a
pinned well-founded ordering over typed values and accepts a candidate only when
it remains in-domain and native replay preserves the same failing clause and
disposition.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-190-AC-1 | Generated cases inhabit the exact declared domains and include required boundary partitions. | Test (TC-212) |
| FR-190-AC-2 | Shrinking terminates at a domain-valid counterexample with the same clause verdict. | Test (TC-212) |
| FR-190-AC-3 | Same package/generator/seed/options reproduce cases; invalid shrink or oracle disagreement is a method failure. | Test (TC-212) |
| FR-190-AC-4 | Generated partitions include every inclusive endpoint and adjacent refusal value representable by the source type. | Test (TC-212) |
| FR-190-AC-5 | Every shrink step is strictly smaller, domain-valid and replay-equivalent for the failing disposition. | Test (TC-212) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
