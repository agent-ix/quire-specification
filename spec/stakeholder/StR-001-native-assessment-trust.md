---
id: StR-001
title: "Trust native finite-state assessments"
type: StR
relationships:
  - target: "ix://agent-ix/quire-specification/FR-001"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-002"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-003"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-004"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-005"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-006"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-007"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-008"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-009"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-010"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-011"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-012"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-013"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-014"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: satisfied_by
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: satisfied_by
---
# StR-001: Trust native finite-state assessments

## Stakeholder Need

Verification operators require that native finite-state assessments shall retain the meaning and identity of the selected authored obligation.

## Rationale

Operators need to distinguish an actual violation from stale data, unsupported semantics and incomplete observation. A syntax milestone alone does not answer that operational need.

## Validation Criteria

| ID | Criteria | Validation |
| --- | --- | --- |
| StR-001-VC-1 | The supported healthy and violating workflow identifies the exact authored source. | Demonstration |
| StR-001-VC-2 | An incomplete required observation is reported without a logical result. | Demonstration |

## Stakeholders

Specification authors, verification operators and existing toolchain integrators.

## Dependencies

- [US-001](../usecase/US-001-understand-native-meaning.md)
- [US-002](../usecase/US-002-exchange-exact-subjects.md)
- [US-003](../usecase/US-003-assess-finite-state.md)
- [US-004](../usecase/US-004-retain-evidence-meaning.md)
