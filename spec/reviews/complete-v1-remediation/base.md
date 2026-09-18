---
id: SR-599
title: "Base review of complete V1 after cross-row contradiction remediation"
type: SpecReview
analysis: base
scope: "the 176-row complete V1 capability set, its owning requirements, TM-009 and the family test matrices"
review_set: subset
evaluated_revision: "3dcf31b on task/63-full-v1-specification"
review_date: "2026-09-17"
---

## Summary

PASS after remediation. Every one of the 176 atomic capabilities has one owning
requirement with acceptance criteria, and every one of the 1,263 acceptance
criteria in the tree is represented in a Test Matrix. ID formats, AC id
formats, verification methods and internal link targets check clean. Five
cross-repository boundary conflicts are posed as ADR-001 through ADR-005, and
the protocol bounded-control contradiction as ADR-006, rather than decided
here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-601 | high | Resolved: four clusters of acceptance criteria pinned a narrower behavior than the complete V1 requirement for the same capability. Each is now keyed on the `quire.*` profile or law identity that selects it, with a complete-V1 counterpart criterion. | FR-003; FR-007; FR-008; FR-044; FR-091; FR-092 |
| FND-605 | high | Open: the protocol bounded-control cluster has no identity to key on. AD-003 supplies one protocol admission identity, `quire.protocol.complete/v1`, so FR-052-AC-3, FR-049-AC-3, FR-220-AC-4 and FR-228-AC-2 bind the same loop opposite to FR-171-AC-2. Posed as ADR-006 and left stated on both sides rather than scoped on a name no hierarchy supplies. | ADR-006; FR-049; FR-052; FR-171; FR-220; FR-228 |
| FND-606 | medium | Resolved: FR-044 stated without condition that no float is permitted, while FR-148 states IEEE 754-2019 binary32 and binary64 arithmetic and FR-019-AC-1 admits explicit IEEE float in the complete-V1 facets. FR-044 now states the admitted float contract per selection, backed by FR-044-AC-9. | FR-019; FR-044; FR-148 |
| FND-602 | medium | Resolved: five capabilities were described in requirement prose with no acceptance criterion exercising them. Eight criteria now state them, and the branch adds sixteen criteria in total. | FR-134; FR-151; FR-181; FR-192; FR-306 |
| FND-603 | medium | Resolved: five acceptance criteria carried no Test Matrix row and three SpecReview documents shared ids with earlier reviews. | TM-009; FR-142; FR-148; SR-596; SR-597; SR-598 |
| FND-604 | low | Resolved: the Plan-008 Task-003 coverage audit read a tree the `/specify` cycle then replaced, so its per-row Coverage and Delta columns no longer held. It is deleted; its classification key, the delivered 94/39/23/19/1 totals and the V1-TYPE-024 reconciliation are now stated in TM-009, and the four documents that cited it name TM-009 or the inventory instead. | TM-009; docs/v1-capability-inventory.md |
