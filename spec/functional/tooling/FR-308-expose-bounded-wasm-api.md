---
id: FR-308
title: "Expose a bounded deterministic WASM API"
type: FR
relationships:
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-308-AC-1 | Native and WASM APIs return byte-equivalent canonical results for every supported corpus item. | Test (TC-228) |
| FR-308-AC-2 | Unknown wire/features and oversized inputs refuse before semantic execution. | Test (TC-228) |
| FR-308-AC-3 | Missing host authority cannot be bypassed through ambient filesystem, network or clock access. | Test (TC-228) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
