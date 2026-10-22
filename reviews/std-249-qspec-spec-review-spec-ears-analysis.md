---
id: SR-14031
title: 'Spec review of PR #330: binder record of an abstract node (STD-249), EARS analysis'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@2f0a09d; git diff origin/main...2f0a09d (PR #330, branch std-protocol-249-qspec, based on main, not stacked): spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-386-explicit-protocol-step-rows.md; ticket STD-249'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-432
  type: reviews
---

## Summary

Ticket: STD-249. PR: quire-specification#330, head 2f0a09d, based on main. Method: spec-review with the EARS sub-analysis, over the FR-432 statement and the added Behavior section. Tooling: quoin 0.28.3, quire 0.36.2 (engine 0.50.2), spec-artifacts-process@b82e42e.

The FR-432 statement is unchanged and is a conditional (`When ... the checker SHALL ...`) with one subject. The new section is behaviour-only prose: it states fields, types and refusals and records no build history.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-432 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:21-25 | examined |
| FR-432 binder-record section | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:87-107 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-432's SHALL statement covers deciding each concrete step class and reading visibility and labels; it does not cover typing a node row's arguments against the binder record or the two new refusals, which live only in Behavior prose as "refuses". Every other Behavior rule of FR-432 is written the same way, so this follows the FR's own style, but AC-14 and AC-15 now verify behaviour the statement does not name. Fix: extend the statement, for example "... and type each node row's arguments against the node's binder record". | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:21-25 |

## Verdict

Mergeable for EARS apart from one low finding: the statement does not name the behaviour the two new ACs verify.

## Dispositions

Re-checked at head 3d88428. Gates at head 3d88428: `quire validate --scope . spec/**/*.md` reports 134 failed documents, none of them FR-432 or TC-386 (touched files add none); `cargo test --test checked_package_v2 --test backend_provider_v1` passes (117 and 14 tests). The head contains origin/main. FR-432-AC-14/15 and TC-386 BR-01/02 are used by no other open QSpec PR. The PR body states behaviour, defaults, gates and remaining work and paraphrases no review finding.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed text at the new head: When a refinement declaration's concrete or abstract side is a protocol subject, the checker SHALL decide every concrete protocol step class by exactly one explicit deciding `step` row, refuse a class that no row decides or that two rows of one kind decide, type each node row's arguments against the node's binder record, and read each step's visibility and observation label from its deciding row alone. |
