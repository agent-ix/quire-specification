---
id: SR-14613
title: 'Spec integrity analysis of PR #348: FR-322 node dependencies for every node kind (STD-93)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@c2a9875; git diff origin/main(97bfd65)...c2a9875 (PR #348, branch std-fr322-std-93): FR-322 `dependencies` property, FR-322-AC-96, TC-233 DP-01..DP-07, native-diagnostics.md step 3, checked-package-v2 README, positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json; ticket STD-93'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: STD-93. PR: quire-specification#348, head c2a9875. The rule is faithful to the ticket: reference targets at any depth, a recursion-group member counted by its wire `node_id`, `bounded_domain` → `semantic_type`, type annotations not listed, and a `dependency_reference` adds no entry (QSL QC-10). It also restates the existing application, nominal and frame joins. The reader join for every node is a stated lead default, and it leaves no unbuildable AC. Ids: FR-322-AC-96 is unique. Main ends at AC-93; #338 holds AC-94, #342 AC-95 and #349 AC-97. No other open PR adds `DP-` vectors. Independent recompute (own RFC 8785 encoder, Python): every positive fixture's `package_id` equals SHA-256 of its JCS `identity_preimage`. That includes all-families `1b62f696…` (was `09e949d6…`) and operation-identities `01740176…` (was `1ccf2e8d…`), and each `identity_projection` entry is its node minus `occurrences`. In dependency-selection-vectors.json, the top `package_id` `85d55f62…`, both closure vectors' `test/a` `35c25eaf…`, both `test/leaf` selections and the transitive `dependency_reference.package` (`1b62f696…`) recompute from the base fixtures, and `test/b` is unchanged. A rebuild of every node's `dependencies` by rules 1 to 3 matches all ten positive fixtures. The only differences are nodes whose list rule 4 or a body artifact supplies: frame `modifies` declarations, `enum_value` → enum declaration, `unit` → dimension, and FR-451 `abstraction_relation`. Each of those matches its stated join. Node ids are unchanged. The old ids `09e949d6`, `331fed52`, `c7ff6307` and `1ccf2e8d` occur nowhere under spec/, proposals/, tests/ or src/. #347's new positive fixtures (positive-dependency-call.json, positive-dependency-library.json) also conform to rules 1 to 4, so AC-96 still holds after #347 merges. A grep over spec/ and proposals/ (reviews/ excluded) finds no remaining statement that a non-application node's list is free. QSL FR-093 "Node dependencies" ("No v2 reader joins the list of such a node") goes stale on merge, which the PR lists as QSL remaining work. Gates: `quire validate --scope . spec/**/*.md` reports 134 failed documents with a diagnostic multiset identical to main 97bfd65. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 117 and 14.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 dependencies (definition and reader rule) | spec/objects/interfaces/FR-322-checked-package-artifact.md:491 | examined |
| FR-322 dependencies rules 1-4 | spec/objects/interfaces/FR-322-checked-package-artifact.md:498 | examined |
| FR-322 dependencies type annotations and body-artifact lists | spec/objects/interfaces/FR-322-checked-package-artifact.md:513 | examined |
| FR-322 dependencies example table | spec/objects/interfaces/FR-322-checked-package-artifact.md:531 | examined |
| FR-322 dependencies refusal | spec/objects/interfaces/FR-322-checked-package-artifact.md:542 | examined |
| FR-322-AC-96 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1895 | examined |
| native-diagnostics.md step 3 | proposals/quire-v1/definitions/native-diagnostics.md:138 | examined |
| checked-package-v2 README dependencies sentence | proposals/checked-package-v2/README.md:313 | examined |
| positive fixtures and dependency-selection-vectors.json | proposals/checked-package-v2/fixtures/positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json | examined |
| FR-322 semantic_graph cycle rule | spec/objects/interfaces/FR-322-checked-package-artifact.md:260 | context_only |
| FR-322 derived-shape self-typed rule | spec/objects/interfaces/FR-322-checked-package-artifact.md:461 | context_only |
| FR-322 application-node dependencies rule | spec/objects/interfaces/FR-322-checked-package-artifact.md:382 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new property says a reader follows `dependencies` "beside `semantic_type` and the body's references when it builds a node's closure and checks graph cycles". Every `scalar_type`, `composite_type` and model declaration node is its own `semantic_type` (derived-shape key stage), so read literally each is a one-node cycle with no `recursion_group`. FR-322's `semantic_graph` rule refuses such a cycle ("all other graph cycles refuse"). State that a self-typed node's `semantic_type` is not a cycle edge, or name the edges the cycle check reads. The sentence is copied from QSL FR-093. | spec/objects/interfaces/FR-322-checked-package-artifact.md:491-497 |

## Verdict

Mergeable after an optional wording fix. One minor finding (FND-001). The rules, the example table, the refusal (`invalid_semantic_graph` at `/dependencies`, graph-shape step 3), the README and native-diagnostics step 3 agree with each other and with FR-322's existing application, carrier, nominal and frame text. Every corrected fixture and vector id recomputes independently. The PR body contains no review-finding paraphrase.
