---
id: SR-11764
title: "Failure-domain review of STD-616 / PR #557"
type: SpecReview
analysis: failure-domain
scope: "spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/functional/protocol/FR-430-apply-protocol-fairness.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/temporal-fairness-target-vectors.json; tests/checked_package_v2.rs"
review_set: all
---

## Summary

The typed identities reject absent, foreign-package, and wrong-kind parallel targets. A same-package node owned by a different protocol clause is not represented in the reader cases, leaving “foreign” ambiguous between package-foreign and protocol-foreign.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-370 does not define or exercise refusal when an existing node from another protocol clause in the same package is supplied as a root, attempt, compensation, or parallel target. | FR-370-AC-15 |
