---
id: SR-4977
title: "Base spec review of QSpec PR #206 (QSL-692)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f5cb390c0904b9a556d31a90e07720aa61257932; FR-322-checked-package-artifact.md; checked-package-v2 proposal"
review_set: subset
---

## Summary

Reviewed the changed derived-type rules and their accepted/refusal examples against the current spec and test evidence. One AC overstates the stale-key evidence for the newly introduced derived forms.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-62 says retaining each original key exercises stale-key detection, but the new key mutation test only hashes the original and mutated preimages and checks they differ; it never submits a retained-key candidate to the reader. Existing stale-key coverage does not include `model_population` or `compound_unit`. Add package-reader stale-key cases for those forms or narrow the AC's claim. | `FR-322-checked-package-artifact.md:1229`; `tests/checked_package_v2.rs:10769-10850`; `tests/checked_package_v2.rs:10655-10759` |
