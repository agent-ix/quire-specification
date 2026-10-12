---
id: SR-7838
title: "Base spec review of QSL-729 native allocation diagnostics"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@00ab825082ca541527beaf02dfc35aba72a92680; proposals/quire-v1/definitions/native-diagnostics.md resource_exhausted row; QSL FR-082-AC-10/11, FR-259-AC-5, TC-914 steps 2/4/5"
review_set: base
---

## Summary

The changed `resource_exhausted` row preserves the distinction between measured allocator denial, reservation-size overflow and insufficient next charge, including the native requested unit required by FR-082 and the bytes-only `quire-canonical` allocation required by FR-259-AC-5. One low-severity wording defect remains: the reservation-overflow context is repeated at the end of the row, creating redundant wording that can drift from the preceding definition.

## Findings

| ID      | Severity | Summary                                                     | Refs |
| ------- | -------- | ----------------------------------------------------------- | ---- |
| FND-001 | low      | Reservation overflow semantics are stated twice in the row | proposals/quire-v1/definitions/native-diagnostics.md:82 |
