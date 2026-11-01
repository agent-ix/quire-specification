---
id: SR-16281
title: "base review of QSpec PR #560 (STD-619)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@71261bd2480ef2f975416f8188b2a7454fce52c9; FR-322-AC-288, FR-322-AC-289, FR-322-AC-290, FR-322-AC-291, FR-322-AC-292; ticket STD-619"
review_set: all
---

## Summary

Base checklist for changed FR-322 AC-288..292 and the TC-233 carrier vector test. The representation change is consistent with the body grammar: every composite application is a separate referenced node, nested equality facts point to the child application NodeRef with empty term_path, and EQ-R6 refuses at the reference without traversing it. The vector metadata is explicitly described as outside the wire shape. One stale work-bound statement remains.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-288 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-289 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-290 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-291 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-292 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-292 still states an exact 29-unit boundary and one-less 28, while the changed vector now declares exact_limit 24 and one_less_limit 23 after removing seven equality-path units and adding two equality rows. Align the AC with the vector/test or update the vector back to the stated contract. | FR-322-AC-292 |

## Verdict

**CONDITIONAL** — the exact work-limit contract must match its vector and test.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-322-AC-292 now states the exact vector boundary: `The vectors contain 17 sidecar rows, 3 unit facts, 2 enum facts and 2 dispatch candidates, with zero equality `term_path` segments, for an exact 24-unit boundary; admission succeeds at 24 units. A 23-unit limit refuses `resource_exhausted` at `/checked_term_data/dispatch_tables/0/candidates/1`, the final charged candidate, with no partial data view.` |
