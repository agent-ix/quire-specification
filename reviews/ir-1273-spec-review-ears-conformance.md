---
id: SR-605002
title: 'EARS analysis of PR #605 new normative text: FR-322 Source-byte integrity (IR-1273)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@5f483b1; git diff origin/main(aaf8c65)...5f483b1 (PR #605): FR-322 "Source-byte integrity" and FR-322-AC-319..327, FR-272-AC-26; tickets IR-1273, QSL-1501'
review_set: subset
date: '2026-11-02'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: IR-1273, QSL-1501. The new section follows FR-322's existing declarative prose convention (no "shall"), the same form as neighboring sections, so no EARS pattern is matched by it or by the surrounding text. The ACs are single outcome-oriented statements except as noted in SR-605001 FND-004.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Source-byte integrity | spec/objects/interfaces/FR-322-checked-package-artifact.md:3191 | examined |
| FR-322-AC-319..327 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4201 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | New normative text uses declarative prose ("A reference fails when...", "The failure refuses...") with no subject-shall, so it is unclassifiable under EARS; consistent with the file's convention | FR-322:3191 |

## Verdict

One low, convention-consistent finding; no change required unless the owner wants EARS form for new sections.
