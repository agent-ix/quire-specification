---
id: FR-149
title: "Apply the complete typed equality matrix"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## Equality matrix

| Value kind | Equality relation |
| --- | --- |
| Boolean | identical truth value |
| Integer, rational and decimal | exact mathematical value after an explicitly declared lossless common-type conversion |
| Quantity | identical dimension and exact equality after explicit conversion to the selected canonical unit |
| Text | identical code-point sequence under the same pinned text profile |
| Enumeration | identical qualified enumeration declaration and case |
| Record and tuple | same declared structural type and recursively equal fields in declaration order |
| Sequence | equal length and recursively equal values at every index |
| Set | recursively equal members independent of iteration order |
| Bag | recursively equal members with equal multiplicity |
| Ordered set | recursively equal unique values in the same occurrence order |
| Object reference | identical qualified object identity; referenced state is not inspected |
| IEEE binary32/binary64 | the explicitly selected numeric equality, `totalOrder` equality or bit identity from FR-148 |

Operands of different types are comparable only after an explicit conversion
declares a common type. If conversion is lossy, undefined or absent, the result
is `ill_typed`; it is never `false`. Conversion creates a comparison value but
does not rewrite the source value's declaration or object identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-149-AC-1 | Every complete-V1 value kind has positive, negative and cross-type equality vectors. | Test (TC-194) |
| FR-149-AC-2 | Records compare structurally while object references compare qualified identity even when referenced fields match. | Test (TC-194) |
| FR-149-AC-3 | Values without a selected common equality relation refuse rather than returning false. | Test (TC-194) |
| FR-149-AC-4 | Positive, negative and cross-type vectors exercise every row of the equality matrix. | Test (TC-194) |
| FR-149-AC-5 | Explicit conversion does not mutate either source value's type, declaration identity or object-reference identity. | Test (TC-194) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
