---
id: SR-024
title: "Risk review of the native-source ruling amendment"
type: SpecReview
analysis: risk-complexity
scope: "FR-001/019 amendment at bd6f5ea"
review_set: all
---
## Summary

The main risk is advertising a replacement identity before its complete meaning
and implementation are established. The proposal keeps adoption separate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New profile adoption must wait for a closed definition and the original reference-workflow disposition; a narrower successful example is insufficient. | SR-019; FR-001; FR-019 |

## Risk register

| Requirement | Technical risk | Volatility | Mitigation |
| --- | --- | --- | --- |
| FR-001 | Medium | Medium | Preserve immutable pins; review the complete profile/edition identity after rational grammar is fixed. |
| FR-019 | Medium | High | Keep exact FS03 and FCD #95 dependencies visible; implement reviewed slices under #30 and retain the full ConfigVersion gate. |

Top hazards are historical reinterpretation, field-presence inference and
substituting old graph successes for new-profile conformance. Source-bound
refusals and independent negative controls address failures identified by SR-020.
Ongoing capability research may add tickets; it is not implicit feature adoption.
