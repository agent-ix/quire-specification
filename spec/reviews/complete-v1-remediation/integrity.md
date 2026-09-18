---
id: SR-600
title: "Integrity review of complete V1 identifiers and criterion backing"
type: SpecReview
analysis: integrity
scope: "every id, acceptance criterion id and Test Matrix criterion row in spec/"
review_set: subset
evaluated_revision: "3dcf31b on task/63-full-v1-specification"
review_date: "2026-09-17"
---

## Summary

Mechanical audit of artifact ids, acceptance criterion ids and criterion
backing across the whole tree. Two integrity defects were found and corrected;
the tree now carries no duplicate id and no unbacked criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-611 | high | Resolved: three SpecReview documents under i04-checked-package reused SR-580, SR-581 and SR-582, already held by other reviews. Reallocated to SR-596, SR-597 and SR-598 after scanning every remote branch. | SR-596; SR-597; SR-598 |
| FND-612 | medium | Resolved: FR-142-AC-9, FR-142-AC-10, FR-148-AC-8, FR-148-AC-9 and FR-148-AC-10 had no Test Matrix criterion row. All five are now backed. | TM-009; FR-142; FR-148 |
| FND-613 | medium | Resolved: a new criterion was appended to FR-044 as AC-7, an id that requirement already used. Renumbered to AC-8. | FR-044 |
| FND-614 | low | Two criterion-backing shapes coexist: one criterion per matrix row in the complete-v1 and composed-foundation matrices, and comma-grouped criterion cells in the observation, protocol and temporal matrices. A grouped cell can hide an unbacked criterion from a row-level count, so backing was verified per criterion token. | spec/complete-v1/tests.md; spec/protocol/tests.md; spec/temporal/tests.md |
