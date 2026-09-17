---
id: SR-021
title: "Integrity review of the native-source ruling amendment"
type: SpecReview
analysis: integrity
scope: "FR-001/019 and historical definition applicability at bd6f5ea"
review_set: all
---
## Summary

The replacement admission rule is scoped to its own proposed identity. Legacy
division, equality, reachability and parameter-observation rules are retained
as history, with no claim that their tests qualify the replacement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The complete inherited-rule closure and its precedence must be fixed before adoption; currently the draft identifies required changes rather than a closed executable definition. | FR-001; native-source-1-draft/README.md; SR-019 |

## Trace and assumptions

| Story | Requirement | Stakeholder | Verification |
| --- | --- | --- | --- |
| US-001 | FR-001 | StR-001 | Test, TC-005 |
| US-001 | FR-019 | StR-001 | Test, TC-006–011 |

NFR-001 owns historical compatibility; original state-core and shared-reference
trees have zero diff from 8ab058b. Profile/rules SHA-256 values still match the
compiler's e897f81 pins. No missing model input is replaced with a fabricated
default.
The source-admission requirement is observable through named adverse cases;
independent implementation acceptance remains open.
