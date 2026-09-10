---
id: StR-002
title: "Express state, temporal and choreography obligations together"
type: StR
relationships:
  - target: ix://agent-ix/quire-specification/FR-020
    type: satisfied_by
---
## Stakeholder Need

Specification authors require that native Quire v1 shall express the selected state, temporal and protocol/choreography obligations of a distributed workflow under one source authority.

## Rationale

A state-only language cannot express participants, correlated messages, deadlines
or compensation. Separate authored languages risk inconsistent types, identities
and assumptions. An implementation slice must not redefine the intended product.

## Validation Criteria

| ID | Criteria | Validation |
| --- | --- | --- |
| StR-002-VC-1 | One native package states a state condition, a timed obligation and a branching compensated choreography with shared model and source identity. | Demonstration |
| StR-002-VC-2 | Healthy, violating, untriggered, incomplete and unsupported requests remain distinguishable in the composed workflow. | Demonstration |
| StR-002-VC-3 | A reviewer can find every intended family, its v1/later/refuse disposition, its owned specification and its implementation task without inferring support from the current state compiler. | Inspection |

## Dependencies

- [V1 capability matrix](../../proposals/quire-v1/capabilities.md).
- [Existing state need](StR-001-native-assessment-trust.md).
