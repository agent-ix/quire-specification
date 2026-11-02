---
id: SR-58512
title: "Evidence re-review of QSL-585 task-search and no-release closure contract"
type: SpecReview
analysis: evidence
scope: "FR-415-AC-26, FR-418-AC-14, FR-419-AC-6; backend-provider-v1 vectors and reader evidence"
review_set: all
---

## Summary

`quoin advise` reports no method mismatch for FR-415-AC-26, FR-418-AC-14, or FR-419-AC-6. The work-budget criterion is now tagged to TC-431 and the derived-item whitelist is tagged. The unresolved-dynamics malformed vector now exercises the required dimension rule. The new no-release certificate has wire-positive and wire-malformed cases, but its strict-reader closure condition lacks a reader vector.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-418-AC-14 requires the reader to recompute all source successors and reject `closed-no-release-product/v1` when a checked release binding is reachable. ZC-25 exercises only canonical wire validity and ZC-26 only wire rejection for a positive cap; `task_reader_vectors` adds no case for a reachable release under this method. Add a reader case with the expected rejection locus/rule and bind it to a strict-reader test. | FR-418-AC-14; proposals/backend-provider-v1/zone-certificate-vectors.json (`ZC-25`, `ZC-26`, `task_reader_vectors`); tests/zone_certificate_wire.rs | correct-requirement-no-evidence |
