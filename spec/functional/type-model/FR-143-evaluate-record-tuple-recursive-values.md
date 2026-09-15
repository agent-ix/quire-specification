---
id: FR-143
title: "Evaluate records, tuples and finite recursive values"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-009
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-143: Evaluate records, tuples and finite recursive values

## Description

When checking a record, tuple or recursive value, the semantic kernel SHALL
apply its declared field order, types, structural equality and finite-value
construction rules.

## Inputs

Named record or positional tuple definitions, recursive constructors, values
and traversal limits.

## Outputs

A typed finite value/equality result, refusal or incomplete traversal outcome.

## Behavior

Record fields use declaration-owned identities; tuple positions are ordered.
Structural equality recursively compares the same declared value type and does
not compare object-reference targets structurally. Recursive definitions are
allowed, but each runtime value must be finite and cycles require explicit
object identity rather than value containment.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-143-AC-1 | Equal finite values of the same declared record/tuple type compare structurally equal. | Test (TC-188) |
| FR-143-AC-2 | Equal field shapes from different declared types do not become interchangeable. | Test (TC-188) |
| FR-143-AC-3 | Cyclic value containment refuses, while bounded recursive construction and object-reference cycles remain distinct. | Test (TC-188) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
