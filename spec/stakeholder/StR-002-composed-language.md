---
id: StR-002
title: "Express state, temporal and choreography obligations together"
type: StR
relationships:
  - target: ix://agent-ix/quire-specification/FR-030
    type: satisfied_by
  - target: ix://agent-ix/quire-specification/FR-035
    type: satisfied_by
  - target: ix://agent-ix/quire-specification/FR-041
    type: satisfied_by
  - target: ix://agent-ix/quire-specification/FR-020
    type: satisfied_by
---
## Stakeholder Need

Specification authors require that native Quire V1 shall express, execute and
analyze the complete declared language capability inventory under one source
authority while preserving exact meaning across runtimes, monitors, proof
backends, mappings, tools and evidence consumers.

## Rationale

A state-only or bounded-slice language cannot express the required value/type,
model, temporal, protocol, analysis and workflow capabilities. Separate authored
languages risk inconsistent types, identities and assumptions. An implementation
or backend slice must not redefine, narrow or defer the intended product.

## Validation Criteria

| ID | Criteria | Validation |
| --- | --- | --- |
| StR-002-VC-1 | One native package states complete value/model semantics, state conditions, bounded and unbounded temporal obligations, and dynamic compensated choreography with shared model and source identity. | Demonstration |
| StR-002-VC-2 | Healthy, violating, untriggered, incomplete and unsupported requests remain distinguishable in the composed workflow. | Demonstration |
| StR-002-VC-3 | A reviewer can trace every one of the 194 V1 inventory capabilities to normative semantics, an implementation owner, delivery tickets and qualification evidence without a deferred product-scope disposition. | Inspection |
| StR-002-VC-4 | One enterprise example relates concurrent orders, payment/shipment/refund instances and a declared finite population/window without treating unknown membership as successful aggregate evidence. | Demonstration |
| StR-002-VC-5 | Native runtime, model testing, simulation, formal analysis, proof generation, authoring tools, output mappings and authorized workflow adapters consume the same immutable semantic package and report typed capability limitations without approximation. | Demonstration |

## Dependencies

- [V1 capability matrix](../../proposals/quire-v1/capabilities.md).
- [Existing state need](StR-001-native-assessment-trust.md).
