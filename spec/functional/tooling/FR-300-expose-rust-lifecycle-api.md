---
id: FR-300
title: "Expose a stable typed Rust lifecycle API"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-300: Expose a stable typed Rust lifecycle API

## Description

When a Rust consumer uses the native lifecycle API, the library SHALL expose
versioned parse, check, lower, execute, replay, monitor and inspect operations
over typed request/result contracts.

## Inputs

Exact source/package/request inputs and explicit limits/capability selections.

## Outputs

Typed artifacts/results or stable classified errors without panics across the
public boundary.

## Behavior

Each stage consumes the previous stage's typed artifact and preserves source,
definition and capability identity. No lifecycle call reparses diagnostic text,
uses ambient configuration or silently performs a later stage. Public crates are
Rust-only and `publish = false` until their owning release changes that policy.

## Lifecycle surface

The public API exposes typed `parse`, `select_model`, `check`, `execute`,
`monitor`, `analyze`, `lower`, `generate`, `replay` and `render` operations.
Every request carries source/artifact identity, exact profile, limits and
cancellation; every response carries one typed disposition, diagnostics,
provenance and the stage artifact when successful. Artifacts are immutable,
thread-safe values and later stages require their exact predecessor type.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-300-AC-1 | Every lifecycle operation accepts/returns its versioned typed envelope and preserves exact identity. | Test (TC-220) |
| FR-300-AC-2 | Wrong-stage/version/capability input returns a stable typed error and no partial artifact. | Test (TC-220) |
| FR-300-AC-3 | Equivalent API and CLI requests produce the same semantic disposition. | Test (TC-220) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
