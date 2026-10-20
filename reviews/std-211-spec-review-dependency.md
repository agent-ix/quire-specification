---
id: SR-13327
title: "Dependency review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md, spec/functional/analysis/FR-377-decide-model-refinement.md, spec/test-cases/TC-315-refinement-undetermined-and-replay.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-376
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
---

## Summary

Ticket: STD-211. Checks the edges the new text creates: FR-375 to FR-143, FR-147, FR-001; FR-376 to FR-149 and native-diagnostics; TC-315 to FR-375 (added). Ordering against open #286 (STD-210) checked.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-deps | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-376-deps | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | examined |
| FR-376-AC-5 | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | context_only |
| FR-149-AC-21 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | New prose dependencies are not recorded as edges. FR-375's Option rows and Value profile paragraphs rely on FR-143 (null, none, absence) and FR-147 (division selection) and on profile selection (FR-001), but its frontmatter and Dependencies list none of them; FR-376-AC-5 now rests on FR-149 and FR-376-AC-7 on the native-diagnostics Undefined reasons, neither in FR-376's frontmatter. Blast-radius queries on FR-143, FR-147 or FR-149 miss these FRs. Add the edges. | spec/functional/analysis/FR-375-declare-model-refinement.md:1-30,401-420; spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md:1-15,107-118 |

## Verdict

Mergeable on this lens after FND-001 (bookkeeping). TC-315's new `verifies` edge to FR-375 is correct. No ordering dependency on open #286: it changes only `<`/`<=`/`>`/`>=` and leaves FR-149-AC-21's `=` rule, which FR-376-AC-5's rationale cites, unchanged. The QSL side depends on this PR: QSL #1038's FR-140 optional-field "remaining work STD-211" note, its order wording and its `null` rule should be revised to match whatever this PR settles.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The fix round adds prose dependencies without edges: FR-377's failure precedence now cites FR-383 (internal fault settles `failed`) but its frontmatter has no FR-383 relationship; FR-375's Row layers paragraph relies on FR-149 (`Reference` comparison under the alias's layers), with no FR-149 edge; FR-001's new bullet and AC-7 are about refinements but FR-001 has no edge to FR-375. Add the edges. | spec/functional/analysis/FR-377-decide-model-refinement.md:1-25,94-97; spec/functional/analysis/FR-375-declare-model-refinement.md:1-36,274-276; spec/functional/foundation/FR-001-select-native-profile.md:1-20,65-68,85 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Round 2 removes FR-375's FR-001 and FR-147 relationships (added in round 1 for SR-13327 FND-001) although the Row layers paragraph still cites both, and adds no edge to FR-090 or FR-250, which the paragraph now rests on. FR-001's Dependencies also still says how refinement rows obtain their layers "is pending an owner ruling stated there", which A2 settled. Restore the two edges, add FR-090 and FR-250, and update FR-001's note. | spec/functional/analysis/FR-375-declare-model-refinement.md:1-36,265-279; spec/functional/foundation/FR-001-select-native-profile.md:98-100 |

## Dispositions

Round 1, against head 5a66bf9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a66bf9: the four requested edges are added; the fix round's own new citations lack edges: see new FND-002 |

Round 2, against head 1f15551.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | 1f15551: the three requested edges are added; the same commit drops FR-375's FR-001 and FR-147 edges: see new FND-003 |

Round 3, against head 7e85fb7.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | still-open | 7e85fb7: the edges are restored and added (FR-375 to FR-001, FR-147, FR-090, FR-250; FR-001 to FR-090, FR-250), but FR-001's Dependencies still reads "how their rows obtain complete-value and complete-model layers is pending an owner ruling stated there" (FR-001:107-109), which A2 settled |

Round 4, against head 73aad4e.

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | 73aad4e: FR-001's Dependencies now states the A2 ruling (rows admitted under the layers the governing infinite-trace profile requires, FR-090/FR-250, counted for the lock selection); the edges were restored in round 3 |
