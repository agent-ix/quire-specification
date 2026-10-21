---
id: SR-13630
title: "Integrity review of STD-146 FR-467 (PR #306)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; FR-467, TC-949, FR-133 (PR #306 head 7dd14f3)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-467
    type: references
  - target: ix://agent-ix/quire-specification/TC-949
    type: references
---

# Integrity review of STD-146 FR-467 (PR #306)

## Summary

Ticket: STD-146. Checked FR-467 Behavior, ACs and TC-949 against each other and against FR-133 and QSL FR-354.

Units examined:

- `FR-467` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-1` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-2` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-3` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-4` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-5` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-6` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `FR-467-AC-7` (examined): spec/functional/source-package/FR-467-define-extension-definition-members.md
- `TC-949` (examined): spec/test-cases/TC-949-extension-definition-members.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-2 refuses `missing-member` for "a subnode type for a clause" that is missing. Behavior item 1 (line 48) lists only a missing grammar schema, a missing typed-node schema, an uncovered production and a production with no node kind. QSL FR-354's table has no such row either. Add the missing subnode type to item 1, or drop it from AC-2 and TC-949 step 2. | spec/functional/source-package/FR-467-define-extension-definition-members.md:59 |
| FND-002 | low | For the core-keyword case (`function`) only one extension definition is involved, so "naming both definitions" has no second definition to name. QSL FR-354-AC-3 says "naming the extension, the core grammar and `function`". Say what the refusal names in the core case. | spec/functional/source-package/FR-467-define-extension-definition-members.md:60 |

## Verdict

Not mergeable until FND-001 is fixed: AC-2 refuses a case that Behavior does not list. FND-002 is a wording nit. TC-949 covers AC-1..7 step for step. FR-133's new sentence and Dependencies bullet agree with FR-467.
