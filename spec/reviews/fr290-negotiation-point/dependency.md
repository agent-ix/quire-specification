---
id: SR-605
title: "Dependency review of the FR-290/AD-010 negotiation-point amendment"
type: SpecReview
analysis: dependency
scope: "quire-specification #116 residuals against QSL #229/#185, contract-ir #131 and quire-analyze #40"
review_set: subset
evaluated_revision: "3a79dce plus FR-290/AD-010 negotiation-point diff"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/AD-010, type: references }
---

## Summary

PASS. The amendment makes FR-290 the vocabulary and absence-policy authority
that the dependent tickets cite. It points QSL-side alignment at QSL #229,
the ticket #185 depends on. It takes nothing from contract-ir or
quire-analyze.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Planned QSL work: QSL `main` has four `Capability` kinds (`requests.rs`), while FR-290 has six. QSL #229 owns the alignment to FR-290, including how older four-kind artifacts are read. This spec does not decide that question. | FR-290; QSL #229; QSL #185 |
| FND-002 | low | The FR-036 absence-path wording is owned by contract-ir and tracked there as contract-ir #131. #116 lists it as out of scope, so this repo does not edit it. | contract-ir #131; FR-036 |

## Dependency allocation

| Owner | Consumes | Remaining owned work |
| --- | --- | --- |
| QSL #229 | FR-290 six kinds, absence policy and negotiation point | Align `Capability`, and define how older four-kind artifacts are read. |
| QSL #185 | FR-290, AD-010, #229 | Replace the fixed lowering catalog with a registry. |
| contract-ir #131 | FR-290 absence policy | Add FR-036 absence-path wording. |
| quire-analyze #40 | FR-290 registration contract, V1-BACK-017 | Register the implication backend. |
