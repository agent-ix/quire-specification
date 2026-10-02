---
id: IT-077
title: "Rust plugins and JIT providers remain contained and exact"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-305
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-306
    type: verifies
---

# IT-077: Rust plugins and JIT providers remain contained and exact

## Objective

Verify compile-time and process Rust providers negotiate exact capabilities,
stay within their run budgets, and that JIT execution keeps interpreter parity
and the content-keyed cache.

## Target Integration

The real Rust host registry loads one trait provider and one isolated executable;
the AOT/JIT provider consumes the same I13 contract package.

## Preconditions

Manifests, options and run budgets are given in the request.

## Inputs

Supported and unsupported requests plus malformed output, wrong wire identity,
crash, timeout and cache-key mutation cases.

## Test Procedure

1. Negotiate and execute valid providers.
   - IT-077-SC-01: capabilities and results are exact and interpreter parity holds.
2. Mutate each cache-key input and the provider manifest.
   - IT-077-SC-02: each mutation is a cache miss, and a plugin result is never cached.
3. Inject malformed, crashing and timed-out plugin behaviour.
   - IT-077-SC-03: the host contains failure and emits no semantic substitute.

## Expected Results

Extensions remain explicitly selected, bounded and non-authoritative; JIT cache
reuse occurs only under an identical complete key.
