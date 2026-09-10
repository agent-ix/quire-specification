---
id: FR-061
title: "Report orthogonal protocol and adequacy results"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-031
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-059
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-060
    type: depends_on
---
## Description

When any protocol assessment is requested, the result producer SHALL retain separate execution, truth, activation, participation, completeness, adequacy, claim and provenance facts for the exact requested subject.

## Inputs

An exact assessment request plus conformance or analysis observations, prerequisite
outcomes, limitations and derived artifacts.

## Outputs

A portable per-request result consumed by the composed-language dispatcher,
temporal/observation integrations and Engineering Assurance, or a typed strict-
reader refusal if the result is inconsistent.

## Behavior

The result producer SHALL NOT attach a successful truth value to a refused,
unsupported, failed or resource-incomplete execution. The result producer SHALL
NOT classify an inactive obligation as satisfied or demonstrated. If required
participants, observations, population members, progress or adverse cases are
missing, then the result producer SHALL NOT change an unsuccessful or unknown
truth, adequacy or package status to successful. The result producer SHALL retain
proven, bounded-checked, tested and observed claim strengths as distinct
source-bound values. The result producer SHALL reuse Quoin evidence authority.
The result producer SHALL NOT create a local evidence store or verification-method
registry.

The strict reader SHALL enforce the following result-combination rules.

### Result combination rules

| Condition | Required relation |
| --- | --- |
| Execution is refused, unsupported, resource-incomplete or failed | Truth is unavailable. Already-established activation and provenance facts remain retained. |
| Truth is satisfied | Execution is completed, activation is triggered and completeness is complete. |
| Truth is violated | Execution is completed and activation is triggered. Completeness may be incomplete only when the retained finite violation witness does not depend on missing facts. |
| Truth is pending | Execution is completed, activation is triggered and completeness is open or incomplete. |
| Activation is inactive | Execution is completed, truth is unavailable and completeness is complete under an exact closure authority. |
| Activation is activation-unknown | Truth is unavailable and completeness is open or incomplete with the missing trigger dependencies. |
| Adequacy is demonstrated | Execution and required inputs are complete and the exact required case set is present. An obligation-path demonstration additionally requires triggered activation. |
| Package status is complete success | Every selected required request satisfies its claim-specific success rule. Unsupported or incomplete requests remain represented and prevent complete success. |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-061-AC-1 | Healthy, violating, inactive, pending, incomplete, refused, unsupported and failed cases remain discriminable without diagnostic prose. | Test (TC-061) |
| FR-061-AC-2 | Removing a trigger, required role, provider observation, population member or negative case cannot improve truth, adequacy or package-level status. | Test (TC-061) |
| FR-061-AC-3 | Every result binds exact source/profile/model/protocol/request/binding/input/progress/backend identities and derived artifact digests. | Test (TC-061) |
| FR-061-AC-4 | A result with an impossible dimension combination, stale identity or conflicting claim strength is refused by a strict reader. | Test (TC-061) |
| FR-061-AC-5 | One unsupported request does not remove an independent supported result, while package complete success remains unavailable. | Test (TC-061) |
| FR-061-AC-6 | Every row of the normative result-combination table admits its valid boundary case and refuses every one-axis forbidden combination. | Test (TC-061) |

## Dependencies

- [Shared capability dispatcher](./FR-031-report-requested-capabilities.md).
- [FR-059](./FR-059-assess-finite-global-conformance.md).
- [FR-060](./FR-060-separate-protocol-claims.md).
- [Protocol contract](../../proposals/quire-v1/protocol-contract.md).
