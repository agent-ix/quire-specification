---
id: FR-014
title: "Read the selected closed interchange version"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: implements
---
# FR-014: Read the selected closed interchange version

## Description

When a reference envelope is decoded, the reader shall validate the exact selected closed wire contract.

## Inputs

One UTF-8 JSON document, supported version/kind registry and the candidate structural schema.

## Outputs

A structurally valid selected record or a wire diagnostic.

## Behavior

Decoded duplicate keys, unknown members, omitted required members, invalid scalar strings and unsupported versions refuse. requiredFeatures is explicit, duplicate-free and compared as a set. Schema validation is separate from authority, digest, canonicalization and source-map validation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-014-AC-1 | A duplicate decoded member name receives invalid-wire. | Test |
| FR-014-AC-2 | An omitted requiredFeatures member receives invalid-wire. | Test |
| FR-014-AC-3 | An unknown wire version receives unsupported-version. | Test |
| FR-014-AC-4 | A reordered unique feature set compares equal. | Test |
| FR-014-AC-5 | A duplicate required feature receives invalid-wire. | Test |

## Dependencies

- [US-004](../usecase/US-004-retain-evidence-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/interchange.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
