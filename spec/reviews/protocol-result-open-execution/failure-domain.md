---
id: SR-201
title: "Failure-domain review of settled truth on an open subject"
type: SpecReview
analysis: failure-domain
scope: "FR-061, TC-061 and the shared result section of protocol-contract.md"
review_set: all
evaluated_revision: "cb2748d"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
---
## Summary

The review exercised early truth, missing decision support, false closure,
non-decisive open prefixes and later premise contradictions. Each failure now
has a typed outcome and a discriminating planned mutation.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: an open subject may settle only from a profile-authorized decisive witness or counterexample with complete exact support; other open prefixes remain pending. | FR-061-AC-8; TC-061 | wrong-requirement |
| FND-002 | high | Resolved: deleting a support fact, substituting a non-decisive basis or restamping open progress as closed refuses rather than improving the result. | FR-061 result table; TC-061 | missing-requirement |
| FND-003 | medium | Resolved: a later contradiction is separated from an admitted continuation and cannot rewrite or reseal prior result bytes. | FR-061-AC-9; FR-094-AC-5 | missing-requirement |

## Failure-domain checks

| Domain | Disposition |
| --- | --- |
| Premature settlement | Requires a closed decision scope or exact monotone decisive basis under the selected profile. |
| Missing current facts | Truth becomes unavailable when the missing fact belongs to the decision-support set. Unrelated gaps remain visible without erasing settled truth. |
| False closure | Required-scope closure and whole-execution closure are independent typed facts; one cannot substitute for the other. |
| Late data and contradiction | A new linked result supersedes or invalidates; historical bytes and identities remain immutable. |
| Execution failure | Refused, unsupported, failed and resource-incomplete assessment execution cannot carry Boolean truth or a settlement basis. |
