---
id: SR-13636
title: 'Spec review of QSpec PR #312: FR-468, EARS conformance'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@3fa26ceda; PR #312 diff origin/main...HEAD: spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md; ticket STD-217'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-468
  type: reviews
---

## Summary

Ticket: STD-217. PR quire-specification#312, head 3fa26cedad15da7e78f66ccf84060f4912ea408d.

EARS lens over FR-468's requirement statement, Behavior rules and ACs. The Description is a state-driven SHALL ('When a refinement declaration has no ..., the checker SHALL ...') with a named subject. The ACs are each single-outcome and testable through TC-950.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-468 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Inputs | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Outputs | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Initial query | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Step query | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Items | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Certificate | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Settlement/certified | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Settlement/uncertified | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Settlement/counterexample | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Settlement/failed-verification | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Exclusion | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-1 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-2 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-3 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-4 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-5 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-354 Behavior | spec/functional/ir-proof/FR-354-record-proof-trust-basis.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Unverifiable' and 'fails verification' are not defined, and they lead to different results: an 'unverifiable' certificate settles `proved`, `Uncertified`, while one that 'fails verification' settles `inconclusive`. Without a definition, a checker that cannot parse a proof, or that meets an unchecked rule, could go either way. FR-354 draws the line (a step whose rule is outside the checked set, with no failed check, is `uncertified`; an invalid step, a proof that ends short of the empty clause, a mismatched query or shape is `certificate-rejected`), and QSL FR-314 calls the first `Unverifiable`. Fix: define both terms by FR-354's table, or replace both bullets with a reference to it. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:50, 52 |
| FND-002 | low | The Description puts two requirements in one SHALL sentence: stating the queries, and settling `Certified`. The Behavior rules that tests depend on are written as facts with no SHALL: 'is not an inductive certificate', 'a step part is never accepted in place of the base part', the four settlement bullets, and 'its safety half is decided by the explicit-state engine only'. Fix: split the Description into its two parts, and write the refusal, settlement and routing rules as 'The checker SHALL ...' statements, so each one can be traced to the AC that tests it. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:22 |

## Verdict

Mergeable on this lens alone after FND-001 (an undefined term that decides between two results); FND-002 is wording.

## Dispositions

Round 1, head a56c80c5c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in a56c80c5c: The checker SHALL check the certificate as FR-354 does, so a certificate that arrives without its base part, without a row's step part, with a step part in the base position, or with a part whose carried query differs from the core's encoding, settles `inconclusive`, cause `certificate-rejected{rule, at}` with `at` naming the part (FR-354's shape and query mismatch cases). A proof that arrives with no certificate at all, and a part using a rule outside the checked set with no failed check, are not rejected: the corresponding query settles `proved` with certification `uncert |
| FND-002 | fixed | Fixed in a56c80c5c: The checker SHALL settle the safety half by the following rules, taking the first that applies in the order initial query, hypothesis items, then step queries in `steps` order: - **Initial counterexample.** A backend counterexample to the initial query is a concrete initial state whose image is no a |
