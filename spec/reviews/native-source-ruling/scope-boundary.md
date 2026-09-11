---
id: SR-025
title: "Scope review of the native-source ruling amendment"
type: SpecReview
analysis: scope-boundary
scope: "FS01 amendment and ADR handoff at bd6f5ea"
review_set: all
---
## Summary

A owns native definition/admission and consumes declared model contracts. The
packet leaves the capability ledger, model presence decision and producer ADR
with their existing owners.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings | - |

## Allocation and external boundaries

| Requirement | Owner | Class |
| --- | --- | --- |
| FR-001 | A, FS01 definition selection | Core |
| FR-019 | A, native source admission | Core |
| NFR-001 | Standard definition/artifact identity | Cross-cutting |

Source/model producer → exact declarations/regions → A's native frontend →
derived IR and export consumers. Existing APIs are implemented, but complete
replacement-profile correspondence is not yet guaranteed. FCD #95's eventual
source representation remains an external decision; TC-007 names the contract
test needed before claiming that boundary. Shared IR/ADR and model producers
remain C/D; B owns portable verification, and tl-* owns temporal semantics.
The ADR handoff contains no producer edit or public promotion. OCL/SysML/FRETish
remain wanted Rust outputs with explicit losses, not editable clause sources.
