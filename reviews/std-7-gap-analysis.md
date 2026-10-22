---
id: SR-15212
title: 'Gap analysis of PR #358: FR-154-AC-17 against TC-234 I11 (STD-7)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@2f3494c; git diff origin/main...2f3494c (PR #358, branch std-7-native-typeref): proposals/quire-v1/definitions/model-complete.md (Identity, Fields rows); spec/functional/type-model/FR-154-admit-domain-package-model.md (admission row, two declaration rows, typeRef paragraph, FR-154-AC-17); spec/test-cases/TC-234-domain-package-model-intake.md (I11); spec/test-cases/TC-195-model-normalization-provenance.md (N16); ticket STD-7'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
---
# Gap analysis of PR #358: FR-154-AC-17 against TC-234 I11 (STD-7)

## Summary

Ticket: STD-7. PR #358 at 2f3494cc9d9cfa99a138cfd8f1991e5d2dc54eda, reviewed in a detached worktree; diff scope origin/main...head. Method: gap-analysis.

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
| FND-001 | medium | [minor] New normative statements have no AC or TC row: the two declaration rows cover parameter and result typeRefs (and the paragraph names Operation.params[i].typeRef and Operation.returns.typeRef as their loci), but AC-17 and I11 retype fields only; only Decimal is tested of the three parameterized natives (no Rational or Text row); and 'a constraint on a natively typed field mints no type node' has no vector. | spec/functional/type-model/FR-154-admit-domain-package-model.md:319; spec/test-cases/TC-234-domain-package-model-intake.md:52 |

## Verdict

FR-154-AC-17 has a TC row (TC-234 I11) that covers every clause of the AC; recomputed by script, run 1's eight keys and their order are right. The coverage gaps are in paragraph content outside the AC (minor). TC-195 N16 only changes spelling. The ticket's TC-197 item needs nothing: TC-197 uses package value types only.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | [minor] QSL main (b453e10) no longer matches the revised FR-154 and no follow-up ticket was found (ix-board search): `read_value_type_ref` still refuses bare `ix://quire/native/Rational`, `Decimal` and `Text` at intake `unsupported_construct`/`declaration-form` through `NativeValueType::unsupported_parameters`, where FR-154 now admits them and leaves the refusal to member resolution; and it reads only the native and own-package forms, so `ix://quire/unit/ms` refuses `malformed-declaration` ("is not ix://quire/native/<Name> or ix://{package}/<artifact id>") where FR-154 and FR-208 admit it unresolved. Spec-only PR, so this needs a QSL implementation ticket, not a change here. | qsl-semantics/src/model/intake.rs:2957-2995; qsl-semantics/src/model/domain_package.rs:300-312; spec/functional/type-model/FR-154-admit-domain-package-model.md:85,166 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | c930f02: FR-154-AC-17 now adds "an operation parameter and an operation result typed `ix://quire/native/Int` refuse the same at the operation node and `Operation.params[i].typeRef` and `Operation.returns.typeRef`, while the same parameter and result typed `ix://quire/native/Text` are admitted", plus admitted Text, Rational, Decimal and `ix://quire/unit/ms` fields; TC-234 I11 covers every field clause (twelve keys, recomputed by script, byte order correct) and new I12 covers the parameter and result runs. The constraint clause is now "A field typed by a native value type declares no type node of its own", which I11's "none is a type declaration for a native-typed field" asserts. |
