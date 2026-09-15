---
id: FR-152
title: "Bind complete systems-model structures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## Systems-model roles

| Kind | Required identity and constraints |
| --- | --- |
| Part | qualified declaration identity, owning composite, declared type and multiplicity |
| Port | qualified declaration identity, owning part, direction, interface type and multiplicity |
| Interface | qualified declaration identity and ordered feature signatures |
| Connection | qualified relationship identity plus compatible source and target port identities |
| Allocation | qualified relationship identity plus explicitly typed source and target element identities |

An `in` port may be a connection target, an `out` port may be a source and an
`inout` port may be either. Connection endpoint types must conform under FR-151;
allocation compatibility is determined only by the selected producer profile.
Navigation starts from a resolved identity and follows a named relationship
edge in the same closed correspondence bundle.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-152-AC-1 | Valid references to every supported systems-model kind resolve with exact producer and effective-declaration identity. | Test (TC-197) |
| FR-152-AC-2 | Substituting a part for a port or a connection for an allocation refuses even when names match. | Test (TC-197) |
| FR-152-AC-3 | Missing relationship closure or an ambiguous effective target refuses without ambient lookup. | Test (TC-197) |
| FR-152-AC-4 | Port direction, endpoint type and multiplicity are all enforced before a connection is exposed. | Test (TC-197) |
| FR-152-AC-5 | Relationship navigation never crosses correspondence bundles or substitutes display-name equality for identity. | Test (TC-197) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
