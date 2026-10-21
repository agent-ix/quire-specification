---
id: SR-13680
title: 'Integrity review of QSpec PR #321: FR-322 owner join population identity'
type: SpecReview
analysis: integrity
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

Ticket: QSL-869. Consistency of the changed text with FR-154, FCD's population schema and the rest of FR-322. Checked against filament-core-data crates/semantic-ir/src/schema.rs:818-830 (POPULATION_MEMBERS and POPULATION_OPTIONAL_MEMBERS) and QSL qsl-semantics/src/model/domain_package.rs:729 (QSL's `PopulationRecord` struct). Gates at HEAD (own clone, own CARGO_TARGET_DIR): `cargo test --test model_member_types --test checked_package_v2 --test backend_provider_v1 --test checked_package_model` passes all: model_member_types 24, checked_package_v2 114, backend_provider_v1 14, checked_package_model 4. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, identical to main apart from one FR-322 warning shifting 9 lines; the touched files add none. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Population references: Owner join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-280 PR-07 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-322 Model-owned members step 1 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-154 identity rule | spec/functional/type-model/FR-154-admit-domain-package-model.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The text names the entry type `PopulationRecord` and attributes it to 'FCD semantic IR 2.0.0 and FR-154', with the closed member list `{identity, displayName, kind, members, extent, origin}`. Neither FCD nor FR-154 defines a `PopulationRecord`. FCD's schema calls it a population, and its members are those six plus an optional `symmetric` (FR-381/FR-150 symmetric populations). The only `PopulationRecord` in the stack is QSL's internal struct (qsl-semantics domain_package.rs:729: `key`, `member_types`, `extent`, `symmetric`), a different shape. A strict reader that takes the braces as the closed shape would refuse a symmetric population. Fix: 'each entry is an FCD semantic IR 2.0.0 population (`identity`, `displayName`, `kind`, `members`, `extent`, `origin`, optional `symmetric`)', without the `PopulationRecord` name, or with an explicit statement that the list is not closed. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1276-1278 |
| FND-002 | low | AC-82 refuses 'an entry that carries only a `node` member'. The FR says 'an entry that carries a `node` member and no `identity`', which is broader, since it includes an entry with `node`, `displayName` and the rest but no `identity`. The criterion and the statement name different classes. Fix: make them the same (and see SR-13681 FND-001 for which refusal applies). | spec/objects/interfaces/FR-322-checked-package-artifact.md:1765 |
| FND-003 | low | 'as FR-154 gives every declaration its artifact id (`ix://<package identity>/<artifact id>`)'. FR-154 gives that form to a type. For other declarations it defers to FCD FR-095's identities, and populations are not among its four exceptions. A population's identity is FCD's. Fix: 'as FR-154 takes a declaration's IR node identity from FCD (FR-095)'. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1279-1280 |
| FND-004 | low | Step 1 lists the selected document's members as `contractVersion`, `package`, `constructs` and `types`. The owner join now says the projection's `populations` is that document's own top-level list, which step 1 never names. Fix: add `populations` (and what each entry carries, or a reference to the owner-join bullet) to step 1's list. | spec/objects/interfaces/FR-322-checked-package-artifact.md:822-825 |

## Verdict

Mergeable for integrity once FND-001 is fixed (medium, wording); FND-002 to FND-004 are low.
