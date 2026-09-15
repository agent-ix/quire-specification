---
id: NFR-074
title: "Preserve lifecycle API and wire compatibility"
type: NFR
quality_attribute: compatibility
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-133
    type: constrains
---

# NFR-074: Preserve lifecycle API and wire compatibility

## Statement

The complete-V1 lifecycle SHALL preserve historical selected semantics and use
explicit version negotiation for every incompatible Rust API or wire change.

## Scope

Public Rust traits/types, CLI machine output, WASM, plugin/provider wires,
packages, counterexamples and evidence intake.

## Rationale

Compatibility must not be achieved by silently accepting changed or incomplete
meaning.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Historical corpus semantic regressions | 0 | 0 | Compatibility corpus |
| Incompatible changes with new selection/migration | 100% | 100% | API/wire diff inspection |

## Verification

Run historical and complete-V1 corpora through old/new readers and verify exact
acceptance, explicit refusal and migration correspondence.
