---
id: SR-023
title: "Evidence review of the native-source ruling amendment"
type: SpecReview
analysis: evidence
scope: "New FR-001-AC-4 and FR-019-AC-1–6 at bd6f5ea"
review_set: all
---
## Summary

`quoin advise --json` failed CLI-version detection. The installed catalog was
read; the selections below are author judgment, not deterministic advice.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Retain Test with integration-testing plus negative-abuse-testing for exact profile selection. | FR-001-AC-4; TC-005 |
| FND-002 | low | Retain Test with integration-testing plus negative-abuse-testing for normalized arithmetic and deferred numeric operations. | FR-019-AC-1; TC-006 |
| FND-003 | medium | Retain Test at the actual model producer/consumer boundary; no executable representation is selected before FCD #95. | FR-019-AC-2; TC-007 |
| FND-004 | low | Retain Test with negative-abuse-testing for containment/reference admission; keep the original design gate open. | FR-019-AC-3; TC-008 |
| FND-005 | low | Retain Test with integration-testing and boundary/refusal controls; assess reuse of compiler PR31. | FR-019-AC-4; TC-009 |
| FND-006 | low | Retain Test with negative-abuse-testing for root/call admission and explicit incomplete outcomes. | FR-019-AC-5; TC-010 |
| FND-007 | low | Retain Test with integration-testing plus negative-abuse-testing for actual source/anchor correspondence. | FR-019-AC-6; TC-011 |

## Evidence limits

Both selected Test methods have catalog evidence kind Integration, to be produced
by the compiler's Rust harness and the selected model consumer. These new cases
have no runnable suite in the standard repository. Catalog inspection (Manual)
supports review of the draft only, not execution acceptance. Broader property,
fuzz, mutation and backend qualification remain later assurance. No Loom claim
applies to this definition-only change; no optional semantic gap review ran.
