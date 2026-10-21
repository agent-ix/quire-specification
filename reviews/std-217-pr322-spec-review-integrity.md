---
id: SR-13692
title: "Integrity review of QSpec PR #322 (STD-217) refinement step occurrences"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@dcc70c07d1a0905407a5242fbbbb5620727aae33 vs main 6ab82c5; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, proposals/checked-package-v2/README.md (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-217 (PR #322, head dcc70c07d1a0905407a5242fbbbb5620727aae33). FR-322-AC-88 and AC-89 are the next free ids. No open PR (#267, #207, #200, #59) uses them. Both are traced by `#[trace("TC-233", "FR-322-AC-88", "FR-322-AC-89")]`, and TC-233 names every vector. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents on both branch and main, and the line-normalized output is identical, so the touched files add no failure.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-88 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-89 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 refinement step step | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| proposals README | proposals/checked-package-v2/README.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-88's clause "with the rows written in the other source order, `steps` order and so each ordinal are unchanged" cannot fail in TC-233. Each vector supplies `rows` already in `steps` order, and nothing in the test derives that order from a source order, so RS-02 is just a second, independent input. The invariance belongs to FR-375-AC-19 (TC-310). Either drop the clause from AC-88 and cite FR-375-AC-19, or give the vectors source-order rows with sort keys that the test orders. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1803; tests/checked_package_v2.rs:14531-14555 |
| FND-002 | low | `proposals/checked-package-v2/README.md` describes each vector file under "Contract contents", but the new `refinement-step-occurrence-vectors.json` is not listed. Add an entry for it. | proposals/checked-package-v2/README.md |

## Verdict

Medium FND-001; fix in this PR. Ids, trace tags and validation are otherwise clean.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fe8930c: AC-89 now states a concrete written order (`R1`, `R0`) against the `steps` order (`R0`, `R1`); the oracle maps each `steps` name to its row by name, and an oracle using written order fails RS-02. |
| FND-002 | fixed | fe8930c: README lists the vector file. |
