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
  - target: ix://agent-ix/quire-specification/FR-134
    type: references
---
# FR-003: Bind clause locations to exact source bytes

## Description

When a clause location is exchanged, the producer shall bind it to the exact authored source document.

## Inputs

Original document identity, clause and expression regions, optional extracted body and segment map.

## Outputs

Verified original locations or source-correspondence diagnostics.

## Behavior

Regions use zero-based half-open UTF-8 byte offsets and one-based scalar columns. A map may delete only the indentation, CRLF and final-newline bytes that its selected container contract marks as container-owned, as [FR-134](../source-package/FR-134-preserve-source-transform-loci.md) states. Fence ownership and the explicit mapping of a requirement's positive authored revision to the IR requirement revision are separate obligations. The extractor treats expressions as opaque.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-003-AC-1 | Withdrawn: the declared document digest is removed. | Withdrawn |
| FR-003-AC-2 | A span splitting a UTF-8 scalar receives invalid_source_map. | Test |
| FR-003-AC-3 | A segment map that deletes an interior source token receives invalid_source_map. | Test |
| FR-003-AC-4 | A missing mapping from a requirement's positive authored revision to the IR requirement revision refuses executable projection. | Test |

## Dependencies

- [US-002](../../usecase/US-002-exchange-exact-subjects.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/boundary.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
