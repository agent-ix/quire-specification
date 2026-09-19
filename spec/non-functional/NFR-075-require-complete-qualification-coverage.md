---
id: NFR-075
title: "Require complete qualification coverage"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-specification/FR-311
    type: constrains
---

# NFR-075: Require complete qualification coverage

## Statement

A complete-V1 qualification claim SHALL account for 100 percent of the 188
inventory capabilities and every normative acceptance criterion in its selected
qualification profile.

## Scope

Language, runtime, monitor, analyzer/backend, tooling and consuming-system
qualification profiles.

## Rationale

Family-level samples and implementation labels previously obscured large missing
capabilities.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Inventory primary-row coverage | 188 of 188 | 188 of 188 | Matrix validation |
| Selected acceptance-criterion coverage | 100% | 100% | Gap analysis |

## Verification

Validate the inventory/matrix ID set mechanically, then verify every matrix row
points to a real planned or implemented case and exact qualification subject.
