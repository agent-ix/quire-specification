---
id: NFR-071
title: "Bound all complete-V1 execution resources"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-181
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-193
    type: constrains
---

# NFR-071: Bound all complete-V1 execution resources

## Statement

Every complete-V1 execution path SHALL charge input, state, work, recursion,
artifact and retained-history resources against explicit limits before expansion.

## Scope

Native execution, monitoring/repair, protocol analysis, simulation, method
providers, plugins, compilation and mapping.

## Rationale

New recursive, dynamic and analyzer capabilities must remain safe on untrusted
inputs without changing semantic outcomes.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Expansion paths with pre-charge tests | 100% | 100% | Static inspection and fault injection |
| Limit breaches returning typed non-result | 100% | 100% | Boundary/property tests |

## Verification

Inject each limit immediately before and after exhaustion and confirm no partial
completed semantic result or unaccounted artifact is exposed.
