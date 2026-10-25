---
id: SR-17124
title: "Base review of QSpec refinement wire correction"
type: SpecReview
analysis: base
scope: "QSpec PR 439 at b703774b; FR-322, FR-375, TC-310 and checked-package-v2 schemas"
review_set: base
---

## Summary

The refinement node is excluded from the general application and structural key stages and receives a dedicated identity check after abstraction. The V2 schema forbids the four members outside its closed preimage. The acceptance-evidence finding is resolved on the updated head. No applicable AssuranceProfile was found in the repository spec tree.

Targeted Quire validation passed for FR-375 and TC-310. FR-322 failed with missing `id` and `fields` catalog errors; both occur in the unchanged `origin/main` file. The changed JSON schema parses, and its refinement condition composes with `NodeBase`'s owner, declaration and occurrence rules.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Resolved at b703774b: TC-310 mutation 26 adds RW-01 through RW-08, covering admitted read/write stability, both stale-key edits, canonical row ordering, absent local and external header keys, wrong-kind row target and wrong-type inline expression. Each refusal has a specified code, cause, pointer and reader step. FR-375 now identifies inline term `type` or `result_type` as the row type source. | FR-375 Reader order and AC-20/21; TC-310 mutation 26; FR-322 Body steps |
