---
id: SR-16358
title: "Integrity review of STD-449 timed source identity refusals"
type: SpecReview
analysis: integrity
scope: "QSpec PR #431: FR-322, FR-415, FR-272, TC-367 and timed-model vectors TM-23..TM-29"
review_set: subset
---

## Summary

The reader rules agree across FR-322, FR-415, the diagnostic cause catalog/schema and the vector table. Each invalid dimension class is tested for both dense and tick sources, and the exact source-unit and clock-place binding pointers are stated in both criteria and vectors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found; the two refusal causes are paired to their required diagnostic codes and published vector loci. | FR-322-AC-126..127; FR-415-AC-18; FR-272; TM-23..TM-29 |

## Scope and method

Checked cross-document cause/code consistency, vector coverage, criterion testability, and the rule that the root names the nominal base Time node without name or declaration-order inference.
