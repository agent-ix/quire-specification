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
  - target: ix://agent-ix/quire-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-094
    type: depends_on
  - target: ix://agent-ix/quire-verification/FR-007
    type: depends_on
---
## Description

When any protocol assessment is requested, the result producer SHALL retain separate assessment-execution, subject-progress/closure, truth, settlement-basis, activation, participation, assumptions, completeness, observation-adequacy, protocol-adequacy, claim and provenance facts for the exact requested subject.

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
Assessment execution states whether the requested evaluator completed or why it
did not. Subject progress/closure states whether the assessed workflow, trace or
temporal decision scope and the whole surrounding execution are independently
open, closed or incomplete. Completeness states which facts required by the
returned claim are available and identifies every missing dependency. The
result producer SHALL NOT use completeness as a substitute for either closure
state.

Every assumption SHALL retain its exact assumption identity and one state from
`met`, `not-met`, `unknown` or `not-assessed`. Observation adequacy states
whether the exact observations required to interpret the requested result are
`adequate`, `inadequate`, `unknown` or `not-assessed`. Protocol adequacy states
whether the exact declared obligation and case set is `demonstrated`,
`not-demonstrated`, `inapplicable` or `unknown`. The result producer SHALL NOT
use either adequacy domain as evidence for the other.

Every truth result SHALL carry exactly one settlement basis from
`closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled` or
`unavailable`, plus the exact supporting fact and progress identities. A truth
may settle while the subject remains open only when the selected profile proves
that the retained witness or counterexample preserves that truth under every
admitted continuation. Missing facts outside that exact decision-support set
remain visible in completeness and SHALL NOT force the producer to delay or
falsify the settled truth. A missing fact inside the decision-support set SHALL
make that truth unavailable.

When a later observation, progress or closure record contradicts a premise in a
settled result, the result producer SHALL retain the earlier result bytes and
emit a typed superseding or invalidating result that identifies the earlier
result, the contradicted premise and the corrected input identity. The result
producer SHALL NOT rewrite, reseal or silently replace the earlier result. A
premise contradiction is not an admitted continuation under the earlier input
identity.

When the result is mapped to the portable quire-verification assessment, the
projection adapter SHALL preserve every independent execution, activation,
observation, participation, assumption and logical finding required by
quire-verification FR-007. The adapter SHALL describe protocol adequacy only in
the canonical protocol result through the exact exercised protocol obligations
and cases. The adapter SHALL NOT represent protocol adequacy as E02 observation
adequacy or as an evidence-sufficiency decision.

### Portable verification projection

The selected target is `quire-verification/e02-draft-2`. The adapter consumes
one strict-reader-accepted protocol result and one exact E02 evidence obligation,
plan, task, run, profile, binding and dependency-closure context. It emits a
derived `TechniqueResult`; it does not replace or re-identify the canonical
protocol result.

The derived `TechniqueResult.resultId` SHALL be distinct from the canonical
protocol-result identity and stable for the exact tuple of canonical result
identity and digest, target obligation and execution-binding identities, target
contract version, and adapter identity and version. Changing any member of that
tuple SHALL change the derived result identity. `nativeArtifacts` SHALL retain
one exact URI, digest and media type for the canonical protocol-result bytes,
and `provenance` SHALL identify the protocol producer plus this adapter and their
versions. The E02 property, subject, profile, binding, dependency-closure,
execution-scope, plan, task and run fields SHALL equal their selected context;
the adapter SHALL NOT infer or substitute one from display text.

The following mapping is total for the selected source and target revisions:

| Protocol source fact | Required E02 fact |
| --- | --- |
| Assessment execution `completed` / `refused` / `unsupported` / `resource-incomplete` / `failed` | `execution.status` is respectively `completed` / `invalid-input` / `unsupported` / `exhausted` / `failed`; the source state remains in the canonical-result artifact. |
| Truth `satisfied` / `violated` / `pending` | `logical.outcome` is respectively `satisfied` / `violated` / `inconclusive`. |
| Truth `unavailable` after non-completed assessment execution or inactive activation | `logical.outcome` is `not-evaluated`. |
| Truth `unavailable` after completed execution with triggered or unknown activation | `logical.outcome` is `unknown`. |
| Activation `triggered` / `inactive` / `activation-unknown` | `activation.status` is respectively `activated` / `not-activated` / `unknown`. |
| Known required participants all present / some present / none present / no participant required | `participation.status` is respectively `complete` / `partial` / `not-started` / `not-required`, with exact participant and missing-participant identities. |
| Participation or its required set is unknown | `participation.status` is `unknown`, with every known participant and missing-dependency identity retained. |
| Assumption `met` / `not-met` / `unknown` / `not-assessed` | The same exact assumption identity and E02 assumption status. |
| Observation adequacy `adequate` / `inadequate` / `unknown` / `not-assessed` | The same E02 `adequacy.status`, with exact missing-observation identities when known. |
| Protocol adequacy, completeness, subject progress/closure, settlement basis and claim | Retained in the canonical protocol-result artifact; never substituted for an E02 finding axis. |

Every `satisfied` or `violated` projection SHALL derive its E02 oracle
contributions from the exact decision-support identities and retain the true
independent, shared, derived or unknown relation. The adapter SHALL NOT invent
an independent oracle relation. A valid source state that simultaneously implies
several FR-007 gaps SHALL produce all of them after E02 assessment, regardless
of the E02 compatibility summary status.

