---
id: SR-13638
title: 'Spec review of QSpec PR #312: FR-468, failure domain'
type: SpecReview
analysis: failure-domain
scope: 'agent-ix/quire-specification@3fa26ceda; PR #312 diff origin/main...HEAD: spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md; ticket STD-217'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-468
  type: reviews
---

## Summary

Ticket: STD-217. PR quire-specification#312, head 3fa26cedad15da7e78f66ccf84060f4912ea408d.

Failure-domain lens: unstated failure modes of the per-step proof route: unproved hypotheses, backend outcomes other than proof and counterexample, the initial query's counterexample, partial mapping rows, and how the two halves combine. The exclusion of hidden-field, history-field and `any` declarations is explicit and routes them to the explicit-state engine; a per-step counterexample is never `refuted`, which agrees with FR-379.

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
| FR-377 Engines | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| FR-377-AC-3 | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| FR-379 Verdicts | spec/functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md | context_only |
| QSL ADR-020 RE-2 | quire-spec-language spec/decisions/ADR-020-refinement-mappings-between-qsl-models.md | context_only |
| FR-290 refinement row | spec/objects/protocol/FR-290-protocol-claim-kind.md | context_only |
| FR-331 Certification | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The safety half can settle `Certified` on hypotheses that nothing has proved. Every step query assumes the concrete invariants at `s`. The safety half settles `proved` / `Certified` when the initial query and the step queries are refuted. Nothing in FR-468 requires the invariant clauses' own items (each its own `operation-contract` record, per QSL ADR-020 RE-2) to be proved, or certified, before that. So a concrete model that declares an invariant which fails at a reachable state, or is simply `false`, makes every step query's hypotheses unsatisfiable. Every step query is then refuted with a valid certificate, and the refinement settles `proved`, `Certified`. The lost-update counter, which FR-379-AC-7 refutes, would certify under the invariant `self.value < 0`. Fix: make the safety half's `proved` depend on every hypothesis clause's items (initial-state and preservation) settling `proved`. Its certification should be the weakest of the parts and those items. If a hypothesis item is not `proved`, the half settles `inconclusive` (for example `premise-unproved{premise, result}`, as FR-380 does). Add an AC with a false invariant. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:49, 41 |
| FND-002 | medium | The settlement list covers only proved, counterexample and failed verification. It has no row for a query whose backend returns neither a proof nor a counterexample: a solver `unknown`, a timeout or limit, a missing solver or tool, or a backend fault. FR-331 has results for these (`inconclusive` `solver-inconclusive`, `incomplete` `limit-reached{...}`, `unsupported`, `failed`). Without a row, it is unclear what the safety half is when seven queries are proved and one is unknown. Fix: add the row(s), including how one such query combines with the rest (for example, the safety half takes that query's result). | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:51 |
| FND-003 | medium | The counterexample bullet treats the initial query like a step query: 'cause induction not closed, and never `refuted`, because the counterexample's pre-state may be unreachable'. A counterexample to the initial query is a concrete initial state whose image is not abstract-initial. There is no pre-state and no induction, and FR-377-AC-3 makes exactly that state refute the refinement. Fix: give the initial query's counterexample its own outcome. Either `refuted`, `InitialNotAbstract`, after an explicit-state replay of that initial state, or `inconclusive` with a cause other than induction not closed, with the reason. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:51 |
| FND-004 | medium | The step query does not say what happens when a mapping row has no value. FR-377 reads `map` with a failure order: an undefined row refutes the claim (`undefined-evaluation`), and a refused or incomplete row is `mapping-undetermined`. The step query writes `map(s)` and `map(s')` as if they were total. A query encoding that gives an undefined row (division by zero, an out-of-range index) an arbitrary value, or that drops the case, can be refuted, and so settle `proved`, for a refinement the explicit-state engine refutes. Fix: state that each query also requires every row, receiver and argument it reads to be defined at `s` and `s'` (or that such a row makes the query fail and the half `inconclusive`), and add an AC with an undefined row. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:39 |
| FND-005 | medium | FR-468 settles the 'safety half', and the Description says 'the refinement settles `Certified`'. But FR-379's verdict table and FR-331's `results` settle the refinement item (a `temporal-satisfaction` item, per FR-290), not its halves. With `F_A` non-empty the liveness half is decided by the explicit-state engine, and FR-468 does not say how a per-step `proved` / `certified` safety half and an explicit-state liveness result combine into the item's one result and certification. Fix: say whether the per-step route is used only when `F_A` is empty, or how the two halves' results and certifications combine, and correct the Description to 'the safety half'. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:30 |

## Verdict

Not mergeable: FND-001 lets the safety half settle `proved`, `Certified` for a refinement that does not hold, through an unproved or false invariant hypothesis. FND-002 to FND-005 are real gaps that two implementers would read differently.

## Dispositions

Round 1, head a56c80c5c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in a56c80c5c: - **Hypotheses not established.** If any hypothesis item is not `proved`, the half settles `inconclusive`, cause `hypotheses-not-established{hypothesis, result}`, `hypothesis` the invariant clause's node and `result` its item's result. A false invariant makes every step query vacuous, so no step query's refutation may settle the half `proved` while a hypothesis is unproved. - **Step counterexample.** A backend counterexample to a step query, including to its definedness requirement, is never a refutation, because its pre-state may be unreachable ([FR-379](FR-379-report-and- |
| FND-002 | fixed | Fixed in a56c80c5c: - **Other backend results.** A query the backend settles by none of these (a solver `unknown`, a timeout or limit, a missing solver or tool, or a backend fault) takes the result FR-331 records for that outcome (`inconclusive` `solver-inconclusive`, `incomplete` `limit-reached{limit, value, setting}`, `unsupported`, or `failed`), and the half takes the result of the first such query. When the declaration's `F_A` is non-empty, the liveness half is decided by the explicit-state engine and FR-379's verdict table decides how the two halves combine into the item's one result; thi |
| FND-003 | fixed | Fixed in a56c80c5c: - **Initial counterexample.** A backend counterexample to the initial query is a concrete initial state whose image is no abstract initial state: it refutes the refinement ([FR-377](FR-377-decide-model-refinement.md) FR-377-AC-3). The half settles `refuted`, `decisive-counterexample`, refinement failure `InitialNotAbstract`, only after the explicit-state replay of that initial state reproduces it ([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)); a replay that does not reproduce it settles `inconclusive`, cause `replay-parity`. - **Hypotheses not establishe |
| FND-004 | fixed | Fixed in a56c80c5c: Each query also requires every mapping row, receiver and argument it reads to be defined and determined at `s` and `s'`: a state satisfying the premises at which one is undefined or undetermined is a counterexample to the query. Each query is one `operation-contract` item ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)) and is settled on its own. Each hypothesis (invariant clause) has its own `operation-contract` items, its initial-state item and its preservation item, which are *hypothesis items*; they are not part of the refinement's certificate. ### Certi |
| FND-005 | fixed | Fixed in a56c80c5c: When the declaration's `F_A` is non-empty, the liveness half is decided by the explicit-state engine and FR-379's verdict table decides how the two halves combine into the item's one result; this requirement supplies only the safety half's result and certification. A declaration with a hidden field, a history field or an `any` row has no per-step queries and no inductive certificate (FR-377-AC-7); its safety half is decided by the explicit-state engine only. ## Acceptance Criteria \| ID \| Criteria \| Verification \| \| --- \| --- \| --- \| \| FR-468-AC-1 \| A refinement of |
