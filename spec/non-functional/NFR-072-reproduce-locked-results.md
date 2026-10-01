---
id: NFR-072
title: "Reproduce results deterministically"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-132
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-311
    type: constrains
---

# NFR-072: Reproduce results deterministically

## Statement

The ecosystem SHALL reproduce canonical artifacts and typed results bit-for-bit
for equal semantic inputs.

## Scope

Compilation, runtime/replay, monitoring repair, simulation, methods, Kani,
AOT/JIT, WASM, mappings, evidence and qualification.

## Rationale

Evidence and counterexamples cannot be trusted if unstable ordering or ambient
state changes their meaning.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Repeated canonical artifact equality | 100% | 100% | Differential replay |

## Verification

Repeat the corpus from clean local builds and compare the canonical artifacts
and typed results byte for byte.
