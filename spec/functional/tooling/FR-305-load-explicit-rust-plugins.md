---
id: FR-305
title: "Load explicit versioned Rust plugins"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-305: Load explicit versioned Rust plugins

## Description

When a plugin capability is selected, the host SHALL validate an explicit Rust
plugin manifest, contract version, effects/rights and conformance identity before
dispatch.

## Inputs

Compile-time trait implementation or isolated Rust executable, manifest,
capability request, rights grant and resource limits.

## Outputs

Typed provider result with provenance, or version/capability/rights/isolation
refusal/failure.

## Behavior

V1 defines compile-time Rust traits and a versioned isolated-process wire; it
does not define an unstable dynamic-library ABI. Plugins cannot alter admission,
source identity or native meaning. Pure providers receive no effects; effectful
tools require explicit rights and remain outside evaluators. Output is validated
as untrusted input.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-305-AC-1 | A matching manifest/provider executes only its declared capability and returns a validated typed result. | Test (TC-225) |
| FR-305-AC-2 | Version, capability, purity/effect, provenance or rights mismatch refuses before dispatch. | Test (TC-225) |
| FR-305-AC-3 | Malformed, oversized, timed-out or crashing isolated output becomes provider failure without host semantic corruption. | Test (TC-225) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
