---
id: FR-001
title: "Select an explicit native language profile"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-001"
    type: traces_to
---
# FR-001: Select an explicit native language profile

## Description

When a source unit declares its language and profile, the conforming frontend shall select the exact admitted definition.

## Inputs

UTF-8 source with language tag, edition and semantic profile declarations.

## Outputs

Selected definition identity or a located unsupported-profile diagnostic.

## Behavior

The adopted internal implementation target is ix:native, edition 0-draft,
state-finite/0-draft, bound to the definitions adopted at e897f81. Grammar
admission remains distinct from successful typing or execution.

The native-source ruling amendment proposes state-finite/1-draft under the same
language and edition. This is a replacement definition candidate, not a second
clause language or an already enabled compiler profile. Its admission contract
is [FR-019](FR-019-admit-native-source-profile.md). A frontend shall reject an
unavailable selected definition without falling back to the earlier definition.
The standard shall preserve the historical definition bytes and their identities
when adopting the replacement. The candidate's final revision/digests and
implementation adoption remain explicit review work.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-001-AC-1 | A unit selecting the admitted initial definition receives that exact definition identity. | Test |
| FR-001-AC-2 | A unit selecting an unknown profile receives unknown_profile. | Test |
| FR-001-AC-3 | A changed definition digest prevents reuse of the earlier semantic subject. | Test |
| FR-001-AC-4 | An unavailable replacement profile receives a located refusal; selecting it cannot execute under the earlier profile or reuse that profile's semantic subject. | Test |

## Dependencies

- [US-001](../usecase/US-001-understand-native-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/profile.md) supplies the scoped context.
- [Native-source amendment](../../proposals/native-source-1-draft/README.md) states the replacement candidate and retained admission gaps.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
