---
id: IT-071
title: "Compose native, temporal and protocol runtime results"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-180
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# IT-071: Compose native, temporal and protocol runtime results

## Objective

Verify that the `quire-integration` Rust facade coordinates the real QSL,
temporal/observation and protocol APIs while preserving every result axis.

## Target Integration

The local Rust facade calls pinned domain-library APIs directly and emits I10
execution bundles; no hosted service or mock evaluator participates.

## Preconditions

All component crates build with the ecosystem lock and use distinct local state.

## Inputs

ConfigVersion, timed-refund and fan-out/join/compensation corpus packages with
healthy, violating, incomplete, unsupported and exhausted cases.

## Test Procedure

1. Execute each package through every owning runtime.
   - IT-071-SC-01: domain results match their standalone results.
2. Compose the returned results.
   - IT-071-SC-02: ordering, prerequisites and independent axes are preserved.
3. Replay the same inputs and mutate one pin.
   - IT-071-SC-03: equal locks reproduce; changed pins create distinct runs.

## Expected Results

The bundle neither flattens results to Boolean nor hides unsupported/incomplete
siblings, and it remains canonically reproducible.
