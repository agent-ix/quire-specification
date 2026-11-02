---
id: SR-58512
title: "Evidence re-review of QSL-585 task-search and no-release closure contract"
type: SpecReview
analysis: evidence
scope: "FR-415-AC-26, FR-418-AC-14, FR-419-AC-6; backend-provider-v1 vectors and reader evidence"
review_set: all
---

## Summary

`quoin advise` reports no method mismatch for FR-415-AC-26, FR-418-AC-14, or FR-419-AC-6. The work-budget criterion is now tagged to TC-431 and the derived-item whitelist is tagged. The unresolved-dynamics malformed vector exercises the required dimension rule. At `c9e67ada71508a19dce74088048ed130410957fd`, ZC-30 now carries the complete TC-372/CtlFp identity and all three task rows; ZC-28 changes only the edge list to make the reachable `Cell::set` release binding produce the expected reader refusal. FND-001 is fixed in disposition round 3.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-418-AC-14 requires the reader to recompute all source successors and reject `closed-no-release-product/v1` when a checked release binding is reachable. ZC-25 exercises only canonical wire validity and ZC-26 only wire rejection for a positive cap; `task_reader_vectors` adds no case for a reachable release under this method. Add a reader case with the expected rejection locus/rule and bind it to a strict-reader test. | FR-418-AC-14; proposals/backend-provider-v1/zone-certificate-vectors.json (`ZC-25`, `ZC-26`, `task_reader_vectors`); tests/zone_certificate_wire.rs | correct-requirement-no-evidence |

## Dispositions

Round 2, at 4cfad11c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | ZC-28 exists, but it is based on ZC-30 whose FR-482 claim NodeRef body and task_product row describe only Ctl.a; TC-372/CtlFp contains all three tasks a, b and c. The strict reader can reject the identity or incomplete task-product data before testing the reachable Cell::set release. Align the full identity and rows with the checked CtlFp fixture so successor-missing at node 0 is the distinguishing result. |

```yaml
dispositions:
  - fnd: FND-001
    outcome: still-open
    reason: "ZC-30 is not otherwise a matching CtlFp certificate: its claim identity path and task_product contain only task a, whereas TC-372/CtlFp contains a, b, and c. The reader may reject that mismatch before the reachable-release check, so ZC-28 does not isolate the expected successor-missing outcome."
```


Round 3, at c9e67ada71508a19dce74088048ed130410957fd.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | ZC-30's identity path contains all three TC-372/CtlFp task aggregates (`t1`, `t2`, `t3`) with the exact period/deadline/WCET/priority values, and its three `queue_bounds` rows match those tasks. ZC-28 mutates only `/edges`, adding the reachable node-0 `Cell::set` self-edge; the expected strict-reader result is `certificate-rejected`/`successor-missing` at node 0. No earlier claim identity or task-product mismatch remains. |

```yaml
dispositions:
  - fnd: FND-001
    outcome: fixed
    after_excerpt: |-
      ZC-28: base ZC-30; one edge 0 --Cell::set--> 0; expected certificate-rejected/successor-missing at node 0. ZC-30 carries all three CtlFp task rows and matching queue bounds.
```
