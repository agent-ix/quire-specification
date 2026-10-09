---
id: NFR-073
title: "Contain tool, plugin and generated-code execution"
type: NFR
quality_attribute: security
relationships:
  - target: ix://agent-ix/quire-specification/FR-305
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-308
    type: constrains
---

# NFR-073: Contain tool, plugin and generated-code execution

## Statement

The host SHALL pass every tool, plugin, WASM module and generated executable
only bounded inputs through a typed Rust contract, and SHALL read every result
it returns with the typed reader before that result settles any item.

## Scope

All first-party and third-party extension/provider execution paths.

## Rationale

An installed plugin is a trusted program, like any program the user installs.
Its output still crosses a typed boundary: it can settle only the items routed
to it, its counterexamples count only after replay, and its proofs carry the
certification `trusted` (FR-305, FR-331).

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Plugin results settling an item without passing the typed reader | 0 | 0 | Test |
| Malformed/crash/timeout isolation cases contained | 100% | 100% | Fault injection |

## Verification

Exercise malformed output, oversized frames, timeout, crash, a result for an
unrouted item and a result for an unadvertised capability locally.
