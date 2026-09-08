---
id: US-002
title: "Exchange exact source and model subjects"
type: US
relationships:
  - target: "ix://agent-ix/quire-specification/StR-001"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-002"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-003"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-004"
    type: exercises
---
# US-002: Exchange exact source and model subjects

## Story

**As a** model producer
**I want** to identify the exact declarations and source regions a clause uses
**So that** consumers cannot join my clause to stale or foreign data.

## Context

This story drives the native finite-state work assigned to Agent A. The requirements below remain draft until the requested specification review is completed. Existing implementation observations do not establish the unimplemented pipeline stages.

## Acceptance Examples (Illustrative)

### US-002-EX-1

- **Given** an exact source and model package.
- **When** a consumer resolves the clause.
- **Then** the intended stable identities are selected.

### US-002-EX-2

- **Given** conflicting package exports.
- **When** a consumer resolves the clause.
- **Then** the ambiguity is surfaced.

## Priority and Risk (Informative)

Priority: High. Confusing a source/model identity or incomplete execution with a verified result would invalidate the intended assessment.

## Traceability (Informative)

- [FR-002](../functional/FR-002-resolve-owned-declarations.md)
- [FR-003](../functional/FR-003-bind-source-correspondence.md)
- [FR-004](../functional/FR-004-separate-artifact-identities.md)
