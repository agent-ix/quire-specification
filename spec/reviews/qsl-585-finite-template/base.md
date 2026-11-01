---
id: SR-22501
title: "Base review of QSL-585 finite task certificate identities"
type: SpecReview
analysis: base
scope: "FR-418, FR-419, TC-371, backend-provider-v1 schema and vectors"
review_set: all
---

## Summary

Reviewed the exact task-step identity, the new finite certificate template,
the certificate wire schema, and the TC-371 positive and malformed cases.
FR-418 keeps source-operation edges distinct, FR-419 retains exact identities
for replay, and the finite reader contract and vectors cover growing periodic
occurrences, completed-job renaming, live-job preservation, and bad rank/slot
references. No issue found in the changed contract. Target Quire validation
reports 3/3 changed Markdown files grammar-clean and structurally valid; its
`verifies` relationship warnings on FR-418 and FR-419 predate this change and
are unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found in the changed requirements, wire schema, vectors, or trace links. | FR-418, FR-419, TC-371 |
