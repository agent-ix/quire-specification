---
id: SR-544
title: "Base review of FS05 interchange and migration acceptance"
type: SpecReview
analysis: base
scope: "FS05 #6; strict shared references, canonical identity domains, explicit migration, composed package references and portable controls"
review_set: subset
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-014, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-015, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-016, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-017, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-018, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-032, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-130, type: reviews }
---
# Base review of FS05 interchange and migration acceptance

## Summary

PASS. FS05 accepts the already-authored exact wire, canonical-domain,
linked-package and explicit-migration meanings. It does not rename historical
artifacts, infer cross-version equivalence, register a public format or claim
that Planned matrices have executed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the owning requirements and definitions still described the reviewed closure as Draft/Proposed even though PR #8, QSL's strict `/2` producer/reader and protocol's independent `/2` consumer had merged. FS05 now accepts the semantic contract without promoting consumer-specific evidence. | FR-014–018; FR-032; FR-130; QSpec PR #8; QSL PR #69; protocol PR #52 |
| FND-002 | high | Resolved: the earlier qualification paragraph could be read as requiring every B/C refusal path before the standard meaning could be accepted. It now requires each consumer to qualify its applicable claims and keeps unrun matrices Planned. | interchange; IT-002; IT-003; TM-001/TM-003 |
| FND-003 | medium | Resolved: Plan-001 still said PR #8 was open. Its task and plan now record the actual `8ab058b` merge while retaining independent consumer adoption outside scope. | Plan-001; Task-002 |
| FND-004 | medium | Resolved: composed-package identity was still labelled a candidate. FS05 now freezes `quire.package.composed/v1` `1-draft.2`, exact inner-profile retention, closed feature meaning and null whole-package canonical identity. | FR-130; package-reference |

## Acceptance check

- Historical selected definitions, reference schemas, fixture bytes and digests
  are unchanged; a semantic change requires a new exact selection.
- Unknown wire versions, required features and canonical domains refuse before
  semantic use; malformed maps and exhausted work yield their distinct outcomes.
- Artifact-byte, IR canonical, IR bound and verification-JCS identity
  domains retain their separate preimages and digest syntax.
- Migration is an exact selected mapping with named preserved aspects. Reverse,
  transitive and semantic equivalence are never inferred from similar bytes.
- A composed-package reference retains every inner profile and cannot use outer
  features or an unrelated digest as proof of payload support or conformance.

## Evidence boundary

QSL PR #69 supplies strict source/model/config versus compiled-artifact/result
digest-domain and source-map controls. Protocol PR #52 supplies an independently
implemented public `/2` consumer with cross-version, mutation and resource
refusals. Protocol #11 remains open for the older `/1` regression, and IT-002,
IT-003, TM-001 and TM-003 retain their own unexecuted rows.
