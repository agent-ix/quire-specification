---
id: SR-14619
title: 'Spec integrity analysis of PR #349: FR-322 structural node preimage, parameter and compound-unit forms (STD-97)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@c6bd361; git diff origin/main(97bfd65)...c6bd361 (PR #349, branch std-fr322-std-97): FR-322 value/parameter paragraph, new "Structural node preimage" section with P4 and U2, FR-322-AC-97, TC-233 SP-01..SP-05, FR-142 compound-unit identity paragraph, value-compound-unit.md; QSL FR-092 (quire-spec-language origin/main); ticket STD-97'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-142
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: STD-97. PR: quire-specification#349, head c6bd361. Part of the ticket was already on main 97bfd65: `ValueNode` lists `parameter` with `ParameterBody`, `ScalarTypeNode` lists `compound_unit`, and FR-322 defines the compound-unit body and U2. This PR adds what was still missing. It publishes the `quire.structural-node/v1` preimage so IR can re-derive keys, spells the parameter body, and separates the runtime `quire.value.compound-unit/v1` identity from the I04 `scalar_type`/`compound_unit` node, which resolves the FR-142 / value-compound-unit.md contradiction the ticket names. The ticket's compound-unit `dependencies` ("exactly those units") is not restated here; it lands with #348's `scalar_type`/`compound_unit` table row. The member table agrees member by member with QSL FR-092, including the binder list and `level` rule. Both lead defaults leave no unbuildable AC: QSpec restates the preimage while QSL FR-092 mints, and no parameter-type restriction is added. Ids: FR-322-AC-97 is unique. Main ends at AC-93; #338 holds AC-94, #342 AC-95 and #348 AC-96. No other open PR adds `SP-` vectors. Independent recompute (own RFC 8785 encoder, Python): the P4 preimage built from the member table hashes to `ebe64b4c3bc2cc5f3460a43480f8979f5df24c5e7665d9e89890f43517dd8eda`. Its JCS text is byte-identical to the preimage the section spells out, and it equals QSL FR-092's P4 row on quire-spec-language origin/main. P4 at level 1 gives `97542d61…` and P4 named `y` gives `44459043…`, so they differ from P4 and from each other. U2 hashes to `bcf431d15781d66fcdd2514e251004373eb64a3ef21e37f7b2535f2a871821e7`, equal to FR-322's existing U2 row. The same preimage reproduces the `node_id` of every ungrouped structural node in all ten positive fixtures (101 nodes, model declaration nodes included, none failing) and in #347's two new fixtures, so the table matches the shipped keys. Every `value`/`parameter` node in the fixtures binds `name` then `level` and carries no `declaration` or `owner`. schema.json's `ParameterBody` (two `prefixItems`, `maxItems` 2) refuses `level` before `name` and a third member, as SP-04 states. A grep over spec/ and proposals/ (reviews/ excluded) finds no remaining "is not an I04 checked-semantic node" statement. Gates: `quire validate --scope . spec/**/*.md` reports 134 failed documents with a diagnostic multiset identical to main 97bfd65. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 117 and 14.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 value/parameter paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:255 | examined |
| FR-322 Structural node preimage (scope sentence) | spec/objects/interfaces/FR-322-checked-package-artifact.md:766 | examined |
| FR-322 Structural node preimage member table | spec/objects/interfaces/FR-322-checked-package-artifact.md:776 | examined |
| FR-322 P4 vector | spec/objects/interfaces/FR-322-checked-package-artifact.md:792 | examined |
| FR-322 U2 vector | spec/objects/interfaces/FR-322-checked-package-artifact.md:804 | examined |
| FR-322 compound-unit node vs value identity | spec/objects/interfaces/FR-322-checked-package-artifact.md:817 | examined |
| FR-322-AC-97 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1906 | examined |
| FR-142 compound-unit value identity paragraph | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:226 | examined |
| value-compound-unit.md identity note | proposals/quire-v1/definitions/value-compound-unit.md:5 | examined |
| FR-322 derived-shape key stage | spec/objects/interfaces/FR-322-checked-package-artifact.md:409 | context_only |
| QSL FR-092 structural-node/v1 preimage and P4 | agent-ix/quire-spec-language@origin/main spec/functional/FR-092-key-type-parameter-and-declared-nodes.md:90-108,352-377,509 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The preimage, both vectors and the parameter body are exact and agree with QSL FR-092 and with every shipped fixture key. The compound-unit identity split removes the contradiction without changing any key. The PR body contains no review-finding paraphrase.
