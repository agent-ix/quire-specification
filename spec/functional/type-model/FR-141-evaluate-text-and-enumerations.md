---
id: FR-141
title: "Evaluate text and declaration-qualified enumerations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-141: Evaluate text and declaration-qualified enumerations

## Description

When checking text or enumeration values, the semantic kernel SHALL apply the
selected text normalization and declaration-qualified member identity rules.

## Inputs

Text profile, text bounds, enum declarations, literals and comparison requests.

## Outputs

Typed text/enum values, comparison results or located refusals.

## Behavior

Text equality and ordering operate on the selected Unicode scalar/normalization
profile and enforce declared length bounds. Enumeration members are identified
by declaration identity plus member identity; source order defines ordering
only when the declaration explicitly selects ordered enumeration semantics.
Evolution never reuses a removed member identity for different meaning.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-141-AC-1 | Canonically equivalent text compares according to the selected normalization profile and distinct text remains distinct. | Test (TC-186) |
| FR-141-AC-2 | Equal member spellings from different enum declarations are not equal. | Test (TC-186) |
| FR-141-AC-3 | Missing text bounds, invalid scalars, unordered enum ordering or reused member identity refuses. | Test (TC-186) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
