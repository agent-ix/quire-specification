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

The implementation target is `ix:native`, edition `0-draft`,
`state-finite/0-draft`, defined by the `proposals/state-core` definition and
schema files whose bytes `proposals/state-core/SHA256SUMS` pins; the fixtures
pin those bytes as well (`typing-cases.json`, `reference-cases.json` and
`fixtures/SHA256SUMS`). Grammar admission remains distinct from successful
typing, execution and backend support.

FS01 accepts `Quire` as the product/language name, `native Quire` as its
descriptive source-language name and `ix:native` as its stable machine identity.
It also accepts the composed `1-draft` profile hierarchy recorded by
[AD-003](../../assurance/AD-003-native-quire-language-charter.md). When a
frontend recognizes that hierarchy, the frontend shall select semantic profiles
by exact identity, revision and digest without treating recognition as compiler
or backend support.

The earlier `state-finite/1-draft` replacement proposal is superseded as an
adoption candidate. Its bounded admission constraints are carried into the
composed state definitions and [FR-019](FR-019-admit-native-source-profile.md),
but its identity is neither enabled nor aliased to a historical or composed
profile. When a selected definition is unavailable, the frontend shall refuse
at the selected source location without falling back to another definition.
The standard shall preserve historical definition bytes and identities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-001-AC-1 | A unit selecting the admitted initial definition receives that exact definition identity. | Test |
| FR-001-AC-2 | A unit selecting an unknown profile receives unknown_profile. | Test |
| FR-001-AC-3 | A changed definition digest prevents reuse of the earlier semantic subject. | Test |
| FR-001-AC-4 | An unavailable selected profile receives a located refusal; selecting it cannot execute under another profile or reuse that profile's semantic subject. | Test |

## Dependencies

- [US-001](../../usecase/US-001-understand-native-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/profile.md) supplies the scoped context.
- [Native-source amendment](../../../proposals/native-source-1-draft/README.md) preserves the superseded candidate and its carried-forward constraints.
- [AD-003](../../assurance/AD-003-native-quire-language-charter.md) fixes the language name, profile hierarchy and support-stage boundaries.

## Status

Accepted FS01 selection and evolution requirement. The released historical
profile implements a bounded subset; each composed semantic profile and backend
retains its independently tracked implementation and qualification status.
