---
id: SR-7829
title: "Integrity review — QSL-727 operation-body identity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@56012a865ab4a8510b271be3299ec35627af3d3a; spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md, spec/test-cases/TC-938-operation-body-identity-is-unique.md, spec/complete-v1/tests.md, proposals/quire-v1/package-contract.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-151
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-938
    type: reviews
---

## Summary

Disposition pass 1 rechecked completeness, consistency, identity scope, testability, and relevant failure-domain probes. The prior cross-unit collision gap is fixed by package-wide resolved-identity wording and a two-unit alias-resolution test; no new integrity finding was identified.

## Verdict

**PASS** — the identity constraint covers all body declarations in the linked package and TC-938 exercises a duplicate across units after alias resolution.

## Coverage

- Completeness: the named two-unit positive control and negative mutation are both linked to FR-151-AC-16 by the test index.
- Consistency: FR-151 uses the linked package as the uniqueness domain and requires alias resolution per unit before comparing operation identities, consistent with unit-local aliases and shared package identity semantics.
- Identity probe: TC-938 uses aliases V/M and W/N for the same model identity, succeeds for distinct operation identities, and refuses when the second unit targets the first unit's operation identity.
- Atomicity/testability: the one-body rule is observable as either successful linking or a named typed refusal with both declarations; no partial package/dispatch table is produced.
- Failure-domain probe: identity is named; extension/purity/topological probes do not apply to this static declaration-multiplicity rule. This integrity pass applies the relevant identity probe; no separate failure-domain analysis was selected.
- No new external dependency/relationship edges were added; TC-938 adds a `verifies` link to FR-151.
- No applicable AssuranceProfile with a `review_selection` was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new uniqueness rule is limited to one unit, so duplicate bodies in separate source units can still resolve to the same operation identity without the required refusal | spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md:181-184; spec/test-cases/TC-938-operation-body-identity-is-unique.md:19-24; proposals/quire-v1/package-contract.md:20-24 |

## Dispositions

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed 56012a865ab4a8510b271be3299ec35627af3d3a | FR-151 and TC-938 now cover cross-unit resolved identity after local alias resolution |

### Round 3

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed fd2f87159179485e2a38dc9fb3e50d9fe03b5e7c | TC-938 now runs the duplicate case with reversed source-unit order and expects the same refusal with no partial output |

after_excerpt: `4. Keep the duplicate bodies and reverse the two source units from step 3; compile the linked package again.`

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | TC-938 expects source-unit-order invariance for duplicate refusal, but its procedure never reverses the units, so that behavior is not exercised | spec/test-cases/TC-938-operation-body-identity-is-unique.md:41-42 |
