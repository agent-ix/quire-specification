---
id: SR-12005
title: "Evidence review of QSL-585 task automaton specification"
type: SpecReview
analysis: evidence
scope: "FR-181, FR-331, FR-364, FR-415, FR-418, FR-419"
review_set: all
---

## Summary

The FR-364 queue-growth wire criterion and FR-418 finite-gate wire criterion are backed by executable QSpec tests. Runtime task scheduling, replay, and certificate-closure semantics remain untagged here and belong to the downstream QSL implementation suites.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-419-AC-1 through FR-419-AC-10 remain untagged by executable QSpec tests; do not bind schema-only tests to those runtime assertions. | FR-419-AC-1, FR-419-AC-2, FR-419-AC-3, FR-419-AC-4, FR-419-AC-5, FR-419-AC-6, FR-419-AC-7, FR-419-AC-8, FR-419-AC-9, FR-419-AC-10 |
