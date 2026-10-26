---
id: SR-85903
title: "Integrity review of FR-451 population target identity"
type: SpecReview
analysis: integrity
scope: "FR-451-AC-8, TC-412 AN-14..AN-16, FR-322 P7/P8"
review_set: subset
review_date: "2026-10-25"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-451, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-322, type: reviews }
---

## Summary

The criterion, two positive vectors, and wrong-kind refusal are aligned. The
vectors reuse the published FR-322 owner preimages; TC-412 specifies how each
target is placed in the relation and what the enclosing relation depends on.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No integrity issue found: AC-8 has concrete, distinct P7/P8 cases and AN-14..AN-16 cover both admissions and the malformed target kind. | FR-451-AC-8; TC-412 AN-14..AN-16; FR-322 P7/P8 |
