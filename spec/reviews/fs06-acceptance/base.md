---
id: SR-552
title: "Base review of FS06 output-mapping acceptance"
type: SpecReview
analysis: base
scope: "FS06 #7; common output mapping, three target profiles, portable controls and qcir handoff"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-120, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-121, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-122, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-123, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-124, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-125, type: reviews }
---
# Base review of FS06 output-mapping acceptance

## Summary

PASS. FS06 accepts one non-authoritative output-mapping subsystem and three
bounded target profiles. It does not claim full target-language conformance,
read generated text as native source or promote any Planned executable row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the inherited mapping vocabulary lacked exact consumer/mapping identity and allowed an unqualified adequacy field. FR-297/298 now bind exact profiles and keep observation/protocol adequacy separate. | FR-269; FR-297; FR-298 |
| FND-002 | high | Resolved: three output targets could otherwise drift into unrelated envelopes or fallback behavior. AD-004 and FR-120/121/125 require one common intake, record, loss, resource and package boundary with exactly one target per request. | AD-004; FR-120/121/125; FR-299 |
| FND-003 | high | Resolved: plausible target text or parser acceptance could be mistaken for equivalence. Every source obligation now receives an explicit disposition, and structural observation is downstream only. | FR-121–125; TC-150; TC-155 |
| FND-004 | medium | Resolved: generic FR-116 output wording overlapped concrete target profiles. FR-116 remains the fact-preserving result boundary; FS06 now owns exact OCL, SysML/KerML and FRETish selections. | FR-116; output-mapping contract; FR-122–124 |

## Acceptance check

- One exact target standard/profile/revision/digest is selected before dispatch.
- Every requested obligation is preserved, conditional, unrepresented or
  refused with the facts required by that classification.
- Output packages bind immutable bytes to complete records and separate raw/JCS
  identities; failure exposes no partial package.
- OCL, SysML/KerML and FRETish mappings are deliberately narrower than their
  target languages and name every unsupported source family.
- Optional target tools observe immutable bytes but cannot alter source,
  mapping, truth, adequacy or preservation.

## Evidence boundary

TM-008 and IT-060 remain Planned. Qcir #55/#56/#57 own executable Rust
exporters and correspondence evidence; FS06 supplies their normative input but
does not close them.
