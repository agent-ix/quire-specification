---
id: SR-14614
title: 'EARS analysis of PR #348: FR-322 dependencies rule (STD-93)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@c2a9875; git diff origin/main(97bfd65)...c2a9875 (PR #348, branch std-fr322-std-93): FR-322 `dependencies` property, FR-322-AC-96, TC-233 DP-01..DP-07, native-diagnostics.md step 3, checked-package-v2 README, positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json; ticket STD-93'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: STD-93. PR: quire-specification#348, head c2a9875. The PR adds one normative statement to the FR-322 `dependencies` property ("A producer SHALL write, and a reader SHALL require, exactly the nodes these rules name for every node") and FR-322-AC-96. The check covers grammar fit, a single response per actor, and behaviour-only wording.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 dependencies (definition and reader rule) | spec/objects/interfaces/FR-322-checked-package-artifact.md:491 | examined |
| FR-322 dependencies rules 1-4 | spec/objects/interfaces/FR-322-checked-package-artifact.md:498 | examined |
| FR-322 dependencies refusal | spec/objects/interfaces/FR-322-checked-package-artifact.md:542 | examined |
| FR-322-AC-96 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1895 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The statement is a ubiquitous EARS requirement with two actors, each with one response, both bound to the same enumerated rules. The refusal sentence gives the response for the unwanted case (a differing list refuses `invalid_semantic_graph` at `/dependencies`). The new text has no changelog, status or review language. The cycle-edge ambiguity is recorded in SR-14613.
