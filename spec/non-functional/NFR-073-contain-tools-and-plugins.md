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

Every tool, plugin, WASM module and generated executable SHALL receive only
declared capabilities, effects, rights and bounded inputs through a validated
Rust contract.

## Scope

All first-party and third-party extension/provider execution paths.

## Rationale

Untrusted extensions must not gain semantic authority or ambient access through
the formal-analysis path.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Undeclared ambient authority paths | 0 | 0 | Rust/security inspection |
| Malformed/crash/timeout isolation cases contained | 100% | 100% | Fault injection |

## Verification

Exercise manifest mismatch, malformed output, oversized input, timeout, crash
and unauthorized effect cases locally.
