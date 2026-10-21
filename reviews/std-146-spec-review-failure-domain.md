---
id: SR-13631
title: "Failure-domain review of STD-146 FR-467 (PR #306)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; FR-467, FR-322, proposals/quire-v1/shared-grammar.md (PR #306 head 7dd14f3)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-467
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Failure-domain review of STD-146 FR-467 (PR #306)

## Summary

Ticket: STD-146. Looked for identity collisions, unstated failure modes and edges in FR-467's validation.

Units examined:

- `FR-467` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-322` (context_only): spec/objects/interfaces/FR-322-checked-package-artifact.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Collisions inside one extension are not covered. `conflicting-definition` compares only against the core and "another selected extension". Two productions of one extension with the same leading keyword, or with the same node kind, match no refusal. Line 42's uniqueness sentence also compares only across the core and other extensions. Add the within-definition case to item 2 and an AC. | spec/functional/source-package/FR-467-define-extension-definition-members.md:42,49 |
| FND-002 | medium | "Core keyword" is undefined. In the shared grammar, keywords are selected by edition, words are either reserved (`ident` excludes them) or contextual (`operation`, `receiver`, `empty`, `refused` are keywords only in their own productions), and some declarations begin with a `qualified-name`. Say which set a leading keyword must avoid: which edition, and whether contextual keywords and reserved words count. Also say which positions count as "the first token of a declaration" (top level only, or member declarations inside a model or record body). | spec/functional/source-package/FR-467-define-extension-definition-members.md:38,49 |
| FND-003 | medium | FR-467 gives each production a "node kind" that must differ from "the core node kinds" of FR-322. But FR-322's `quire.checked-semantic-graph/v2` is a closed tagged union with a fixed tag list and a closed body grammar. It has no carrier for an extension node and does not say whether a node kind is a tag or something finer. QSL FR-354-AC-1 needs the checked package to hold that node. Either define the carrier, or defer it to a named follow-up ticket. | spec/functional/source-package/FR-467-define-extension-definition-members.md:42 |

## Verdict

Two collision and edge gaps that QSL-604 would have to guess at (FND-001, FND-002). One downstream carrier gap that can be deferred to its own ticket (FND-003). The cycle refusal and the cause order are sound.
