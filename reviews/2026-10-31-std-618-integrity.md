---
id: SR-2026103007
title: "Integrity review — STD-618 FR-322 selected operation witness"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; FR-322 AC-25/30/288/290/292, checked-package-v2 schema and checked-term-data vectors"
review_set: all
---

## Summary

The schema and vectors encode the two-part operation witness and include a schema-invalid invented path plus a semantic non-member refusal. The revised AC-290 is inconsistent with the earlier member-resolution rules about relationship declaring nodes.

## Verdict

**FAIL** — conflicting permitted declaring types leave the carrier contract with two interpretations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-290 allows relation declaring nodes that AC-25 and AC-30 reject | FR-322-AC-290 |
