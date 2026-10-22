---
id: SR-7827
title: "Base spec review — QSL-727 operation-body identity"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@56012a865ab4a8510b271be3299ec35627af3d3a; spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md, spec/test-cases/TC-938-operation-body-identity-is-unique.md, spec/complete-v1/tests.md, proposals/quire-v1/package-contract.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-151
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-938
    type: reviews
---

## Summary

Disposition pass 1 rechecked the changed FR, TC, and tests index at the exact head. IDs, links, and index binding remain coherent; the prior package-wide identity coverage gap is fixed, with no new base-checklist finding.

## Verdict

**PASS** — the requirement and TC cover the ticket-authorized cross-unit duplicate after local alias resolution, as well as the distinct-operation control.

## Coverage

- ID format/uniqueness: FR-151-AC-16 follows AC numbering; TC-938 is present once and linked to FR-151; the complete-V1 index contains the new AC→TC row and TC summary row.
- Verification uses method `Test` in FR-151; the named TC is linked in the tests index, satisfying the spec-side trace path. The computed matrix separately reports this AC as `untagged` with no source-test binders; the matrix does not consume the authored TC/index mapping as an implementation trace tag.
- The TC control has one body each for distinct A/size and B/size identities in two units. The duplicate mutation uses a different local label and aliases M/N to the same model identity, requiring `conflicting_declaration`/`duplicate-declaration` across units.
- FR-151 constrains all source units linked into one package and explicitly requires resolution after each unit's local model/import alias; the declared key is identity, not alias spelling.
- Rechecked ticket scope: the patch preserves operation-body source/dispatch semantics, adds no diagnostic family, and confines the new requirement to the approved one-body-per-operation-identity contract.
- No options, state transitions, or new hash/digest/pin/manifest requirements are introduced. The new code/cause is the one authorized by QSL-727.
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
