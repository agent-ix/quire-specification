---
id: SR-602
title: "Scope boundary review of the 176-row complete V1 capability set"
type: SpecReview
analysis: scope-boundary
scope: "docs/v1-capability-inventory.md against the owning requirements named in TM-009"
review_set: subset
evaluated_revision: "c9c5e16 on task/63-full-v1-specification"
review_date: "2026-09-17"
---

## Summary

Each of the 176 atomic capabilities was judged against the tree as it stands,
not against the Plan-008 baseline reading. Every row has an owning requirement
with acceptance criteria. Three inventory observations are recorded for the
scope owner; the inventory is unchanged here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-631 | medium | Open: FR-049 is the owning requirement for ten inventory rows, FR-019 for eight and FR-048 for six. A requirement owning that many independently observable obligations is worth an atomicity reading by the scope owner. | docs/v1-capability-inventory.md; FR-049; FR-019; FR-048 |
| FND-632 | low | Resolved: three rows previously reported as unspecified are specified. V1-TYPE-027, V1-RUN-013 and V1-TOOL-004 each have an owning requirement; the first two lacked a criterion and now have one, and FR-303 states navigation and completion outputs with criteria. | FR-151; FR-181; FR-303 |
| FND-633 | low | Resolved: the inventory Spec column reads C for all 176 rows and that reading now matches the tree. | docs/v1-capability-inventory.md; TM-009 |
