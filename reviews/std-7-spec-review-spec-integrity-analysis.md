---
id: SR-15210
title: 'Spec integrity analysis of PR #358: native value type reference form (STD-7)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@2f3494c; git diff origin/main...2f3494c (PR #358, branch std-7-native-typeref): proposals/quire-v1/definitions/model-complete.md (Identity, Fields rows); spec/functional/type-model/FR-154-admit-domain-package-model.md (admission row, two declaration rows, typeRef paragraph, FR-154-AC-17); spec/test-cases/TC-234-domain-package-model-intake.md (I11); spec/test-cases/TC-195-model-normalization-provenance.md (N16); ticket STD-7'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
---
# Spec integrity analysis of PR #358: native value type reference form (STD-7)

## Summary

Ticket: STD-7. PR #358 at 2f3494cc9d9cfa99a138cfd8f1991e5d2dc54eda, reviewed in a detached worktree; diff scope origin/main...head. Method: spec-review/spec-integrity-analysis.

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
| FND-001 | high | [major] The new rule that every field, parameter and result typeRef is either ix://quire/native/<Name> or ix://<package identity>/<artifact id>, with any other form refused invalid_model_binding/malformed-declaration (FR-154 row and paragraph, model-complete Fields row 'Every typeRef takes one of these two forms'), contradicts main's third form: FR-208 Field types (:86-91) says the binder SHALL admit a typeRef ix://quire/unit/<name> without resolving it, FR-322 step 4 (:1112) and FR-322-AC-80 resolve it to Quantity<u>, and TC-280 MU-01..MU-04 expect Timer/window typed ix://quire/unit/ms to resolve and ix://quire/unit/ghost to refuse missing_declaration/missing-name, not malformed-declaration. | spec/functional/type-model/FR-154-admit-domain-package-model.md:85; spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:86-91; spec/test-cases/TC-280-model-owned-member-types.md:162-165 |
| FND-002 | high | [major] A field typed bare ix://quire/native/Text is now refused at intake unsupported_construct/declaration-form, but main's FR-322-AC-31 and TC-280 MA-06 expect that field to be admitted and to refuse later at member resolution ill_typed/operator-ineligible, and the published domain-package-acme-orders.json (FR-154-AC-9's document) carries Order/label with typeRef ix://quire/native/Text (:633), so under the new row that package refuses at intake and every TC-280 vector over acme/orders becomes unreachable. QSL main already refuses Text unconditionally (domain_package.rs unsupported_parameters), so the PR records the code but leaves FR-322-AC-31, TC-280 MA-06 and the fixture contradicting it. | spec/functional/type-model/FR-154-admit-domain-package-model.md:86; spec/objects/interfaces/FR-322-checked-package-artifact.md:1952; spec/test-cases/TC-280-model-owned-member-types.md:147; proposals/checked-package-v2/domain-package-acme-orders.json:633 |
| FND-003 | high | [major] model-complete's Intake rule still says intake 'runs these checks in order' starting with the digest-domain check and lists no reserved-identity check, while FR-154 now puts the quire/native check first ('before any other admission check') and TC-234 I11 expects that check to be reported instead of digest-domain-mismatch for a quire/native selection under quire-native-bytes-1. For that input model-complete gives stale_dependency/digest-domain-mismatch and FR-154 gives invalid_model_binding/malformed-declaration; the Identity row's 'before admission reads its bytes' does not settle the order against the digest-domain check. | proposals/quire-v1/definitions/model-complete.md:66-74; spec/functional/type-model/FR-154-admit-domain-package-model.md:72 |
| FND-004 | low | [minor] AD-016:430, which the ticket's re-measure names as one of the two disagreeing native lists, still says the native set follows the grammar's type-ref keywords (no String, UUID or Timestamp) and that Rational, Decimal and Text are admitted when inline constraints carry every parameter; the PR's closed ten-name set and QSL main (which never resolves those three) leave it stale. | spec/assurance/AD-016-semantic-family-extension-path.md:430 |

## Verdict

NOT MERGEABLE: three major contradictions with main (ix://quire/unit typeRef form, bare Text at intake vs FR-322-AC-31, model-complete Intake order). Faithful to the ticket's ruling ix://quire/native/<Name>; the ten-name set, the malformed-declaration refusal for unknown names, the unsupported_construct/declaration-form refusal for parameterized natives and the reserved quire/native selection refusal (invalid_model_binding/malformed-declaration, checked before the digest domain) all match QSL main domain_package.rs NativeValueType, intake.rs read_value_type_ref and admit (:997). Ids FR-154-AC-17 and TC-234 I11 are unique against main and open PR #267 (I10). quire validate: 134 failing documents on head, identical diagnostics to merge-base 0a11d84, none in touched files (current main a3a79a3 is 116 because of STD-106, not this PR). cargo test checked_package_v2 126 passed, backend_provider_v1 23 passed. PR body carries no review-finding paraphrase.
