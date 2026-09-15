---
id: FR-308
title: "Expose a bounded deterministic WASM API"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-300
    type: references
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-308: Expose a bounded deterministic WASM API

## Description

When a WASM consumer invokes a supported lifecycle operation, the module SHALL
validate a versioned request, enforce explicit memory/work limits and return the
same semantic disposition as the native Rust API.

## Inputs

Versioned bytes/request, supported feature selection and host-provided limits.

## Outputs

Versioned deterministic bytes/result or typed unsupported/resource/validation
failure.

## Behavior

The WASM surface exposes only declared pure lifecycle capabilities and no ambient
filesystem/network/time authority. Allocation and input/output sizes are charged
before expansion. Unsupported native features refuse exactly rather than run a
smaller meaning.

## WASM resource contract

The module imports only bounded memory and host cancellation; it imports no
filesystem, network, clock, randomness or process authority. Each request
declares byte, node, collection, step and output limits. The host charges input
before decoding, charges semantic expansion before allocation and returns a
typed incomplete result at the first exhausted budget. Canonical versioned
envelopes are identical to native lifecycle envelopes, so native/WASM parity is
checked without translating verdicts.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-308-AC-1 | Native and WASM APIs return byte-equivalent canonical results for every supported corpus item. | Test (TC-228) |
| FR-308-AC-2 | Unknown wire/features and oversized inputs refuse before semantic execution. | Test (TC-228) |
| FR-308-AC-3 | Missing host authority cannot be bypassed through ambient filesystem, network or clock access. | Test (TC-228) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
