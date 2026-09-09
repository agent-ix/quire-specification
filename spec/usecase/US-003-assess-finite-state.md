---
id: US-003
title: "Assess a finite state under explicit assumptions"
type: US
relationships:
  - target: "ix://agent-ix/quire-specification/StR-001"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-005"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-006"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-007"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-008"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-009"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-010"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-011"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-012"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-013"
    type: exercises
---
# US-003: Assess a finite state under explicit assumptions

## Story

**As a** verification operator
**I want** to distinguish a satisfied condition from a violation or missing information
**So that** an incomplete observation cannot be mistaken for a trustworthy result.

## Context

This story drives the native finite-state work assigned to Agent A. The requirements below remain draft until the requested specification review is completed. Existing implementation observations do not establish the unimplemented pipeline stages.

## Acceptance Examples (Illustrative)

### US-003-EX-1

- **Given** a complete valid population.
- **When** the condition is evaluated.
- **Then** its logical result is reported.

### US-003-EX-2

- **Given** an unavailable required observation.
- **When** evaluation is requested.
- **Then** the missing information is reported.

## Priority and Risk (Informative)

Priority: High. Confusing a source/model identity or incomplete execution with a verified result would invalidate the intended assessment.

## Traceability (Informative)

- [FR-005](../functional/FR-005-guard-optional-values.md)
- [FR-006](../functional/FR-006-preserve-control-evaluation.md)
- [FR-007](../functional/FR-007-prove-integer-definedness.md)
- [FR-008](../functional/FR-008-preserve-sequence-semantics.md)
- [FR-009](../functional/FR-009-preserve-object-identity.md)
- [FR-010](../functional/FR-010-evaluate-exact-reachability.md)
- [FR-011](../functional/FR-011-validate-populations.md)
- [FR-012](../functional/FR-012-bind-operation-observations.md)
- [FR-013](../functional/FR-013-validate-operation-frames.md)
