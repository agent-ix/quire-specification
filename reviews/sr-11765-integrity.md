---
id: SR-11765
title: "Integrity review of STD-616 / PR #557"
type: SpecReview
analysis: integrity
scope: "spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/functional/protocol/FR-430-apply-protocol-fairness.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/temporal-fairness-target-vectors.json; tests/checked_package_v2.rs"
review_set: all
---

## Summary

The new target union is closed in schema and positive/malformed vectors cover the discriminants. A duplicated normative definition in FR-430 and FR-370 creates a consistency hazard, and AC-15 verification does not assert the promised diagnostic locus.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The same authored protocol target shapes are independently specified in FR-430 and FR-370, with no single source of truth. | FR-370, FR-430 |
| FND-002 | medium | The AC-15 reader vector test checks only nonempty diagnostic loci rather than the exact target identity or branch-label locus. | FR-370-AC-15 |
