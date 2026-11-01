---
id: SR-2026103008
title: "Failure-domain review — STD-618 FR-322 selected operation witness"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; FR-322 dispatch table prose and AC-290/292; checked-package-v2 schema and vectors"
review_set: all
---

## Summary

The witness has an explicit identity pair, forbids inference from names or paths, and distinguishes malformed wire from a structurally valid non-member name. The unresolved type boundary is the relationship declaring form, which conflicts with earlier FR-322 refusal behavior.

## Verdict

**FAIL** — the same declaring identity can be prohibited by AC-30 and allowed by AC-290.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The declaring-type failure domain is contradictory for relationship nodes | FR-322-AC-290 |
