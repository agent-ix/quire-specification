---
id: US-004
title: "Retain evidence meaning across versions"
type: US
relationships:
  - target: "ix://agent-ix/quire-specification/StR-001"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-014"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: exercises
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: exercises
---
# US-004: Retain evidence meaning across versions

## Story

**As a** verification integrator
**I want** to know whether historical artifacts still refer to the same subject
**So that** a format or dependency change cannot silently reuse unrelated evidence.

## Context

This story originated the native finite-state slice and is accepted within the complete-V1 specification review. Existing implementation observations establish only their traced stages; implementation planning owns every remaining delivery stage.

## Acceptance Examples (Illustrative)

### US-004-EX-1

- **Given** unchanged qualified references.
- **When** I compare subjects.
- **Then** their exact correspondence is established.

### US-004-EX-2

- **Given** an unaccepted migration.
- **When** I compare versions.
- **Then** the unsupported join is reported.

## Priority and Risk (Informative)

Priority: High. Confusing a source/model identity or incomplete execution with a verified result would invalidate the intended assessment.

## Traceability (Informative)

- [FR-014](../functional/foundation/FR-014-read-exact-wire.md)
- [FR-015](../functional/foundation/FR-015-preserve-canonical-domains.md)
- [FR-016](../functional/foundation/FR-016-migrate-explicitly.md)
