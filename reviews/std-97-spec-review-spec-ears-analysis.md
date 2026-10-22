---
id: SR-14620
title: 'EARS analysis of PR #349: FR-322 structural node preimage and FR-142 compound-unit identity (STD-97)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@c6bd361; git diff origin/main(97bfd65)...c6bd361 (PR #349, branch std-fr322-std-97): FR-322 value/parameter paragraph, new "Structural node preimage" section with P4 and U2, FR-322-AC-97, TC-233 SP-01..SP-05, FR-142 compound-unit identity paragraph, value-compound-unit.md; QSL FR-092 (quire-spec-language origin/main); ticket STD-97'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-142
  type: reviews
---

## Summary

Ticket: STD-97. PR: quire-specification#349, head c6bd361. The PR edits one FR-142 descriptive paragraph and adds FR-322 property and section text and FR-322-AC-97. No EARS requirement statement changes. The check covers single-meaning wording, testability of the AC, and behaviour-only language.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 value/parameter paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:255 | examined |
| FR-322 Structural node preimage (scope sentence) | spec/objects/interfaces/FR-322-checked-package-artifact.md:766 | examined |
| FR-322 compound-unit node vs value identity | spec/objects/interfaces/FR-322-checked-package-artifact.md:817 | examined |
| FR-322-AC-97 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1906 | examined |
| FR-142 compound-unit value identity paragraph | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:226 | examined |
| value-compound-unit.md identity note | proposals/quire-v1/definitions/value-compound-unit.md:5 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The new text states one structure each (parameter body, preimage members, identity separation) with no conditional left implicit. FR-322-AC-97 names exact keys and schema outcomes. None of the new text contains changelog, status or review language.
