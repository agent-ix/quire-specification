---
id: SR-871
title: "EARS review — STD-116/117/119/121 abstraction relation and refinement gates"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@896aa54d5cc4fdc17325c918959fe130294adcd4; PR #171; FR-353 AC-1, FR-450, FR-451, FR-452, FR-453 descriptions and ACs"
review_set: subset
---
# EARS review — STD-116/117/119/121 abstraction relation and refinement gates

## Summary

Ticket: STD-116. EARS conformance of each new or amended requirement
statement. FR-450, FR-451 and FR-452 descriptions are event-driven
(`When ... SHALL`) with one subject each. Ticket ids appear only in
References.

## Verdict

**PASS with one low finding.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-453's Description joins two SHALL responses under one trigger and omits the per-layer witness behaviour that FR-453-AC-5 tests, so the statement does not cover all of the FR's ACs. | spec/functional/tooling/FR-453-preserve-parent-profile-refusals-under-layering.md:24-29 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | bc860c9 |
