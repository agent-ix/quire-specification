---
id: NFR-002
title: "Keep exhaustion distinct from logical violation"
type: NFR
quality_attribute: reliability
relationships:
  - target: "ix://agent-ix/quire-specification/FR-005"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-010"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-011"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: constrains
---
# NFR-002: Keep exhaustion distinct from logical violation

## Statement

If a selected work budget is exhausted, then the conforming implementation shall report incomplete execution.

## Scope

Parsing, linking, population validation, canonicalization and evaluation.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Boolean outcomes after exhaustion | 0 | 0 | Budget-boundary tests |
| Accepted partial populations | 0 | 0 | Validation-budget tests |

## Verification

Test zero, sufficient and exhausted budgets in each implemented phase. Retain missing phase coverage as an unresolved review finding.

## Dependencies

- [FR-005](../functional/foundation/FR-005-guard-optional-values.md)
- [FR-010](../functional/foundation/FR-010-evaluate-exact-reachability.md)
- [FR-011](../functional/foundation/FR-011-validate-populations.md)
- [FR-015](../functional/foundation/FR-015-preserve-canonical-domains.md)

## Status

Accepted by FS05 for interchange parsing, canonicalization and reference
validation. Other processing phases retain their owning acceptance tickets.
