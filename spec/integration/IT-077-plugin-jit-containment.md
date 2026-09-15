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

Verify compile-time and isolated Rust providers negotiate exact capabilities,
enforce rights/resources and preserve native parity/cache identity.

## Target Integration

The real Rust host registry loads one trait provider and one isolated executable;
the AOT/JIT provider consumes the same I13 contract package.

## Preconditions

Manifests, executable digests, toolchains, options, rights and limits are locked.

## Inputs

Supported/unsupported requests plus malformed output, version/rights mismatch,
crash, timeout and cache-key mutation cases.

## Test Procedure

1. Negotiate and execute valid providers.
   - IT-077-SC-01: capabilities/results/provenance are exact and native-parity holds.
2. Mutate each cache and manifest identity component.
   - IT-077-SC-02: stale artifacts are invalidated or refused.
3. Inject malformed/crash/timeout/unauthorized behavior.
   - IT-077-SC-03: the host contains failure and emits no semantic substitute.

## Expected Results

Extensions remain explicitly selected, bounded and non-authoritative; JIT cache
reuse occurs only under an identical complete key.
