---
id: SR-589
title: "Failure-domain review of complete scalar contracts"
type: SpecReview
analysis: failure-domain
scope: "scalar refusal, undefined, incomplete, identity and topology behavior in QSpec #68"
review_set: all
evaluated_revision: "cd4c0a0 plus QSpec #68 working-tree remediation"
review_date: "2026-09-15"
relationships:
  - target: ix://agent-ix/quire-specification/issues/68
    type: reviews
---

## Summary

PASS after remediation. Type-check refusal, evaluator undefined/refused/
incomplete outcomes and I13 unsupported/requires-bound dispositions are staged
separately. No scalar failure path publishes a substitute, partial value or
Boolean false.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-5891 | high | Resolved: division by zero is uniformly undefined, while malformed profile selection and finite-domain misuse are refused at their owning stages. | FR-140; FR-147; TC-185; TC-192 |
| FND-5892 | high | Resolved: nested operand-construction non-results occur before an outer value exists; traversal exhaustion never aliases false. | FR-149; TC-194 |
| FND-5893 | medium | Resolved: stale nominal keys, invalid owner joins, noncanonical rationals/order and invalid unit topology refuse before exposing semantic nodes or quantities. F4 re-review: an owner absent from an otherwise consistent lock and identity preimage refuses through the owner join alone. | FR-141; FR-142; FR-322; TC-187; TC-217 |
| FND-5894 | medium | Resolved: each one-less vector denies an exact named charge and exposes no implementation-relative partial state. | value-accounting.md; TC-185–194 |
| FND-5895 | high | Resolved: a valid nominal preimage cannot authorize a contradictory enum literal, semantic type or dependency graph; package-level mutations recompute the outer package identity and still refuse the invalid cross-field join. | checked-package-v2/README.md; FR-322; checked_package_v2.rs |
| FND-5896 | high | Resolved: version confusion cannot admit a partially understood package. Frozen V1 rejects V2/version-only members and V2 refuses V1 rather than fabricating missing nominal evidence. | FR-322-AC-11; TC-217 |
| FND-5897 | high | Resolved: migration failure is staged before V2 admission. Missing, stale, ambiguous, incompatible and byte-only reconstruction cannot expose a target package or correspondence, and V1 graph bytes cannot fill V2 nominal evidence. F2 re-review: each failure maps to exactly one closed code in a fixed evaluation order with a typed input subject. | NFR-074; FR-322-AC-12; interface-004; TC-217 |
