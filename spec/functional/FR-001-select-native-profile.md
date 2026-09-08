---
id: FR-001
title: "Select an explicit native language profile"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-001"
    type: implements
---
# FR-001: Select an explicit native language profile

## Description

When a source unit declares its language and profile, the conforming frontend shall select the exact admitted definition.

## Inputs

UTF-8 source with language tag, edition and semantic profile declarations.

## Outputs

Selected definition identity or a located unsupported-profile diagnostic.

## Behavior

The initial target is ix:native, edition 0-draft, state-finite/0-draft. Grammar admission remains distinct from successful typing or execution. A later semantic amendment requires a newly identified definition. The approved initial target and separately proposed FS03 refinements remain distinguishable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-001-AC-1 | A unit selecting the admitted initial definition receives that exact definition identity. | Test |
| FR-001-AC-2 | A unit selecting an unknown profile receives unknown_profile. | Test |
| FR-001-AC-3 | A changed definition digest prevents reuse of the earlier semantic subject. | Test |

## Dependencies

- [US-001](../usecase/US-001-understand-native-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/profile.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
