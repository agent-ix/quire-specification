---
id: SR-306
title: "Scope-boundary review of native temporal lifecycle correspondence"
type: SpecReview
analysis: scope-boundary
scope: "E temporal ownership across A/B/F, contract IR, TL, Quoin and Engineering Assurance"
review_set: all
evaluated_revision: "2f347b8"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
---
## Summary

The remediation stays inside E's ownership of temporal truth, admissible
continuations, clocks, history and closure interpretation. It consumes but does
not redefine native grammar, model identity, observation authority, shared
result schema, IR projection, evidence retention or TL release authority.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: E supplies temporal settlement/support facts without making the TL formula or evaluator the native clause/result authority. | FR-095; IT-040 |
| FND-002 | medium | Resolved: activation classification aligns with B/F inputs while E retains only temporal trigger-scope interpretation. | FR-093; FR-061; observation-contract.md |
| FND-003 | high | Open F handoff: F owns preservation through observation result consumers; E must not patch FR-115/116 or their tests. | observation-output-mapping-contract.md; FR-115/116 |
| FND-004 | low | Open A handoff: A owns candidate definition revisions, exact digest selection and adoption; E leaves `1-draft.2` immutable. | definitions/temporal-common.md; definitions/README.md |
| FND-005 | medium | Retained boundary: OCL, SysML v2/KerML and FRETish remain export-only; R2U2/C2PO or other foreign runtimes are not production/qualification dependencies. | owner ruling; FR-095 |

## Responsibility allocation

| Owner | Responsibility |
| --- | --- |
| A | Sole native grammar, exact profile definitions/digests and adoption. |
| E | Temporal profiles, truth, captures, history, progress/closure interpretation and exact TL correspondence. |
| B | Shared orthogonal result relation and strict-reader combinations. |
| F | Observation/progress authority and lossless result-consumer handoff. |
| contract IR | Typed total-Boolean projection and public bridge interface. |
| TL repositories | Internal syntax/evaluator profile implementations and release authority. |
| Quoin / Engineering Assurance | Verification-method and retained evidence authority. |
