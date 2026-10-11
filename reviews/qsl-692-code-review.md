---
id: SR-4976
title: "Code review of QSpec PR #206 (QSL-692)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@f5cb390c0904b9a556d31a90e07720aa61257932; tests/checked_package_v2.rs; proposals/checked-package-v2; FR-322-AC-62"
review_set: subset
---

## Summary

Static code and Rust-test review at `f5cb390c0904b9a556d31a90e07720aa61257932`; no build or test suite was run. The qualification tests exercise admission and rekeyed semantic refusals, but do not exercise retained-key rejection for the newly added derived forms.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new mutation test proves the three preimages hash differently after mutation, but does not send each mutated node with its original key through `read_package_semantics`; existing full-package stale-key vectors cover reference/set/range and other forms, not `model_population` or `compound_unit`. Add retained-key reader cases for these forms (and keep the rekeyed semantic-refusal cases) so AC-62's stale-key statement is directly verified. | `tests/checked_package_v2.rs:10769`; `tests/checked_package_v2.rs:10655`; `spec/objects/interfaces/FR-322-checked-package-artifact.md:1229` |
