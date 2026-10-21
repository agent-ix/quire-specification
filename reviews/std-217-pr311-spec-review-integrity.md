---
id: SR-13647
title: "Integrity review of QSpec PR #311 (STD-217) FR-322-AC-84 and FR-375-AC-19"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d vs main d080afa; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/functional/analysis/FR-375-declare-model-refinement.md, spec/test-cases/TC-310-refinement-declaration-admission.md; context spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-217 (PR #311, head 0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d). `quire validate --scope . 'spec/**/*.md'` gives the same 134 failing documents as main. FR-375-AC-19 is cited by TC-310 mutation 23. FR-322-AC-84 collides with PR #310 and has no TC row.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-322-AC-84 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 generated role | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-310 mutation 23 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-84 is also added by PR #310 (STD-107, dependency supply). A trial merge of the two conflicts in the criteria table, and keeping both rows gives two criteria with one id. Renumber in whichever PR merges second (AC-85) | spec/objects/interfaces/FR-322-checked-package-artifact.md:1744 |
| FND-002 | medium | AC-84 says `Test (TC-233)`, but the PR does not touch TC-233 and no fixture or test has a compiler-derived node. Its example, "a per-step refinement record", is defined nowhere in merged QSpec (that record is STD-217 item 2, still open, and PR #312 is unmerged), and "its owner's total order" names no owner for a derived node. As written no test can be built from it. Add a TC-233 row over a concrete derived node once one exists, or define the record and its owner here | spec/objects/interfaces/FR-322-checked-package-artifact.md:1744 |

## Verdict

Not mergeable: FND-001 (high) and FND-002 (medium).
