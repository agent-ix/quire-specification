---
id: SR-521
title: "EARS review of FS01 language charter ratification"
type: SpecReview
analysis: ears-conformance
scope: "Changed FR-001 and FR-019 normative statements"
review_set: subset
review_date: "2026-09-13"
---
# EARS review of FS01 language charter ratification

## Summary

PASS. The changed normative FR statements use named subjects, one required
response per statement and canonical `When` triggers where behavior is
conditional. Charter/status prose introduces no hidden requirement substitute.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No EARS conformance defect was found in the changed normative statements. | FR-001; FR-019 |

| Statement class | Review result |
| --- | --- |
| Profile selection | “When a source unit declares…” names the frontend response and selected definition. |
| Recognized composed hierarchy | “When a frontend recognizes…” names exact identity/revision/digest selection and prohibits support inference. |
| Unavailable definition | “When a selected definition is unavailable…” requires a located refusal and prohibits fallback. |
| Admission contract | “When a source unit selects an adopted native definition…” requires enforcement before checked-clause production. |
| Unconditional safeguards | Frontend/standard subjects are explicit; refusal, preservation and historical-identity statements each carry one normative response. |

The tables in AD-003 summarize accepted decisions and boundaries. Normative
compiler behavior remains in the linked FRs, so the architecture description
does not create competing modal requirements.
