---
id: SR-13635
title: 'Spec review of QSpec PR #312: FR-468 per-step refinement queries and certificate,
  base checklist'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@3fa26ceda; PR #312 diff origin/main...HEAD: spec/functional/analysis/FR-377-decide-model-refinement.md, spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md, spec/spec.md, spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md; ticket STD-217'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-468
  type: reviews
- target: ix://agent-ix/quire-specification/TC-950
  type: reviews
- target: ix://agent-ix/quire-specification/FR-377
  type: reviews
---

## Summary

Ticket: STD-217. PR quire-specification#312, head 3fa26cedad15da7e78f66ccf84060f4912ea408d.

Base checklist over FR-468, TC-950, the FR-377 dependency link and the spec.md index row and edge. FR-468 and TC-950 are new and unique ids (also against open PRs #308-#311); the index row and the `contains` edge are present; the ACs each name TC-950 and TC-950 scopes AC-1 to AC-5; no changelog, status or owner wording; no hash, digest or pin use. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents on main and on the PR head, the same set; the four touched files validate clean. Read against FR-375, FR-377, FR-379, FR-290, FR-331, FR-354, QSL FR-314 and ADR-020 (RE-2, RS-2 to RS-4) on main, and QSV FR-123 and src/step_simulation.rs at PR #59's head.

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
| TC-950 | spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md | examined |
| FR-377 Dependencies/FR-468 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| spec.md FR-468 row | spec/spec.md | examined |
| PR #312 body | PR #312 description | examined |
| FR-468-AC-1 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-2 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-3 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-4 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468-AC-5 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-377 Engines | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| FR-377-AC-3 | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| FR-379 Verdicts | spec/functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md | context_only |
| FR-375 Checked-package node order | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |
| FR-354 Behavior | spec/functional/ir-proof/FR-354-record-proof-trust-basis.md | context_only |
| FR-331 Certification | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| FR-290 refinement row | spec/objects/protocol/FR-290-protocol-claim-kind.md | context_only |
| QSL FR-314 shape | quire-spec-language spec/functional/FR-314-check-an-smt-proof-certificate.md | context_only |
| QSL ADR-020 RE-2 | quire-spec-language spec/decisions/ADR-020-refinement-mappings-between-qsl-models.md | context_only |
| QSV FR-123 Use case | quire-semantic-value spec/functional/FR-123-step-simulation-record-and-obligation.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A missing step part has two different outcomes. AC-2 and the Certificate paragraph say a certificate without the base part, or without one row's step part, is 'refused as not an inductive certificate', and a step part in the base position is refused. AC-3 and the second Settlement bullet say that when one query's certificate is absent the safety half settles `proved`, `Uncertified`. A certificate that lacks row k's step part is exactly row k's query with no certificate, so one implementer settles it `uncertified` and another refuses it. 'Refused' is also not a result: FR-354 settles a certificate whose shape does not match the proof basis, or whose carried query differs from the core's, as `inconclusive`, cause `certificate-rejected{rule, at}`, and QSL FR-314 does the same (`ShapeMismatch` / `QueryMismatch`, `CertificateRejected`). Fix: say that a certificate that arrives without its base part, without a row's step part, or with a step part in the base position settles `inconclusive`, `certificate-rejected{rule, at}` naming the part (FR-354), and that only a proof with no certificate at all settles `proved`, `uncertified`; make AC-2 and AC-3 state those two outcomes. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:61, 62, 50 |
| FND-002 | high | The initial query's hypotheses contradict each other. The Initial query bullet says it 'has no hypotheses beyond the declared types', yet AC-5 requires 'the initial query's identity differs with and without the clause' (TC-950 step 5 checks it). If the clause is not in the initial query, its identity does not change. The Items paragraph adds a third reading: 'the initial query is what establishes that the initial states satisfy them, which is its own item for each invariant clause', so the initial query both is one query for the initial-state rule and is an item per invariant clause. The count is unclear too: AC-1's 'one initial query and seven step queries' leaves out the per-clause initial-state items. QSV FR-123 (PR #59, same ticket) writes the `Initial` record with the invariant clauses as `hypotheses`, which matches neither 'no hypotheses' nor a separate item. Fix: state once whether the initial-state query carries the invariant clauses, and whether each clause's initial-state check is a separate item outside the refinement's certificate; then make AC-1's count, AC-5's identity claim and the QSV `Initial` record agree with it. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:64, 38, 41 |
| FND-003 | medium | The settlement uses QSL's Rust spellings, not QSpec's wire vocabulary. FR-331 makes `certification` the closed values `certified`, `uncertified` and `trusted`, and FR-468 writes `Certified` and `Uncertified` (also in AC-3 and TC-950). The inconclusive causes are prose ('cause induction not closed', 'with the rejected part and proof step named'), while FR-331 closes the causes as `induction-not-closed{depth}` and `certificate-rejected{rule, at}`. FR-468 also does not cite FR-354, which owns the checker outcome to certification table, or FR-331. Fix: use `certified` / `uncertified`, name `induction-not-closed{depth}` with the depth a per-step query carries (QSL ADR-020 RE-2 uses `depth: 1`) and `certificate-rejected{rule, at}`, and add FR-354 and FR-331 to the relationships and Dependencies. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:49-52 |
| FND-004 | low | The owner ruling is not stated in the PR body, and the body leaves the rejected option open. It says QSL FR-314's `Inductive{base, step}` 'will need a many-step variant or a step query that is the disjunction of the rows' negations. I did not decide that QSL-side encoding.' The ruling is one step part per row (a disjunction of the rows' negations would lose which row failed and does not match FR-377's per-step obligation), so QSL FR-314 needs a follow-up to carry a step part per row. Fix: edit the PR body to record that ruling and drop the disjunction alternative, and name the QSL follow-up for FR-314. | PR #312 description |
| FND-005 | low | The exclusion sentence covers three cases, but AC-1 and TC-950 step 1 test only one. A declaration with a hidden field, a history field or an `any` row has no per-step queries, but AC-1 tests only the `any` row. A history field is the case FR-377 gives a reason for (a per-step obligation cannot see history values). Fix: extend AC-1 and TC-950 step 1 with a hidden-field and a history-field variant, or cite FR-377-AC-7 for those two. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:60 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | FR-331's prose adds `hypotheses-not-established{hypothesis, result}` to the closed inconclusive causes, but FR-331's normative wire, the closed schema proposals/backend-provider-v1/schema.json ('The normative wire of this value is the closed JSON schema', FR-331 Wire schema), was not changed. Its `InconclusiveCause` `oneOf` (schema.json:670-688, every branch `additionalProperties: false`) has no `hypotheses-not-established` branch, so a result that FR-468's settlement produces fails the schema and a conforming reader refuses it. FR-331's prose and its schema now disagree, and FR-468-AC-6's expected result cannot be put on the wire. Fix: add the branch to `InconclusiveCause` (`hypothesis` as a checked-package `NodeId` ref and `result` as `#/$defs/ResultValue`, like `premise-unproved`), add `FR-468` to that definition's description list, and add a positive and an adverse vector to vectors.json with a TC-431 row, then rerun `cargo test --test backend_provider_v1`. PR #309 does not touch schema.json, and #312 merges cleanly with #309, #310 and #311. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:279-282 |

## Verdict

Not mergeable. FND-001 and FND-002 are contradictions inside FR-468 that change a test's expected result: a missing step part is both `uncertified` and refused, and the initial query's hypotheses are read three ways. FND-003 puts QSpec's wire vocabulary in place of QSL's spellings. FND-004 is a PR-body fix, and FND-005 a coverage gap. The rest passes: the base/step query split; per-row step parts in FR-375 `steps` order (the owner ruling); the exclusion of hidden, history and `any` declarations, which are routed to the explicit-state engine; `inconclusive` and never `refuted` for a step counterexample, which agrees with FR-379; the TC procedure covers every AC.

## Dispositions

Round 1, head a56c80c5c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in a56c80c5c: A certificate that arrives without the base part, without one row's step part, with a step part in the base position, or with a carried query differing from the core's settles the half `inconclusive`, `certificate-rejected{rule, at}` naming the part; a proof with no certificate at all settles that query `proved`, `uncertified`. |
| FND-002 | fixed | Fixed in a56c80c5c: A concrete invariant clause is a hypothesis of every step query and of the initial query, so the initial query's identity differs with and without the clause, and an invariant of the abstract model is not a hypothesis. |
| FND-003 | fixed | Fixed in a56c80c5c: - **Certificate verdicts.** A certificate the checker shows wrong settles the half `inconclusive`, cause `certificate-rejected{rule, at}` (FR-354). Otherwise, with every query and hypothesis item `proved`, the half settles `proved`, basis `decisive-witness`, with `certification` `certified` when every part and every hypothesis item is certified and `uncertified` when any of them is not (the weakest of the parts and the hypothesis items). - **Other backend results.** A query the backend settles by none of these (a solver `unknown`, a timeout or limit, a missing solver or too |
| FND-004 | fixed | Fixed in a56c80c5c: PR #312 description: Owner ruling recorded: the inductive certificate is **per-row step parts, not a disjunction**: a base part (the initial query's refutation) and one step part per row in FR-375 `steps` order, so a failure names its row. QSL FR-314's `Inductive{base, step}` carries a single step query, so QSL FR-314 needs a follow-up to carry a step part per row. |
| FND-005 | fixed | Fixed in a56c80c5c: A refinement of a model with seven step rows and no hidden field, history field or `any` row has one initial query and seven step queries, each its own `operation-contract` item, the step queries in `steps` order; the same declaration with one row changed to `any`, with a hidden abstract field, or with a history field has no per-step query. |
| FND-006 | still-open | New this round; not yet fixed. |

Round 2, head ac1f71dcb (proposals/backend-provider-v1/schema.json, vectors.json, TC-431 BP-21).

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | Fixed in ac1f71dcb: {"description": "FR-468: hypothesis is the invariant clause's checked-package node; result is the result of its unproved item.", "properties": {"type": {"const": "hypotheses-not-established"}, "hypothesis": {"$ref": "#/$defs/NodeId"}, "result": {"$ref": "#/$defs/ResultValue"}}, "required": ["type", "hypothesis", "result"], "additionalProperties": false} |
