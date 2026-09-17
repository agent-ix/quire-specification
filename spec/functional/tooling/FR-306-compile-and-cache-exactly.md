---
id: FR-306
title: "Compile and cache exact AOT/JIT artifacts"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## Compilation and cache identity

Compilation is preceded by per-node capability negotiation and emits only when
all dependencies of that node are exact. The cache key covers semantic-subject
digest, IR/profile versions, provider binary/configuration, target, toolchain,
dependencies, options and relevant limits. The cached value includes the full
typed disposition and provenance, is content-verified on read and is never
reused after any key-field mismatch. Concurrent writers may publish only
byte-identical values for one key.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-306-AC-1 | AOT/JIT and native execution agree on the complete supported conformance corpus. | Test (TC-226) |
| FR-306-AC-2 | Changing package, target, provider, toolchain or semantic option invalidates the cache key. | Test (TC-226) |
| FR-306-AC-3 | Unsupported constructs emit no artifact and resource/codegen failure cannot return a semantic verdict. | Test (TC-226) |
| FR-306-AC-4 | A result produced through ahead-of-time compilation, through just-in-time compilation and through interpretation carries the same qualification identity and the same evidence provenance for the same subject and ecosystem lock; the execution mode is recorded and is not part of that identity. | Test (TC-226) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
