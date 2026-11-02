---
id: SR-58511
title: "Base re-review of QSL-585 task-search and no-release closure contract"
type: SpecReview
analysis: base
scope: "FR-331, FR-368, FR-415, FR-418, FR-419; backend-provider-v1 schemas and vectors"
review_set: all
---

## Summary

The initial pass against `origin/main` found the empty `queue_bounds` schema gap below. In disposition round 2 at `4cfad11c`, the no-release schema condition added `minItems: 1` and malformed vector ZC-29; FND-001 remains fixed at final head `c9e67ada71508a19dce74088048ed130410957fd`, with no base finding open.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The `closed-no-release-product/v1` schema branch permits an empty `queue_bounds` array, while FR-418-AC-14 requires checked task demand intervals and the normative text requires verifying both endpoints against each checked task declaration. Require at least one row or state and test the reader's exact completeness rule. | FR-418-AC-14; `TaskProductCertificate` allOf in proposals/backend-provider-v1/schema.json; ZC-25 | correct-requirement-no-evidence |

## Dispositions

Round 2, at 4cfad11c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The schema now rejects an empty no-release demand-row list, and ZC-29 tests that rejection. |

```yaml
dispositions:
  - fnd: FND-001
    outcome: fixed
    after_excerpt: |-
      "queue_bounds": {"minItems": 1}
```
