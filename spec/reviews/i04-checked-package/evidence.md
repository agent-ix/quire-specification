---
id: SR-584
title: "Evidence review of I04 CheckedPackage V2"
type: SpecReview
analysis: evidence
scope: "FR-322, TC-217, checked-package-v2"
review_set: all
---
## Summary

The new schema/vector controls are Rust tests and the identity control uses the
pinned shared JCS implementation. The real reader still needs boundary and
duplicate-key evidence in Contract IR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-6 requires real-reader exact/one-over resource controls; QSpec vectors specify the expectation but cannot discharge consumer behavior. | FR-322-AC-6; TC-217 |
