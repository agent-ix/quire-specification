---
id: SR-14714
title: "EARS analysis of STD-236 (QSpec PR #341)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@59636cbbbf03420bfe01001bd43e5744562e8776 (diff against origin/main 97bfd65); spec/functional/source-package/FR-133-extend-language-uniformly.md, spec/functional/source-package/FR-467-define-extension-definition-members.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-469-checked-package-extension-nodes.md, spec/spec.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-951-checked-package-extension-nodes.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-469
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-951
    type: reviews
---
# SR-14714: EARS analysis of STD-236 (QSpec PR #341)

## Summary

Ticket: STD-236. PR: quire-specification#341, head 59636cb.

FR-469's statement uses the event pattern ("When a ... graph carries a declaration form of a selected extension"), names its subjects, and its ACs are concrete, with codes, causes and pointers. The FR-322, FR-133 and FR-467 edits are dependency and description lines with no SHALL change. There is no status, changelog or review wording in the normative text.

Units examined:

- `FR-469-AC-1` (examined)
- `FR-469-AC-2` (examined)
- `FR-469-AC-3` (examined)
- `FR-469-AC-4` (examined)
- `FR-469-AC-5` (examined)
- `FR-469-AC-6` (examined)
- `FR-469-AC-7` (examined)
- `FR-469` (examined)
- `FR-322-AC-13` (examined)
- `TC-951` (examined)
- `TC-233` (examined)
- `FR-133-AC-3` (context_only)
- `FR-467-AC-1` (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Minor: the FR-469 statement is compound, with two subjects and two responses in one sentence (the producer SHALL encode, and the reader SHALL refuse). Split it into one producer statement and one reader statement. | spec/objects/interfaces/FR-469-checked-package-extension-nodes.md:26-31 |

## Verdict

One minor EARS finding (compound statement).
