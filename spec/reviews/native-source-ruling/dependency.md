---
id: SR-022
title: "Dependency review of the native-source ruling amendment"
type: SpecReview
analysis: dependency
scope: "FR-001/019 replacement adoption dependencies at bd6f5ea"
review_set: all
---
## Summary

The proposal can be reviewed now. Definition adoption and affected compiler
admission depend on the explicit remaining semantic decisions, not a new #54 reader.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Do not freeze replacement identities before the FS03 rational/anchor closure; separately retain the original reference-gate disposition. | SR-019; compiler #30 |

## Classification and order

| Requirement | Class | Prerequisite |
| --- | --- | --- |
| NFR-001 | Enablement | Preserve historical bytes/meaning |
| FR-001 | Enablement | Exact complete replacement definition and adoption |
| FR-019 | Feature | Selected definition, model contract and source/anchor bindings |

Order: preserved historical definitions → completed replacement contract →
adopted FR-001 selection → FR-019 compiler enforcement → consumer qualification.
No cycle exists. FCD #95 precedes representing independent member presence;
explicit source refusal remains possible where that distinction is unsupported.
The decision does not block unrelated native engineering or authorize A to edit
the producer. Wider backend/temporal capabilities remain their owners' work.
