---
id: SR-12007
title: "Scope-boundary review of QSL-585 task automaton specification"
type: SpecReview
analysis: scope-boundary
scope: "FR-181, FR-331, FR-364, FR-415, FR-418, FR-419"
review_set: all
---

## Summary

QSpec owns the wire contracts, normative scheduler/product semantics, and replay obligations. The QSL implementation owns executable task-product search, certificate verification, and behavioral trace evidence; the revised criteria do not claim QSpec schema tests verify those behaviors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No remaining responsibility-allocation ambiguity found in the changed scope. | - |
