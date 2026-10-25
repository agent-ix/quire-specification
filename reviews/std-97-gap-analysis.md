---
id: SR-14621
title: 'Gap analysis of PR #349: FR-322-AC-97 against TC-233 SP-01..SP-05 (STD-97)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@c6bd361; git diff origin/main(97bfd65)...c6bd361 (PR #349, branch std-fr322-std-97): FR-322 value/parameter paragraph, new "Structural node preimage" section with P4 and U2, FR-322-AC-97, TC-233 SP-01..SP-05, FR-142 compound-unit identity paragraph, value-compound-unit.md; QSL FR-092 (quire-spec-language origin/main); ticket STD-97'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: STD-97. Plan completion: not assessed. This is a manual mapping of new and changed ACs to TC rows. FR-322-AC-97 (new) maps to TC-233 SP-01 (P4 key), SP-02 (level 1 and name `y` differ), SP-03 (U2 key), SP-04 (`level` before `name` and a third binding fail the V2 schema, `malformed_wire`) and SP-05 (every parameter node current, full reader admits each fixture). No existing AC changed. No test is tagged to FR-322-AC-97 yet; the PR lists it as STD-97 remaining work, and the existing every-structural-node key test already exercises SP-05. Independent recompute (own RFC 8785 encoder, Python): the P4 preimage built from the member table hashes to `ebe64b4c3bc2cc5f3460a43480f8979f5df24c5e7665d9e89890f43517dd8eda`. Its JCS text is byte-identical to the preimage the section spells out, and it equals QSL FR-092's P4 row on quire-spec-language origin/main. P4 at level 1 gives `97542d61…` and P4 named `y` gives `44459043…`, so they differ from P4 and from each other. U2 hashes to `bcf431d15781d66fcdd2514e251004373eb64a3ef21e37f7b2535f2a871821e7`, equal to FR-322's existing U2 row. The same preimage reproduces the `node_id` of every ungrouped structural node in all ten positive fixtures (101 nodes, model declaration nodes included, none failing) and in #347's two new fixtures, so the table matches the shipped keys. Every `value`/`parameter` node in the fixtures binds `name` then `level` and carries no `declaration` or `owner`. schema.json's `ParameterBody` (two `prefixItems`, `maxItems` 2) refuses `level` before `name` and a third member, as SP-04 states.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-97 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1906 | examined |
| TC-233 Structural node preimage vectors SP-01..SP-05 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:363 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Each clause of FR-322-AC-97 has an SP row, and each stated key and schema outcome holds under independent recompute.
