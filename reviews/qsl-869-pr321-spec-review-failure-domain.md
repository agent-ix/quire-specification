---
id: SR-13681
title: 'Failure-domain review of QSpec PR #321: legacy population entries'
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@8602df3; git diff origin/main...8602df3 (PR #321, branch qspec-population-identity): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-280-model-owned-member-types.md, proposals/checked-package-v2/model-owner-form-vectors.json, tests/model_member_types.rs; ticket QSL-869"
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
---

## Summary

Ticket: QSL-869. Failure modes and reachability of the new 'an entry with `node` and no `identity` declares no population' clause and its PR-07 case. A full reader admits the document under FR-154 (FR-322 step 1) before the owner join runs. FCD's population schema requires `identity` and forbids extra members (schema.rs:842-848), and FR-154 refuses a population with no valid artifact id `invalid_model_binding`/`malformed-declaration`. The PR body agrees: 'QSL's intake rejects a `node` member on a population'. Identity confusion between `node` and `identity` is otherwise closed: P7 and P8 come from the declaration key, and the join reads `identity` only. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `cargo test --test model_member_types --test checked_package_v2 --test backend_provider_v1 --test checked_package_model` passes all: model_member_types 24, checked_package_v2 114, backend_provider_v1 14, checked_package_model 4. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, identical to main apart from one FR-322 warning shifting 9 lines; the touched files add none. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Population references: Owner join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-280 PR-07 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-154 declaration refusals | spec/functional/type-model/FR-154-admit-domain-package-model.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-82 and the owner-join bullet say that a population entry with a `node` member and no `identity` reaches the owner join and refuses `missing_declaration`/`missing-selection`. A conforming full reader never gets there. Under step 1 the document is read under FR-154, and such an entry is malformed: FCD requires `identity` and forbids `node`, and FR-154 refuses a population with no artifact id. So the read refuses `invalid_model_binding`/`malformed-declaration` at stage `package_read`, the very refusal QSL saw at /lock/model_selections/0. As written, the AC contradicts FR-154's refusal order. PR-07 passes only because the harness overrides the projection after the read, a state no admitted document produces. An implementer following AC-82 could skip identity-less entries instead of refusing the document. Fix: state the full-reader outcome, 'a selected document whose population entry lacks `identity` or carries `node` refuses `invalid_model_binding`/`malformed-declaration` at `package_read` (FR-154)'. Keep PR-07 only as a join-level guard that the join reads `identity` alone, worded as such (e.g. 'the join finds no population whose `identity` is `owner.node`'), or move it to a full-reader case that expects the FR-154 refusal. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1284-1285 |

## Verdict

Not mergeable: FND-001 (high) puts AC-82 in conflict with FR-154. The fix is wording plus the PR-07 description.
