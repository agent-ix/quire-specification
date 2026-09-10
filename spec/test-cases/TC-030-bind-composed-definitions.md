---
id: TC-030
title: "Bind every clause to its exact semantic definitions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-030
    type: verifies
---
## Description

Planned acceptance controls for [FR-030](../functional/FR-030-bind-composed-definitions.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Construct an admitted symbolic three-family package with one model closure. Independently change a definition revision, declared digest, and duplicate authority. For the unmodified package, substitute a second runtime trace; then change a semantic definition. Observe admission and static subject identities without relying on diagnostic English.

## Expected Results

The valid package retains exact bindings. Each adverse selection refuses its dependent admission with a typed cause and source location. Runtime trace substitution preserves static subject identity; the semantic change does not. No execution result is produced for refused dependencies.

