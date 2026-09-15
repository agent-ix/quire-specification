---
id: US-001
title: "Understand the meaning of a native clause"
type: US
relationships:
  - target: "ix://agent-ix/quire-specification/StR-001"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-001"
    type: exercises
---
# US-001: Understand the meaning of a native clause

## Story

**As a** specification author
**I want** to know which rules give my clause meaning
**So that** a change in profile cannot silently change what I asserted.

## Context

This story originated the native finite-state slice and is accepted within the complete-V1 specification review. Existing implementation observations establish only their traced stages; implementation planning owns every remaining delivery stage.

## Acceptance Examples (Illustrative)

### US-001-EX-1

- **Given** an admitted explicit profile.
- **When** I inspect its clause meaning.
- **Then** the selected definition is identified.

### US-001-EX-2

- **Given** an unfamiliar profile.
- **When** I request interpretation.
- **Then** the unsupported selection is reported.

## Priority and Risk (Informative)

Priority: High. Confusing a source/model identity or incomplete execution with a verified result would invalidate the intended assessment.

## Traceability (Informative)

- [FR-001](../functional/foundation/FR-001-select-native-profile.md)
