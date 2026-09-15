---
id: FR-305
title: "Load explicit versioned Rust plugins"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## Provider boundary

A manifest binds plugin identity/version, host protocol range, capability IDs,
Rust/wire ABI choice, executable or crate digest, configuration schema and
requested rights. Negotiation intersects exact capabilities and rejects version
or digest mismatch before invocation. Isolated providers exchange length-bounded
canonical envelopes with request IDs, deadlines and cancellation. Host
validation treats crashes, malformed output and undeclared capabilities as tool
failure and never loads native dynamic-library symbols.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-305-AC-1 | A matching manifest/provider executes only its declared capability and returns a validated typed result. | Test (TC-225) |
| FR-305-AC-2 | Version, capability, purity/effect, provenance or rights mismatch refuses before dispatch. | Test (TC-225) |
| FR-305-AC-3 | Malformed, oversized, timed-out or crashing isolated output becomes provider failure without host semantic corruption. | Test (TC-225) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
