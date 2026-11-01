---
id: SR-16283
title: "integrity review of QSpec PR #560 (STD-619)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@71261bd2480ef2f975416f8188b2a7454fce52c9; FR-322-AC-288, FR-322-AC-289, FR-322-AC-290, FR-322-AC-291, FR-322-AC-292; ticket STD-619"
review_set: all
---

## Summary

Integrity analysis of the edited carrier requirements and corresponding TC-233 test. Identity is explicit, failure outcomes are enumerated, vectors exercise positive and malformed cases, and the focused test is tagged. The AC-292 work-limit arithmetic conflicts with its vector values.

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
| FND-001 | medium | FR-322-AC-292 retains the old 29/28 work-limit values, but the updated vector and assertion use 24/23. This leaves the requirement, vector, and test with different exact admission boundaries. | FR-322-AC-292 |

## Verdict

**CONDITIONAL** — the exact work-limit contract must match its vector and test.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-322-AC-292 now states the exact vector boundary: `The vectors contain 17 sidecar rows, 3 unit facts, 2 enum facts and 2 dispatch candidates, with zero equality `term_path` segments, for an exact 24-unit boundary; admission succeeds at 24 units. A 23-unit limit refuses `resource_exhausted` at `/checked_term_data/dispatch_tables/0/candidates/1`, the final charged candidate, with no partial data view.` |
