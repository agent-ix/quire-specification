---
id: SR-7886
title: "Base review of QSpec PR #241 (ancestor-typed slots, event_type model form)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cde3a67651d092fdb8c16d3e54b12fc8646b7b89; spec/functional/type-model/FR-143,FR-144,FR-149; spec/objects/foundation/FR-208; spec/objects/interfaces/FR-322; spec/test-cases/TC-188,TC-189,TC-194,TC-233,TC-235; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; tests/checked_package_v2.rs"
review_set: subset
date: 2026-10-13
---

## Summary

Ticket: QSL-737. PR: quire-specification#241 (branch spec/child-composite-slots-and-event-form). Requirement source: QSL FR-359 'Cross-repo requirements' 5 and 7 (quire-spec-language PR #800), with the planner rulings: an event uses a new model form (family record, field projection admitted, equality refused), and a model record's runtime key is its model node key.

Base checklist over the PR diff. All new IDs are unique across origin/main, every remote branch and the open PRs: FR-322-AC-68/69, FR-208-AC-14, FR-144-AC-15, FR-149-AC-17, TC-235 M23, TC-189 C13, TC-188 R12, TC-194 E31 and TC-233 ET-01..07 (the RC- label collision with TC-281 is in the integrity review). Every new AC names its TC and every new TC row names its criterion. The three-dot diff removes only rewrapped or replaced lines (21 deletions across FR-143/144/149/208/322 and the test oracle). The FR-208 and FR-322 `quire validate` errors are on lines this PR did not touch. No hash, pin or tracking record is introduced.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-208 | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | examined |
| FR-208-AC-14 | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | examined |
| FR-144 | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md | examined |
| FR-144-AC-15 | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md | examined |
| FR-143 | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md | examined |
| FR-143-AC-5 | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md | examined |
| FR-149 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-149-AC-17 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-31 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-68 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-69 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| FR-340 | spec/objects/interfaces/FR-340-checked-package-frame-location.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean for this analysis.
