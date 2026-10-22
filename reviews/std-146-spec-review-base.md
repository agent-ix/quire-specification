---
id: SR-13628
title: "Base review of STD-146 FR-467 extension definition members (PR #306)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-467, TC-949, FR-133, spec/spec.md, docs/v1-capability-inventory.md (PR #306 head 7dd14f3)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-467
    type: references
  - target: ix://agent-ix/quire-specification/TC-949
    type: references
  - target: ix://agent-ix/quire-specification/FR-133
    type: references
---

# Base review of STD-146 FR-467 extension definition members (PR #306)

## Summary

Ticket: STD-146 (blocks QSL-604). Checked against FR-133, FR-322, the shared grammar, QSL FR-354 (qsl-lane2 2e6c836) and QSL FR-111. The gap is real: main has no grammar-schema or typed-node-schema text, and QSL has no extension-schema code yet. FR-467, TC-949 and V1-SRC-016 are new ids. quire validate: 134 failures on main and on the PR head, none in a touched file. Base checklist: ids, links, index row and edge, US-010 link and AC-to-TC coverage are correct, and the text has no changelog, status or owner wording.

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
- `FR-133` (examined): spec/functional/source-package/FR-133-extend-language-uniformly.md
- `V1-SRC-016` (examined): docs/v1-capability-inventory.md
- `QSL FR-354` (context_only): agent-ix/quire-spec-language spec/functional/FR-354-admit-extensions-from-declarative-grammar-and-typed-node-schemas.md
- `FR-322` (context_only): spec/objects/interfaces/FR-322-checked-package-artifact.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The grammar schema is defined only by constraints ("productions over the declarative token vocabulary", "begins with one keyword", "clauses"). FR-467 never says what a production consists of: whether a declared name follows the keyword, how the clause list is delimited and separated, or how a clause binds a name to a `type-ref`. It also gives no encoding for either member in a definition. QSL FR-354-AC-1 expects `retention Audit { subject: "audit", days: 30 }`, a surface FR-467 does not define, so two readers of FR-467 would accept different source. Define the production structure and clause form, and the member encoding. | spec/functional/source-package/FR-467-define-extension-definition-members.md:36-42 |
| FND-002 | medium | The validation list names causes (`missing-member`, `conflicting-definition`, `definition-cycle`) but no FR-271 code. QSL FR-354's refusal table uses `invalid_package`, as do this repo's other uses of these causes (TC-937, TC-227, TC-233). Name the code in FR-467's Behavior and in its ACs. | spec/functional/source-package/FR-467-define-extension-definition-members.md:46-52 |

## Verdict

Not mergeable. FR-467 says what the two members must satisfy but not what a production is or how the members are written, so QSL-604 cannot implement a reader from it (FND-001). It also names no refusal code (FND-002). The causes match FR-354, and the ACs are testable once the member shape exists.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-949 Expected Results still says steps 3 and 4 refuse "naming both definitions". AC-3 and AC-4 now name the extension and the core grammar in the core-keyword and core-node-kind cases. Align the expected result. | spec/test-cases/TC-949-extension-definition-members.md:30 |
| FND-004 | low | A node entry whose `keyword` matches no production, and two node entries with one `keyword` but different node kinds, fit neither item 1 nor item 2. Matching entries to productions by `K` is then undefined. Add both to `missing-member` or `conflicting-definition`. | spec/functional/source-package/FR-467-define-extension-definition-members.md:58 |
| FND-005 | low | AC-9 says three sources "are not" forms, and TC-949 step 9 says they are refused, but no code or cause is named. A clause out of production order and an unknown clause have no stated outcome either. QSL FR-354's table has no row for these. Name the refusal (for example `invalid_syntax`/`unexpected-token`) or point to the owning rule. TC-949 also says "Every refusal is `invalid_package`", which would wrongly cover these source refusals. | spec/functional/source-package/FR-467-define-extension-definition-members.md:76 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
