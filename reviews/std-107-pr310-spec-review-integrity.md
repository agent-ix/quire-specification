---
id: SR-13643
title: "Integrity review of QSpec PR #310 (STD-107) FR-322-AC-84, TC-233, vectors"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@d54caee35ed62feafe6019ef048ec50ea249132c vs main d080afa; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, proposals/checked-package-v2/dependency-selection-vectors.json, proposals/checked-package-v2/README.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-107 (PR #310, head d54caee35ed62feafe6019ef048ec50ea249132c). `quire validate --scope . 'spec/**/*.md'` gives the same 134 failing documents as main; FR-322 and TC-233 add no failure. A trial merge of PR #310 and PR #311 conflicts in FR-322's criteria table, and each side adds a row `FR-322-AC-84`.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-84 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| README schema_only | proposals/checked-package-v2/README.md | examined |
| dependency-selection-vectors.json schema_only | proposals/checked-package-v2/dependency-selection-vectors.json | examined |
| TC-233 procedure | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-84 is also added by PR #311 (STD-217, the `generated` occurrence role). Both rows sit after AC-83, so the second PR to merge conflicts, and resolving the conflict by keeping both rows gives two criteria with one id. One PR must renumber (the second to merge takes AC-85) before merge | spec/objects/interfaces/FR-322-checked-package-artifact.md:1753 |
| FND-002 | low | AC-84 says `Test (TC-233)`, but the PR adds only TC-233 procedure prose: no test in tests/checked_package_v2.rs carries `FR-322-AC-84` and no published vector holds the supply-order, locus or signature cases (that half is split to STD-237). Note the reader verification lives in the IR lane, or keep the AC unbound until STD-237 lands | spec/objects/interfaces/FR-322-checked-package-artifact.md:1753 |
| FND-003 | low | `schema_only` lists `dependency_selections` and `order_vectors`, but `model_selections` (digest `8888…`) and the `entry_mutations`/`model_entry_mutations` entries also carry placeholder digests, while the README says only `closure_vectors` name suppliable packages. List every placeholder member, or say the list names only the members a reader would try to admit | proposals/checked-package-v2/README.md:74-80 |

## Verdict

Not mergeable: FND-001 (high) must be resolved with PR #311.
