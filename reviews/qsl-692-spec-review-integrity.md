---
id: SR-4978
title: "Integrity review of QSpec PR #206 (QSL-692)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@f5cb390c0904b9a556d31a90e07720aa61257932; FR-322-checked-package-artifact.md; checked-package-v2/schema.json"
review_set: subset
---

## Summary

The changed rows and schema additions are structurally coherent with the existing checked-package node vocabulary. AC-62's stale-key evidence wording is broader than the tests linked to it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-62 states that retaining each changed preimage's original key exercises stale-key detection, but its linked mutation test performs no reader admission attempt with that retained key; the older generic stale-key test has no `model_population` or `compound_unit` case. Align the criterion and test evidence. | `FR-322-checked-package-artifact.md:1229`; `tests/checked_package_v2.rs:10769-10850`; `tests/checked_package_v2.rs:10655-10759` |
