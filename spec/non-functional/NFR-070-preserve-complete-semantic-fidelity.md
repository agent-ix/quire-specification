---
id: NFR-070
title: "Preserve complete semantic fidelity"
type: NFR
quality_attribute: functional_suitability
relationships:
  - target: ix://agent-ix/quire-specification/FR-131
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-195
    type: constrains
---

# NFR-070: Preserve complete semantic fidelity

## Statement

The complete-V1 ecosystem SHALL produce zero approximated, silently omitted or
semantically narrowed outcomes across the 180-capability conformance corpus.

## Scope

All native compilers, runtimes, monitors, analyzers, proof/codegen providers,
mappings and tools claiming a complete-V1 capability.

## Rationale

A plausible result over changed meaning is more dangerous than an explicit
unsupported or incomplete outcome.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Silent approximation/omission findings | 0 | 0 | Conformance corpus and gap analysis |
| Requested items with exactly one disposition | 100% | 100% | Automated accounting test |

## Verification

Execute positive, boundary, mutation and refusal vectors for every inventory
row and inspect all generated-artifact accounting.
