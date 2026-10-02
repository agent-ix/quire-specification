---
id: Task-002
title: "Define bounded target output profiles"
type: Task
status: done
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/FR-122
    type: references
  - target: ix://agent-ix/quire-specification/FR-123
    type: references
  - target: ix://agent-ix/quire-specification/FR-124
    type: references
---
# Task-002: Define bounded target output profiles

## Scope

Accept narrow, versioned and independent OCL, SysML/KerML and FRETish output
correspondences over the shared Task-001 boundary.

## Subtasks

- [x] Bound exact OCL 2.4 invariant/pre/post and expression mappings.
- [x] Bound exact SysML 2.0/KerML 1.0 requirement/constraint mappings.
- [x] Bound exact FRETish 3.1 state and selected fixed-sample temporal mappings.
- [x] Classify unsupported constructs explicitly with no target substitution.
- [x] Record target versions, source links, dependency boundaries and rights.

## Deliverables

- FR-122–124 and the three target profiles.
- Portable controls TC-151–153 and the target cases in TC-155.
- Independent implementation handoffs to qcir #55/#56/#57.
