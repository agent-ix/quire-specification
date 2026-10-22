---
id: SR-14715
title: "gap analysis of STD-236 new and changed ACs against TC rows (QSpec PR #341)"
type: SpecReview
analysis: gap-analysis
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
# SR-14715: gap analysis of STD-236 new and changed ACs against TC rows (QSpec PR #341)

## Summary

Ticket: STD-236. PR: quire-specification#341, head 59636cb.

Plan completion: not assessed. Manual mapping. FR-469-AC-1 to X01; AC-2 to X02 to X04; AC-3 to X05 and X06; AC-4 to X07 to X09; AC-5 to X10 to X12; AC-6 to X13 to X15; AC-7 to X16. FR-322-AC-13 (changed, 13 to 14 tags) is verified by the existing all-families test, which still requires 13 tags. Every new AC has a row, and each AC clause has a vector: the pointers, the clause order (`subject` 0, `days` 1) and the selection-before-held order of X06 agree with the reader joins.

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
| FND-001 | high | Major: every FR-469 AC is verified only by TC-951, and no TC-951 vector can run on the merged tree. Its preconditions need `proposals/checked-package-v2/fixtures/positive-extension-nodes.json` and both V2 schemas to define the `extension` tag, `ExtensionFormBody` and `ExtensionFormNode`, and none of these exist on the PR head. `tests/checked_package_v2.rs` has no TC-951 test. AC-7 ("both V2 schemas admit it without change") is false against the current schemas. This is the test-side face of SR-14713 FND-001, and the fix is the same. | spec/test-cases/TC-951-checked-package-extension-nodes.md:22-33 |
| FND-002 | low | Minor: TC-951 X12 needs an extension node of node kind `quota`, but only X16 introduces a held, selected `quota` extension, and X12 does not say which definition carries the new `policy` production (adding it to `retention` changes that definition's identity, and so every X01 key). Name the definition that declares `policy`, and give X12 its own `quota` setup or point at X16. | spec/test-cases/TC-951-checked-package-extension-nodes.md:56 |

## Verdict

Rows exist for every AC, but none is runnable yet (major FND-001); one minor setup gap (FND-002).