The adapter SHALL refuse the projection without emitting a plausible
`TechniqueResult` when the source result fails its strict reader, the target
contract version or a required feature is unknown, an exact context or canonical
result reference is missing or conflicting, a source state has no mapping in the
selected table, or the output would lose an applicable FR-007 finding. The
adapter SHALL reuse Quoin evidence authority and SHALL NOT emit a sufficiency
decision.

The strict reader SHALL enforce the following result-combination rules.

### Result combination rules

| Condition | Required relation |
| --- | --- |
| Assessment execution is refused, unsupported, resource-incomplete or failed | Truth and settlement basis are unavailable. Already-established activation, subject-progress and provenance facts remain retained. |
| Truth is satisfied | Assessment execution is completed, activation is triggered and the settlement basis is `closed-scope` or `decisive-witness`. Every fact in the exact decision-support set is complete. An open decision scope is valid only with a decisive witness whose truth is preserved by every admitted continuation; the surrounding execution may remain open after either basis settles. |
| Truth is violated | Assessment execution is completed, activation is triggered and the settlement basis is `closed-scope` or `decisive-counterexample`. Every fact in the exact decision-support set is complete. An open decision scope is valid only with a decisive counterexample whose truth is preserved by every admitted continuation; the surrounding execution may remain open after either basis settles. |
| Truth is pending | Assessment execution is completed, activation is triggered, subject progress is open and the settlement basis is `unsettled`. Required observations are complete through the retained progress point, but admitted continuations do not yet preserve one Boolean truth. |
| A required decision-support fact is missing | Truth and settlement basis are unavailable for that decision. A truth already settled from a disjoint exact support set remains settled; completeness retains the missing fact and prevents dependent adequacy or package success. |
| A retained decision-support or closure premise is later contradicted | The earlier result bytes remain unchanged. A new result identifies the earlier result and contradiction, makes dependent package success unavailable and recomputes truth only under the corrected exact input identity. |
| Activation is inactive | Assessment execution is completed, truth is unavailable and the trigger scope is complete under an exact closure authority. The surrounding subject execution may remain open. |
| Activation is activation-unknown | Truth is unavailable and completeness is open or incomplete with the missing trigger dependencies. |
| Protocol adequacy is demonstrated | Every assessment execution and required input for the declared case set is complete and the exact required case set is present. Subject closure is required only when the declared adequacy claim depends on it. An obligation-path demonstration additionally requires triggered activation. Observation adequacy remains independently adequate. |
| Package status is complete success | Every selected required request satisfies its claim-specific success rule. Unsupported or incomplete requests remain represented and prevent complete success. |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-061-AC-1 | Healthy, violating, inactive, pending, incomplete, refused, unsupported, failed and settled-open-subject cases remain discriminable without diagnostic prose, including independent assumption, observation-adequacy and protocol-adequacy facts. | Test (TC-061) |
| FR-061-AC-2 | Removing a trigger, required role, provider observation, population member or negative case cannot improve truth, adequacy or package-level status. | Test (TC-061) |
| FR-061-AC-3 | Every result binds exact source/profile/model/protocol/request/binding/input/subject-progress/closure/backend identities, settlement basis, decision-support identities and derived artifact digests. | Test (TC-061) |
| FR-061-AC-4 | A result with an impossible dimension combination, stale identity or conflicting claim strength is refused by a strict reader. | Test (TC-061) |
| FR-061-AC-5 | One unsupported request does not remove an independent supported result, while package complete success remains unavailable. | Test (TC-061) |
| FR-061-AC-6 | Every row of the normative result-combination table admits its valid boundary case and refuses every one-axis forbidden combination. | Test (TC-061) |
| FR-061-AC-7 | Every source-state row maps to the selected portable verification contract and retains every simultaneous independent FR-007 finding with no sufficiency decision. | Test (TC-064) |
| FR-061-AC-8 | A decisive temporal witness or counterexample may settle truth while the subject remains open only with completed assessment execution, exact complete decision support and proof that every admitted continuation preserves the result; missing support, a non-decisive basis or false closure refuses that combination. | Test (TC-061) |
| FR-061-AC-9 | A late contradiction to decision support, progress or closure preserves the earlier result bytes and emits a source-bound superseding or invalidating result; it cannot restamp or silently replace the earlier result. | Test (TC-061) |
| FR-061-AC-10 | Every projected result has a distinct deterministic identity, exact target execution binding and retained URI/digest/media-type reference to unchanged canonical protocol-result bytes; substituting any identity or digest refuses the projection. | Test (TC-064) |
| FR-061-AC-11 | E02 adequacy is derived only from observation adequacy; protocol adequacy remains in the canonical result and neither adequacy domain can establish the other or an evidence-sufficiency decision. | Test (TC-064) |
| FR-061-AC-12 | An invalid source, unknown target revision or required feature, unrepresented source state, missing exact context, or projection that would lose an applicable FR-007 finding is refused without a `TechniqueResult`. | Test (TC-064) |

## Dependencies

- [Shared capability dispatcher](./FR-031-report-requested-capabilities.md).
- [FR-059](./FR-059-assess-finite-global-conformance.md).
- [FR-060](./FR-060-separate-protocol-claims.md).
- [Protocol contract](../../proposals/quire-v1/protocol-contract.md).
- [Portable independent-finding contract](ix://agent-ix/quire-verification/FR-007).
