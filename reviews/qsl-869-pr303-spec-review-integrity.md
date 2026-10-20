---
id: SR-13603
title: 'Spec review of QSpec PR #303: FR-322 population reference operand, integrity'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@ab6ff70; git diff origin/main...ab6ff70 (PR #303, branch qspec-fr322-popref): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/functional/type-model/FR-153-query-closed-model-environments.md, spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-280-model-owned-member-types.md; ticket QSL-869'
review_set: subset
date: '2026-10-20'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-153
  type: reviews
- target: ix://agent-ix/quire-specification/FR-451
  type: reviews
---

## Summary

Ticket: QSL-869. PR: quire-specification#303, head ab6ff70. Integrity lens over the changed requirements and test cases: ids, completeness, consistency, atomicity. New ids FR-322-AC-82 and AC-83, the TC-280 rows PR-01..PR-06 and the vectors P7/P8 are unique in the tree. TC-233 verifies AC-82 and AC-83 and TC-280 verifies AC-82 in frontmatter, and both new tests carry `#[trace]` tags to them. FR-322 adds `references` edges to QSL FR-391 and FR-451. FR-153 and FR-208 were already edges. Consistency: the family-rule edit removes main's contradiction between 'a relation reference is `population`' and 'A relation node ... have no family'. The owner-presence table (F322:765-770), the model-owned list (F322:568-573), the step-2 kind table, the README and both schemas all enumerate `relation`/`population`. FR-451's population entry key wording matches its object entry wording. AC-82 and AC-83 are compound in this file's house style, and each clause has a case. Validation: the only FR-322 differences from main are line shifts, and the other four touched documents report nothing.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-62 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-83 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-family-rule | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-153-population-name-in-package | spec/functional/type-model/FR-153-query-closed-model-environments.md | examined |
| FR-451-population-entry-key | spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md | examined |
| TC-233-population-reference-cases | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| TC-280-PR-01..PR-06 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-322-population-references-relation-to-population | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The sentence appended to AC-62, 'The `Population<Order>[3]` form types a population binding and names no declaration; a declared population is a population reference (FR-322-AC-82)', is a definition, not an outcome. The AC's listed tests (tc_322_derived_form_key_mutations, tc_322_recomputed_derived_forms_refuse_semantic_mutations) cannot fail on it. Fix: move it to the 'Relation to Population<O>[N]' bullet, where it already appears, and leave AC-62 as it was. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1711 |
| FND-002 | low | FR-153 now depends on FR-322's population reference and links it in the body, its only body link, but the frontmatter has no `references` edge to FR-322 (it lists US-010, FR-043 and AD-006 only). The relationship graph therefore misses FR-153 -> FR-322. Fix: add `- target: ix://agent-ix/quire-specification/FR-322` with `type: references`. | spec/functional/type-model/FR-153-query-closed-model-environments.md:151-155 |

## Verdict

No blocking integrity defects. Two low findings: a definitional sentence was added to AC-62 (FND-001), and FR-153 is missing its frontmatter edge to FR-322 (FND-002). The cross-document enumeration of the new form is complete and consistent.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at c58cd9f: Node` schema. Retaining each original key exercises stale-key detection, and recomputing each key exercises semantic-shape refusal for a non-model reference target, negative bound, arbitrary set element type and noncanonical term order as `invalid_package`/`invalid-derived-shape`. \| Test (`tests/checked_package_v2.rs`, `tc_322_derived_form_key_mutations`, `tc_322_recomputed_derived_forms_refuse_semantic_mutations`) \| |
| FND-002 | fixed | Round 1 at c58cd9f: relationships: ... - target: ix://agent-ix/quire-specification/AD-006 type: references - target: ix://agent-ix/quire-specification/FR-322 type: references |

Round 1 at c58cd9f: both fixed. AC-62 ends at its semantic-shape refusals again, and the definition lives only in the 'Relation to' bullet. FR-153's frontmatter has a `references` edge to FR-322.
