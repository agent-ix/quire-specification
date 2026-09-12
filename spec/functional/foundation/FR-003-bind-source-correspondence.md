---
id: FR-003
title: "Bind clause locations to exact source bytes"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-002"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-202
    type: references
  - target: ix://agent-ix/quire-specification/FR-203
    type: references
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
---
# FR-003: Bind clause locations to exact source bytes

## Description

When a clause location is exchanged, the producer shall bind it to the exact authored source document.

## Inputs

Original document identity/revision/digest, clause and expression regions, optional extracted body and segment map.

## Outputs

Verified original locations or source-correspondence diagnostics.

## Behavior

Regions use zero-based half-open UTF-8 byte offsets and one-based scalar columns. A map may delete only the explicitly selected indentation/CRLF/final-newline layout. Fence ownership and explicit positive numeric IR revision mapping are separate obligations. The extractor treats expressions as opaque.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-003-AC-1 | The selected expression region reproduces its declared digest. | Test |
| FR-003-AC-2 | A span splitting a UTF-8 scalar receives invalid_source_map. | Test |
| FR-003-AC-3 | A segment map that deletes an interior source token receives invalid_source_map. | Test |
| FR-003-AC-4 | A missing numeric IR revision mapping refuses executable projection. | Test |

## Dependencies

- [US-002](../../usecase/US-002-exchange-exact-subjects.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/boundary.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
