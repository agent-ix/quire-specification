---
id: FR-303
title: "Format and diagnose native source incrementally"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-302
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-303: Format and diagnose native source incrementally

## Description

When an editor requests formatting or language diagnostics, the tool SHALL
operate on the lossless CST and publish deterministic source edits and typed
incremental diagnostics for the exact document revision.

## Inputs

Lossless CST, formatting profile, document revision and incremental change set.

## Outputs

Non-overlapping edits, versioned diagnostics, symbols/navigation/completion or
typed refusal.

## Behavior

Formatting is idempotent and preserves semantic tokens/string spellings under
the selected edition. Diagnostics carry catalog codes and source/runtime loci,
never message-derived identity. Stale revision responses are marked stale and
cannot be applied. Language-server requests use the same typed parser/checker.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-303-AC-1 | Formatting twice produces no further edits and rechecking preserves the same semantic package. | Test (TC-223) |
| FR-303-AC-2 | Incremental and full reparse/check produce equivalent diagnostics and symbols for the same bytes. | Test (TC-223) |
| FR-303-AC-3 | A stale document revision or overlapping/semantic-changing edit set refuses application. | Test (TC-223) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
