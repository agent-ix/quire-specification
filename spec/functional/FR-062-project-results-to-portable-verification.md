---
id: FR-062
title: "Project protocol results to portable verification"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-061
    type: depends_on
  - target: ix://agent-ix/quire-verification/FR-007
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
  - target: ix://agent-ix/quire-specification/FR-244
    type: references
  - target: ix://agent-ix/quire-specification/FR-245
    type: references
  - target: ix://agent-ix/quire-specification/FR-246
    type: references
  - target: ix://agent-ix/quire-specification/FR-248
    type: references
  - target: ix://agent-ix/quire-specification/FR-249
    type: references
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
  - target: ix://agent-ix/quire-specification/FR-280
    type: references
  - target: ix://agent-ix/quire-specification/FR-282
    type: references
  - target: ix://agent-ix/quire-specification/FR-283
    type: references
---
## Description

When a canonical protocol result is submitted for portable evidence assessment, the projection adapter SHALL emit one source-bound `quire-verification/e02-draft-2` `TechniqueResult` or one typed refusal without losing or conflating an FR-007 finding.

## Inputs

One FR-061 `quire.protocol.result/v1-draft.1` result plus one exact E02 evidence
obligation, plan, task, run, profile, binding, dependency closure and adapter
identity/version context.

## Outputs

One derived E02 `TechniqueResult`, or one refusal carrying a stable code from
`invalid-source-result`, `unsupported-source-version`,
`unsupported-target-version`, `identity-mismatch`, `unrepresented-source-state`
or `finding-loss`, plus the affected exact identity.

## Behavior

The projection adapter SHALL validate the canonical protocol result and
recompute its `sha256-jcs` digest before mapping it. The adapter SHALL preserve
every independent execution, activation, observation, participation, assumption
and logical finding required by quire-verification FR-007. The adapter SHALL
describe protocol adequacy only in the canonical protocol result through the
exact exercised protocol obligations and cases. It SHALL NOT represent protocol
adequacy as E02 observation adequacy or as an evidence-sufficiency decision.

The derived `TechniqueResult.resultId` SHALL be distinct from the canonical
protocol-result identity and stable for the exact tuple of canonical result
identity and digest, target obligation and execution-binding identities, target
contract version, and adapter identity and version. Changing any member of that
tuple SHALL change the derived result identity. The result ID is
`sha256-jcs:<lowercase-hex>` over this RFC 8785 canonical identity preimage:

```json
{
  "identityVersion": "quire.protocol.e02-projection-identity/v1-draft.1",
  "sourceResultRef": { "identity": "<exact-result-id>", "digest": "<sha256-jcs>" },
  "targetContractVersion": "quire-verification/e02-draft-2",
  "targetBinding": {
    "propertyRef": "<exact-artifact-reference>",
    "planId": "<exact-plan-id>",
    "taskId": "<exact-task-id>",
    "runRef": "<exact-artifact-reference>",
    "profile": "<exact-profile>",
    "bindingRefs": ["<ordered-exact-artifact-reference>"],
    "dependencyClosure": "<exact-digest>",
    "executionScope": "<exact-scope>"
  },
  "adapter": { "identity": "<exact-adapter-id>", "version": "<exact-version>" }
}
```

The angle-bracket strings describe typed values rather than literal wire text;
the preimage contains their complete JSON values. `bindingRefs` uses the selected
E02 semantic order and SHALL NOT be resorted by presentation text.
`nativeArtifacts` SHALL contain
exactly one canonical-result entry with the source URI, recomputed digest and
media type
`application/vnd.agent-ix.quire-protocol-result+json;version=1-draft.1`.
`provenance` SHALL identify the protocol producer plus this adapter and their
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
| Protocol adequacy, completeness, decision-scope progress/closure, surrounding-execution progress/closure, complete-global-conformance closure, settlement basis and claim | Retained in the canonical protocol-result artifact; never substituted for an E02 finding axis. |

Every `satisfied` or `violated` projection SHALL derive its E02 oracle
contributions from the exact decision-support identities and preserve the
source `independent`, `shared`, `derived` or `unknown` relation and lineage. The
adapter SHALL NOT invent an independent oracle relation. A valid source state
that simultaneously implies several FR-007 gaps SHALL produce all of them after
E02 assessment, regardless of the E02 compatibility summary status.

The adapter SHALL refuse without emitting a `TechniqueResult` when the source
result fails its strict reader, the source or target contract version or a
required feature is unknown, an exact context or canonical-result reference is
missing or conflicting, a source state has no mapping in the selected table, or
the output would lose an applicable FR-007 finding. The adapter SHALL reuse
Quoin evidence authority and SHALL NOT emit a sufficiency decision.

The intended implementation owner is the future first-party Rust
`agent-ix/quire-protocol` library after private setup ticket #33 is authorized.
quire-verification owns the selected E02 contract and strict assessment reader;
Engineering Assurance supplies the accepted plan/task/run execution context and
typed result transport; Quoin retains evidence authority. This requirement does
not authorize a local runner, evidence store, E02 fork or substitute adapter in
another repository.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-062-AC-1 | Every source-state row maps to the selected E02 contract and retains every simultaneous independent FR-007 finding with no sufficiency decision. | Test (TC-064) |
| FR-062-AC-2 | Every projected result has a distinct deterministic identity, exact target execution binding and retained URI/digest/media-type reference to unchanged canonical protocol-result bytes; substituting any identity or digest refuses the projection. | Test (TC-064) |
| FR-062-AC-3 | E02 adequacy is derived only from observation adequacy; protocol adequacy remains in the canonical result and neither adequacy domain can establish the other or an evidence-sufficiency decision. | Test (TC-064) |
| FR-062-AC-4 | An invalid source, unknown source or target revision or required feature, unrepresented source state, missing exact context, or projection that would lose an applicable FR-007 finding is refused with the corresponding stable code and no `TechniqueResult`. | Test (TC-064) |

## Dependencies

- [Canonical protocol result](./FR-061-report-orthogonal-results.md).
- [Portable independent-finding contract](ix://agent-ix/quire-verification/FR-007).
