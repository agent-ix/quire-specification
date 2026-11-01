---
id: SR-2026103004
title: "Code review — STD-618 FR-322 selected operation witness"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; proposals/checked-package-v2/{schema.json,checked-term-data-vectors.json,README.md}, spec/objects/interfaces/FR-322-checked-package-artifact.md, tests/checked_package_v2.rs"
review_set: all
---

## Summary

Reviewed the changed schema, vectors, FR-322 contract and Rust qualification test. The new witness addresses the invented-node defect, but the test code narrows the spec's accepted declaring node and does not check the exact refusal code for the new semantic negative vector.

## Verdict

**CONDITIONAL** — fix the two test-contract gaps before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The positive-vector assertion rejects a relation declaring NodeRef allowed by FR-322 | tests/checked_package_v2.rs:19068 |
| FND-002 | low | The DT-R6 test accepts any string as its expected refusal code | tests/checked_package_v2.rs:19310 |
