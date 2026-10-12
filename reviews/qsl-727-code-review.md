---
id: SR-7826
title: "Code review — quire-specification QSL-727 operation-body identity"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@56012a865ab4a8510b271be3299ec35627af3d3a; spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md, spec/test-cases/TC-938-operation-body-identity-is-unique.md, spec/complete-v1/tests.md, proposals/quire-v1/package-contract.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-151
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-938
    type: reviews
---

## Summary

Disposition pass 1 re-reviewed PR #229's spec/test-only diff at the exact head, including FR-151 semantics, TC-938, the complete-V1 test index, and multi-unit package context. The prior cross-unit gap is fixed: the rule and named test now exercise identity after unit-local aliases resolve across one linked package. No new issue was found.

## Verdict

**PASS** — FR-151 states package-wide resolved-identity uniqueness; TC-938 covers two units, distinct aliases for the same model identity, the distinct-operation control, and cross-unit duplicate refusal.

## Coverage

- Code-review scope: spec/code faithfulness, test-case procedure versus expectation, and requirement-to-test/index alignment. No implementation files changed; no Rust lane or implementation tests applied.
- Re-examined FR-151-AC-16, the package-wide FR-151 rule, TC-938, its FR relationship, both added `spec/complete-v1/tests.md` rows, and package-contract source-unit/alias semantics.
- TC-938 compiles two linked units using local aliases V/M and W/N for the same model identity; its control targets distinct A/size and B/size identities, and its mutation targets A/size from the second unit. The expected refusal is alias-independent and identifies both declarations.
- The computed matrix reports FR-151-AC-16 `untagged` with no implementation test binders. The test index does bind the criterion to TC-938 and the TC has a concrete procedure; no implementation trace tag is expected in this spec-only ticket. This is recorded as an index-versus-code-matrix distinction, not a missing ticket-scope test-case finding.
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
