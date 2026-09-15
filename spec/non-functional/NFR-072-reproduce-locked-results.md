---
id: NFR-072
title: "Reproduce version-locked results"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-132
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-311
    type: constrains
---

# NFR-072: Reproduce version-locked results

## Statement

The ecosystem SHALL reproduce canonical artifacts and typed results bit-for-bit
for equal semantic inputs and one exact ecosystem lock.

## Scope

Compilation, runtime/replay, monitoring repair, simulation, methods, Kani,
AOT/JIT, WASM, mappings, evidence and qualification.

## Rationale

Evidence and counterexamples cannot be trusted if ambient versions or unstable
ordering change their meaning.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Repeated canonical artifact equality | 100% | 100% | Differential replay |
| Meaning-affecting pins present | 100% | 100% | Lock inspection |

## Verification

Repeat the corpus from clean local builds and mutate each pin to verify that it
creates a distinct run/cache identity.
