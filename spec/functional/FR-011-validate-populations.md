---
id: FR-011
title: "Validate complete immutable runtime populations"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: implements
---
# FR-011: Validate complete immutable runtime populations

## Description

When predicate evaluation is requested, the runtime validator shall validate every required population entry before evaluating the predicate.

## Inputs

Immutable typed populations, completeness declarations, required observations and field/reference values.

## Outputs

Validated snapshots or invalid/incomplete input diagnostics.

## Behavior

Duplicate identities, wrong types and dangling targets in a claimed complete universe are invalid. Missing observations and incomplete universes remain incomplete. Optional absence differs from nullable and unavailable data. Skipped predicate branches do not bypass population validation. A completeness declaration is an input assumption, not deployment-wide proof.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-011-AC-1 | Duplicate object identities receive duplicate_object. | Test |
| FR-011-AC-2 | A dangling target in a complete universe receives dangling_reference. | Test |
| FR-011-AC-3 | An incomplete required universe receives incomplete_population. | Test |
| FR-011-AC-4 | Unavailable required data yields no Boolean value. | Test |
| FR-011-AC-5 | A skipped field containing an invalid reference still refuses the input. | Test |

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/boundary.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
