---
id: FR-149
title: "Apply the complete typed equality matrix"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-009
    type: references
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
---

# FR-149: Apply the complete typed equality matrix

## Description

When evaluating equality, the semantic kernel SHALL select exactly the equality
relation declared for the operands' common complete-V1 type.

## Inputs

Two typed values and the exact selected equality operation/profile.

## Outputs

Boolean equality, typed incomparable refusal or incomplete nested comparison.

## Behavior

The matrix distinguishes scalar numeric rules, enum declaration identity, text
profile, structural records/tuples, collection algebra, object-reference
identity and IEEE numeric/total/bitwise relations. No structural resemblance,
implicit conversion or local-name equality creates a common type.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-149-AC-1 | Every complete-V1 value kind has positive, negative and cross-type equality vectors. | Test (TC-194) |
| FR-149-AC-2 | Records compare structurally while object references compare qualified identity even when referenced fields match. | Test (TC-194) |
| FR-149-AC-3 | Values without a selected common equality relation refuse rather than returning false. | Test (TC-194) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
