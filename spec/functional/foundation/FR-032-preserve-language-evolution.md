---
id: FR-032
title: "Refuse unsupported required features"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
---
## Description

When a reader meets a source or artifact that selects a required feature (a native operator, declaration form, profile or semantic family) it does not support, it SHALL refuse with a located unsupported-selection refusal.

## Inputs

A source subject or artifact, its exact definition selections and required-feature set, and the reader's supported feature set.

## Outputs

The subject interpreted under its selected definitions, or a located unsupported-selection refusal.

## Behavior

Edition, semantic-profile, wire/canonicalization and backend implementation identities are separate dimensions. A selected required feature the reader does not support refuses; the reader never drops it or evaluates a smaller subset as the whole subject. Optional metadata is ignorable only where the selected wire contract explicitly permits it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-032-AC-3 | A reader that lacks the selected new required feature refuses instead of dropping it or evaluating a smaller subset as the whole subject. | Test (TC-032) |

## Dependencies

- [Shared drafting foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Composed architecture](../../assurance/AD-001-composed-native-language.md).
