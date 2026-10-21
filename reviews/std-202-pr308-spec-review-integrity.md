---
id: SR-13637
title: "Integrity review of QSpec PR #308 (STD-202) FR-432 and TC-386"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@b9d097109bddaa4843fc8a425152ba2f69f9db7a vs main d080afa; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/test-cases/TC-386-explicit-protocol-step-rows.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-386
    type: reviews
---

## Summary

Ticket: STD-202 (PR #308, head b9d097109bddaa4843fc8a425152ba2f69f9db7a). New AC ids FR-432-AC-11..13 are unused elsewhere in the repo and each is cited by a TC-386 row (SR-10, IL-05, IL-06). `quire validate --scope . 'spec/**/*.md'` gives the same 134 failing documents at the head as on main; FR-432 and TC-386 add none.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-432-AC-11 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-12 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-13 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| TC-386 SR-10 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386 IL-05 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386 IL-06 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-13 names the IL-01 fixture's nodes (`node Spec::Full::to`, `fe`, `par`), but the TC row that cites it, SR-10, builds a different fixture (`Spec::Req` extended with `Tm`, `Fe`, `Par`, plus `Ghost`). Either state AC-13 over the SR fixture or move the vector onto IL-01's `Full` | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:208 |
| FND-002 | low | AC-11, AC-12 and AC-13 are inserted above AC-10 in the criteria table, so the table no longer reads in id order | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:206-209 |

## Verdict

Low findings only.

## Dispositions

Disposition pass 1, against head 2d36bd89cd04354984efa21c81a7df2ff2f4c159.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | verified at 2d36bd89cd04 |
| FND-002 | fixed | verified at 2d36bd89cd04 |
