---
id: FR-306
title: "Compile and cache exact AOT/JIT artifacts"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# FR-306: Compile and cache exact AOT/JIT artifacts

## Description

When AOT or JIT execution is selected, the provider SHALL preserve native
semantics and key every artifact/cache entry by the complete semantic,
target/toolchain and options identity.

## Inputs

Checked/IR package, target, provider/toolchain/options, resource policy and
runtime inputs.

## Outputs

Compiled artifact and execution result with parity/provenance, or explicit
unsupported/refused/failed outcome.

## Behavior

Compilation negotiates exact node support before emission. Cache identity covers
all meaning-affecting inputs; mismatches invalidate rather than reuse. Executed
results obey the same typed value/failure contract as native execution. Runtime
code cannot access ambient effects absent an explicit non-evaluator tool contract.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-306-AC-1 | AOT/JIT and native execution agree on the complete supported conformance corpus. | Test (TC-226) |
| FR-306-AC-2 | Changing package, target, provider, toolchain or semantic option invalidates the cache key. | Test (TC-226) |
| FR-306-AC-3 | Unsupported constructs emit no artifact and resource/codegen failure cannot return a semantic verdict. | Test (TC-226) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
