---
id: SR-4980
title: "Evidence review of QSpec PR #206 (QSL-692)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-specification@f5cb390c0904b9a556d31a90e07720aa61257932; FR-322-checked-package-artifact.md:FR-322-AC-62"
review_set: subset
---

## Summary

`quoin advise` recommends Test evidence for FR-322-AC-62 (unit testing and BDD/spec-by-example); the authored Test class is not a mismatch. The criterion's retained-key claim is not directly evidenced for all new derived shapes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-62 test binding checks preimage hashes and schema shape, while the separate reader mutation test rekeys invalid semantic shapes. Neither reader test covers stale-key rejection for `model_population` and `compound_unit`, despite the AC saying retained original keys exercise stale-key detection for each. Add those reader cases or narrow the claim. | `FR-322-checked-package-artifact.md:1229`; `tests/checked_package_v2.rs:10769-10850`; `tests/checked_package_v2.rs:10655-10759` |
