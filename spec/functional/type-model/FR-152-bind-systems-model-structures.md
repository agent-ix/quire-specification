---
id: FR-152
title: "Bind complete systems-model structures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---

# FR-152: Bind complete systems-model structures

## Description

When a clause references a systems-model element, the binder SHALL resolve its
typed part, port, interface, connection or allocation identity through the exact
producer correspondence.

## Inputs

Checked package, authoritative model export and qualified element reference.

## Outputs

A typed resolved model element/relationship or located kind, identity or
closure refusal.

## Behavior

Parts, ports, interfaces, connections and allocations retain distinct kinds and
their declared endpoint/direction/type constraints. Display names and equal
shapes are not identities. Navigation follows only explicit producer relations
and preserves original/effective declaration provenance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-152-AC-1 | Valid references to every supported systems-model kind resolve with exact producer and effective-declaration identity. | Test (TC-197) |
| FR-152-AC-2 | Substituting a part for a port or a connection for an allocation refuses even when names match. | Test (TC-197) |
| FR-152-AC-3 | Missing relationship closure or an ambiguous effective target refuses without ambient lookup. | Test (TC-197) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
