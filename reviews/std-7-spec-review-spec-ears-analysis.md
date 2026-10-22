---
id: SR-15211
title: 'EARS analysis of PR #358: native value type reference form (STD-7)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@2f3494c; git diff origin/main...2f3494c (PR #358, branch std-7-native-typeref): proposals/quire-v1/definitions/model-complete.md (Identity, Fields rows); spec/functional/type-model/FR-154-admit-domain-package-model.md (admission row, two declaration rows, typeRef paragraph, FR-154-AC-17); spec/test-cases/TC-234-domain-package-model-intake.md (I11); spec/test-cases/TC-195-model-normalization-provenance.md (N16); ticket STD-7'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
---
# EARS analysis of PR #358: native value type reference form (STD-7)

## Summary

Ticket: STD-7. PR #358 at 2f3494cc9d9cfa99a138cfd8f1991e5d2dc54eda, reviewed in a detached worktree; diff scope origin/main...head. Method: spec-review/spec-ears-analysis.

Examined:
- FR-154-AC-17 (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- FR-154 (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- FR-154 admission row (reserved identity) (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- FR-154 declaration row (typeRef form) (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- FR-154 declaration row (parameterized native) (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- model-complete Identity row (examined): proposals/quire-v1/definitions/model-complete.md
- model-complete Fields row (examined): proposals/quire-v1/definitions/model-complete.md
- TC-234 I11 (examined): spec/test-cases/TC-234-domain-package-model-intake.md
- TC-195 N16 (examined): spec/test-cases/TC-195-model-normalization-provenance.md
- quire.model.normalize.intake/v1 (context_only): proposals/quire-v1/definitions/model-complete.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | [minor] 'Rational, Decimal and Text resolve only with every parameter of their type-ref production' (FR-154 paragraph and model-complete Fields row) states a necessary condition only and gives no wire member for those parameters on a typeRef, so whether a typeRef carrying them resolves is unspecified; QSL main refuses the three unconditionally ('cannot resolve no matter what a producer's own document supplies', domain_package.rs), while AD-016:430 says they are admitted with the parameters. Two implementers would read it differently. | spec/functional/type-model/FR-154-admit-domain-package-model.md:161-162; proposals/quire-v1/definitions/model-complete.md:165 |

## Verdict

Wording is behaviour-only and EARS-conformant apart from one ambiguous condition (minor).
